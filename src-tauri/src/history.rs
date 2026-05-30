use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KillRecord {
    pub pid: u32,
    pub name: String,
    pub ts: u64,
    pub success: bool,
    pub reason: Option<String>,
}

pub struct History {
    items: Mutex<VecDeque<KillRecord>>,
    max: Mutex<usize>,
}

impl History {
    pub fn new(max: usize) -> Self {
        Self {
            items: Mutex::new(VecDeque::with_capacity(max)),
            max: Mutex::new(max),
        }
    }

    pub fn set_max(&self, max: usize) {
        *self.max.lock() = max;
        let mut items = self.items.lock();
        while items.len() > max {
            items.pop_front();
        }
    }

    pub fn push(&self, pid: u32, name: String, success: bool, reason: Option<String>) {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let mut items = self.items.lock();
        let max = *self.max.lock();
        if items.len() >= max {
            items.pop_front();
        }
        items.push_back(KillRecord { pid, name, ts, success, reason });
    }

    pub fn snapshot(&self) -> Vec<KillRecord> {
        self.items.lock().iter().rev().cloned().collect()
    }

    pub fn clear(&self) {
        self.items.lock().clear();
    }
}
