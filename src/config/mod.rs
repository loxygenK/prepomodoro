use serde::{Deserialize, Serialize};

pub mod convert;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Configuration {
    pub tasks: Vec<Task>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    name: String,
    #[serde(default)]
    intervals: Interval,
    #[serde(default)]
    checklist: CheckList,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Interval {
    SimplePomodoro {
        active: u64,
        rest: u64,
    },
    Pomodoro {
        active: u64,
        rest: u64,
        large_rest: u64,
        interation_util: usize,
    },
}

impl Default for Interval {
    fn default() -> Self {
        Self::Pomodoro {
            active: 25,
            rest: 5,
            large_rest: 15,
            interation_util: 4,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckList {
    #[serde(default)]
    before_start: Vec<String>,
    #[serde(default)]
    before_iteration: Vec<String>,
    #[serde(default)]
    before_rest: Vec<String>,
    #[serde(default)]
    after_finish: Vec<String>,
}
