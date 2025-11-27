use crate::database::{ProxyConfig, ProxyMode};
use reqwest::{Client, ClientBuilder, Proxy};
use std::sync::{OnceLock, RwLock};

static HTTP_CLIENT: OnceLock<RwLock<Client>> = OnceLock::new();

fn client_lock() -> &'static RwLock<Client> {
    HTTP_CLIENT.get_or_init(|| {
        let client = build_client(None).expect("Failed to create initial HTTP client");
        RwLock::new(client)
    })
}

fn build_client(proxy: Option<&ProxyConfig>) -> Result<Client, reqwest::Error> {
    let mut builder = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .connect_timeout(std::time::Duration::from_secs(10));

    if let Some(config) = proxy {
        if config.enabled {
            builder = apply_proxy(builder, config)?;
        }
    }

    builder.build()
}

fn apply_proxy(
    builder: ClientBuilder,
    config: &ProxyConfig,
) -> Result<ClientBuilder, reqwest::Error> {
    match config.mode {
        ProxyMode::System => configure_system_proxy(builder),
        ProxyMode::Https | ProxyMode::Http | ProxyMode::Socks5 => {
            let server = config.server.trim();
            if server.is_empty() {
                return Ok(builder);
            }
            let proxy = match config.mode {
                ProxyMode::Https => Proxy::https(server)?,
                ProxyMode::Http => Proxy::http(server)?,
                ProxyMode::Socks5 => Proxy::all(server)?,
                ProxyMode::System => unreachable!(),
            };
            apply_single_proxy(builder, proxy)
        }
    }
}

fn apply_single_proxy(
    builder: ClientBuilder,
    proxy: Proxy,
) -> Result<ClientBuilder, reqwest::Error> {
    Ok(builder.proxy(proxy))
}

fn configure_system_proxy(builder: ClientBuilder) -> Result<ClientBuilder, reqwest::Error> {
    let mut builder = builder;

    if let Some(url) = env_proxy_value(&["ALL_PROXY", "all_proxy"]) {
        builder = builder.proxy(Proxy::all(url)?);
    }

    if let Some(url) = env_proxy_value(&["HTTPS_PROXY", "https_proxy"]) {
        builder = builder.proxy(Proxy::https(url)?);
    }

    if let Some(url) = env_proxy_value(&["HTTP_PROXY", "http_proxy"]) {
        builder = builder.proxy(Proxy::http(url)?);
    }

    Ok(builder)
}

fn env_proxy_value(keys: &[&str]) -> Option<String> {
    for key in keys {
        if let Ok(value) = std::env::var(key) {
            let trimmed = value.trim().to_string();
            if !trimmed.is_empty() {
                return Some(trimmed);
            }
        }
    }
    None
}

pub fn http_client() -> Client {
    client_lock()
        .read()
        .expect("Failed to read HTTP client")
        .clone()
}

pub fn configure_http_client(proxy: Option<&ProxyConfig>) -> Result<(), String> {
    let client = build_client(proxy).map_err(|e| e.to_string())?;
    if let Ok(mut guard) = client_lock().write() {
        *guard = client;
        Ok(())
    } else {
        Err("无法更新HTTP客户端".to_string())
    }
}

pub fn validate_http_client(proxy: Option<&ProxyConfig>) -> Result<(), String> {
    build_client(proxy)
        .map(|_| ())
        .map_err(|e| e.to_string())
}
