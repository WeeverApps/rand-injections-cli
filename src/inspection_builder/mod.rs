use colored::Colorize;
extern crate rand;
use rand::thread_rng;
use rand::Rng;

use fake::faker::chrono::en::DateTimeAfter;
use fake::faker::lorem::raw::*;
use fake::locales::EN;
use fake::Dummy;
use fake::{Fake, Faker};

use chrono::{Datelike, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid_5::Uuid;

pub mod inspection_form;
pub mod inspection_type;
pub mod inspections_command;

use crate::dsm::{entity, tier};
use crate::frequency::frequency;
use crate::shift::shift;
use crate::user::user;

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

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, Dummy)]
#[serde(rename_all = "lowercase")]
pub enum ScaleOfDateRange {
    Day,
    Week,
    Month,
    Quarter,
    Year,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, Dummy)]
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

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, Dummy)]
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
        let mut rand_num;
        // get inspection type by app
        let fetch_inspection_types = inspection_type::fetch(&app_slugs[app], token.clone()).await;

        if fetch_inspection_types.inspection_types.len() > 1 {
            rand_num = rng.gen_range(1..fetch_inspection_types.inspection_types.len());
        } else {
            rand_num = 0;
        }
        if fetch_inspection_types.inspection_types.len() <= 0 {
            println!(
                "{}",
                "ERROR: There isn't any inspection type for this app.".red()
            );
            break;
        }
        let rand_inspection_type = fetch_inspection_types.inspection_types[rand_num].clone();

        // get inspection forms by app
        let inspection_forms = inspection_form::fetch(&app_slugs[app], token.clone()).await;
        if inspection_forms.len() <= 0 {
            println!(
                "{}",
                "ERROR: There isn't any inspection forms for this app.".red()
            );
            break;
        }
        rand_num = rng.gen_range(1..inspection_forms.len());
        // get dsm by app
        // get tiers
        let fetched_tiers = tier::tiers(&app_slugs[app], token.clone()).await;
        if fetched_tiers.tiers.len() <= 0 {
            println!("{}", "ERROR: There isn't any tiers for this app.".red());
            break;
        };
        // get lowest tier entity
        let entities = entity::get_entities(
            &app_slugs[app],
            token.clone(),
            fetched_tiers.tiers[fetched_tiers.tiers.len() - 1].id,
        )
        .await;
        if entities.assets.len() <= 0 {
            println!(
                "{}",
                "ERROR: There isn't any entities lowest tier for this app.".red()
            );
            break;
        }
        let rand_num_asset: usize = rng.gen_range(0..entities.assets.len());
        let entity_id = entities.assets[rand_num_asset].id;

        // get shift by app
        let shifts = shift::fetch(&app_slugs[app], token.clone()).await;
        if shifts.shifts.len() <= 0 {
            println!("{}", "ERROR: There isn't any shift for this app.".red());
            break;
        }
        let rand_num_shift = rng.gen_range(0..shifts.shifts.len());
        let shift = &shifts.shifts[rand_num_shift];

        // Randomly set a frequency
        let frequency_unit = frequency::fetch(&app_slugs[app], token.clone()).await;
        let rand_num_frequency = rng.gen_range(0..frequency_unit.frequency.len());
        let frequency = &frequency_unit.frequency[rand_num_frequency];

        // Randomly set an Assignee
        let assignees = user::fetch(&app_slugs[app], token.clone()).await;
        let rand_num_user = rng.gen_range(0..assignees.users.len());
        let assignee_id = assignees.users[rand_num_user].id;

        // Randomly set Start Date
        let start_date: chrono::DateTime<Utc> = DateTimeAfter(Utc::now()).fake();
        // Randomly set goal tracking
        let goal_tracking: u32 = rng.gen();
        // Randomly set admin note
        let admin_note: String = Paragraph(EN, 3..5).fake();
        let from_ymd = NaiveDate::from_ymd;
        let date = from_ymd(start_date.year(), start_date.month(), start_date.day());

        let mut fake_schedule = Vec::new();
        fake_schedule.push(CreateScheduleCommand {
            inspection_type_id: rand_inspection_type.id,
            form_id: inspection_forms[rand_num].uuid,
            asset_id: entity_id,
            shift_id: shift.id,
            assignee_id: Some(assignee_id),
            duration_minutes: Some(goal_tracking),
            frequency_id: Some(frequency.id),
            frequency_amount: frequency.frequency_count as u32,
            frequency_unit: frequency.frequency_unit,
            frequency_day_of_week: Some(Faker.fake::<DayOfWeek>()),
            frequency_start_date: date,
            note: Some(admin_note),
            status: Some(Faker.fake::<ScheduleStatus>()),
        });
        println!("schedule: {:?}", fake_schedule);
        inspections_command::service(
            token.clone(),
            app_slugs[app].clone(),
            "CreateSchedule".into(),
            fake_schedule,
        )
        .await;
    }
}
