# Summary:

This is a tool developers can use to generate small or large amounts of random fake data to test out the performance of api-odata and wx-data-agent on it's endpoints.

# Given:

None, does not need anything else running. To see the data that you created with this tool, you can see it in DB or in a local environment urls.

# Commands:

## Run CLI:

`cargo watch -x run` or `cargo run`

View list of commands - under `OPTIONS`:
`./target/debug/mock_injections --help`

## Example:

`./target/debug/mock_injections -a "qa-inspections-manager" --dsm-limit 5`

- Single app slug
  `cargo run -- -a "qa-inspections-manager" --dsm-limit 15`
  `cargo run --  -a "qa-inspections-manager"  --ib-limit 10` 
  
- Multiple app slugs
  `cargo run -- -a "mars-vic" "mars-ark" --dsm-limit 10`
 

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
