use crate::{
    app_state::AppState, commands::start_area_selection, database, system_tray::show_main_window,
};
use std::str::FromStr;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

const PREFILL_EVENT: &str = "prefill-text";

#[cfg(any(target_os = "macos", target_os = "windows"))]
const COPY_SHORTCUT_ATTEMPTS: usize = 3;

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
            // Ã§â€ºÂ´Ã¦Å½Â¥Ã¤Â½Â¿Ã§â€Â¨ on_shortcutÃ¯Â¼Å’Ã¤Â¸ÂÃ©Å“â‚¬Ã¨Â¦ÂÃ¥â€¦Ë† register
            let result =
                app.global_shortcut()
                    .on_shortcut(shortcut, move |_app, _shortcut, event| {
                        eprintln!("Ã°Å¸Å½Â¯ SHORTCUT TRIGGERED: {:?}", shortcut);
                        eprintln!("Event state: {:?}", event.state);
                        if event.state == ShortcutState::Pressed {
                            if app_handle.get_webview_window("main").is_some() {
                                eprintln!("Ã¢Å“â€œ Window found, forcing focus...");
                                show_main_window(&app_handle);
                            } else {
                                eprintln!("Ã¢Å“â€” Window 'main' NOT FOUND!");
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
        eprintln!("Ã¥ÂÂ¯Ã¥Å Â¨Ã¥Å’ÂºÃ¥Å¸Å¸Ã¦Ë†ÂªÃ¥â€ºÂ¾Ã¥Â¤Â±Ã¨Â´Â¥: {}", err);
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
    eprintln!("Ã°Å¸â€œÂ [Capture] Starting text capture...");

    #[cfg(target_os = "windows")]
    {
        if let Some(text) = capture_via_windows_ui_automation() {
            eprintln!(
                "Ã¢Å“â€¦ [Capture] Success via UI Automation: {}",
                truncate_for_display(&text, 50)
            );
            return Some(text);
        }
        eprintln!("Ã¢Å¡ Ã¯Â¸Â  [Capture] UI Automation failed, trying clipboard...");
    }

    #[cfg(target_os = "macos")]
    {
        match capture_via_macos_accessibility() {
            Some(text) if !text.trim().is_empty() => {
                eprintln!(
                    "Ã¢Å“â€¦ [Capture] Success via accessibility: {}",
                    truncate_for_display(&text, 50)
                );
                return Some(text);
            }
            Some(_) => {
                eprintln!("Ã¢ÂÅ’ [Capture] Accessibility returned empty text, continuing...");
            }
            None => {
                eprintln!(
                    "Ã¢Å¡ Ã¯Â¸Â  [Capture] Accessibility API capture failed, trying clipboard..."
                );
            }
        }
    }

    #[cfg(target_os = "linux")]
    let mut should_try_direct_clipboard = false;

    if let Some(text) = capture_via_primary_selection() {
        if !looks_like_file_path(&text) {
            eprintln!(
                "Ã¢Å“â€¦ [Capture] Success: {}",
                truncate_for_display(&text, 50)
            );
            return Some(text);
        } else {
            eprintln!(
                "Ã¢Å¡ Ã¯Â¸Â  [Capture] Got file path from primary selection: {:?}",
                text
            );
            #[cfg(target_os = "linux")]
            {
                should_try_direct_clipboard = true;
            }
            #[cfg(not(target_os = "linux"))]
            {
                eprintln!(
                    "Ã¢ÂÅ’ [Capture] Skipping clipboard fallback on this platform (file path detected)"
                );
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
                "Ã¢ÂÅ’ [Capture] Primary selection capture failed and clipboard fallback is disabled"
            );
        }
    }

    #[cfg(target_os = "linux")]
    {
        if should_try_direct_clipboard {
            eprintln!(
                "Ã¢ÂÂ­Ã¯Â¸Â  [Capture] Primary selection unavailable, trying direct clipboard..."
            );

            if let Some(text) = read_clipboard_directly() {
                eprintln!(
                    "Ã¢Å“â€¦ [Capture] Success via direct clipboard: {}",
                    truncate_for_display(&text, 50)
                );
                return Some(text);
            }
        }
    }

    None
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

#[cfg(target_os = "windows")]
fn capture_via_windows_ui_automation() -> Option<String> {
    use windows::{
        core::{Interface, BSTR},
        Win32::{
            System::Com::{
                CoCreateInstance, CoInitializeEx, CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED,
            },
            UI::Accessibility::{
                CUIAutomation, IUIAutomation, IUIAutomationLegacyIAccessiblePattern,
                IUIAutomationTextPattern, IUIAutomationValuePattern,
                UIA_LegacyIAccessiblePatternId, UIA_TextPattern2Id, UIA_TextPatternId,
                UIA_ValuePatternId,
            },
        },
    };

    eprintln!("Ã°Å¸â€Â [UI Automation] Starting capture...");

    unsafe {
        // Initialize COM
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

        let automation: IUIAutomation =
            match CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER) {
                Ok(a) => a,
                Err(e) => {
                    eprintln!(
                        "Ã¢ÂÅ’ [UI Automation] Failed to create CUIAutomation: {}",
                        e
                    );
                    return None;
                }
            };

        let element = match automation.GetFocusedElement() {
            Ok(e) => e,
            Err(e) => {
                eprintln!(
                    "Ã¢ÂÅ’ [UI Automation] Failed to get focused element: {}",
                    e
                );
                return None;
            }
        };

        // Try TextPattern
        if let Ok(pattern) = element.GetCurrentPattern(UIA_TextPatternId) {
            let pattern: IUIAutomationTextPattern = match pattern.cast() {
                Ok(p) => p,
                Err(_) => return None,
            };

            if let Ok(ranges) = pattern.GetSelection() {
                if let Ok(length) = ranges.Length() {
                    if length > 0 {
                        if let Ok(range) = ranges.GetElement(0) {
                            if let Ok(text) = range.GetText(-1) {
                                let text_str = text.to_string();
                                if !text_str.is_empty() {
                                    return Some(text_str);
                                }
                            }
                        }
                    }
                }
            }
        } else if let Ok(pattern) = element.GetCurrentPattern(UIA_TextPattern2Id) {
            // Fallback to TextPattern2 (though it inherits from TextPattern, sometimes casting helps?)
            let pattern: IUIAutomationTextPattern = match pattern.cast() {
                Ok(p) => p,
                Err(_) => return None,
            };
            if let Ok(ranges) = pattern.GetSelection() {
                if let Ok(length) = ranges.Length() {
                    if length > 0 {
                        if let Ok(range) = ranges.GetElement(0) {
                            if let Ok(text) = range.GetText(-1) {
                                let text_str = text.to_string();
                                if !text_str.is_empty() {
                                    return Some(text_str);
                                }
                            }
                        }
                    }
                }
            }
        } else if let Ok(pattern) = element.GetCurrentPattern(UIA_ValuePatternId) {
            // Fallback to ValuePattern (e.g. for simple text boxes)
            let pattern: IUIAutomationValuePattern = match pattern.cast() {
                Ok(p) => p,
                Err(_) => return None,
            };
            if let Ok(value) = pattern.CurrentValue() {
                let value_str = value.to_string();
                if !value_str.is_empty() {
                    return Some(value_str);
                }
            }
        } else if let Ok(pattern) = element.GetCurrentPattern(UIA_LegacyIAccessiblePatternId) {
            // Fallback to LegacyIAccessiblePattern (older apps)
            let pattern: IUIAutomationLegacyIAccessiblePattern = match pattern.cast() {
                Ok(p) => p,
                Err(_) => return None,
            };

            // Try CurrentValue first
            if let Ok(value) = pattern.CurrentValue() {
                let value_str = value.to_string();
                if !value_str.is_empty() {
                    return Some(value_str);
                }
            }

            // If no value, maybe it's a selection? LegacyIAccessible doesn't have a direct "Selection" text method like TextPattern.
            // But sometimes CurrentName holds the text for static text controls.
            // However, we want *selected* text. LegacyIAccessible doesn't easily give *selected* text unless the whole control is the selection.
            // We'll stick to CurrentValue for now as it maps to "Value" of the control.
        } else {
            // Log debug info
            let name = element.CurrentName().unwrap_or(BSTR::new());
            let class_name = element.CurrentClassName().unwrap_or(BSTR::new());
            let control_type = element
                .CurrentControlType()
                .unwrap_or(windows::Win32::UI::Accessibility::UIA_CONTROLTYPE_ID(0));

            eprintln!(
                "Ã¢Å¡ Ã¯Â¸Â  [UI Automation] Focused element does not support TextPattern. Name: '{}', Class: '{}', ControlType: {}",
                name, class_name, control_type.0
            );
        }
    }

    None
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn capture_via_primary_selection() -> Option<String> {
    use arboard::Clipboard;
    use std::time::{Duration, Instant};

    eprintln!("Ã°Å¸â€Â [Primary Selection] Starting capture...");

    let mut clipboard = Clipboard::new().ok()?;
    let original_clipboard = clipboard.get_text().ok();

    eprintln!(
        "Ã°Å¸â€œâ€¹ [Primary Selection] Original clipboard: {:?}",
        original_clipboard
            .as_ref()
            .map(|s| truncate_for_display(s, 50))
    );

    let mut captured_text = None;

    // Total attempts to trigger the shortcut
    for attempt in 1..=COPY_SHORTCUT_ATTEMPTS {
        eprintln!(
            "Ã¢Å’Â¨Ã¯Â¸Â  [Primary Selection] Triggering copy shortcut (attempt {}/{})...",
            attempt, COPY_SHORTCUT_ATTEMPTS
        );
        trigger_copy_shortcut();

        // Polling loop: check clipboard every 50ms for up to 400ms (increasing with attempts)
        let poll_duration = Duration::from_millis(200 + (attempt as u64 * 100));
        let start_time = Instant::now();
        let poll_interval = Duration::from_millis(50);

        loop {
            std::thread::sleep(poll_interval);

            let new_clipboard = clipboard.get_text().ok();

            match (&original_clipboard, &new_clipboard) {
                (Some(orig), Some(new)) if orig != new => {
                    eprintln!(
                        "Ã¢Å“â€¦ [Primary Selection] Captured new text on attempt {} (took {:?})",
                        attempt,
                        start_time.elapsed()
                    );
                    captured_text = Some(new.clone());
                    break;
                }
                (None, Some(new)) if !new.trim().is_empty() => {
                    eprintln!(
                        "Ã¢Å“â€¦ [Primary Selection] Captured new text on attempt {} (took {:?})",
                        attempt,
                        start_time.elapsed()
                    );
                    captured_text = Some(new.clone());
                    break;
                }
                _ => {}
            }

            if start_time.elapsed() > poll_duration {
                eprintln!(
                    "Ã¢ÂÂ³ [Primary Selection] Timeout waiting for clipboard change on attempt {}",
                    attempt
                );
                break;
            }
        }

        if captured_text.is_some() {
            break;
        }
    }

    if let Some(original) = original_clipboard {
        let _ = clipboard.set_text(original);
        eprintln!("Ã°Å¸â€â€ž [Primary Selection] Restored original clipboard");
    } else {
        let _ = clipboard.clear();
        eprintln!("Ã°Å¸â€â€ž [Primary Selection] Cleared clipboard (was empty)");
    }

    captured_text.filter(|text| !text.trim().is_empty())
}

#[cfg(target_os = "linux")]
fn capture_via_primary_selection() -> Option<String> {
    use arboard::{Clipboard, GetExtLinux, LinuxClipboardKind};

    eprintln!("Ã°Å¸â€Â [Primary Selection] Reading from Linux primary selection...");
    let mut clipboard = Clipboard::new().ok()?;

    match clipboard
        .get()
        .clipboard(LinuxClipboardKind::Primary)
        .text()
    {
        Ok(text) => {
            if text.trim().is_empty() {
                eprintln!("Ã¢ÂÅ’ [Primary Selection] Primary selection was empty");
                None
            } else {
                eprintln!(
                    "Ã¢Å“â€¦ [Primary Selection] Captured text from primary selection: {}",
                    truncate_for_display(&text, 50)
                );
                Some(text)
            }
        }
        Err(err) => {
            eprintln!(
                "Ã¢ÂÅ’ [Primary Selection] Failed to read primary selection via arboard: {}",
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
            "Ã¢ÂÅ’ [Capture] Accessibility script failed with status: {}",
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
    use scopeguard::guard;
    use std::{thread, time::Duration};
    use windows::Win32::System::Console::SetConsoleCtrlHandler;
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VK_C, VK_CONTROL,
    };

    unsafe {
        // Set up a scope guard to temporarily ignore Ctrl+C signals
        let _guard = guard((), |_| {
            // This will be called when the guard goes out of scope
            // Restore default Ctrl+C handler
            let _ = SetConsoleCtrlHandler(None, false);
        });

        // Temporarily disable Ctrl+C handling to prevent the app from exiting
        let _ = SetConsoleCtrlHandler(None, true);

        let inputs = [
            // Press Ctrl
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_CONTROL,
                        ..Default::default()
                    },
                },
            },
            // Press C
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_C,
                        ..Default::default()
                    },
                },
            },
            // Release C
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_C,
                        dwFlags: KEYEVENTF_KEYUP,
                        ..Default::default()
                    },
                },
            },
            // Release Ctrl
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_CONTROL,
                        dwFlags: KEYEVENTF_KEYUP,
                        ..Default::default()
                    },
                },
            },
        ];

        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
        thread::sleep(Duration::from_millis(50));

        // The guard will automatically restore the Ctrl+C handler when it goes out of scope
    }
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn trigger_copy_shortcut() {}
