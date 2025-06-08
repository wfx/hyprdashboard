#[derive(Debug, Clone)]
pub enum Message {
    LaunchApp(String),
    ToggleSettings,
}
