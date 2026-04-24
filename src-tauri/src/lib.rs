
pub mod models;

#[path = "database/db.rs"]
pub mod database;

#[path = "repositories/label_repository.rs"]
pub mod label_repository;

#[path = "repositories/planning_repository.rs"]
pub mod planning_repository;

#[path = "export_service.rs"]
pub mod export_service;

#[path = "commands.rs"]                          // ← ajoute
pub mod commands;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  database::initialize().expect("Failed to initialize database");

  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::get_labels,
      commands::create_label,
      commands::update_label,
      commands::delete_label,
      commands::get_planning_by_month,
      commands::upsert_planning,
      commands::delete_planning,
      commands::get_stats_by_month,
      commands::export_to_excel,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
