use colored::Colorize;
extern crate rand;
use rand::thread_rng;
use rand::Rng;

use fake::faker::company::en::*;
use fake::{Fake, Faker};

use std::{thread, time};

use chrono::{Datelike, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};
use uuid_5::Uuid;

pub mod inspection_form;
pub mod inspection_type;
// pub mod inspections_command;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateScheduleCommand {
    pub inspection_type_id: Uuid,
    pub form_id: Uuid,
    pub asset_id: Uuid,
    pub shift_id: Uuid,
    pub assignee_id: Option<Uuid>,
    pub duration_minutes: Option<u32>,
    pub frequency_id: Option<Uuid>,
    pub frequency_amount: u32,
    pub frequency_unit: ScaleOfDateRange,
    pub frequency_day_of_week: Option<DayOfWeek>,
    pub frequency_start_date: NaiveDate,
    pub note: Option<String>,
    pub status: Option<ScheduleStatus>,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScaleOfDateRange {
    Day,
    Week,
    Month,
    Quarter,
    Year,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScheduleStatus {
    Active,

    // todo: change frontend to use 'paused'
    #[serde(alias = "disabled")]
    Paused,
}

pub async fn create_inspection_builder(limit: i32, app_slugs: Vec<String>, token: String) {
    // Random number generator
    let mut rng = thread_rng();

    // Create dsm for each app
    for app in 0..app_slugs.len() {
        println!("INSPECTION BUILD");
        let mut rand_num;
        // get inspection type by app
        let fetch_inspection_types = inspection_type::fetch(&app_slugs[app], token.clone()).await;
        println!("type: {:?}", fetch_inspection_types);
        if fetch_inspection_types.inspection_types.len() > 1 {
            rand_num = rng.gen_range(1..fetch_inspection_types.inspection_types.len());
        } else {
            rand_num = 0;
        }
        println!("RAND NUM {:?}", rand_num);
        if fetch_inspection_types.inspection_types.len() > 1 {
            let rand_inspection_type = fetch_inspection_types.inspection_types[rand_num].clone();
            println!("rand_inspection_type {:?}", rand_inspection_type);
        } else {
            println!(
                "Inspection type: {:?}",
                fetch_inspection_types.inspection_types
            );
            // TODO: create and fetch. Assuming you already have some.
        }

        // get inspection form by app
        let inspection_forms = inspection_form::fetch(&app_slugs[app], token.clone()).await;
        println!("INSPECTION FORMS: {:?}", inspection_forms);
        // inspection_form::form_categories(&app_slugs[app], token.clone()).await;
        // get dsm by app
        // get shift by app
        // Randomly set a frequency
        // Randomly set an Assignee
        // Randomly set Start Date
        // Randomly set goal tracking
        // Randomly set admin note

        /*
        let mut fake_schedule = Vec::new();
        let fake_schedule.push(CreateScheduleCommand {
            inspection_type_id: rand_inspection_type.id,
            form_id: ,
            asset_id: ,
            shift_id: ,
            assignee_id: ,
            duration_minutes: ,
            frequency_id: ,
            frequency_amount: ,
            frequency_unit: ,
            frequency_day_of_week: Faker.fake::<DayOfWeek>(),
            frequency_start_date: ,
            note: CatchPhase().fake(),
            status: Faker.fake::<ScheduleStatus>(),
        });

        inspections_command::service(
            token,
            app,
            "CreateSchedule".into(),
            fake_schedule.into_inner(),
        )
        .await
        */
    }
}
