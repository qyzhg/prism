use crate::{app_state::AppState, ocr_tasks::run_ocr_on_image_data, system_tray::show_main_window};
use tauri::Emitter;
use tauri::{AppHandle, Manager, State};
use tokio::{fs, process::Command};

#[tauri::command]
pub async fn submit_area_for_ocr(
    app_handle: AppHandle,
    _x: i32,
    _y: i32,
    _width: u32,
    _height: u32,
    _state: State<'_, AppState>,
) -> Result<(), String> {
    start_area_selection(app_handle).await
}

pub async fn start_area_selection(app_handle: AppHandle) -> Result<(), String> {
    use std::time::{SystemTime, UNIX_EPOCH};

    if let Some(main_window) = app_handle.get_webview_window("main") {
        main_window
            .emit("ocr-pending", true)
            .map_err(|e| e.to_string())?;
        main_window
            .hide()
            .map_err(|e| format!("隐藏主窗口失败: {}", e))?;
    }

    tauri::async_runtime::spawn(async move {
        let mut temp_path = std::env::temp_dir();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        temp_path.push(format!("prism_capture_{}.png", timestamp));

        let output = Command::new("screencapture")
            .arg("-i")
            .arg("-x")
            .arg(&temp_path)
            .output()
            .await;

        let capture_result = match output {
            Ok(output) => {
                let has_file = match fs::metadata(&temp_path).await {
                    Ok(metadata) => metadata.len() > 0,
                    Err(_) => false,
                };

                if has_file {
                    fs::read(&temp_path)
                        .await
                        .map_err(|e| format!("读取截图失败: {}", e))
                } else {
                    let stderr_message = String::from_utf8_lossy(&output.stderr).trim().to_string();
                    let stderr_lower = stderr_message.to_lowercase();
                    let permission_denied = stderr_lower
                        .contains("could not create image from rect")
                        || stderr_lower.contains("not authorized")
                        || stderr_lower.contains("permission");

                    if permission_denied {
                        Err("无法截图：请在“系统设置 -> 隐私与安全 -> 屏幕录制”中允许 Prism（或终端）访问屏幕，然后重新尝试。".to_string())
                    } else if matches!(output.status.code(), Some(1) | Some(2)) {
                        Err("用户取消截图".to_string())
                    } else if !stderr_message.is_empty() {
                        Err(format!("截图失败: {}", stderr_message))
                    } else {
                        Err(match output.status.code() {
                            Some(code) => format!("截图命令失败 (code {})", code),
                            None => "截图命令被中断".to_string(),
                        })
                    }
                }
            }
            Err(e) => Err(format!("调用截图工具失败: {}", e)),
        };

        let _ = fs::remove_file(&temp_path).await;

        let ocr_result = match capture_result {
            Ok(image_data) => {
                show_main_window(&app_handle);
                let state = app_handle.state::<AppState>();
                run_ocr_on_image_data(image_data, state).await
            }
            Err(err) => {
                show_main_window(&app_handle);
                Err(err)
            }
        };

        if let Some(main_window) = app_handle.get_webview_window("main") {
            match ocr_result {
                Ok(text) => {
                    let _ = main_window.emit("ocr-result", text);
                }
                Err(err) => {
                    let _ = main_window.emit("ocr-result", format!("Error: {}", err));
                }
            }
        }
    });

    Ok(())
}
