use crate::command::InspectionsCommandRequest;
use crate::config::api::inspections_v2::connection_url as inspections_v2_url;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use uuid_5::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct InspectionsEventTransaction {
    pub stream_id: Uuid,
    pub transaction_id: Uuid,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CommandResponse {
    status: String,
    receipts: Vec<InspectionsEventTransaction>,
}

pub async fn post(app_slug: &str, token: String, command: InspectionsCommandRequest) {
    let hostname = inspections_v2_url();

    let url = format!("{}/{}/command", hostname, app_slug);

    /*
    InspectionsCommandRequest { commands: [CancelDowntime(CancelDowntimeCommand { start_date: 2022-01-10, end_date: 2022-01-10, start_time: 00:00:00, end_time: 00:00:00, shifts: [ShiftIdentifier { id: Uuid("7e8cfe48-ca58-4ec9-9fd1-2fd07ffc341a"), name: "2nd" }], assets: [AssetIdentifier { id: Uuid("ac652bdf-f79b-4b8c-98d3-e6a19e140460"), name: "Capper" }], note: None }), ScheduleDowntime(ScheduleDowntimeCommand { start_date: 2022-01-10, end_date: 2022-01-10, start_time: 00:00:00, end_time: 00:00:00, shifts: [ShiftIdentifier { id: Uuid("f93309d8-2114-425e-99d2-e3b33bb9189e"), name: "1st" }], assets: [AssetIdentifier { id: Uuid("a4b252b8-c09f-4dad-aae8-5e90f4601d45"), name: "Case packer" }], note: None }), ScheduleDowntime(ScheduleDowntimeCommand { start_date: 2022-01-10, end_date: 2022-01-10, start_time: 00:00:00, end_time: 00:00:00, shifts: [ShiftIdentifier { id: Uuid("98b388ec-4af8-4847-bd3a-f05762f812b8"), name: "3rd" }], assets: [AssetIdentifier { id: Uuid("ac652bdf-f79b-4b8c-98d3-e6a19e140460"), name: "Capper" }], note: None })] }
    */

    println!("COMMAND url: {:?}", url);
    println!("@@@@@@ COMMANDs: \n{:?}\n", command);

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .bearer_auth(token)
        .json(&command)
        .send()
        .await
        .unwrap();

    if !response.status().is_success() {
        let error: &str = &format!(
            "ERROR - {}: Command post was unsuccessful.",
            response.status()
        );
        println!("{}", error.red());
    }
}
