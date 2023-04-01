use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Task {
    pub name: String,
    pub segments: TimeSegments,
    pub checklist: CheckList,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TimeSegments {
    Segment(Duration, Kind),
    Iteration(usize, Vec<TimeSegments>),
}

impl TimeSegments {
    pub fn flatten(&self) -> Vec<(Duration, Kind)> {
        match self {
            TimeSegments::Segment(duration, kind) => vec![(*duration, kind.clone())],
            TimeSegments::Iteration(repeat, segment) => segment
                .iter()
                .map(TimeSegments::flatten)
                .cycle()
                .take(*repeat * segment.len())
                .flatten()
                .collect::<Vec<(_, _)>>(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    Active,
    Rest,
    LargeRest,
    Other(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckList {
    pub before_start: Vec<String>,
    pub before_iteration: Vec<String>,
    pub before_rest: Vec<String>,
    pub after_finish: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_segments_can_flatten_single_segment() {
        use TimeSegments::*;

        assert_eq!(
            Segment(Duration::from_secs(10), Kind::Active).flatten(),
            vec![(Duration::from_secs(10), Kind::Active)]
        )
    }

    #[test]
    fn time_segments_can_flatten_iteration() {
        use TimeSegments::*;

        assert_eq!(
            Iteration(
                2,
                vec![
                    Segment(Duration::from_secs(10), Kind::Active),
                    Segment(Duration::from_secs(20), Kind::Rest),
                    Segment(Duration::from_secs(30), Kind::LargeRest),
                ]
            )
            .flatten(),
            vec![
                (Duration::from_secs(10), Kind::Active),
                (Duration::from_secs(20), Kind::Rest),
                (Duration::from_secs(30), Kind::LargeRest),
                (Duration::from_secs(10), Kind::Active),
                (Duration::from_secs(20), Kind::Rest),
                (Duration::from_secs(30), Kind::LargeRest),
            ]
        );
    }

    #[test]
    fn time_segments_can_flatten_nested_iteration() {
        use TimeSegments::*;

        assert_eq!(
            Iteration(
                1,
                vec![
                    Segment(Duration::from_secs(10), Kind::Active),
                    Iteration(
                        2,
                        vec![
                            Segment(Duration::from_secs(20), Kind::Rest),
                            Segment(Duration::from_secs(40), Kind::Active),
                        ]
                    ),
                    Segment(Duration::from_secs(30), Kind::LargeRest),
                ]
            )
            .flatten(),
            vec![
                (Duration::from_secs(10), Kind::Active),
                (Duration::from_secs(20), Kind::Rest),
                (Duration::from_secs(40), Kind::Active),
                (Duration::from_secs(20), Kind::Rest),
                (Duration::from_secs(40), Kind::Active),
                (Duration::from_secs(30), Kind::LargeRest),
            ]
        );
    }
}
