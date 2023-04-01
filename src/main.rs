use config::Configuration;

use crate::entity::Task;

pub mod config;
pub mod entity;

fn main() {
    let content = std::fs::read_to_string("./examples/work.yaml").unwrap();
    let parsed = serde_yaml::from_str::<Configuration>(&content).unwrap();

    dbg!(&parsed);

    let config: Vec<Task> = parsed.tasks.into_iter().map(Task::from).collect();

    dbg!(&config);

    dbg!(&config[0].segments.flatten());
}
