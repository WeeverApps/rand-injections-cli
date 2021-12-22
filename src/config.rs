pub mod api {
    pub mod dsm {
        use std::env;

        pub fn connection_url() -> String {
            match env::var("API_DSM_URL") {
                Ok(url) => url,
                _ => String::from("http://localhost:8118/v1/dsm"), // Default local odata url
            }
        }
    }

    pub mod inspections_v2 {
        use std::env;

        pub fn connection_url() -> String {
            match env::var("ODATA_INSPECTIONS_V2_URL") {
                Ok(url) => url,
                _ => String::from("http://localhost:8118/v1/inspections-v2"), // Default local odata inspections url
            }
        }
    }

    pub mod platform {
        use std::env;

        pub fn connection_url() -> String {
            match env::var("API_PLATFORM_URL") {
                Ok(url) => url,
                _ => String::from("http://localhost:8413"), // Default local platform url
            }
        }
    }
}
