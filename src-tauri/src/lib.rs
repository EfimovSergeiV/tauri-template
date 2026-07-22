use std::{path::PathBuf, process::Command};

#[tauri::command]
fn multiply_by_four(value: f64) -> Result<f64, String> {
    let script_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../python/multiply_by_four.py");

    let output = Command::new("python3")
        .arg(script_path)
        .arg(value.to_string())
        .output()
        .map_err(|error| format!("Не удалось запустить python3: {error}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(if stderr.is_empty() {
            "Python-скрипт завершился с ошибкой".to_string()
        } else {
            stderr
        });
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .trim()
        .parse::<f64>()
        .map_err(|error| format!("Некорректный ответ Python-скрипта: {error}"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![multiply_by_four])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
