pub struct AgentMemory {
    context: Vec<String>,
}

impl AgentMemory {
    pub fn new() -> Self { Self { context: Vec::new() } }

    pub fn get_context(&self) -> String {
        if self.context.len() > 5 {
            self.context.iter().rev().take(5).cloned().collect::<Vec<_>>().join("\n")
        } else {
            self.context.join("\n")
        }
    }

    pub fn remember(&mut self, input: &str, _plan: &str, _action: &str) {
        self.context.push(format!("[User] {input}"));
        if self.context.len() > 50 { self.context.remove(0); }
    }
}
