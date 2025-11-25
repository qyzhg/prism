mod database;
mod ocr;
mod translation;

use chrono;
use database::{AppConfig, Database, TranslationRecord};
use ocr::{OcrRequest, OcrService};
use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, State, WindowEvent,
};
use translation::{
    get_supported_languages, TranslationRequest, TranslationResult, TranslationService,
};

use std::str::FromStr;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

// 应用状态
pub struct AppState {
    db: Mutex<Database>,
    translation_service: Mutex<TranslationService>,
}

fn show_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

fn setup_system_tray(app: &tauri::AppHandle) -> tauri::Result<()> {
    let show_item = MenuItem::with_id(app, "show-main", "显示主界面", true, None::<&str>)?;
    let hide_item = MenuItem::with_id(app, "hide-main", "隐藏到托盘", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show_item, &hide_item, &quit_item])?;

    let mut tray_builder = TrayIconBuilder::new()
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show-main" => show_main_window(app),
            "hide-main" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app_handle = tray.app_handle();
                show_main_window(&app_handle);
            }
        });

    if let Some(icon) = app.default_window_icon() {
        tray_builder = tray_builder.icon(icon.clone());
    }

    tray_builder.build(app)?;

    Ok(())
}

#[cfg(target_os = "macos")]
#[tauri::command]
async fn submit_area_for_ocr(
    app_handle: tauri::AppHandle,
    _x: i32,
    _y: i32,
    _width: u32,
    _height: u32,
    _state: State<'_, AppState>,
) -> Result<(), String> {
    start_macos_area_selection(app_handle).await
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
async fn submit_area_for_ocr(
    app_handle: tauri::AppHandle,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // 1. 立即触发待处理的事件
    if let Some(main_window) = app_handle.get_webview_window("main") {
        main_window
            .emit("ocr-pending", true)
            .map_err(|e| e.to_string())?;
    }

    // 2. 生成后台任务
    let app_handle_clone = app_handle.clone();
    state.inner();

    // 开启异步任务
    tauri::async_runtime::spawn(async move {
        let state = app_handle_clone.state::<AppState>();

        match capture_area_and_ocr(x, y, width, height, state).await {
            Ok(text) => {
                println!("OCR成功，文本长度: {}, 内容: '{}'", text.len(), text);
                if let Some(main_window) = app_handle_clone.get_webview_window("main") {
                    println!("发送ocr-result事件到主窗口");
                    match main_window.emit("ocr-result", &text) {
                        Ok(_) => println!("ocr-result事件发送成功"),
                        Err(e) => println!("ocr-result事件发送失败: {}", e),
                    }
                } else {
                    println!("无法找到主窗口");
                }
            }
            Err(e) => {
                println!("Background OCR failed: {}", e);
                if let Some(main_window) = app_handle_clone.get_webview_window("main") {
                    let error_msg = format!("Error: {}", e);
                    let _ = main_window.emit("ocr-result", error_msg);
                }
            }
        }
    });

    Ok(())
}

fn register_shortcuts(app: &tauri::AppHandle) {
    let state = app.state::<AppState>();
    let db = state.db.lock().unwrap();

    let _ = app.global_shortcut().unregister_all();

    let hotkeys = if let Ok(Some(config)) = db.get_app_config() {
        config.hotkeys
    } else {
        database::HotkeyConfig {
            popup_window: "Ctrl+Shift+T".to_string(),
            slide_translation: "Ctrl+Shift+S".to_string(),
            screenshot_translation: "Ctrl+Shift+A".to_string(),
        }
    };

    if !hotkeys.popup_window.is_empty() {
        if let Ok(shortcut) = Shortcut::from_str(&hotkeys.popup_window) {
            let app_handle = app.clone();
            match app.global_shortcut().register(shortcut) {
                Ok(_) => {
                    let _ = app.global_shortcut().on_shortcut(
                        shortcut,
                        move |_app, shortcut, event| {
                            if event.state == ShortcutState::Pressed {
                                if let Some(window) = app_handle.get_webview_window("main") {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            }
                        },
                    );
                    println!("Registered popup window shortcut: {}", hotkeys.popup_window);
                }
                Err(e) => {
                    eprintln!(
                        "Failed to register popup window shortcut '{}': {}",
                        hotkeys.popup_window, e
                    );
                }
            }
        }
    }

    if !hotkeys.screenshot_translation.is_empty() {
        if let Ok(shortcut) = Shortcut::from_str(&hotkeys.screenshot_translation) {
            let app_handle = app.clone();
            match app.global_shortcut().register(shortcut) {
                Ok(_) => {
                    let _ = app.global_shortcut().on_shortcut(
                        shortcut,
                        move |_app, shortcut, event| {
                            if event.state == ShortcutState::Pressed {
                                let _ = start_area_selection(app_handle.clone());
                            }
                        },
                    );
                    println!(
                        "Registered screenshot translation shortcut: {}",
                        hotkeys.screenshot_translation
                    );
                }
                Err(e) => {
                    eprintln!(
                        "Failed to register screenshot translation shortcut '{}': {}",
                        hotkeys.screenshot_translation, e
                    );
                }
            }
        }
    }

    if !hotkeys.slide_translation.is_empty() {
        if let Ok(shortcut) = Shortcut::from_str(&hotkeys.slide_translation) {
            match app.global_shortcut().register(shortcut) {
                Ok(_) => {
                    println!(
                        "Registered slide translation shortcut: {}",
                        hotkeys.slide_translation
                    );
                }
                Err(e) => {
                    eprintln!(
                        "Failed to register slide translation shortcut '{}': {}",
                        hotkeys.slide_translation, e
                    );
                }
            }
        }
    }
}

#[tauri::command]
async fn translate_text(
    text: String,
    from_language: Option<String>,
    to_language: String,
    _service: String,
    state: State<'_, AppState>,
) -> Result<TranslationResult, String> {
    let request = TranslationRequest {
        text,
        from_lang: from_language.unwrap_or_default(),
        to_lang: to_language,
    };

    let translation_service = {
        let service = state
            .translation_service
            .lock()
            .map_err(|e| format!("获取翻译服务失败: {}", e))?;
        service.clone()
    };

    let (api_key, base_url, model_id) = {
        let db = state
            .db
            .lock()
            .map_err(|e| format!("获取数据库连接失败: {}", e))?;

        match db.get_app_config() {
            Ok(Some(config)) => {
                let translation_config = config.translation;
                (
                    translation_config.api_key,
                    translation_config.base_url,
                    translation_config.model_id,
                )
            }
            Ok(None) => {
                // 如果没有配置，使用默认值
                (
                    "".to_string(),
                    "https://api.openai.com/v1".to_string(),
                    "gpt-5-nano".to_string(),
                )
            }
            Err(e) => {
                return Err(format!("获取应用配置失败: {}", e));
            }
        }
    };

    translation_service
        .translate(request, &api_key, &base_url, &model_id)
        .await
}

#[tauri::command]
async fn save_translation(
    original_text: String,
    translated_text: String,
    from_language: String,
    to_language: String,
    service: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    let record = TranslationRecord {
        id: None,
        original_text: original_text.clone(),
        translated_text: translated_text.clone(),
        service: service.clone(),
        from_language: Some(from_language),
        to_language: Some(to_language),
        created_at: Some(chrono::Utc::now().to_rfc3339()),
    };

    db.save_translation(&record)
        .map(|_| ())
        .map_err(|e| format!("保存翻译记录失败: {}", e))?;

    Ok(())
}

#[tauri::command]
async fn get_translation_history(
    limit: Option<i32>,
    offset: Option<i32>,
    state: State<'_, AppState>,
) -> Result<Vec<TranslationRecord>, String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    db.get_translation_history(limit, offset)
        .map_err(|e| format!("获取翻译历史失败: {}", e))
}

#[tauri::command]
async fn search_history(
    keyword: String,
    limit: Option<i32>,
    state: State<'_, AppState>,
) -> Result<Vec<TranslationRecord>, String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    db.search_history(&keyword, limit)
        .map_err(|e| format!("搜索历史记录失败: {}", e))
}

