use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationRequest {
    pub text: String,
    pub from_lang: String,
    pub to_lang: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationResult {
    pub translated_text: String,
    pub from_lang: String,
    pub to_lang: String,
    pub service: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationResponse {
    pub translated_text: String,
    pub source_lang: String,
    pub target_lang: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TranslationService {
    OpenAI,
    Google,
    Baidu,
}

impl TranslationService {
    pub async fn translate(
        &self,
        request: TranslationRequest,
        api_key: &str,
        base_url: &str,
        model_id: &str,
    ) -> Result<TranslationResult, String> {
        match self {
            TranslationService::OpenAI => {
                let translator = Translator::new(
                    api_key.to_string(),
                    base_url.to_string(),
                    model_id.to_string(),
                    TranslationService::OpenAI,
                );
                let response = translator.translate(request).await?;
                Ok(TranslationResult {
                    translated_text: response.translated_text,
                    from_lang: response.source_lang,
                    to_lang: response.target_lang,
                    service: "OpenAI".to_string(),
                })
            }
            TranslationService::Google => {
                let translator = Translator::new(
                    api_key.to_string(),
                    base_url.to_string(),
                    model_id.to_string(),
                    TranslationService::Google,
                );
                let response = translator.translate(request).await?;
                Ok(TranslationResult {
                    translated_text: response.translated_text,
                    from_lang: response.source_lang,
                    to_lang: response.target_lang,
                    service: "Google".to_string(),
                })
            }
            TranslationService::Baidu => {
                let translator = Translator::new(
                    api_key.to_string(),
                    base_url.to_string(),
                    model_id.to_string(),
                    TranslationService::Baidu,
                );
                let response = translator.translate(request).await?;
                Ok(TranslationResult {
                    translated_text: response.translated_text,
                    from_lang: response.source_lang,
                    to_lang: response.target_lang,
                    service: "Baidu".to_string(),
                })
            }
        }
    }
}

pub struct Translator {
    api_key: String,
    base_url: String,
    model_id: String,
    service: TranslationService,
}

impl Translator {
    pub fn new(
        api_key: String,
        base_url: String,
        model_id: String,
        service: TranslationService,
    ) -> Self {
        Self {
            api_key,
            base_url,
            model_id,
            service,
        }
    }

    pub async fn translate(
        &self,
        request: TranslationRequest,
    ) -> Result<TranslationResponse, String> {
        match self.service {
            TranslationService::OpenAI => self.translate_openai(&request).await,
            TranslationService::Google => self.translate_google(&request).await,
            TranslationService::Baidu => self.translate_baidu(&request).await,
        }
    }

    async fn translate_openai(
        &self,
        request: &TranslationRequest,
    ) -> Result<TranslationResponse, String> {
        let client = reqwest::Client::new();

        println!(
            "开始请求大模型翻译从 {} 到 {}.",
            request.from_lang, request.to_lang
        );
        // 将蛇形&驼峰都转成标准的格式
        let normalized_text = Self::normalize_naming_convention(&request.text);
        let text_to_translate = &normalized_text;
        let prompt = format!(
            "Translate the following text from {} to {}. Only return the translated text, no explanations:\n\n{}",
            request.from_lang, request.to_lang, text_to_translate
        );

        let body = serde_json::json!({
            "model": self.model_id,
            "messages": [
                {
                    "role": "system",
                    "content": "You are a professional translator. Translate the given text accurately while preserving the original meaning and tone."
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "max_tokens": 1000,
            "temperature": 0.3
        });

        let endpoint = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));

        let response = client
            .post(&endpoint)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("请求AI失败: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("AI状态错误: {}", error_text));
        }

        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("无法解析响应: {}", e))?;

        let translated_text = response_json
            .get("choices")
            .and_then(|choices| choices.get(0))
            .and_then(|choice| choice.get("message"))
            .and_then(|message| message.get("content"))
            .and_then(|content| content.as_str())
            .unwrap_or("")
            .trim()
            .to_string();

        if translated_text.is_empty() {
            return Err("无法获取到翻译内容".to_string());
        }
        println!("翻译成功！结果为：{translated_text}");
        Ok(TranslationResponse {
            translated_text,
            source_lang: request.from_lang.clone(),
            target_lang: request.to_lang.clone(),
        })
    }

    async fn translate_google(
        &self,
        _request: &TranslationRequest,
    ) -> Result<TranslationResponse, String> {
        // todo: 预留口子
        Err("预留的,没实现呢".to_string())
    }

    async fn translate_baidu(
        &self,
        _request: &TranslationRequest,
    ) -> Result<TranslationResponse, String> {
        // todo: 预留口子
        Err("预留的,没实现呢".to_string())
    }

    fn normalize_naming_convention(text: &str) -> String {
        // 将蛇形命名法转换为标准格式（用空格替换下划线）
        // 将驼峰命名法转换为标准格式（在大写字母前插入空格）
        let mut result = String::new();
        let mut prev_is_lower = false;

        for ch in text.chars() {
            if ch == '_' || ch == '-' {
                result.push(' ');
                prev_is_lower = false;
            } else if ch.is_uppercase() && prev_is_lower {
                result.push(' ');
                result.push(ch);
                prev_is_lower = false;
            } else {
                result.push(ch);
                prev_is_lower = ch.is_lowercase();
            }
        }
        println!("{}", result);
        result
    }
}

#[tauri::command]
pub fn get_supported_languages() -> Vec<(String, String)> {
    vec![
        ("auto".to_string(), "自动检测".to_string()),
        ("zh-CN".to_string(), "中文".to_string()),
        ("en".to_string(), "英语".to_string()),
        ("ja".to_string(), "日语".to_string()),
        ("ko".to_string(), "韩语".to_string()),
        ("fr".to_string(), "法语".to_string()),
        ("de".to_string(), "德语".to_string()),
        ("es".to_string(), "西班牙语".to_string()),
        ("ru".to_string(), "俄语".to_string()),
        ("ar".to_string(), "阿拉伯语".to_string()),
        ("pt".to_string(), "葡萄牙语".to_string()),
        ("it".to_string(), "意大利语".to_string()),
    ]
}
