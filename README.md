# Commands:

Run CLI:
`cargo watch -x run` or `cargo run`

View list of commands - under `OPTIONS`:
`./target/debug/mock_injections --help`

Example:
`./target/debug/mock_injections -a "qa-inspections-manager" -l 5`

- Single app slug
  `cargo run -- -a "qa-inspections-manager" -l 5`
- Multiple app slugs
  `cargo run -- -a "qa-inspections-manager" "mars-clv" -l 5`