#[tauri::command]
async fn clear_history(state: State<'_, AppState>) -> Result<(), String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    db.clear_history()
        .map_err(|e| format!("清空历史记录失败: {}", e))
}

#[tauri::command]
async fn save_setting(
    key: String,
    value: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    db.save_setting(&key, &value)
        .map_err(|e| format!("保存设置失败: {}", e))
}

#[tauri::command]
async fn get_setting(key: String, state: State<'_, AppState>) -> Result<Option<String>, String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    db.get_setting(&key)
        .map_err(|e| format!("获取设置失败: {}", e))
}

#[tauri::command]
async fn save_api_key(
    service: String,
    api_key: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    db.save_api_key(&service, &api_key)
        .map_err(|e| format!("保存 API 密钥失败: {}", e))
}

#[tauri::command]
async fn get_api_key(
    service: String,
    state: State<'_, AppState>,
) -> Result<Option<String>, String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    db.get_api_key(&service)
        .map_err(|e| format!("获取 API 密钥失败: {}", e))
}

#[tauri::command]
async fn save_app_config(config: AppConfig, state: State<'_, AppState>) -> Result<(), String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    db.save_app_config(&config)
        .map_err(|e| format!("保存应用配置失败: {}", e))
}

#[tauri::command]
async fn get_app_config(state: State<'_, AppState>) -> Result<AppConfig, String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    db.get_app_config()
        .map_err(|e| format!("获取应用配置失败: {}", e))
        .and_then(|config_opt| config_opt.ok_or_else(|| "应用配置不存在".to_string()))
}

