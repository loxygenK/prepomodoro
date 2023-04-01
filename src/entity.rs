use std::time::Duration;

#[derive(Debug, PartialEq, Eq)]
pub struct Task {
    pub name: String,
    pub segments: TimeSegments,
    pub checklist: CheckList,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TimeSegments {
    Segment(Duration, Kind),
    Iteration(usize, Vec<TimeSegments>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Kind {
    Active,
    Rest,
    LargeRest,
    Other(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct CheckList {
    pub before_start: Vec<String>,
    pub before_iteration: Vec<String>,
    pub before_rest: Vec<String>,
    pub after_finish: Vec<String>,
}
