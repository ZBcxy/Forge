use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::process::{Child, Command};
use tracing::{info, warn, error};

const MAX_RESTART: Duration = Duration::from_secs(60);
const INITIAL_RESTART: Duration = Duration::from_secs(1);

pub struct ServiceDefinition {
    pub name: String,
    pub binary: String,
    pub args: Vec<String>,
    pub critical: bool,
    pub max_restarts: usize,
}

pub struct RunningService {
    pub def: ServiceDefinition,
    pub child: Option<Child>,
    pub restart_count: usize,
    pub restart_interval: Duration,
    pub last_start: Instant,
}

impl RunningService {
    fn is_running(&mut self) -> bool {
        if let Some(ref mut child) = self.child {
            match child.try_wait() {
                Ok(None) => return true,
                Ok(Some(st)) => { warn!("Exited: {st:?}"); self.child = None; false }
                Err(_) => { self.child = None; false }
            }
        } else { false }
    }
}

pub struct ServiceState {
    svcs: HashMap<String, RunningService>,
}

impl ServiceState {
    pub fn new(defs: Vec<ServiceDefinition>) -> Self {
        let svcs = defs.into_iter().map(|d| {
            let name = d.name.clone();
            (name.clone(), RunningService {
                def: d, child: None, restart_count: 0,
                restart_interval: INITIAL_RESTART, last_start: Instant::now(),
            })
        }).collect();
        Self { svcs }
    }

    pub async fn start_service(&mut self, name: &str) -> anyhow::Result<()> {
        let svc = self.svcs.get_mut(name).ok_or_else(|| anyhow::anyhow!("Unknown: {name}"))?;
        info!("Launching {} {}", svc.def.binary, svc.def.args.join(" "));
        let child = Command::new(&svc.def.binary).args(&svc.def.args).kill_on_drop(true).spawn()?;
        svc.child = Some(child);
        svc.restart_interval = INITIAL_RESTART;
        svc.last_start = Instant::now();
        Ok(())
    }

    pub async fn check_and_restart(&mut self) {
        let names: Vec<String> = self.svcs.keys().cloned().collect();
        for name in names {
            let do_restart = {
                if let Some(svc) = self.svcs.get_mut(&name) {
                    !svc.is_running()
                } else { false }
            };
            if do_restart {
                if let Some(svc) = self.svcs.get_mut(&name) {
                    svc.restart_count += 1;
                    if svc.restart_count > svc.def.max_restarts {
                        error!("{name}: max restarts exceeded");
                        continue;
                    }
                    svc.restart_interval = std::cmp::min(svc.restart_interval * 2, MAX_RESTART);
                    let interval = svc.restart_interval;
                    info!("Restarting {name} in {interval:?} (attempt {})", svc.restart_count);
                    tokio::time::sleep(interval).await;
                    if let Err(e) = self.start_service(&name).await {
                        error!("Restart {name} failed: {e}");
                    }
                }
            }
        }
    }
}
