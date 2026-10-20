use crate::models::{Label, PlanningEntry, StatEntry};
use crate::{label_repository, planning_repository};
use tauri::AppHandle;
use crate::school_holiday_repository;
use chrono::Datelike;

// LABELS

#[tauri::command]
pub fn get_labels() -> Result<Vec<Label>, String> {
    label_repository::get_all()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_label(name: String, color_hex: String) -> Result<Label, String> {
    if name.trim().is_empty() {
        return Err("Le nom ne peut pas être vide".to_string());
    }
    if !Label::is_valid_color(&color_hex) {
        return Err(format!("Couleur invalide : {}", color_hex));
    }
    let label = Label::new(&name, &color_hex);
    let id = label_repository::create(&label).map_err(|e| e.to_string())?;
    Ok(Label { id: Some(id), name: label.name, color_hex: label.color_hex, created_at: None })
}

#[tauri::command]
pub fn update_label(id: i64, name: String, color_hex: String) -> Result<(), String> {
    if name.trim().is_empty() {
        return Err("Le nom ne peut pas être vide".to_string());
    }
    let label = Label { id: Some(id), name, color_hex, created_at: None };
    label_repository::update(&label).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_label(id: i64, delete_entries: bool) -> Result<(), String> {
    label_repository::delete(id, delete_entries).map_err(|e| e.to_string())
}

// PLANNING

#[tauri::command]
pub fn get_planning_by_month(year: i32, month: u32) -> Result<Vec<PlanningEntry>, String> {
    planning_repository::get_by_month(year, month).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn upsert_planning(
    date: String, period: String, label_id: i64, note: Option<String>,
) -> Result<i64, String> {
    let valid_periods = ["matin", "après-midi", "soir", "journée"];
    if !valid_periods.contains(&period.as_str()) {
        return Err(format!("Période invalide : {}", period));
    }
    let entry = PlanningEntry {
        id: None, date, period, label_id, note,
        label_name: None, label_color: None, created_at: None, updated_at: None,
    };
    planning_repository::upsert(&entry).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_planning(date: String, period: String) -> Result<bool, String> {
    planning_repository::delete_by_date_and_period(&date, &period)
        .map_err(|e| e.to_string())
}

// STATISTICS

#[tauri::command]
pub fn get_stats_by_month(year: i32, month: u32) -> Result<Vec<StatEntry>, String> {
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
    let school_years = vec![
        format!("{}-{}", year - 1, year),
        format!("{}-{}", year, year + 1),
    ];

    let zone_filter = format!("Zone {}", zone);
    let mut all_holidays: Vec<crate::models::SchoolHoliday> = Vec::new();

    let client = reqwest::Client::new();

    for school_year in &school_years {
        let url = format!(
            "https://data.education.gouv.fr/api/explore/v2.1/catalog/datasets/fr-en-calendrier-scolaire/records\
            ?where=zones%20like%20%22{}%22%20and%20annee_scolaire%3D%22{}%22\
            &limit=100",
            zone_filter, school_year
        );

        let resp = client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Erreur réseau : {}", e))?;

        let api: ApiResponse = resp
            .json()
            .await
            .map_err(|e| format!("Erreur JSON : {}", e))?;

        for record in api.results {
            let name = record.description.unwrap_or_else(|| "Vacances scolaires".to_string());
            let start = match record.start_date { Some(d) => d, None => continue };
            let end   = match record.end_date   { Some(d) => d, None => continue };

            let start_dt = chrono::DateTime::parse_from_rfc3339(&start)
                .map_err(|e| format!("Date invalide : {}", e))?;
            let end_dt = chrono::DateTime::parse_from_rfc3339(&end)
                .map_err(|e| format!("Date invalide : {}", e))?;

            // L'API stocke les dates en UTC à 22h/23h = minuit heure Paris.
            // +4h pour convertir correctement en date locale française.
            // Le end_date UTC correspond au début du lendemain → on soustrait 1 jour.
            let start_date = (start_dt + chrono::Duration::hours(4)).date_naive();
            let end_date   = (end_dt   + chrono::Duration::hours(4)).date_naive()
                - chrono::Duration::days(1);

            let mut current = start_date;
            while current <= end_date {
                if current.year() == year {
                    all_holidays.push(crate::models::SchoolHoliday {
                        date: current.format("%Y-%m-%d").to_string(),
                        zone: zone.clone(),
                        name: name.clone(),
                    });
                }
                current = current.succ_opt().unwrap();
            }
        }
    }

    let count = all_holidays.len();
    school_holiday_repository::upsert_many(&all_holidays)
        .map_err(|e| e.to_string())?;

    Ok(count)
}

#[tauri::command]
pub fn get_school_holidays_by_month(
    year: i32,
    month: u32,
    zone: String,
) -> Result<Vec<crate::models::SchoolHoliday>, String> {
    school_holiday_repository::get_by_month(year, month, &zone)
        .map_err(|e| e.to_string())
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
    start_year: i32, start_month: u32,
    end_year: i32,   end_month: u32,
) -> Result<String, String> {
    let file_name = format!(
        "planning_{:04}{:02}_{:04}{:02}.xlsx",
        start_year, start_month, end_year, end_month
    );
    let output_path = get_output_path(&file_name)?;
    crate::export_service::export_months_to_excel(
        start_year, start_month, end_year, end_month, &output_path,
    ).map_err(|e| e.to_string())?;
    Ok(output_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn export_to_pdf(
    _app: AppHandle,
    start_year: i32, start_month: u32,
    end_year: i32,   end_month: u32,
) -> Result<String, String> {
    let file_name = format!(
        "planning_{:04}{:02}_{:04}{:02}.pdf",
        start_year, start_month, end_year, end_month
    );
    let output_path = get_output_path(&file_name)?;
    crate::pdf_export_service::export_months_to_pdf(
        start_year, start_month, end_year, end_month, &output_path,
    ).map_err(|e| e.to_string())?;
    Ok(output_path.to_string_lossy().to_string())
}

fn get_output_path(file_name: &str) -> Result<std::path::PathBuf, String> {
    let base = dirs::document_dir()
        .or_else(|| dirs::home_dir())
        .ok_or_else(|| "Impossible de trouver le dossier Documents".to_string())?;
    let dir = base.join("Smart Planning Pro");
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("Erreur création dossier : {}", e))?;
    Ok(dir.join(file_name))
}