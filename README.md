# Summary:

This is a tool developers can use to generate small or large amounts of random fake data to test out the performance of api-odata and wx-data-agent on it's endpoints.

# Given:

NEEDS `api-odata` and `wx-data-agent` running to see it injections. To see the data that you created with this tool, you can see it in DB or in a local environment urls.

# Commands:

## Run CLI:

`cargo watch -x run` or `cargo run`

View list of commands - under `OPTIONS`:
`./target/debug/mock_injections --help`

## Example:

`./target/debug/mock_injections -a "qa-inspections-manager" --dsm-limit 5`

- Single app slug
  - `cargo run -- -a "qa-inspections-manager" --dsm-limit 15` OR `cargo run -- -a "qa-inspections-manager" --d 15`
  - `cargo run -- -a "qa-inspections-manager" --ib-limit 10` OR `cargo run -- -a "qa-inspections-manager" --i 10`
- Multiple app slugs

  - `cargo run -- -a "mars-vic" "mars-ark" --dsm-limit 10`

- Running Schedule Scramble
  - `cargo run -- -a "qa-inspections-manager" --random-rs true`

# Code description

## DSM injections:

1. For each app mentioned it gets number of tiers.
1. Inserts random number between 1 to users's dsm limit number of entities for top tier.
1. Gets the entities of the (1st: top tier then iterates tiers)tier and for every entity it inserts a random number between 1 to users's dsm limit number of child entities.

## Inspection Builder injections:

1. For each app mentioned it gets,
   1. GET Inspection types by app
   1. GET Inspection forms by app
   1. GET DSM by app
   1. GET Shift by app
1. Create the user input limit number of inspection builder. Randomly set,
   1. status
   1. frequency
   1. Assignee
   1. Start Date
   1. goal tracking
   1. Admin note
   1. Inspection type
   1. Inspection form
   1. DSM
   1. Shift
1. Push each generated inspection to an array
1. Insert whole array to inspection command for odata.

## Running Schedule injections:

1. For each app mentions it gets,
1. Selects random Date Range
1. GET shifts for app
1. GET tiers for app to know lowest tier
1. GET lowest tier entities
1. Collect and generate random runtime changes from the number of date ranges, shifts, and lowest tier entities
1. Collect all similar random runtime types to create a vector of CancelDowntimeCommand and/or ScheduleDowntimeCommand
1. POST commands

# Issues

???? Caused by: feature `edition2021` is required consider adding `cargo-features = ["edition2021"]` to the manifest

**Solution:**

- Update the Rust to satisfy the new edition 2021 by running this command `rustup default nightly && rustup update`

???? `ERROR: There isn't any tiers for this app` but there are tiers for this app OR any fetching issues.

**Possible Solution:**
_unconsistent fix_

- Fetch/pull main branch for wx-data-agent and api-odata and ensure that it's connected properly then try command again.
- Ensure lagoon.dsm.tiers table has meta_event_id that isn't 0. If it does then rebuild database then try command again.
- Open this repo on a text edit and resave. No changes need to be made then try command again.
- Run `Cargo build` then try command again.

???? `ERROR - 400 Bad Request: Command post was unsuccessful.` from running schedule injections

- This is likely because we are sending too many commands since the limit is 10,000. This will frequently happen for Custom data range since it random pick 2 dates or if you rand DSM injection and created many low tier entities.
- There is no solutions other than to redo the command to have it pick different date ranges or to rebuild your dsm have less lowest tier entities.
