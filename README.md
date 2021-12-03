# Commands:

Run CLI:
`cargo watch -x run` or `cargo run`

View list of commands - under `OPTIONS`:
`./target/debug/mock_injections --help`

Example:
`./target/debug/mock_injections -a "qa-inspections-manager" --dsm-limit 5`

- Single app slug
  `cargo run -- -a "qa-inspections-manager" --dsm-limit 15`
- Multiple app slugs
  `cargo run -- -a "mars-vic" "mars-ark" --dsm-limit 10`