#[tauri::command]
async fn reload_shortcuts(app_handle: tauri::AppHandle) -> Result<(), String> {
    register_shortcuts(&app_handle);
    Ok(())
}

#[tauri::command]
async fn capture_screen() -> Result<String, String> {
    use base64::Engine;
    use screenshots::Screen;

    // 获取所有屏幕
    let screens = Screen::all().map_err(|e| format!("获取屏幕列表失败: {}", e))?;

    if screens.is_empty() {
        return Err("未找到任何屏幕".to_string());
    }

    // 使用第一个屏幕进行截图
    let screen = &screens[0];
    let image = screen.capture().map_err(|e| format!("截图失败: {}", e))?;

    #[cfg(target_os = "windows")]
    let buffer = image
        .to_png(None)
        .map_err(|e| format!("图片编码失败: {}", e))?;

    #[cfg(target_os = "macos")]
    let buffer = {
        use screenshots::image::ImageFormat;
        use std::io::Cursor;
        let mut buf = Vec::new();
        image
            .write_to(&mut Cursor::new(&mut buf), ImageFormat::Png)
            .map_err(|e| format!("图片编码失败: {}", e))?;
        buf
    };

    // 将图片数据转换为base64
    let base64_string = base64::engine::general_purpose::STANDARD.encode(&buffer);

    Ok(base64_string)
}

#[tauri::command]
async fn capture_screen_area(x: u32, y: u32, width: u32, height: u32) -> Result<String, String> {
    use base64::Engine;
    use screenshots::Screen;

    // 获取所有屏幕
    let screens = Screen::all().map_err(|e| format!("获取屏幕列表失败: {}", e))?;

    if screens.is_empty() {
        return Err("未找到任何屏幕".to_string());
    }

    // 使用第一个屏幕进行截图
    let screen = &screens[0];
    let image = screen.capture().map_err(|e| format!("截图失败: {}", e))?;

    #[cfg(target_os = "windows")]
    let png_buffer = image
        .to_png(None)
        .map_err(|e| format!("图片编码失败: {}", e))?;

    #[cfg(target_os = "macos")]
    let png_buffer = {
        use screenshots::image::ImageFormat;
        use std::io::Cursor;
        let mut buf = Vec::new();
        image
            .write_to(&mut Cursor::new(&mut buf), ImageFormat::Png)
            .map_err(|e| format!("图片编码失败: {}", e))?;
        buf
    };

    // 使用image crate加载图片
    let mut dynamic_image =
        image::load_from_memory(&png_buffer).map_err(|e| format!("加载图片失败: {}", e))?;

    // 裁剪图片
    let cropped_image = dynamic_image.crop(x, y, width, height);

    // 将裁剪后的图片编码为PNG
    let mut cropped_buffer = Vec::new();
    cropped_image
        .write_to(
            &mut std::io::Cursor::new(&mut cropped_buffer),
            image::ImageFormat::Png,
        )
        .map_err(|e| format!("编码裁剪图片失败: {}", e))?;

    // 将图片数据转换为base64
    let base64_string = base64::engine::general_purpose::STANDARD.encode(&cropped_buffer);

    Ok(base64_string)
}

