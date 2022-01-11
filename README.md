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

# Issues

💡 Caused by: feature `edition2021` is required consider adding `cargo-features = ["edition2021"]` to the manifest

**Solution:**

- Update the Rust to satisfy the new edition 2021 by running this command `rustup default nightly && rustup update`

💡 `ERROR: There isn't any tiers for this app` but there are tiers for this app

**Possible Solution:**
_unconsistent fix_

- Fetch/pull main branch for wx-data-agent and api-odata and ensure that it's connected properly.
- Ensure lagoon.dsm.tiers table has meta_event_id that isn't 0. If it does then rebuild database.
