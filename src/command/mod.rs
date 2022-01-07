pub mod commands;
use super::running_schedule;
use serde::{Deserialize, Serialize};

// #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
// pub enum CommandRequestDetails {
//     ScheduleDowntime(running_schedule::DowntimeCommand::ScheduleDowntime),
//     CancelDowntime(running_schedule::DowntimeCommand::CancelDowntime),
// }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InspectionsCommandRequest {
    /// Set of actionable commands to be processed
    pub commands: Vec<running_schedule::DowntimeCommand>,
}
