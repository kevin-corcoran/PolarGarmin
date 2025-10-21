// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod database;
use database::get_running_data;
use serde::Serialize;

// fn main() {
//     garminlab_lib::run()
// }


#[derive(Serialize, Clone)]
struct RunningData {
    date: String,
    start_time: String,
    distance: f64,
    day: String,
    month: String,
}

// Tauri command to get running data
// #[tauri::command,tnd]
#[tauri::command]
async fn fetch_running_data() -> Result<Vec<RunningData>, String> {
    match get_running_data() {
        Ok(data) => {
            // Convert database::RunningData to our local RunningData
            let converted_data: Vec<RunningData> = data.into_iter().map(|db_data| RunningData {
                date: db_data.date,
                start_time: db_data.start_time,
                distance: db_data.distance,
                day: db_data.day,
                month: db_data.month,
            }).collect();
            Ok(converted_data)
        },
        Err(e) => Err(format!("Failed to fetch running data: {}", e)),
    }
} 

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![fetch_running_data])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![fetch_running_data])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
    garmin_app_lib::run()
}
