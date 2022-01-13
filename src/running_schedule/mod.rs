use super::dsm::{entity, tier};
use super::shift;

use colored::Colorize;
extern crate rand;
use crate::command;
use chrono::{Datelike, Duration, NaiveDate, NaiveTime, Utc};
use derive_more::Display;
use fake::faker::chrono::en::{DateTime, DateTimeAfter};
use fake::faker::lorem::en::Paragraph;
use fake::Dummy;
use fake::{Fake, Faker};
use serde::{Deserialize, Serialize};
use std::mem;
use uuid_5::Uuid;

#[derive(Dummy, Debug, Display)]
pub enum DowntimeType {
    CancelDowntime,
    ScheduleDowntime,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CancelDowntimeCommand {
    start_date: NaiveDate,
    end_date: NaiveDate,
    start_time: NaiveTime,
    end_time: NaiveTime,
    shifts: Vec<shift::ShiftIdentifier>,
    assets: Vec<shift::AssetIdentifier>,
    note: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ScheduleDowntimeCommand {
    start_date: NaiveDate,
    end_date: NaiveDate,
    start_time: NaiveTime,
    end_time: NaiveTime,
    shifts: Vec<shift::ShiftIdentifier>,
    assets: Vec<shift::AssetIdentifier>,
    note: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum DowntimeCommand {
    CancelDowntime(CancelDowntimeCommand),
    ScheduleDowntime(ScheduleDowntimeCommand),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Runtime {
    time_type: RuntimeType,
    date: NaiveDate,
    shift_id: Uuid,
    shift_name: String,
    asset_id: Uuid,
    asset_name: String,
}

#[derive(Clone, Copy, PartialEq, Debug, Dummy)]
pub enum RuntimeType {
    Run,
    Down,
}

#[derive(Dummy, Debug, Display, PartialEq)]
pub enum DateRangeType {
    Last30Days,
    LastWeek,
    ThisWeek,
    Yesterday,
    Today,
    NextWeek,
    Next30Days,
    Custom,
}

struct DateRange(NaiveDate, NaiveDate);

impl Iterator for DateRange {
    type Item = NaiveDate;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let next = self.0 + Duration::days(1);
            Some(mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}

pub async fn random_schedule(app_slugs: Vec<String>, token: String) {
    for app in app_slugs {
        println!("Randomizing running schedules for {}...", app);
        let rand_date_range = Faker.fake::<DateRangeType>();
        // let rand_date_range = DateRangeType::Custom;
        let from_ymd = NaiveDate::from_ymd;
        let today: chrono::DateTime<Utc> = Utc::now();
        let mut start_date = from_ymd(today.year(), today.month(), today.day());
        let mut end_date = from_ymd(today.year(), today.month(), today.day());

        match rand_date_range {
            DateRangeType::Last30Days => {
                start_date =
                    from_ymd(today.year(), today.month(), today.day()) - Duration::days(30);
            }
            DateRangeType::LastWeek => {
                start_date = from_ymd(today.year(), today.month(), today.day()) - Duration::days(7);
            }
            DateRangeType::ThisWeek => {
                end_date = from_ymd(today.year(), today.month(), today.day()) + Duration::days(7);
            }
            DateRangeType::Yesterday => {
                start_date = from_ymd(today.year(), today.month(), today.day()) - Duration::days(1);
                end_date = start_date;
            }
            DateRangeType::NextWeek => {
                end_date = from_ymd(today.year(), today.month(), today.day()) + Duration::days(7);
            }
            DateRangeType::Next30Days => {
                end_date = from_ymd(today.year(), today.month(), today.day()) + Duration::days(30);
            }
            DateRangeType::Custom => {
                // Note: Custom has a restriction to only move 24 days in the past and 6 days into the future in the front end. Does this error out in the backend properly if we send random dates?
                let start_datetime: chrono::DateTime<Utc> = DateTime().fake();

                let end_datetime: chrono::DateTime<Utc> = DateTimeAfter(start_datetime).fake();
                // Change date format to naive date.
                end_date = from_ymd(
                    end_datetime.year(),
                    end_datetime.month(),
                    end_datetime.day(),
                );
                start_date = from_ymd(
                    start_datetime.year(),
                    start_datetime.month(),
                    start_datetime.day(),
                );
            }
            _ => {}
        }
        // map out each date in date range into DateTime[]
        let date_interval: Vec<NaiveDate> = DateRange(start_date, end_date).collect();
        // println!("", rand_date_range);
        println!(
            "Date Range: {:?} -> {} to {}",
            rand_date_range, start_date, end_date
        );

        let fetched_shifts = shift::shift::fetch(&app, token.clone()).await;
        if fetched_shifts.shifts.len() == 0 {
            println!(
                "{}",
                "ERROR: There isn't any entities lowest tier for this app.".red()
            );
            break;
        }

        // Get number of tiers
        let fetched_tiers = tier::tiers(&app, token.clone()).await;
        if fetched_tiers.tiers.len() <= 0 {
            println!("{}", "ERROR: There isn't any tiers for this app.".red());
            break;
        }
        let lowest_tier = &fetched_tiers.tiers[fetched_tiers.tiers.len() - 1];

        let fetched_entities = entity::get_entities(&app, token.clone(), lowest_tier.id).await;
        if fetched_entities.assets.len() <= 0 {
            println!(
                "{}",
                "ERROR: There isn't any low tier entities for this app.".red()
            );
            break;
        }

        // There should be # date range * # shift * # lowest tier dsm = # downtime/runtime
        let mut rand_runtimes: Vec<Runtime> = Vec::new();

        // Collects current runtime for date range, shift and lowest tier entities will creating a random runtime for POST
        for entity in fetched_entities.assets {
            for date in &date_interval {
                for shift in &fetched_shifts.shifts {
                    // randomize time_type to simulate checkbox change.
                    rand_runtimes.push(Runtime {
                        time_type: Faker.fake::<RuntimeType>(),
                        date: *date,
                        shift_id: shift.id,
                        shift_name: shift.name.clone(),
                        asset_id: entity.id,
                        asset_name: entity.name.clone(),
                    });
                }
            }
        }

        let mut same_runtime_types: Vec<Runtime> = Vec::new();
        let mut downtime_changes: Vec<DowntimeCommand> = Vec::new();
        // Collect similar random runtimes types for combined command
        for rand_runtime in rand_runtimes {
            //  create downtime command if the current type is different than the 1st saved type, push into a vec, and reset saved data
            if same_runtime_types.len() > 0
                && same_runtime_types[0].time_type != rand_runtime.time_type
            {
                let mut shifts: Vec<shift::ShiftIdentifier> = Vec::new();
                let mut entities: Vec<shift::AssetIdentifier> = Vec::new();
                same_runtime_types.iter().for_each(|value| {
                    shifts.push(shift::ShiftIdentifier {
                        id: value.shift_id,
                        name: value.shift_name.clone(),
                    });
                    entities.push(shift::AssetIdentifier {
                        id: value.asset_id,
                        name: value.asset_name.clone(),
                    });
                });

                // Create command for all the similar runtime types changes
                match same_runtime_types[0].time_type {
                    RuntimeType::Run => downtime_changes.push(DowntimeCommand::CancelDowntime(
                        CancelDowntimeCommand {
                            start_date: same_runtime_types[0].date,
                            end_date: same_runtime_types[same_runtime_types.len() - 1].date,
                            start_time: NaiveTime::from_hms(0, 0, 0),
                            end_time: NaiveTime::from_hms(0, 0, 0),
                            shifts: shifts,
                            assets: entities,
                            note: Paragraph(3..5).fake(),
                        },
                    )),
                    RuntimeType::Down => downtime_changes.push(DowntimeCommand::ScheduleDowntime(
                        ScheduleDowntimeCommand {
                            start_date: same_runtime_types[0].date,
                            end_date: same_runtime_types[same_runtime_types.len() - 1].date,
                            start_time: NaiveTime::from_hms(0, 0, 0),
                            end_time: NaiveTime::from_hms(0, 0, 0),
                            shifts: shifts,
                            assets: entities,
                            note: Paragraph(3..5).fake(),
                        },
                    )),
                }
                // Reset collection of saved runtime
                same_runtime_types = Vec::new();
            }
            // Collect rand_runtime
            same_runtime_types.push(rand_runtime.clone());
        }
        // POST commands
        command::commands::post(
            &app,
            token.clone(),
            command::InspectionsCommandRequest {
                commands: downtime_changes,
            },
        )
        .await;
    }
}
