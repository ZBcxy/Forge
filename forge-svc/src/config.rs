use crate::service::ServiceDefinition;

pub fn load_services() -> Vec<ServiceDefinition> {
    vec![
        ServiceDefinition { name: "forge-hal".into(),   binary: "/usr/bin/forge-hal".into(),   args: vec![], critical: true,  max_restarts: 10 },
        ServiceDefinition { name: "forge-llm".into(),   binary: "/usr/bin/forge-llm".into(),   args: vec!["--model".into(), "/opt/forge/models/forge-v1.gguf".into()], critical: true, max_restarts: 5 },
        ServiceDefinition { name: "forge-voice".into(), binary: "/usr/bin/forge-voice".into(), args: vec![], critical: false, max_restarts: 5 },
        ServiceDefinition { name: "forge-agent".into(), binary: "/usr/bin/forge-agent".into(), args: vec!["--daemon".into()], critical: true, max_restarts: 10 },
        ServiceDefinition { name: "forge-gui".into(),   binary: "/usr/bin/forge-gui".into(),   args: vec![], critical: false, max_restarts: 5 },
    ]
}
