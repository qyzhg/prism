use crate::{
    app_state::AppState,
    database::{AppConfig, TranslationRecord},
    ocr_tasks::run_ocr_on_image_data,
    platform,
    shortcuts::register_shortcuts,
    translation::{TranslationRequest, TranslationResult},
};
#[cfg(not(target_os = "macos"))]
use crate::system_tray::show_main_window;
use tauri::Emitter;
use tauri::{AppHandle, Manager, State};

type ScreenshotImage = screenshots::image::ImageBuffer<screenshots::image::Rgba<u8>, Vec<u8>>;

fn screenshot_to_dynamic_image(image: &ScreenshotImage) -> Result<image::DynamicImage, String> {
    let (width, height) = (image.width(), image.height());
    let data = image.clone().into_vec();
    let rgba = image::RgbaImage::from_raw(width, height, data)
        .ok_or_else(|| "无法转换截图到图像缓冲区".to_string())?;
    Ok(image::DynamicImage::ImageRgba8(rgba))
}

fn encode_image_to_png(image: &ScreenshotImage) -> Result<Vec<u8>, String> {
    use image::ImageFormat;
    use std::io::Cursor;

    let dynamic_image = screenshot_to_dynamic_image(image)?;
    let mut cursor = Cursor::new(Vec::new());
    dynamic_image
        .write_to(&mut cursor, ImageFormat::Png)
        .map_err(|e| format!("图片编码失败: {}", e))?;
    Ok(cursor.into_inner())
}

#[tauri::command]
pub async fn translate_text(
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
            Ok(None) => (
                "".to_string(),
                "https://api.openai.com/v1".to_string(),
                "gpt-5-nano".to_string(),
            ),
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
pub async fn save_translation(
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
pub async fn get_translation_history(
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
pub async fn search_history(
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
pub async fn clear_history(state: State<'_, AppState>) -> Result<(), String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    db.clear_history()
        .map_err(|e| format!("清空历史记录失败: {}", e))
}

#[tauri::command]
pub async fn save_setting(
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
pub async fn get_setting(
    key: String,
    state: State<'_, AppState>,
) -> Result<Option<String>, String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    db.get_setting(&key)
        .map_err(|e| format!("获取设置失败: {}", e))
}

#[tauri::command]
pub async fn save_api_key(
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
pub async fn get_api_key(
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
pub async fn save_app_config(config: AppConfig, state: State<'_, AppState>) -> Result<(), String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    db.save_app_config(&config)
        .map_err(|e| format!("保存应用配置失败: {}", e))
}

#[tauri::command]
pub async fn get_app_config(state: State<'_, AppState>) -> Result<AppConfig, String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    db.get_app_config()
        .map_err(|e| format!("获取应用配置失败: {}", e))
        .and_then(|config_opt| config_opt.ok_or_else(|| "应用配置不存在".to_string()))
}

#[tauri::command]
pub async fn reload_shortcuts(app_handle: AppHandle) -> Result<(), String> {
    register_shortcuts(&app_handle);
    Ok(())
}

#[tauri::command]
pub async fn capture_screen() -> Result<String, String> {
    use base64::Engine;
    use screenshots::Screen;

    let screens = Screen::all().map_err(|e| format!("获取屏幕列表失败: {}", e))?;

    if screens.is_empty() {
        return Err("未找到任何屏幕".to_string());
    }

    let screen = &screens[0];
    let image = screen.capture().map_err(|e| format!("截图失败: {}", e))?;

    let buffer = encode_image_to_png(&image)?;

    let base64_string = base64::engine::general_purpose::STANDARD.encode(&buffer);

    Ok(base64_string)
}

#[tauri::command]
pub async fn capture_screen_area(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<String, String> {
    use base64::Engine;
    use screenshots::Screen;

    let screens = Screen::all().map_err(|e| format!("获取屏幕列表失败: {}", e))?;

    if screens.is_empty() {
        return Err("未找到任何屏幕".to_string());
    }

    let screen = &screens[0];
    let image = screen.capture().map_err(|e| format!("截图失败: {}", e))?;

    let mut dynamic_image = screenshot_to_dynamic_image(&image)?;

    let cropped_image = dynamic_image.crop(x, y, width, height);

    let mut cropped_buffer = Vec::new();
    cropped_image
        .write_to(
            &mut std::io::Cursor::new(&mut cropped_buffer),
            image::ImageFormat::Png,
        )
        .map_err(|e| format!("编码裁剪图片失败: {}", e))?;

    let base64_string = base64::engine::general_purpose::STANDARD.encode(&cropped_buffer);

    Ok(base64_string)
}

#[tauri::command]
pub async fn start_area_selection(app_handle: AppHandle) -> Result<(), String> {
    platform::start_area_selection(app_handle).await
}

#[tauri::command]
pub async fn set_ocr_result(app_handle: AppHandle, text: String) -> Result<(), String> {
    if let Some(main_window) = app_handle.get_webview_window("main") {
        main_window
            .emit("ocr-result", text)
            .map_err(|e| format!("发送OCR结果失败: {}", e))?;
    }
    Ok(())
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub async fn submit_area_for_ocr(
    app_handle: AppHandle,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    if let Some(main_window) = app_handle.get_webview_window("main") {
        main_window
            .emit("ocr-pending", true)
            .map_err(|e| e.to_string())?;
    }
    show_main_window(&app_handle);

    let app_handle_clone = app_handle.clone();
    state.inner();

    tauri::async_runtime::spawn(async move {
        let state = app_handle_clone.state::<AppState>();

        match capture_area_and_ocr(x, y, width, height, state).await {
            Ok(text) => {
                println!("OCR成功,文本长度: {}, 内容: '{}'", text.len(), text);
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

#[tauri::command]
pub async fn capture_area_and_ocr(
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    state: State<'_, AppState>,
) -> Result<String, String> {
    use screenshots::Screen;

    let screens = Screen::all().map_err(|e| format!("获取屏幕列表失败: {}", e))?;

    if screens.is_empty() {
        return Err("未找到任何屏幕".to_string());
    }

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

    let mut dynamic_image = screenshot_to_dynamic_image(&image)?;

    let relative_x = (x - screen.display_info.x) as u32;
    let relative_y = (y - screen.display_info.y) as u32;

    let cropped_image = dynamic_image.crop(relative_x, relative_y, width, height);

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
pub async fn capture_and_ocr(state: State<'_, AppState>) -> Result<String, String> {
    use screenshots::Screen;

    let screens = Screen::all().map_err(|e| format!("获取屏幕列表失败: {}", e))?;

    if screens.is_empty() {
        return Err("未找到任何屏幕".to_string());
    }

    let screen = &screens[0];
    let image = screen.capture().map_err(|e| format!("截图失败: {}", e))?;

    let buffer = encode_image_to_png(&image)?;

    run_ocr_on_image_data(buffer, state).await
}
