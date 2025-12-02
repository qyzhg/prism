use chrono::Utc;
use rusqlite::{params, Connection, Error as RusqliteError, ErrorCode, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TranslationRecord {
    pub id: Option<i64>,
    pub original_text: String,
    pub translated_text: String,
    pub service: String,
    pub from_language: Option<String>,
    pub to_language: Option<String>,
    pub created_at: Option<String>,
}

fn default_service() -> String {
    "openai".to_string()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TranslationConfig {
    #[serde(default = "default_service")]
    pub service: String,
    pub base_url: String,
    pub api_key: String,
    pub model_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OcrConfig {
    pub base_url: String,
    pub api_key: String,
    pub model_id: String,
    pub reuse_translation: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HotkeyConfig {
    pub popup_window: String,
    pub slide_translation: String,
    pub screenshot_translation: String,
}

impl HotkeyConfig {
    pub fn platform_default() -> Self {
        let (popup, slide, screenshot) = if cfg!(target_os = "macos") {
            ("Option+A", "Option+D", "Option+S")
        } else {
            ("Alt+A", "Alt+D", "Alt+S")
        };

        HotkeyConfig {
            popup_window: popup.to_string(),
            slide_translation: slide.to_string(),
            screenshot_translation: screenshot.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ProxyMode {
    System,
    Https,
    Http,
    Socks5,
}

impl Default for ProxyMode {
    fn default() -> Self {
        ProxyMode::System
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProxyConfig {
    pub enabled: bool,
    pub mode: ProxyMode,
    pub server: String,
}

impl Default for ProxyConfig {
    fn default() -> Self {
        ProxyConfig {
            enabled: false,
            mode: ProxyMode::System,
            server: String::new(),
        }
    }
}

fn default_user_max_tokens() -> u32 {
    4096
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenLimitConfig {
    #[serde(default)]
    pub enable_user_max_tokens: bool,
    #[serde(default = "default_user_max_tokens")]
    pub user_max_tokens: u32,
}

impl Default for TokenLimitConfig {
    fn default() -> Self {
        TokenLimitConfig {
            enable_user_max_tokens: false,
            user_max_tokens: default_user_max_tokens(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AutostartConfig {
    #[serde(default)]
    pub enabled: bool,
}

impl Default for AutostartConfig {
    fn default() -> Self {
        AutostartConfig { enabled: false }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub translation: TranslationConfig,
    pub ocr: OcrConfig,
    pub hotkeys: HotkeyConfig,
    #[serde(default)]
    pub proxy: ProxyConfig,
    #[serde(default)]
    pub token_limits: TokenLimitConfig,
    #[serde(default)]
    pub autostart: AutostartConfig,
}

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new(app_handle: &AppHandle) -> Result<Self> {
        let app_dir = resolve_app_data_dir(app_handle).map_err(io_to_rusqlite_error)?;

        // 确保应用数据目录存在
        fs::create_dir_all(&app_dir).map_err(io_to_rusqlite_error)?;

        println!("配置目录: {}", app_dir.display());
        let db_path = app_dir.join("trans.db");
        let conn = Connection::open(db_path)?;
        let conn = Arc::new(Mutex::new(conn));

        let db = Database { conn };
        db.init_tables()?;
        Ok(db)
    }

    fn init_tables(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        // 创建翻译历史表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS translation_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                original_text TEXT NOT NULL,
                translated_text TEXT NOT NULL,
                service TEXT NOT NULL,
                from_language TEXT,
                to_language TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // 创建用户设置表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS user_settings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                key TEXT UNIQUE NOT NULL,
                value TEXT NOT NULL,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // 创建 API 密钥表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS api_keys (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                service TEXT UNIQUE NOT NULL,
                api_key TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // 创建索引
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_translation_history_created_at 
             ON translation_history(created_at DESC)",
            [],
        )?;

        Ok(())
    }

    // 保存翻译记录
    pub fn save_translation(&self, record: &TranslationRecord) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let created_at = record
            .created_at
            .clone()
            .unwrap_or_else(|| Utc::now().to_rfc3339());

        conn.execute(
            "INSERT INTO translation_history 
             (original_text, translated_text, service, from_language, to_language, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                record.original_text,
                record.translated_text,
                record.service,
                record.from_language,
                record.to_language,
                created_at
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    // 获取翻译历史
    pub fn get_translation_history(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<TranslationRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, original_text, translated_text, service, from_language, to_language, 
                    created_at
             FROM translation_history 
             ORDER BY datetime(created_at) DESC
             LIMIT ?1 OFFSET ?2",
        )?;

        let rows = stmt.query_map(params![limit.unwrap_or(50), offset.unwrap_or(0)], |row| {
            Ok(TranslationRecord {
                id: Some(row.get(0)?),
                original_text: row.get(1)?,
                translated_text: row.get(2)?,
                service: row.get(3)?,
                from_language: row.get(4)?,
                to_language: row.get(5)?,
                created_at: Some(row.get(6)?),
            })
        })?;

        let mut records = Vec::new();
        for row in rows {
            records.push(row?);
        }
        Ok(records)
    }

    // 搜索翻译历史
    pub fn search_history(
        &self,
        keyword: &str,
        limit: Option<i32>,
    ) -> Result<Vec<TranslationRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, original_text, translated_text, service, from_language, to_language, 
                    created_at
             FROM translation_history 
             WHERE original_text LIKE ?1 OR translated_text LIKE ?1
             ORDER BY datetime(created_at) DESC
             LIMIT ?2",
        )?;

        let search_pattern = format!("%{}%", keyword);
        let rows = stmt.query_map(params![search_pattern, limit.unwrap_or(50)], |row| {
            Ok(TranslationRecord {
                id: Some(row.get(0)?),
                original_text: row.get(1)?,
                translated_text: row.get(2)?,
                service: row.get(3)?,
                from_language: row.get(4)?,
                to_language: row.get(5)?,
                created_at: Some(row.get(6)?),
            })
        })?;

        let mut records = Vec::new();
        for row in rows {
            records.push(row?);
        }
        Ok(records)
    }

    // 保存用户设置
    pub fn save_setting(&self, key: &str, value: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO user_settings (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
        Ok(())
    }

    // 获取用户设置
    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT value FROM user_settings WHERE key = ?1")?;

        let mut rows = stmt.query_map(params![key], |row| row.get::<_, String>(0))?;

        if let Some(row) = rows.next() {
            Ok(Some(row?))
        } else {
            Ok(None)
        }
    }

    // 保存 API 密钥
    pub fn save_api_key(&self, service: &str, api_key: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO api_keys (service, api_key) VALUES (?1, ?2)",
            params![service, api_key],
        )?;
        Ok(())
    }

    // 获取 API 密钥
    pub fn get_api_key(&self, service: &str) -> Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT api_key FROM api_keys WHERE service = ?1")?;

        let mut rows = stmt.query_map(params![service], |row| row.get::<_, String>(0))?;

        if let Some(row) = rows.next() {
            Ok(Some(row?))
        } else {
            Ok(None)
        }
    }

    // 删除翻译记录
    #[allow(dead_code)]
    pub fn delete_translation(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM translation_history WHERE id = ?1", params![id])?;
        Ok(())
    }

    // 清空翻译历史
    pub fn clear_history(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM translation_history", [])?;
        Ok(())
    }

    // 保存应用配置
    pub fn save_app_config(&self, config: &AppConfig) -> Result<()> {
        println!("正在保存应用配置到数据库...");
        let config_json = serde_json::to_string(config)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

        match self.save_setting("app_config", &config_json) {
            Ok(_) => {
                println!("配置已保存到数据库");
                Ok(())
            }
            Err(e) => {
                eprintln!("保存配置到数据库失败: {}", e);
                Err(e)
            }
        }
    }

    // 获取应用配置
    pub fn get_app_config(&self) -> Result<Option<AppConfig>> {
        println!("尝试从数据库加载配置...");
        if let Some(config_json) = self.get_setting("app_config")? {
            match serde_json::from_str::<AppConfig>(&config_json) {
                Ok(config) => {
                    println!("从数据库加载配置成功");
                    Ok(Some(config))
                }
                Err(e) => {
                    eprintln!("解析数据库中的配置失败: {}", e);
                    // 解析失败时返回错误，以便前端知道发生了什么
                    Err(rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    ))
                }
            }
        } else {
            println!("未找到配置，使用默认配置");
            Ok(self.get_default_config())
        }
    }

    fn get_default_config(&self) -> Option<AppConfig> {
        Some(AppConfig {
            translation: TranslationConfig {
                service: "openai".to_string(),
                base_url: "https://api.openai.com/v1".to_string(),
                api_key: "".to_string(),
                model_id: "gpt-5-nano".to_string(),
            },
            ocr: OcrConfig {
                base_url: "https://api.openai.com/v1".to_string(),
                api_key: "".to_string(),
                model_id: "gpt-4-vision-preview".to_string(),
                reuse_translation: true,
            },
            hotkeys: HotkeyConfig::platform_default(),
            proxy: ProxyConfig::default(),
            token_limits: TokenLimitConfig::default(),
            autostart: AutostartConfig::default(),
        })
    }
}

fn io_to_rusqlite_error(err: std::io::Error) -> RusqliteError {
    RusqliteError::SqliteFailure(
        rusqlite::ffi::Error {
            code: ErrorCode::Unknown,
            extended_code: err.raw_os_error().unwrap_or(0),
        },
        Some(err.to_string()),
    )
}

fn resolve_app_data_dir(app_handle: &AppHandle) -> std::io::Result<PathBuf> {
    if let Ok(dir) = app_handle.path().app_data_dir() {
        return Ok(dir);
    }

    if let Ok(dir) = app_handle.path().app_config_dir() {
        return Ok(dir);
    }

    let identifier = app_handle.config().identifier.clone();

    if cfg!(target_os = "macos") {
        if let Some(mut dir) = dirs::home_dir() {
            dir.push("Library");
            dir.push("Application Support");
            dir.push(&identifier);
            return Ok(dir);
        }
    }

    if let Some(mut dir) = dirs::data_dir() {
        dir.push(&identifier);
        return Ok(dir);
    }

    if let Some(mut dir) = dirs::home_dir() {
        dir.push(".config");
        dir.push(&identifier);
        return Ok(dir);
    }

    let mut dir = std::env::current_dir()?;
    dir.push(&identifier);
    Ok(dir)
}
