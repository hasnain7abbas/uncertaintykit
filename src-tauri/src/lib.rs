mod commands;
mod formulas;
mod math;
mod storage;

use formulas::library::{get_formula_library, FormulaEntry};
use tauri::command;

#[command]
fn get_formulas() -> Vec<FormulaEntry> {
    get_formula_library()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            commands::propagate_uncertainty,
            commands::detect_variables,
            commands::formula_to_latex,
            commands::get_derivative_latex,
            get_formulas,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
