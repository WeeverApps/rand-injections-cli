use super::dsm::{entity, tier};
use super::shift::shift;

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
pub enum DowntimeCommand {
    CancelDowntime {
        start_date: NaiveDate,
        end_date: NaiveDate,
        start_time: NaiveTime,
        end_time: NaiveTime,
        shifts: Vec<Uuid>,
        assets: Vec<Uuid>,
        note: Option<String>,
    },
    ScheduleDowntime {
        start_date: NaiveDate,
        end_date: NaiveDate,
        start_time: NaiveTime,
        end_time: NaiveTime,
        shifts: Vec<Uuid>,
        assets: Vec<Uuid>,
        note: Option<String>,
    },
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Runtime {
    time_type: RuntimeType,
    date: NaiveDate,
    shift_id: Uuid,
    asset_id: Uuid,
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
                println!("start_datetime: {:?}", start_datetime);

                let end_datetime: chrono::DateTime<Utc> = DateTimeAfter(start_datetime).fake();
                // Change date format to naive date.
                end_date = from_ymd(
                    end_datetime.year(),
                    end_datetime.month(),
                    end_datetime.day(),
                );
                println!("TEST END: {:?}", end_date);
                start_date = from_ymd(
                    start_datetime.year(),
                    start_datetime.month(),
                    start_datetime.day(),
                );
                println!("start_date: {:?}", start_date);
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

        let fetched_shifts = shift::fetch(&app, token.clone()).await;
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

        println!("DATE INTERVAL {:?}", date_interval.len());
        println!("ENTITY {:?}", fetched_entities.assets.len());
        println!("SHIFT {:?}", fetched_shifts.shifts.len());

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
                        asset_id: entity.id,
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
                            asset_id: entity.id,
                        });
                    } else {
                        runtime.push(Runtime {
                            time_type: RuntimeType::Run,
                            date: *date,
                            shift_id: shift.id,
                            asset_id: entity.id,
                        });
                    }
                }
            }
        }

        let mut save_runtime: Vec<Runtime> = Vec::new();
        let mut downtime_changes: Vec<DowntimeCommand> = Vec::new();
        // Loop and Compare original runtime vec with randomized copy of runtime.
        println!("RUNTIME: {:?}", runtime.len());
        for x in 0..runtime.len() {
            println!(
                "{} CHECK ------ OG: {:?} RAND: {:?}",
                x, runtime[x].time_type, rand_runtime[x].time_type
            );

            //  create downtime command if the current type is different than the 1st saved type, push into a vec, and reset saved data
            if save_runtime.len() > 0 && save_runtime[0].time_type != rand_runtime[x].time_type {
                println!("SAVE RUNTIME VEC: {:?}", save_runtime);
                let mut shifts: Vec<Uuid> = Vec::new();
                let mut entities: Vec<Uuid> = Vec::new();
                save_runtime.iter().for_each(|value| {
                    shifts.push(value.shift_id);
                    entities.push(value.asset_id);
                });

                // Create command for all the similar runtime types changes
                match save_runtime[0].time_type {
                    RuntimeType::Run => downtime_changes.push(DowntimeCommand::CancelDowntime {
                        start_date: save_runtime[0].date,
                        end_date: save_runtime[save_runtime.len() - 1].date,
                        start_time: NaiveTime::from_hms(0, 0, 0),
                        end_time: NaiveTime::from_hms(0, 0, 0),
                        shifts: shifts,
                        assets: entities,
                        note: Paragraph(3..5).fake(),
                    }),
                    RuntimeType::Down => downtime_changes.push(DowntimeCommand::ScheduleDowntime {
                        start_date: save_runtime[0].date,
                        end_date: save_runtime[save_runtime.len() - 1].date,
                        start_time: NaiveTime::from_hms(0, 0, 0),
                        end_time: NaiveTime::from_hms(0, 0, 0),
                        shifts: shifts,
                        assets: entities,
                        note: Paragraph(3..5).fake(),
                    }),
                }

                save_runtime = Vec::new();
            }
            save_runtime.push(rand_runtime[x]);
        }
        println!("DOWNTIME CHANGES: {:?}", downtime_changes);

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
