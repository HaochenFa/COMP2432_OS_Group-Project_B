# Project Blaze (COMP2432 Project B)

## Quick start

- Demo (dev logging enabled):
  - `cargo run`
- Demo (release, logging suppressed):
  - `cargo run --release`
- Benchmark (CSV output):
  - `cargo run --release -- bench`

## Notes

- Dev logs are gated by `cfg!(debug_assertions)` and are suppressed in release builds.
- The benchmark prints a CSV header and one data row to stdout.
