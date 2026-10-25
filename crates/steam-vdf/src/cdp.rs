use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetKind {
    SharedJsContext,
    Library,
    BigPicture,
    Community,
    Other,
}

pub fn classify_target(title: &str, url: &str) -> TargetKind {
    if title == "SharedJSContext" || url.contains("SharedJSContext") {
        TargetKind::SharedJsContext
    } else if title == "Steam Big Picture Mode" || title.starts_with("MainMenu_") {
        TargetKind::BigPicture
    } else if title == "Steam" && url.contains("steamloopback.host") {
        TargetKind::Library
    } else if url.contains("steamcommunity.com") {
        TargetKind::Community
    } else {
        TargetKind::Other
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementToast {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon_url: String,
    pub app_id: u32,
}

impl AchievementToast {
    pub fn build_eval_script(&self) -> String {
        format!(
            r#"(() => {{
                try {{
                    if (window.SteamClient && window.SteamClient.Notifications) {{
                        window.SteamClient.Notifications.ShowCustomNotification(
                            "{}",
                            "{}",
                            "{}",
                            "{}"
                        );
                        return true;
                    }}
                }} catch (e) {{}}
                return false;
            }})()"#,
            self.title.replace('"', r#"\""#),
            self.description.replace('"', r#"\""#),
            self.icon_url,
            self.app_id
        )
    }
}

#[derive(Debug, Serialize)]
pub struct CdpRequest<'a> {
    pub id: u64,
    pub method: &'a str,
    pub params: serde_json::Value,
    #[serde(rename = "sessionId", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<&'a str>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CdpTarget {
    pub id: String,
    pub title: String,
    pub url: String,
    #[serde(rename = "webSocketDebuggerUrl")]
    pub websocket_url: Option<String>,
}

pub fn format_cdp_call(
    id: u64,
    method: &str,
    params: serde_json::Value,
    session_id: Option<&str>,
) -> String {
    let req = CdpRequest {
        id,
        method,
        params,
        session_id,
    };
    serde_json::to_string(&req).unwrap_or_default()
}

/// Dispatches a native Steam achievement toast to the running Steam client via Chrome DevTools Protocol
pub async fn trigger_steam_toast_cdp(toast: &AchievementToast, port: u16) -> Result<bool, String> {
    use futures_util::SinkExt;
    use tokio_tungstenite::connect_async;
    use tokio_tungstenite::tungstenite::Message;

    let url = format!("http://127.0.0.1:{}/json", port);
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(800))
        .build()
        .map_err(|e| e.to_string())?;

    let targets: Vec<CdpTarget> = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Could not reach Steam CDP on port {}: {}", port, e))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse CDP targets: {}", e))?;

    let target = targets
        .iter()
        .find(|t| {
            let k = classify_target(&t.title, &t.url);
            k == TargetKind::SharedJsContext
                || k == TargetKind::BigPicture
                || k == TargetKind::Library
        })
        .or_else(|| targets.first())
        .ok_or_else(|| "No CDP targets available in Steam client".to_string())?;

    let ws_url = target
        .websocket_url
        .as_ref()
        .ok_or_else(|| "Target does not have a webSocketDebuggerUrl".to_string())?;

    let (mut ws_stream, _) = connect_async(ws_url)
        .await
        .map_err(|e| format!("WebSocket connection failed: {}", e))?;

    let eval_script = toast.build_eval_script();
    let msg_str = format_cdp_call(
        1,
        "Runtime.evaluate",
        serde_json::json!({ "expression": eval_script }),
        None,
    );

    ws_stream
        .send(Message::Text(msg_str))
        .await
        .map_err(|e| format!("Failed to send CDP message: {}", e))?;

    Ok(true)
}
