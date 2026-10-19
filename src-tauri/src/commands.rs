use crate::models::{Label, PlanningEntry, StatEntry};
use crate::{label_repository, planning_repository};
use tauri::AppHandle;

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

// STATISTIQUES

#[tauri::command]
pub fn get_stats_by_month(year: i32, month: u32) -> Result<Vec<StatEntry>, String> {
    planning_repository::get_stats_by_month(year, month).map_err(|e| e.to_string())
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