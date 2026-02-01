# Project Blaze (COMP2432 Project B)

Lightweight OS core that coordinates multiple medical-care robots with safe concurrency.
This repository implements the minimal required scope: task queue, zone access control, and
health monitoring with heartbeat timeouts.

## Scope and non-goals

In scope:

- Task queue: safe push/pop of tasks across threads.
- Zone access control: mutual exclusion per zone.
- Health monitor: heartbeat tracking and offline detection.

Non-goals (by design):

- No preemption, deadlock prevention, or complex scheduling policies.

## Architecture (core modules)

- `TaskQueue` (`src/task_queue.rs`): `Mutex<VecDeque<Task>>` + `Condvar` for safe blocking pop.
- `ZoneAccess` (`src/zones.rs`): `Mutex<HashMap<ZoneId, RobotId>>` + `Condvar` for exclusive zones.
- `HealthMonitor` (`src/health_monitor.rs`): `Mutex<HealthState>` with `last_seen` + `offline` set.
- `sim` (`src/sim.rs`): demo runner, benchmark, and stress sweep.

## Concurrency invariants

- A task is consumed at most once.
- A zone is occupied by at most one robot at a time.
- Offline robots are detected after heartbeat timeouts.
- All shared state is guarded by synchronization primitives.

## Build and test gates

- `cargo build --release`
- `cargo test`

## Quick start

Demo (dev logging enabled):

```bash
cargo run
```

Demo (release, logging suppressed):

```bash
cargo run --release
```

Benchmark (CSV output):

```bash
cargo run --release -- bench
cargo run --release -- bench [robots] [tasks_per_robot] [zones] [work_ms] [validate] [--offline-demo]
```

Stress sweep (CSV output):

```bash
cargo run --release -- stress
cargo run --release -- stress [robot_sets] [task_sets] [zone_sets] [work_ms] [validate] [--offline-demo]
```

Notes:

- Sets are comma-separated lists (e.g., `1,2,4`).
- Use `-` to keep defaults for robot/task/zone sets.
- Omit `work_ms` to keep its default.

## Demo expectations

The demo spawns multiple robot threads, coordinates zones, and emits a summary at the end.
One robot stops sending heartbeats partway through the demo; offline detection is performed
by a background monitor thread. If you want a clearer offline signal, run a longer benchmark
with `offline` enabled so the timeout window is exceeded, for example:

```bash
cargo run --release -- bench 4 50 2 20 validate --offline-demo
```

## Benchmark and stress output

`bench` and `stress` print a CSV header followed by one or more rows:

```
robots,tasks_per_robot,zones,total_tasks,elapsed_ms,throughput_tasks_per_s,avg_zone_wait_us,cpu_user_s,cpu_sys_s,max_occupancy,zone_violation,duplicate_tasks,offline_robots
```

CPU usage columns are populated on Unix platforms; otherwise they show `NA`.

## Project layout

- `src/task_queue.rs`: thread-safe queue + tests
- `src/zones.rs`: zone mutex logic + tests
- `src/health_monitor.rs`: heartbeat tracking + tests
- `src/sim.rs`: demo, benchmark, stress harness
- `src/logging.rs`: dev-only logging macro
- `src/types.rs`: shared type aliases and task struct

## Project file architecture (ASCII)

```
project_blaze/
|-- Cargo.toml
|-- README.md
|-- ROADMAP.md
|-- project_B_guidelines.md
|-- Project-B.pdf
|-- report_0201.md
|-- DIAGRAMS.md
`-- src/
    |-- main.rs
    |-- sim.rs
    |-- task_queue.rs
    |-- zones.rs
    |-- health_monitor.rs
    |-- logging.rs
    `-- types.rs
```

Mermaid diagrams for architecture and logic flows are in `DIAGRAMS.md`.

## Logging

Dev logs are gated by `cfg!(debug_assertions)` and are suppressed in release builds.
Benchmarks and stress runs always print CSV output to stdout.

## Notes

- Official requirements are in `project_B_guidelines.md` and `Project-B.pdf`.
- Demo and benchmark parameters are tuned for clarity, not realism.
- The CLI accepts `--offline-demo` (alias: `offline`) for bench/stress runs.
