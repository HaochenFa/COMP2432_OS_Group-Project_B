# Project Blaze (COMP2432 Project B)

## Quick start

- Demo (dev logging enabled):
  - `cargo run`
- Demo (release, logging suppressed):
  - `cargo run --release`
- Benchmark (CSV output):
  - `cargo run --release -- bench`
  - with params: `cargo run --release -- bench [robots] [tasks_per_robot] [zones] [work_ms] [validate] [offline]`
- Stress sweep (CSV output):
  - `cargo run --release -- stress`
  - with params: `cargo run --release -- stress [robot_sets] [task_sets] [zone_sets] [work_ms] [validate] [offline]`
  - use comma-separated sets like `1,2,4` and `-` to keep defaults for a slot

## Notes

- Dev logs are gated by `cfg!(debug_assertions)` and are suppressed in release builds.
- The benchmark prints a CSV header and one data row to stdout.
