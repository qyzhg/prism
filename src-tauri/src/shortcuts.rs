use crate::{
    app_state::AppState, commands::start_area_selection, database, system_tray::show_main_window,
};
use std::str::FromStr;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

const PREFILL_EVENT: &str = "prefill-text";

#[cfg(any(target_os = "macos", target_os = "windows"))]
const COPY_SHORTCUT_ATTEMPTS: usize = 3;
#[cfg(any(target_os = "macos", target_os = "windows"))]
const COPY_RETRY_BASE_DELAY_MS: u64 = 180;
#[cfg(any(target_os = "macos", target_os = "windows"))]
const COPY_RETRY_DELAY_STEP_MS: u64 = 90;

pub fn register_shortcuts(app: &AppHandle) {
    eprintln!("register_shortcuts()");
    let state = app.state::<AppState>();
    let db = state.db.lock().unwrap();

    let _ = app.global_shortcut().unregister_all();

    let hotkeys = if let Ok(Some(config)) = db.get_app_config() {
        config.hotkeys
    } else {
        database::HotkeyConfig::platform_default()
    };

    if !hotkeys.popup_window.is_empty() {
        if let Ok(shortcut) = Shortcut::from_str(&hotkeys.popup_window) {
            let app_handle = app.clone();
            // 直接使用 on_shortcut，不需要先 register
            let result =
                app.global_shortcut()
                    .on_shortcut(shortcut, move |_app, _shortcut, event| {
                        eprintln!("🎯 SHORTCUT TRIGGERED: {:?}", shortcut);
                        eprintln!("Event state: {:?}", event.state);
                        if event.state == ShortcutState::Pressed {
                            if app_handle.get_webview_window("main").is_some() {
                                eprintln!("✓ Window found, forcing focus...");
                                show_main_window(&app_handle);
                            } else {
                                eprintln!("✗ Window 'main' NOT FOUND!");
                            }
                        }
                    });
            match result {
                Ok(_) => println!("Registered popup window shortcut: {}", hotkeys.popup_window),
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
            let result =
                app.global_shortcut()
                    .on_shortcut(shortcut, move |_app, _shortcut, event| {
                        if event.state == ShortcutState::Pressed {
                            let handle = app_handle.clone();
                            tauri::async_runtime::spawn(async move {
                                handle_area_ocr_shortcut(handle).await;
                            });
                        }
                    });
            match result {
                Ok(_) => println!(
                    "Registered screenshot translation shortcut: {}",
                    hotkeys.screenshot_translation
                ),
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
            let app_handle = app.clone();
            let result =
                app.global_shortcut()
                    .on_shortcut(shortcut, move |_app, _shortcut, event| {
                        if event.state == ShortcutState::Pressed {
                            let handle = app_handle.clone();
                            tauri::async_runtime::spawn(async move {
                                handle_slide_translation_shortcut(handle).await;
                            });
                        }
                    });
            match result {
                Ok(_) => println!(
                    "Registered slide translation shortcut: {}",
                    hotkeys.slide_translation
                ),
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
async fn handle_area_ocr_shortcut(app_handle: AppHandle) {
    let handle_for_recovery = app_handle.clone();
    if let Err(err) = start_area_selection(app_handle).await {
        eprintln!("启动区域截图失败: {}", err);
        show_main_window(&handle_for_recovery);
    }
}

async fn handle_slide_translation_shortcut(app_handle: AppHandle) {
    let selected_text = capture_selected_text();
    show_main_window(&app_handle);

    if let Some(window) = app_handle.get_webview_window("main") {
        let payload = selected_text.unwrap_or_default();
        let _ = window.emit(PREFILL_EVENT, payload);
    }
}

fn truncate_for_display(s: &str, max_chars: usize) -> String {
    let char_count = s.chars().count();
    if char_count <= max_chars {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_chars).collect();
        format!("{}... ({} chars)", truncated, char_count)
    }
}

fn capture_selected_text() -> Option<String> {
    eprintln!("📝 [Capture] Starting text capture...");

    #[cfg(target_os = "macos")]
    {
        match capture_via_macos_accessibility() {
            Some(text) if !text.trim().is_empty() => {
                eprintln!(
                    "✅ [Capture] Success via accessibility: {}",
                    truncate_for_display(&text, 50)
                );
                return Some(text);
            }
            Some(_) => {
                eprintln!("❌ [Capture] Accessibility returned empty text, continuing...");
            }
            None => {
                eprintln!("⚠️  [Capture] Accessibility API capture failed, trying clipboard...");
            }
        }
    }

    #[cfg(target_os = "linux")]
    let mut should_try_direct_clipboard = false;

    if let Some(text) = capture_via_primary_selection() {
        if !looks_like_file_path(&text) {
            eprintln!("✅ [Capture] Success: {}", truncate_for_display(&text, 50));
            return Some(text);
        } else {
            eprintln!(
                "⚠️  [Capture] Got file path from primary selection: {:?}",
                text
            );
            #[cfg(target_os = "linux")]
            {
                should_try_direct_clipboard = true;
            }
            #[cfg(not(target_os = "linux"))]
            {
                eprintln!(
                    "❌ [Capture] Skipping clipboard fallback on this platform (file path detected)"
                );
                return None;
            }
        }
    } else {
        #[cfg(target_os = "linux")]
        {
            should_try_direct_clipboard = true;
        }
        #[cfg(not(target_os = "linux"))]
        {
            eprintln!(
                "❌ [Capture] Primary selection capture failed and clipboard fallback is disabled"
            );
            return None;
        }
    }

    #[cfg(target_os = "linux")]
    {
        if should_try_direct_clipboard {
            eprintln!("⏭️  [Capture] Primary selection unavailable, trying direct clipboard...");

            if let Some(text) = read_clipboard_directly() {
                eprintln!(
                    "✅ [Capture] Success via direct clipboard: {}",
                    truncate_for_display(&text, 50)
                );
                return Some(text);
            }
        }
    }
}

fn looks_like_file_path(text: &str) -> bool {
    let trimmed = text.trim();

    let is_path = trimmed.starts_with('/')
        || trimmed.starts_with("C:\\")
        || trimmed.starts_with("file://")
        || trimmed.ends_with(".resolved")
        || trimmed.ends_with(".md")
        || trimmed.ends_with(".rs")
        || trimmed.ends_with(".js")
        || trimmed.ends_with(".ts")
        || trimmed.ends_with(".json")
        || trimmed.ends_with(".txt");

    let has_path_structure =
        (trimmed.contains('/') || trimmed.contains('\\')) && (trimmed.len() > 20);

    is_path || has_path_structure
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn capture_via_primary_selection() -> Option<String> {
    use arboard::Clipboard;

    eprintln!("🔍 [Primary Selection] Starting capture...");

    let mut clipboard = Clipboard::new().ok()?;
    let original_clipboard = clipboard.get_text().ok();

    eprintln!(
        "📋 [Primary Selection] Original clipboard: {:?}",
        original_clipboard
            .as_ref()
            .map(|s| truncate_for_display(s, 50))
    );

    let mut captured_text = None;

    for attempt in 1..=COPY_SHORTCUT_ATTEMPTS {
        eprintln!(
            "⌨️  [Primary Selection] Triggering copy shortcut (attempt {}/{})...",
            attempt, COPY_SHORTCUT_ATTEMPTS
        );
        trigger_copy_shortcut();
        let delay = COPY_RETRY_BASE_DELAY_MS
            + COPY_RETRY_DELAY_STEP_MS.saturating_mul((attempt - 1) as u64);
        std::thread::sleep(std::time::Duration::from_millis(delay));

        let new_clipboard = clipboard.get_text().ok();

        eprintln!(
            "📋 [Primary Selection] New clipboard (attempt {}/{}) : {:?}",
            attempt,
            COPY_SHORTCUT_ATTEMPTS,
            new_clipboard.as_ref().map(|s| truncate_for_display(s, 50))
        );

        match (&original_clipboard, &new_clipboard) {
            (Some(orig), Some(new)) if orig != new => {
                eprintln!(
                    "✅ [Primary Selection] Captured new text on attempt {}",
                    attempt
                );
                captured_text = Some(new.clone());
                break;
            }
            (None, Some(new)) if !new.trim().is_empty() => {
                eprintln!(
                    "✅ [Primary Selection] Captured new text on attempt {} (clipboard was empty)",
                    attempt
                );
                captured_text = Some(new.clone());
                break;
            }
            _ => {
                if attempt == COPY_SHORTCUT_ATTEMPTS {
                    eprintln!(
                        "❌ [Primary Selection] No new text after {} attempts (clipboard unchanged)",
                        COPY_SHORTCUT_ATTEMPTS
                    );
                }
            }
        }
    }

    if let Some(original) = original_clipboard {
        let _ = clipboard.set_text(original);
        eprintln!("🔄 [Primary Selection] Restored original clipboard");
    } else {
        let _ = clipboard.clear();
        eprintln!("🔄 [Primary Selection] Cleared clipboard (was empty)");
    }

    captured_text.filter(|text| !text.trim().is_empty())
}

#[cfg(target_os = "linux")]
fn capture_via_primary_selection() -> Option<String> {
    use arboard::{Clipboard, GetExtLinux, LinuxClipboardKind};

    eprintln!("🔍 [Primary Selection] Reading from Linux primary selection...");
    let mut clipboard = Clipboard::new().ok()?;

    match clipboard
        .get()
        .clipboard(LinuxClipboardKind::Primary)
        .text()
    {
        Ok(text) => {
            if text.trim().is_empty() {
                eprintln!("❌ [Primary Selection] Primary selection was empty");
                None
            } else {
                eprintln!(
                    "✅ [Primary Selection] Captured text from primary selection: {}",
                    truncate_for_display(&text, 50)
                );
                Some(text)
            }
        }
        Err(err) => {
            eprintln!(
                "❌ [Primary Selection] Failed to read primary selection via arboard: {}",
                err
            );
            None
        }
    }
}

#[cfg(all(
    not(target_os = "macos"),
    not(target_os = "windows"),
    not(target_os = "linux")
))]
fn capture_via_primary_selection() -> Option<String> {
    None
}

#[cfg(target_os = "macos")]
fn capture_via_macos_accessibility() -> Option<String> {
    use std::process::Command;

    let script = r#"
        set selectedText to ""
        try
            tell application "System Events"
                set frontApp to first application process whose frontmost is true
                set focusedElement to value of attribute "AXFocusedUIElement" of frontApp
                try
                    set selectedText to value of attribute "AXSelectedText" of focusedElement
                on error
                    try
                        set selectedText to value of attribute "AXValue" of focusedElement
                    on error
                        set selectedText to ""
                    end try
                end try
            end tell
        end try
        selectedText
    "#;

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .ok()?;

    if !output.status.success() {
        eprintln!(
            "❌ [Capture] Accessibility script failed with status: {}",
            output.status
        );
        return None;
    }

    let text = String::from_utf8_lossy(&output.stdout)
        .trim_matches('\u{0}')
        .trim()
        .to_string();

    if text.is_empty() || text == "missing value" {
        None
    } else {
        Some(text)
    }
}

#[cfg(target_os = "linux")]
fn read_clipboard_directly() -> Option<String> {
    use arboard::Clipboard;

    let mut clipboard = Clipboard::new().ok()?;
    let text = clipboard.get_text().ok()?;
    if text.trim().is_empty() {
        None
    } else {
        Some(text)
    }
}

#[cfg(target_os = "macos")]
fn trigger_copy_shortcut() {
    use std::process::Command;

    let script = r#"tell application "System Events" to keystroke "c" using {command down}"#;
    let _ = Command::new("osascript").arg("-e").arg(script).output();
}

#[cfg(target_os = "windows")]
fn trigger_copy_shortcut() {
    use std::process::Command;

    let script = r#"Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.SendKeys]::SendWait('^c'); Start-Sleep -Milliseconds 50"#;
    let _ = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-Command")
        .arg(script)
        .output();
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn trigger_copy_shortcut() {}