#[tauri::command]
async fn start_area_selection(app_handle: tauri::AppHandle) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        return start_macos_area_selection(app_handle).await;
    }

    #[cfg(target_os = "windows")]
    {
        return start_windows_area_selection(app_handle).await;
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        let _ = app_handle;
        return Err("当前平台暂不支持区域截图".to_string());
    }
}

#[cfg(target_os = "windows")]
async fn start_windows_area_selection(app_handle: tauri::AppHandle) -> Result<(), String> {
    use screenshots::Screen;
    use tauri::WebviewUrl;

    let screens = Screen::all().map_err(|e| format!("获取屏幕列表失败: {}", e))?;

    if screens.is_empty() {
        return Err("未找到任何屏幕".to_string());
    }

    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    for screen in &screens {
        let info = &screen.display_info;
        min_x = min_x.min(info.x);
        min_y = min_y.min(info.y);
        max_x = max_x.max(info.x + info.width as i32);
        max_y = max_y.max(info.y + info.height as i32);
    }

    let width = (max_x - min_x) as f64;
    let height = (max_y - min_y) as f64;

    let window = tauri::WebviewWindowBuilder::new(
        &app_handle,
        "area-selector",
        WebviewUrl::App("area-selector.html".into()),
    )
    .title("区域选择")
    .position(min_x as f64, min_y as f64)
    .inner_size(width, height)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .resizable(false)
    .transparent(true)
    .visible(false)
    .build()
    .map_err(|e| format!("创建区域选择窗口失败: {}", e))?;

    window
        .show()
        .map_err(|e| format!("显示区域选择窗口失败: {}", e))?;
    window
        .set_focus()
        .map_err(|e| format!("设置区域选择窗口焦点失败: {}", e))?;

    Ok(())
}

