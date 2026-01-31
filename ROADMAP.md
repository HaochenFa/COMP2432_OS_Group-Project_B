# ROADMAP.md

This roadmap is aligned with `Project-B.pdf` and `project_B_guidelines.md`. It is intentionally time-agnostic and focuses on durable gates and compliance.

## Milestone 0: Requirements Baseline

- Read and agree on the minimal scope (task queue, zone access control, health monitor).
- Record non-goals (no preemption, deadlock prevention, complex scheduling).
- Define invariants and demo success criteria.

Exit criteria:

- Team consensus on scope, non-goals, and required demo behaviors.

## Milestone 1: Architecture and Interfaces

- Define data models for tasks, robots, zones, and health status.
- Decide thread ownership and shared-state boundaries.
- Choose synchronization primitives per component.
- Draft module layout for clarity and reporting.

Exit criteria:

- A clear module interface map and an architecture diagram ready for the report.

## Milestone 2: Core Implementation

- Implement a thread-safe task queue with safe fetch/consume semantics.
- Implement zone access control with strict mutual exclusion.
- Implement health monitoring with heartbeat tracking and timeout detection.
- Add a minimal simulation runner that spawns multiple robot threads.

Exit criteria:

- The system can run a basic scenario that exercises all three components.

## Milestone 3: Correctness Tests

- Unit tests for task queue safety and single-consumer behavior.
- Concurrency tests for zone exclusivity.
- Timeout/offline detection tests for the health monitor.

Exit criteria:

- `cargo test` passes and core concurrency properties are validated.

## Milestone 4: Observability and Benchmarking

- Add clear logging or output that demonstrates synchronization behavior.
- Produce benchmark data: throughput, zone latency, CPU usage.
- Run scalability and stress tests (vary robot/task counts).

Exit criteria:

- Data and artifacts exist to support the Benchmark section of the report.

## Milestone 5: Demo Readiness

- Prepare a deterministic demo flow:
  - system overview
  - concurrent run
  - synchronization code walkthrough
- Ensure terminal output is readable and clearly shows required behaviors.

Exit criteria:

- A reliable 3-minute demo script that shows all required behaviors.

## Milestone 6: Report Completion

- Fill all required sections with correct word counts.
- Include architecture diagram, synchronization rationale, and benchmark results.
- Assemble at least 20 APA references (Rust docs, papers, technical articles).

Exit criteria:

- Report complies with all structural and content requirements.

## Quality Gates (Always On)

- Build: `cargo build --release` succeeds.
- Tests: `cargo test` passes.
- Concurrency correctness: shared state remains consistent.
- Demo compliance: all three required behaviors are visible.

## Risks and Mitigations

- **Race conditions**: minimize shared state, guard with locks, stress-test.
- **Deadlocks**: keep lock ordering consistent and critical sections short.
- **Non-deterministic tests**: use timeouts and controlled schedules.
- **Demo fragility**: use deterministic scenarios and clear logging.
