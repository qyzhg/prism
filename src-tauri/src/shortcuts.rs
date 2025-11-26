use crate::{app_state::AppState, commands::start_area_selection, database};
use arboard::Clipboard;
use std::str::FromStr;
use tauri::{AppHandle, Emitter, Manager, WebviewWindow};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

const PREFILL_EVENT: &str = "prefill-text";

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
                            if let Some(window) = app_handle.get_webview_window("main") {
                                eprintln!("✓ Window found, showing...");
                                let _ = window.show();
                                let _ = window.set_focus();
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
    let _ = start_area_selection(app_handle).await;
}

async fn handle_slide_translation_shortcut(app_handle: AppHandle) {
    // Copy the selected text before focusing our window so we don't lose the original selection.
    let selected_text = capture_selected_text();
    let _ = show_main_window(&app_handle);

    if let Some(text) = selected_text {
        if let Some(window) = app_handle.get_webview_window("main") {
            let _ = window.emit(PREFILL_EVENT, text);
        }
    }
}

fn show_main_window(app_handle: &AppHandle) -> Option<WebviewWindow> {
    app_handle.get_webview_window("main").inspect(|window| {
        let _ = window.show();
        let _ = window.set_focus();
    })
}

fn capture_selected_text() -> Option<String> {
    capture_selected_text_without_clipboard().or_else(capture_selected_text_via_clipboard)
}

#[cfg(target_os = "macos")]
fn capture_selected_text_without_clipboard() -> Option<String> {
    capture_selected_text_macos()
}

#[cfg(target_os = "windows")]
fn capture_selected_text_without_clipboard() -> Option<String> {
    capture_selected_text_windows()
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn capture_selected_text_without_clipboard() -> Option<String> {
    None
}

#[cfg(target_os = "macos")]
fn capture_selected_text_macos() -> Option<String> {
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
        return None;
    }

    let text = String::from_utf8_lossy(&output.stdout)
        .trim()
        .trim_matches('\u{0}')
        .to_string();
    if text.is_empty() || text == "missing value" {
        None
    } else {
        Some(text)
    }
}

#[cfg(target_os = "windows")]
fn capture_selected_text_windows() -> Option<String> {
    use std::process::Command;

    let script = r#"
        Add-Type -AssemblyName UIAutomationClient | Out-Null
        $focused = [System.Windows.Automation.AutomationElement]::FocusedElement
        if ($null -ne $focused) {
            $textPattern = $null
            try {
                $textPattern = $focused.GetCurrentPattern([System.Windows.Automation.TextPattern]::Pattern)
            } catch {}
            if ($null -ne $textPattern) {
                $ranges = $textPattern.GetSelection()
                if ($ranges.Length -gt 0) {
                    $txt = $ranges[0].GetText(-1)
                    if ($txt) { $txt }
                }
            } else {
                $valuePattern = $null
                try {
                    $valuePattern = $focused.GetCurrentPattern([System.Windows.Automation.ValuePattern]::Pattern)
                } catch {}
                if ($null -ne $valuePattern) {
                    $txt = $valuePattern.Current.Value
                    if ($txt) { $txt }
                }
            }
        }
    "#;

    let output = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let text = String::from_utf8_lossy(&output.stdout)
        .trim()
        .trim_matches('\u{0}')
        .to_string();
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

fn capture_selected_text_via_clipboard() -> Option<String> {
    trigger_copy_shortcut();
    std::thread::sleep(std::time::Duration::from_millis(120));

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
