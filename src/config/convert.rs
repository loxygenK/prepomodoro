use std::time::Duration;

use crate::entity;

use super::*;

impl From<Task> for entity::Task {
    fn from(value: Task) -> Self {
        Self {
            name: value.name,
            segments: value.intervals.into(),
            checklist: value.checklist.into(),
        }
    }
}

impl From<Interval> for entity::TimeSegments {
    fn from(value: Interval) -> Self {
        match value {
            Interval::SimplePomodoro { active, rest } => Self::Iteration(
                1,
                vec![
                    Self::Segment(Duration::from_secs(60 * active), entity::Kind::Active),
                    Self::Segment(Duration::from_secs(60 * rest), entity::Kind::Rest),
                ],
            ),
            Interval::Pomodoro {
                active,
                rest,
                large_rest,
                interation_util,
            } => Self::Iteration(
                1,
                vec![
                    Self::Iteration(
                        interation_util - 1,
                        vec![
                            Self::Segment(Duration::from_secs(60 * active), entity::Kind::Active),
                            Self::Segment(Duration::from_secs(60 * rest), entity::Kind::Rest),
                        ],
                    ),
                    Self::Segment(Duration::from_secs(60 * active), entity::Kind::Active),
                    Self::Segment(
                        Duration::from_secs(60 * large_rest),
                        entity::Kind::LargeRest,
                    ),
                ],
            ),
        }
    }
}

impl From<CheckList> for entity::CheckList {
    fn from(value: CheckList) -> Self {
        Self {
            before_start: value.before_start,
            before_iteration: value.before_iteration,
            before_rest: value.before_rest,
            after_finish: value.after_finish,
        }
    }
}
