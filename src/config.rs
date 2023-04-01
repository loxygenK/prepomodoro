pub struct Configuration {
    tasks: Vec<Task>,
}

pub struct Task {
    name: String,
    intervals: Interval,
    checklist: CheckList
}

pub enum Interval {
    Pomodoro {
        active: usize,
        rest: usize,
    },
    PomodoroWithBreak {
        active: usize,
        rest: usize,
        large_rest: usize,
        interation_util: usize,
    },
}

impl Default for Interval {
    fn default() -> Self {
        Self::PomodoroWithBreak {
            active: 25,
            rest: 5,
            large_rest: 15,
            interation_util: 4
        }
    }
}

pub struct CheckList {
    before_start: Vec<String>,
    before_iteration: Vec<String>,
    before_rest: Vec<String>,
    after_finish: Vec<String>
}
