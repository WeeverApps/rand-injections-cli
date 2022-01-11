use super::dsm::{entity, tier};
use super::shift;

use colored::Colorize;
extern crate rand;
use fake::faker::lorem::en::Paragraph;
use rand::thread_rng;
// use rand::Rng;
use std::mem;
pub mod downtime;
use crate::command;
// use crate::command::commands;
use chrono::{Datelike, Duration, NaiveDate, NaiveTime, Utc};
use derive_more::Display;
use fake::faker::chrono::en::{DateTime, DateTimeAfter};
use fake::Dummy;
use fake::{Fake, Faker};
use serde::{Deserialize, Serialize};
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

pub async fn random_schedule(_limit: i32, app_slugs: Vec<String>, token: String) {
    // Random number generator
    let mut _rng = thread_rng();

    // Scramble running schedule
    for app in app_slugs {
        // let mut rand_num;

        // randomize cancellation and scheduled downtime.
        // check if downtime is cancelled or not.

        // let rand_date_range = Faker.fake::<DateRangeType>();
        let rand_date_range = DateRangeType::Yesterday;
        let from_ymd = NaiveDate::from_ymd;
        let today: chrono::DateTime<Utc> = Utc::now();
        let mut start_date = from_ymd(today.year(), today.month(), today.day());
        let mut end_date = from_ymd(today.year(), today.month(), today.day());
        println!("date range: {:?}", rand_date_range);

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

        // Get downtime
        let fetched_downtime = downtime::fetch(&app, token.clone(), start_date, end_date).await;
        let downtimes;
        match fetched_downtime {
            Ok(v) => {
                // println!("DOWNTIME: {:?}", v);
                println!("LENGTH:: {:?}", v.downtimes.len());
                downtimes = v.downtimes;
            }
            Err(e) => {
                println!("{}", e.to_string().red());
                break;
            }
        }

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

        // for every date range * shift * lowest tier dsm is the number of downtime/runtime
        let mut runtime: Vec<Runtime> = Vec::new();
        let mut rand_runtime: Vec<Runtime> = Vec::new();

        for entity in fetched_entities.assets {
            for date in &date_interval {
                for shift in &fetched_shifts.shifts {
                    // check if this entity, date, and shift is in this downtime.
                    // randomize time_type to simulate checkbox change.
                    rand_runtime.push(Runtime {
                        time_type: Faker.fake::<RuntimeType>(),
                        date: *date,
                        shift_id: shift.id,
                        shift_name: shift.name.clone(),
                        asset_id: entity.id,
                        asset_name: entity.name.clone(),
                    });
                    if (&downtimes).into_iter().any(|value| {
                        value.asset_id == entity.id
                            && value.shift_id == shift.id
                            && value.date == date.to_string()
                    }) {
                        runtime.push(Runtime {
                            time_type: RuntimeType::Down,
                            date: *date,
                            shift_id: shift.id,
                            shift_name: shift.name.clone(),
                            asset_id: entity.id,
                            asset_name: entity.name.clone(),
                        });
                    } else {
                        runtime.push(Runtime {
                            time_type: RuntimeType::Run,
                            date: *date,
                            shift_id: shift.id,
                            shift_name: shift.name.clone(),
                            asset_id: entity.id,
                            asset_name: entity.name.clone(),
                        });
                    }
                }
            }
        }

        let mut save_runtime: Vec<Runtime> = Vec::new();
        let mut downtime_changes: Vec<DowntimeCommand> = Vec::new();
        // Loop and Compare original runtime vec with randomized copy of runtime.
        for x in 0..runtime.len() {
            //  create downtime command if the current type is different than the 1st saved type, push into a vec, and reset saved data
            if save_runtime.len() > 0 && save_runtime[0].time_type != rand_runtime[x].time_type {
                let mut shifts: Vec<shift::ShiftIdentifier> = Vec::new();
                let mut entities: Vec<shift::AssetIdentifier> = Vec::new();
                save_runtime.iter().for_each(|value| {
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
                match save_runtime[0].time_type {
                    RuntimeType::Run => downtime_changes.push(DowntimeCommand::CancelDowntime(
                        CancelDowntimeCommand {
                            start_date: save_runtime[0].date,
                            end_date: save_runtime[save_runtime.len() - 1].date,
                            start_time: NaiveTime::from_hms(0, 0, 0),
                            end_time: NaiveTime::from_hms(0, 0, 0),
                            shifts: shifts,
                            assets: entities,
                            note: Paragraph(3..5).fake(),
                        },
                    )),
                    RuntimeType::Down => downtime_changes.push(DowntimeCommand::ScheduleDowntime(
                        ScheduleDowntimeCommand {
                            start_date: save_runtime[0].date,
                            end_date: save_runtime[save_runtime.len() - 1].date,
                            start_time: NaiveTime::from_hms(0, 0, 0),
                            end_time: NaiveTime::from_hms(0, 0, 0),
                            shifts: shifts,
                            assets: entities,
                            note: Paragraph(3..5).fake(),
                        },
                    )),
                }

                save_runtime = Vec::new();
            }
            save_runtime.push(rand_runtime[x].clone());
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
