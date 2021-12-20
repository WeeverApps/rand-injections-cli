pub mod shift;

use chrono::{DateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use uuid_5::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ShiftStatus {
    #[serde(alias = "active")]
    Published,

    #[serde(alias = "paused", alias = "disabled")]
    Draft,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShiftIdentifier {
    pub id: Uuid,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetIdentifier {
    pub id: Uuid,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ShiftWorkingDays {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase"), deny_unknown_fields)]
pub struct Shift {
    pub id: Uuid,

    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub working_days: Vec<ShiftWorkingDays>,

    pub name: String,
    pub note: Option<String>,
    pub status: ShiftStatus,

    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>,
}
