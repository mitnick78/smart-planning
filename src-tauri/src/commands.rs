use crate::models::{Label, PlanningEntry, StatEntry};
use crate::{label_repository, planning_repository};

// LABELS 

/// get all labels
///
/// #[tauri::command] = decorate the function for Tauri
/// Result<Vec<Label>, String> :
///   - Ok(labels) → React receives Label[]
///   - Err(msg)   → React receives an error with the message

#[tauri::command]
pub fn get_labels() -> Result<Vec<Label>, String> {
    label_repository::get_all()
        .map_err(|e| e.to_string())
}

/// create label
///   invoke('create_label', { name: 'École', colorHex: '#3B82F6' })
#[tauri::command]
pub fn create_label(name: String, color_hex: String) -> Result<Label, String> {
    // validate inputs
    if name.trim().is_empty() {
        return Err("Le nom ne peut pas être vide".to_string());
    }
    if !Label::is_valid_color(&color_hex) {
        return Err(format!("Couleur invalide : {}", color_hex));
    }

    let label = Label::new(&name, &color_hex);
    let id = label_repository::create(&label)
        .map_err(|e| e.to_string())?;

    // return the created label with its new id
    Ok(Label {
        id: Some(id),
        name: label.name,
        color_hex: label.color_hex,
        created_at: None,
    })
}

/// update existing label by id
#[tauri::command]
pub fn update_label(
    id: i64,
    name: String,
    color_hex: String
) -> Result<(), String> {
    if name.trim().is_empty() {
        return Err("Le nom ne peut pas être vide".to_string());
    }

    let label = Label {
        id: Some(id),
        name,
        color_hex,
        created_at: None,
    };

    label_repository::update(&label)
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Delete a label by id
///
/// delete_entries : if true, also delete all planning entries associated with this label
#[tauri::command]
pub fn delete_label(id: i64, delete_entries: bool) -> Result<(), String> {
    label_repository::delete(id, delete_entries)
        .map_err(|e| e.to_string())
}

// ── PLANNING ──────────────────────────────────────────────────

/// get planning_by_month(year: i32, month: u32) → PlanningEntry[]
///   invoke('get_planning_by_month', { year: 2026, month: 3 })
#[tauri::command]
pub fn get_planning_by_month(
    year: i32,
    month: u32
) -> Result<Vec<PlanningEntry>, String> {
    planning_repository::get_by_month(year, month)
        .map_err(|e| e.to_string())
}

/// assing a label to a date/period, or update the existing entry
///
///   invoke('upsert_planning', {
///     date: '2026-03-15',
///     period: 'matin',
///     labelId: 1,
///     note: null
///   })
#[tauri::command]
pub fn upsert_planning(
    date: String,
    period: String,
    label_id: i64,
    note: Option<String>,
) -> Result<i64, String> {
    let valid_periods = ["matin", "après-midi", "soir", "journée"];
    if !valid_periods.contains(&period.as_str()) {
        return Err(format!("Période invalide : {}", period));
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

    planning_repository::upsert(&entry)
        .map_err(|e| e.to_string())
}

/// delete_planning(date: String, period: String) → bool
#[tauri::command]
pub fn delete_planning(date: String, period: String) -> Result<bool, String> {
    planning_repository::delete_by_date_and_period(&date, &period)
        .map_err(|e| e.to_string())
}

// STATISTIQUES
// get stats_by_month(year: i32, month: u32) → StatEntry[]
#[tauri::command]
pub fn get_stats_by_month(
    year: i32,
    month: u32
) -> Result<Vec<StatEntry>, String> {
    let result = planning_repository::get_stats_by_month(year, month)
        .map_err(|e| e.to_string())?;
    Ok(result)
}

#[tauri::command]
pub fn export_to_excel(year: i32, months: Vec<u32>) -> Result<String, String> {
    let mut path = dirs::document_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    path.push(format!("planning_{}.xlsx", year));
    crate::export_service::export_months_to_excel(year, &months, &path)
        .map_err(|e| e.to_string())?;
    Ok(path.to_string_lossy().to_string())
}
