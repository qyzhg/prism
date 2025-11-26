use crate::system_tray::show_main_window;
use screenshots::Screen;
use tauri::{AppHandle, Manager, PhysicalPosition, PhysicalSize, Position, Size, WebviewUrl};

pub async fn start_area_selection(app_handle: AppHandle) -> Result<(), String> {
    // Hide main window before showing area selector
    let mut main_window_hidden = false;
    if let Some(main_window) = app_handle.get_webview_window("main") {
        main_window
            .hide()
            .map_err(|e| format!("隐藏主窗口失败: {}", e))?;
        main_window_hidden = true;
    }

    let setup_result = (|| -> Result<(), String> {
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

        let width = (max_x - min_x) as u32;
        let height = (max_y - min_y) as u32;

        let window = tauri::WebviewWindowBuilder::new(
            &app_handle,
            "area-selector",
            WebviewUrl::App("area-selector.html".into()),
        )
        .title("区域选择")
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .resizable(false)
        .transparent(true)
        .visible(false)
        .build()
        .map_err(|e| format!("创建区域选择窗口失败: {}", e))?;

        window
            .set_position(Position::Physical(PhysicalPosition { x: min_x, y: min_y }))
            .map_err(|e| format!("设置区域选择窗口位置失败: {}", e))?;

        window
            .set_size(Size::Physical(PhysicalSize { width, height }))
            .map_err(|e| format!("设置区域选择窗口大小失败: {}", e))?;

        window
            .show()
            .map_err(|e| format!("显示区域选择窗口失败: {}", e))?;
        window
            .set_focus()
            .map_err(|e| format!("设置区域选择窗口焦点失败: {}", e))?;

        Ok(())
    })();

    if setup_result.is_err() && main_window_hidden {
        show_main_window(&app_handle);
    }

    setup_result
}
