// IPC protocol — Forge GUI communicates with Agent and other services
// via Unix domain sockets

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GuiCommand {
    ShowNotification { title: String, body: String, urgency: Urgency },
    ShowDialog { title: String, message: String, buttons: Vec<String> },
    SetStatus { text: String },
    UpdateWidget { id: String, data: serde_json::Value },
    SetTheme { name: String },
    TakeScreenshot { path: String },
    LockScreen,
    UnlockScreen,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Urgency { Low, Normal, Critical }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GuiEvent {
    UserSpeech(String),
    ButtonClicked { dialog_id: String, button: String },
    WidgetClicked { widget_id: String },
    ScreenshotCaptured { path: String },
}