#[cfg(target_os = "macos")]
async fn start_macos_area_selection(app_handle: tauri::AppHandle) -> Result<(), String> {
    use std::time::{SystemTime, UNIX_EPOCH};
    use tauri::Manager;
    use tokio::fs;
    use tokio::process::Command;

    if let Some(main_window) = app_handle.get_webview_window("main") {
        main_window
            .emit("ocr-pending", true)
            .map_err(|e| e.to_string())?;
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
                let state = app_handle.state::<AppState>();
                run_ocr_on_image_data(image_data, state).await
            }
            Err(err) => Err(err),
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

#[tauri::command]
async fn set_ocr_result(app_handle: tauri::AppHandle, text: String) -> Result<(), String> {
    // 获取主窗口并发送OCR结果
    if let Some(main_window) = app_handle.get_webview_window("main") {
        main_window
            .emit("ocr-result", text)
            .map_err(|e| format!("发送OCR结果失败: {}", e))?;
    }
    Ok(())
}

async fn run_ocr_on_image_data(
    image_data: Vec<u8>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let (api_key, base_url, model_id) = {
        let db = state
            .db
            .lock()
            .map_err(|e| format!("获取数据库连接失败: {}", e))?;

        match db.get_app_config() {
            Ok(Some(config)) => {
                if config.ocr.reuse_translation {
                    let translation_config = config.translation;
                    (
                        translation_config.api_key,
                        translation_config.base_url,
                        translation_config.model_id,
                    )
                } else {
                    let ocr_config = config.ocr;
                    (ocr_config.api_key, ocr_config.base_url, ocr_config.model_id)
                }
            }
            Ok(None) => {
                return Err("OCR配置不存在，请先在设置中配置OCR".to_string());
            }
            Err(e) => {
                return Err(format!("获取OCR配置失败: {}", e));
            }
        }
    };

    if api_key.is_empty() {
        return Err("OCR API密钥未配置，请在设置中配置API密钥".to_string());
    }

    let ocr_service = OcrService::new(api_key, base_url, model_id);
    let ocr_request = OcrRequest {
        image_data,
        image_format: "png".to_string(),
    };

    let ocr_result = ocr_service
        .extract_text(ocr_request)
        .await
        .map_err(|e| format!("OCR识别失败: {}", e))?;

    Ok(ocr_result.text)
}

#[tauri::command]
async fn capture_area_and_ocr(
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    state: State<'_, AppState>,
) -> Result<String, String> {
    use screenshots::Screen;

    // 1. 区域截图
    let screens = Screen::all().map_err(|e| format!("获取屏幕列表失败: {}", e))?;

    if screens.is_empty() {
        return Err("未找到任何屏幕".to_string());
    }

    // 找到包含截图区域中心的屏幕
    let center_x = x + (width as i32 / 2);
    let center_y = y + (height as i32 / 2);

    let screen = screens
        .iter()
        .find(|s| {
            let info = &s.display_info;
            center_x >= info.x
                && center_x < (info.x + info.width as i32)
                && center_y >= info.y
                && center_y < (info.y + info.height as i32)
        })
        .unwrap_or(&screens[0]);

    let image = screen.capture().map_err(|e| format!("截图失败: {}", e))?;

    #[cfg(target_os = "windows")]
    let buffer = image
        .to_png(None)
        .map_err(|e| format!("图片编码失败: {}", e))?;

    #[cfg(target_os = "macos")]
    let buffer = {
        use screenshots::image::ImageFormat;
        use std::io::Cursor;
        let mut buf = Vec::new();
        image
            .write_to(&mut Cursor::new(&mut buf), ImageFormat::Png)
            .map_err(|e| format!("图片编码失败: {}", e))?;
        buf
    };

    // 使用image crate加载图片
    let mut dynamic_image =
        image::load_from_memory(&buffer).map_err(|e| format!("加载图片失败: {}", e))?;

    // 计算相对于屏幕的坐标
    let relative_x = (x - screen.display_info.x) as u32;
    let relative_y = (y - screen.display_info.y) as u32;

    // 裁剪图片
    let cropped_image = dynamic_image.crop(relative_x, relative_y, width, height);

    // 将裁剪后的图片编码为PNG
    let mut cropped_buffer = Vec::new();
    cropped_image
        .write_to(
            &mut std::io::Cursor::new(&mut cropped_buffer),
            image::ImageFormat::Png,
        )
        .map_err(|e| format!("编码裁剪图片失败: {}", e))?;

    run_ocr_on_image_data(cropped_buffer, state).await
}

#[tauri::command]
async fn capture_and_ocr(state: State<'_, AppState>) -> Result<String, String> {
    use screenshots::Screen;

    // 1. 截图
    let screens = Screen::all().map_err(|e| format!("获取屏幕列表失败: {}", e))?;

    if screens.is_empty() {
        return Err("未找到任何屏幕".to_string());
    }

    let screen = &screens[0];
    let image = screen.capture().map_err(|e| format!("截图失败: {}", e))?;

    #[cfg(target_os = "windows")]
    let buffer = image
        .to_png(None)
        .map_err(|e| format!("图片编码失败: {}", e))?;

    #[cfg(target_os = "macos")]
    let buffer = {
        use screenshots::image::ImageFormat;
        use std::io::Cursor;
        let mut buf = Vec::new();
        image
            .write_to(&mut Cursor::new(&mut buf), ImageFormat::Png)
            .map_err(|e| format!("图片编码失败: {}", e))?;
        buf
    };

    run_ocr_on_image_data(buffer, state).await
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            // 初始化数据库
            let db = Database::new(app.handle()).expect("数据库初始化失败");

            // 初始化翻译服务 - 使用默认的OpenAI配置
            let translation_service = translation::TranslationService::OpenAI;

            // 设置应用状态
            app.manage(AppState {
                db: Mutex::new(db),
                translation_service: Mutex::new(translation_service),
            });

            // Register shortcuts
            register_shortcuts(app.handle());
            setup_system_tray(&app.handle())?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() == "main" {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            translate_text,
            save_translation,
            get_translation_history,
            search_history,
            clear_history,
            save_setting,
            get_setting,
            save_api_key,
            get_api_key,
            save_app_config,
            get_app_config,
            reload_shortcuts,
            capture_screen,
            capture_screen_area,
            capture_and_ocr,
            capture_area_and_ocr,
            submit_area_for_ocr,
            start_area_selection,
            set_ocr_result,
            get_supported_languages
        ])
        .run(tauri::generate_context!())
        .expect("应用启动失败");
}
