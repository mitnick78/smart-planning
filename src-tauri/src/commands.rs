use crate::models::{Label, PlanningEntry, StatEntry};
use crate::school_holiday_repository;
use crate::validation;
use crate::{label_repository, planning_repository};
use chrono::Datelike;
use tauri::AppHandle;

// LABELS

#[tauri::command]
pub fn get_labels() -> Result<Vec<Label>, String> {
    label_repository::get_all().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_label(name: String, color_hex: String) -> Result<Label, String> {
    validation::validate_label_input(&name, &color_hex)?;
    let label = Label::new(&name, &color_hex);
    let id = label_repository::create(&label).map_err(|e| e.to_string())?;
    Ok(Label {
        id: Some(id),
        name: label.name,
        color_hex: label.color_hex,
        created_at: None,
    })
}

#[tauri::command]
pub fn update_label(id: i64, name: String, color_hex: String) -> Result<(), String> {
    validation::validate_label_input(&name, &color_hex)?;
    let label = Label {
        id: Some(id),
        name,
        color_hex,
        created_at: None,
    };
    let updated = label_repository::update(&label).map_err(|e| e.to_string())?;
    if updated == 0 {
        return Err(LABEL_NOT_FOUND.to_string());
    }
    Ok(())
}

#[tauri::command]
/// Supprime un label.
///
/// Si `delete_entries` est faux et que le label est encore utilisé, la
/// suppression est refusée : on ne laisse jamais d'affectations orphelines.
/// La clé étrangère `planning.label_id` garantit aussi ce refus en base.
pub fn delete_label(id: i64, delete_entries: bool) -> Result<(), String> {
    if !delete_entries {
        let used = planning_repository::count_by_label(id).map_err(|e| e.to_string())?;
        if used > 0 {
            return Err(format!(
                "Ce label est utilisé par {used} affectation(s). Choisissez \
                 « Supprimer le label et tous les jours associés », ou \
                 réaffectez ces jours avant de supprimer le label."
            ));
        }
    }
    label_repository::delete(id, delete_entries).map_err(|e| e.to_string())
}

const LABEL_NOT_FOUND: &str = "Label introuvable : il a peut-être été supprimé";

// PLANNING

#[tauri::command]
pub fn get_planning_by_month(year: i32, month: u32) -> Result<Vec<PlanningEntry>, String> {
    validation::validate_year_month(year, month)?;
    planning_repository::get_by_month(year, month).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn upsert_planning(
    date: String,
    period: String,
    label_id: i64,
    note: Option<String>,
) -> Result<i64, String> {
    validation::validate_date(&date)?;
    validation::validate_period(&period)?;
    // Message clair plutôt que l'erreur brute de clé étrangère de SQLite.
    if label_repository::get_by_id(label_id)
        .map_err(|e| e.to_string())?
        .is_none()
    {
        return Err(LABEL_NOT_FOUND.to_string());
    }
    let entry = PlanningEntry {
        id: None,
        date,
        period,
        label_id,
        note,
        label_name: None,
        label_color: None,
        created_at: None,
        updated_at: None,
    };
    planning_repository::upsert(&entry).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_planning(date: String, period: String) -> Result<bool, String> {
    validation::validate_date(&date)?;
    validation::validate_period(&period)?;
    planning_repository::delete_by_date_and_period(&date, &period).map_err(|e| e.to_string())
}

// STATISTICS

#[tauri::command]
pub fn get_stats_by_month(year: i32, month: u32) -> Result<Vec<StatEntry>, String> {
    validation::validate_year_month(year, month)?;
    planning_repository::get_stats_by_month(year, month).map_err(|e| e.to_string())
}

// SCHOOL BREAKS

#[derive(serde::Deserialize)]
struct ApiRecord {
    description: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
}

#[derive(serde::Deserialize)]
struct ApiResponse {
    results: Vec<ApiRecord>,
}

#[tauri::command]
pub async fn fetch_school_holidays(zone: String, year: i32) -> Result<usize, String> {
    validation::validate_zone(&zone)?;
    validation::validate_year(year)?;

    let school_years = vec![
        format!("{}-{}", year - 1, year),
        format!("{}-{}", year, year + 1),
    ];

    let zone_filter = format!("Zone {}", zone);
    let mut all_holidays: Vec<crate::models::SchoolHoliday> = Vec::new();

    let client = reqwest::Client::new();

    for school_year in &school_years {
        // Paramètres encodés par reqwest, jamais concaténés à la main.
        let filter = format!(
            "zones like \"{}\" and annee_scolaire=\"{}\"",
            zone_filter, school_year
        );
        let resp = client
            .get(HOLIDAYS_API_URL)
            .query(&[("where", filter.as_str()), ("limit", "100")])
            .send()
            .await
            .map_err(|e| format!("Erreur réseau : {}", e))?;

        let api: ApiResponse = resp
            .json()
            .await
            .map_err(|e| format!("Erreur JSON : {}", e))?;

        for record in api.results {
            let name = record
                .description
                .unwrap_or_else(|| "Vacances scolaires".to_string());
            let start = match record.start_date {
                Some(d) => d,
                None => continue,
            };
            let end = match record.end_date {
                Some(d) => d,
                None => continue,
            };

            let start_dt = chrono::DateTime::parse_from_rfc3339(&start)
                .map_err(|e| format!("Date invalide : {}", e))?;
            let end_dt = chrono::DateTime::parse_from_rfc3339(&end)
                .map_err(|e| format!("Date invalide : {}", e))?;

            // L'API stocke les dates en UTC à 22h/23h = minuit heure Paris.
            // +4h pour convertir correctement en date locale française.
            // Le end_date UTC correspond au début du lendemain → on soustrait 1 jour.
            let start_date = (start_dt + chrono::Duration::hours(4)).date_naive();
            let end_date =
                (end_dt + chrono::Duration::hours(4)).date_naive() - chrono::Duration::days(1);

            let mut current = start_date;
            while current <= end_date {
                if current.year() == year {
                    all_holidays.push(crate::models::SchoolHoliday {
                        date: current.format("%Y-%m-%d").to_string(),
                        zone: zone.clone(),
                        name: name.clone(),
                    });
                }
                let Some(next) = current.succ_opt() else {
                    break;
                };
                current = next;
            }
        }
    }

    let count = all_holidays.len();
    school_holiday_repository::upsert_many(&all_holidays).map_err(|e| e.to_string())?;

    Ok(count)
}

const HOLIDAYS_API_URL: &str =
    "https://data.education.gouv.fr/api/explore/v2.1/catalog/datasets/fr-en-calendrier-scolaire/records";

#[tauri::command]
pub fn get_school_holidays_by_month(
    year: i32,
    month: u32,
    zone: String,
) -> Result<Vec<crate::models::SchoolHoliday>, String> {
    validation::validate_year_month(year, month)?;
    validation::validate_zone(&zone)?;
    school_holiday_repository::get_by_month(year, month, &zone).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_setting(key: String) -> Result<Option<String>, String> {
    school_holiday_repository::get_setting(&key).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_setting(key: String, value: String) -> Result<(), String> {
    school_holiday_repository::set_setting(&key, &value).map_err(|e| e.to_string())
}

// EXPORT

#[tauri::command]
pub async fn export_to_excel(
    _app: AppHandle,
    start_year: i32,
    start_month: u32,
    end_year: i32,
    end_month: u32,
) -> Result<String, String> {
    validation::validate_month_range(start_year, start_month, end_year, end_month)?;
    let file_name = format!(
        "planning_{:04}{:02}_{:04}{:02}.xlsx",
        start_year, start_month, end_year, end_month
    );
    let output_path = get_output_path(&file_name)?;
    crate::export_service::export_months_to_excel(
        start_year,
        start_month,
        end_year,
        end_month,
        &output_path,
    )
    .map_err(|e| e.to_string())?;
    Ok(output_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn export_to_pdf(
    _app: AppHandle,
    start_year: i32,
    start_month: u32,
    end_year: i32,
    end_month: u32,
) -> Result<String, String> {
    validation::validate_month_range(start_year, start_month, end_year, end_month)?;
    let file_name = format!(
        "planning_{:04}{:02}_{:04}{:02}.pdf",
        start_year, start_month, end_year, end_month
    );
    let output_path = get_output_path(&file_name)?;
    crate::pdf_export_service::export_months_to_pdf(
        start_year,
        start_month,
        end_year,
        end_month,
        &output_path,
    )
    .map_err(|e| e.to_string())?;
    Ok(output_path.to_string_lossy().to_string())
}

fn get_output_path(file_name: &str) -> Result<std::path::PathBuf, String> {
    let base = dirs::document_dir()
        .or_else(dirs::home_dir)
        .ok_or_else(|| "Impossible de trouver le dossier Documents".to_string())?;
    let dir = base.join("Smart Planning Pro");
    std::fs::create_dir_all(&dir).map_err(|e| format!("Erreur création dossier : {}", e))?;
    Ok(dir.join(file_name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::initialize;

    fn label_id(name: &str) -> i64 {
        initialize().expect("init");
        create_label(name.to_string(), "#3B82F6".to_string())
            .expect("création label")
            .id
            .expect("id")
    }

    #[test]
    fn delete_label_without_entries_flag_is_refused_when_label_is_used() {
        let id = label_id("École");
        upsert_planning("2026-03-02".into(), "matin".into(), id, None).expect("affectation");

        let err = delete_label(id, false).expect_err("doit être refusé");

        assert!(err.contains("1 affectation"), "{err}");
        assert!(label_repository::get_by_id(id).expect("lecture").is_some());
        assert_eq!(planning_repository::count_by_label(id).expect("count"), 1);
    }

    #[test]
    fn delete_label_with_entries_flag_removes_label_and_its_entries() {
        let id = label_id("Maman");
        upsert_planning("2026-03-02".into(), "matin".into(), id, None).expect("affectation");

        delete_label(id, true).expect("suppression");

        assert!(label_repository::get_by_id(id).expect("lecture").is_none());
        assert_eq!(planning_repository::count_by_label(id).expect("count"), 0);
    }

    #[test]
    fn delete_unused_label_without_entries_flag_succeeds() {
        let id = label_id("Papy");
        delete_label(id, false).expect("suppression");
        assert!(label_repository::get_by_id(id).expect("lecture").is_none());
    }

    #[test]
    fn update_label_validates_color_and_reports_missing_label() {
        let id = label_id("École");
        assert!(update_label(id, "École".into(), "rouge".into()).is_err());
        assert_eq!(
            update_label(9999, "X".into(), "#000000".into()).unwrap_err(),
            LABEL_NOT_FOUND
        );
        assert!(update_label(id, "Collège".into(), "#000000".into()).is_ok());
    }

    #[test]
    fn upsert_planning_rejects_invalid_date_period_and_unknown_label() {
        let id = label_id("École");
        assert!(upsert_planning("2026-3-2".into(), "matin".into(), id, None).is_err());
        assert!(upsert_planning("2026-03-02".into(), "nuit".into(), id, None).is_err());
        assert_eq!(
            upsert_planning("2026-03-02".into(), "matin".into(), 9999, None).unwrap_err(),
            LABEL_NOT_FOUND
        );
        assert!(get_planning_by_month(2026, 3).expect("lecture").is_empty());
    }

    #[test]
    fn month_queries_reject_invalid_months() {
        initialize().expect("init");
        assert!(get_planning_by_month(2026, 13).is_err());
        assert!(get_stats_by_month(2026, 0).is_err());
        assert!(get_school_holidays_by_month(2026, 3, "D".into()).is_err());
    }
}
