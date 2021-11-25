pub mod api {
    pub mod odata_inspections_v2 {
        use std::env;

        pub fn connection_url() -> String {
            match env::var("ODATA_INSPECTIONS_V2_URL") {
                Ok(url) => url,
                _ => String::from("http://localhost:8118/v1/inspections-v2"), // Default local odata inspections url
            }
        }
    }

    pub mod api_inspections_v2_odata {
        use std::env;

        pub fn connection_url() -> String {
            match env::var("API_INSPECTIONS_V2_ODATA_URL") {
                Ok(url) => url,
                _ => String::from("http://localhost:3310/odata"), // Default local odata inspections url
            }
        }
    }
}
