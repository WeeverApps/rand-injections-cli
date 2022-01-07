use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_qs;
use serde_qs::Config;

pub fn parse_encoded_qs<'a, T: Deserialize<'a>>(query_string: &'a &str) -> T {
    // Deserialize an encoded query string into a struct
    // serde_qs doesn't support deserializing encoded square brackets by default. It needs to be done using a custom config.
    // https://docs.rs/serde_qs/0.4.1/serde_qs/struct.Config.html
    let config = Config::new(5, false);

    let query_params: T = config.deserialize_str(query_string).unwrap();
    return query_params;
}

pub fn to_encoded_qs<T: Serialize>(params: &T) -> String {
    // serde_qs does not provide an option to serialize a struct into a query string with encoded square brackets,
    // it always serializes them into the format '[index]'
    let qs = serde_qs::to_string(params).unwrap();

    // Remove the index from the square brackets
    let index_identifiers = Regex::new(r"\[[0-9]+\]").unwrap();
    let qs_no_indicies = index_identifiers.replace_all(&qs, "[]");

    // There is an encoding crate 'percent-encoding' that can be used, but since we only need to handle this specific case
    // we'll manually replace the square brackets with their encoded equivalents.
    qs_no_indicies.replace("[", "%5B").replace("]", "%5D")
}
