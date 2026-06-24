pub struct BehaviorMonitor {
    history: Vec<String>,
    hourly: u32,
    fails: u32,
}
impl BehaviorMonitor {
    pub fn new() -> Self { Self { history: Vec::new(), hourly: 0, fails: 0 } }
    pub fn record(&mut self, cmd: &str) -> Vec<String> {
        let mut alerts = Vec::new();
        self.history.push(cmd.to_string());
        self.hourly += 1;
        if self.hourly > 100 { alerts.push(">100 cmds/hr".into()); }
        let destructive = self.history.iter().rev().take(10).filter(|c| c.contains("delete") || c.contains("shutdown")).count();
        if destructive >= 5 { alerts.push(format!("{} destructive ops", destructive)); }
        if self.history.len() > 1000 { self.history.drain(..500); }
        alerts
    }
}
pub async fn run_monitor() { loop { tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await; } }