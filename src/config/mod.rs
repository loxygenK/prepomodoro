pub mod convert;

#[derive(Debug, PartialEq, Eq)]
pub struct Configuration {
    tasks: Vec<Task>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Task {
    name: String,
    intervals: Interval,
    checklist: CheckList,
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub struct CheckList {
    before_start: Vec<String>,
    before_iteration: Vec<String>,
    before_rest: Vec<String>,
    after_finish: Vec<String>,
}
