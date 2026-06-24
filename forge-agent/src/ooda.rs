use tracing::{info, warn};
use serde_json;

pub struct OodaResult {
    pub reply: String,
    pub action: String,
    pub success: bool,
    pub tool_output: Option<String>,
}

pub struct OodaAgent {
    llm_url: String,
    memory: crate::memory::AgentMemory,
    tools: crate::tools::ToolRegistry,
}

impl OodaAgent {
    pub async fn new() -> anyhow::Result<Self> {
        let llm_url = std::env::var("FORGE_LLM_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:8080".into());
        Ok(Self {
            llm_url,
            memory: crate::memory::AgentMemory::new(),
            tools: crate::tools::ToolRegistry::new(),
        })
    }

    pub async fn process(&mut self, input: &str) -> OodaResult {
        info!("OODA: {input}");

        let _intent = self.classify_intent(input);
        let context = self.memory.get_context();

        // Query LLM for action + reply
        let (action_tool, action_args) = match self.query_llm(input, &context).await {
            Ok((tool, args)) => (tool, args),
            Err(e) => {
                warn!("LLM failed: {e}");
                return OodaResult {
                    reply: "抱歉，AI 大脑暂时无响应。".into(),
                    action: "fallback".into(),
                    success: false,
                    tool_output: None,
                };
            }
        };

        // Execute tool if specified
        let tool_output = if action_tool == "chat" {
            Some(self.query_llm_chat_only(input, &context).await.unwrap_or_else(|_| "收到".into()))
        } else if !action_tool.is_empty() {
            let result = self.tools.execute(&action_tool, &action_args);
            let out = result.output.clone();
            self.memory.remember(input, &action_tool, &out);
            Some(out)
        } else {
            None
        };

        let reply = tool_output.clone().unwrap_or_else(|| "已处理".into());

        OodaResult {
            reply,
            action: action_tool,
            success: true,
            tool_output,
        }
    }

    fn classify_intent(&self, input: &str) -> &str {
        let input = input.to_lowercase();
        if input.contains("文件") || input.contains("打开") || input.contains("列出") { "file_operation" }
        else if input.contains("搜索") || input.contains("查") || input.contains("找") { "search" }
        else if input.contains("运行") || input.contains("启动") { "launch" }
        else if input.contains("关机") { "shutdown" }
        else if input.contains("重启") { "reboot" }
        else if input.contains("下载") { "download" }
        else { "general" }
    }

    async fn query_llm(&self, input: &str, context: &str) -> anyhow::Result<(String, serde_json::Value)> {
        let client = reqwest::Client::new();
        let resp = client.post(&format!("{}/v1/chat", self.llm_url))
            .json(&serde_json::json!({
                "messages": [
                    {"role": "system", "content": "你是 Forge OS AI 助手。根据用户指令返回需要执行的操作。回复 JSON: {\"tool\":\"工具名\",\"args\":{},\"reply\":\"要说的回复\"}。可用工具: fs_list, fs_read, fs_write, fs_delete, fs_mkdir, fs_search, fs_copy, fs_move, proc_launch, proc_list, proc_kill, sys_info, sys_shutdown, sys_reboot, sys_volume, net_http, net_download, web_search, display_screenshot, chat"},
                    {"role": "user", "content": format!("上下文: {context}\n用户指令: {input}")}
                ],
                "temperature": 0.3,
                "max_tokens": 256
            }))
            .send().await?;

        let data: serde_json::Value = resp.json().await?;
        let raw = data["text"].as_str().unwrap_or("{\"tool\":\"chat\",\"args\":{},\"reply\":\"收到\"}");
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(raw) {
            let tool = parsed["tool"].as_str().unwrap_or("chat").to_string();
            let args = parsed["args"].clone();
            Ok((tool, args))
        } else {
            Ok(("chat".into(), serde_json::json!({})))
        }
    }

    async fn query_llm_chat_only(&self, input: &str, context: &str) -> anyhow::Result<String> {
        let client = reqwest::Client::new();
        let resp = client.post(&format!("{}/v1/chat", self.llm_url))
            .json(&serde_json::json!({
                "messages": [
                    {"role": "system", "content": "你是 Forge OS AI 助手，用简洁中文回复。"},
                    {"role": "user", "content": format!("上下文: {context}\n用户: {input}")}
                ],
                "max_tokens": 256
            }))
            .send().await?;
        let data: serde_json::Value = resp.json().await?;
        Ok(data["text"].as_str().unwrap_or("收到").to_string())
    }
}
