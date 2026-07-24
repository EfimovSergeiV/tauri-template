use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use serde_json::{json, Value};
use tauri::{Emitter, Manager};

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

fn parse_counter_line_to_number(line: &str) -> Result<u32, String> {
    if let Ok(number) = line.parse::<u32>() {
        return Ok(number);
    }

    let value: Value = serde_json::from_str(line)
        .map_err(|error| format!("Некорректный JSON от Python-скрипта: {error}"))?;

    let number = value
        .get("number")
        .and_then(Value::as_u64)
        .ok_or_else(|| "JSON от Python-скрипта не содержит поле number".to_string())?;

    u32::try_from(number)
        .map_err(|_| "Поле number в JSON выходит за пределы u32".to_string())
}

fn parse_counter_line_to_payload(line: &str) -> Result<Value, String> {
    if let Ok(value) = serde_json::from_str::<Value>(line) {
        if value.get("number").and_then(Value::as_u64).is_some() {
            return Ok(value);
        }

        return Err("JSON от Python-скрипта не содержит поле number".to_string());
    }

    let number = line
        .parse::<u32>()
        .map_err(|error| format!("Некорректная строка от Python-скрипта: {error}"))?;
    Ok(json!({ "number": number }))
}

fn resolve_python_script_path(app: &tauri::AppHandle, file_name: &str) -> PathBuf {
    app.path()
        .resolve(
            PathBuf::from("python").join(file_name),
            tauri::path::BaseDirectory::Resource,
        )
        .expect("Python script is missing from the bundled resources")
}

#[derive(Clone)]
struct CounterProcessState {
    child: Arc<Mutex<Option<Child>>>,
    stop_requested: Arc<AtomicBool>,
}

impl Default for CounterProcessState {
    fn default() -> Self {
        Self {
            child: Arc::new(Mutex::new(None)),
            stop_requested: Arc::new(AtomicBool::new(false)),
        }
    }
}

#[tauri::command]
fn multiply_by_four(app: tauri::AppHandle, value: f64) -> Result<f64, String> {
    let script_path = resolve_python_script_path(&app, "multiply_by_four.py");

    #[cfg(windows)]
    let python_bin = "python";
    #[cfg(not(windows))]
    let python_bin = "python3";

    let mut command = Command::new(python_bin);
    command.arg(script_path).arg(value.to_string());

    #[cfg(windows)]
    {
        command.creation_flags(CREATE_NO_WINDOW);
    }

    let output = command
        .output()
        .map_err(|error| format!("Не удалось запустить {python_bin}: {error}"))?;

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

#[tauri::command]
fn count_one_to_hundred(app: tauri::AppHandle) -> Result<Vec<u32>, String> {
    let script_path = resolve_python_script_path(&app, "count_one_to_hundred.py");

    #[cfg(windows)]
    let python_bin = "python";
    #[cfg(not(windows))]
    let python_bin = "python3";

    let mut command = Command::new(python_bin);
    command.arg(script_path);

    #[cfg(windows)]
    {
        command.creation_flags(CREATE_NO_WINDOW);
    }

    let output = command
        .output()
        .map_err(|error| format!("Не удалось запустить {python_bin}: {error}"))?;

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
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            parse_counter_line_to_number(line.trim())
        })
        .collect()
}

#[tauri::command]
fn count_one_to_hundred_stream(
    app: tauri::AppHandle,
    state: tauri::State<CounterProcessState>,
) -> Result<(), String> {
    let script_path = resolve_python_script_path(&app, "count_one_to_hundred.py");

    #[cfg(windows)]
    let python_bin = "python";
    #[cfg(not(windows))]
    let python_bin = "python3";

    {
        let guard = state
            .child
            .lock()
            .map_err(|_| "Состояние счётчика недоступно".to_string())?;
        if guard.is_some() {
            return Err("Скрипт уже выполняется".to_string());
        }
    }

    state.stop_requested.store(false, Ordering::SeqCst);

    let mut command = Command::new(python_bin);
    command.arg(script_path).stdout(Stdio::piped());

    #[cfg(windows)]
    {
        command.creation_flags(CREATE_NO_WINDOW);
    }

    let mut child = command
        .spawn()
        .map_err(|error| format!("Не удалось запустить {python_bin}: {error}"))?;

    let Some(stdout) = child.stdout.take() else {
        let _ = child.kill();
        return Err("Не удалось получить stdout Python-скрипта".to_string());
    };

    {
        let mut guard = state
            .child
            .lock()
            .map_err(|_| "Состояние счётчика недоступно".to_string())?;
        *guard = Some(child);
    }

    let child_state = state.child.clone();
    let stop_requested = state.stop_requested.clone();

    std::thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(value) => {
                    let trimmed = value.trim();
                    if trimmed.is_empty() {
                        continue;
                    }

                    match parse_counter_line_to_payload(trimmed) {
                        Ok(payload) => {
                            let _ = app.emit("counter-stream-value", payload);
                        }
                        Err(error) => {
                            let _ = app.emit(
                                "counter-stream-error",
                                error,
                            );
                            if let Ok(mut guard) = child_state.lock() {
                                if let Some(child) = guard.as_mut() {
                                    let _ = child.kill();
                                }
                            }
                            return;
                        }
                    }
                }
                Err(error) => {
                    let _ = app.emit(
                        "counter-stream-error",
                        format!("Ошибка чтения вывода Python-скрипта: {error}"),
                    );
                    if let Ok(mut guard) = child_state.lock() {
                        if let Some(child) = guard.as_mut() {
                            let _ = child.kill();
                        }
                    }
                    return;
                }
            }
        }

        let maybe_child = child_state.lock().ok().and_then(|mut guard| guard.take());
        if let Some(mut child) = maybe_child {
            match child.wait() {
                Ok(status) if status.success() => {
                    let _ = app.emit("counter-stream-done", true);
                }
                Ok(_) if stop_requested.swap(false, Ordering::SeqCst) => {
                    let _ = app.emit("counter-stream-stopped", true);
                }
                Ok(status) => {
                    let message = format!("Python-скрипт завершился с ошибкой: {status}");
                    let _ = app.emit("counter-stream-error", message);
                }
                Err(error) => {
                    let _ = app.emit(
                        "counter-stream-error",
                        format!("Не удалось дождаться завершения Python-скрипта: {error}"),
                    );
                }
            }
        }
    });

    Ok(())
}

#[tauri::command]
fn stop_count_one_to_hundred_stream(state: tauri::State<CounterProcessState>) -> Result<bool, String> {
    state.stop_requested.store(true, Ordering::SeqCst);

    let mut guard = state
        .child
        .lock()
        .map_err(|_| "Состояние счётчика недоступно".to_string())?;

    if let Some(child) = guard.as_mut() {
        child
            .kill()
            .map_err(|error| format!("Не удалось остановить скрипт: {error}"))?;
        return Ok(true);
    }

    state.stop_requested.store(false, Ordering::SeqCst);
    Ok(false)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(CounterProcessState::default())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            multiply_by_four,
            count_one_to_hundred,
            count_one_to_hundred_stream,
            stop_count_one_to_hundred_stream
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
