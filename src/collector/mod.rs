pub mod claude;

pub use claude::ClaudeCollector;

use crate::model::AgentSession;

/// Trait for agent session collectors.
/// Implement this for each CLI tool (Claude Code, Codex, etc.)
pub trait Collector {
    fn collect(&mut self) -> Vec<AgentSession>;
}

impl Collector for ClaudeCollector {
    fn collect(&mut self) -> Vec<AgentSession> {
        ClaudeCollector::collect(self)
    }
}

/// Aggregates sessions from multiple collectors (Claude, Codex, etc.)
pub struct MultiCollector {
    collectors: Vec<Box<dyn Collector>>,
}

impl MultiCollector {
    pub fn new() -> Self {
        Self {
            collectors: vec![Box::new(ClaudeCollector::new())],
        }
    }

    /// Register an additional collector (e.g., for Codex support)
    #[allow(dead_code)]
    pub fn add<C: Collector + 'static>(&mut self, collector: C) {
        self.collectors.push(Box::new(collector));
    }

    pub fn collect(&mut self) -> Vec<AgentSession> {
        let mut all = Vec::new();
        for c in &mut self.collectors {
            all.extend(c.collect());
        }
        all.sort_by_key(|s| std::cmp::Reverse(s.started_at));
        all
    }
}
