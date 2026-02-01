# Project Blaze Diagrams

The diagrams below are written in Mermaid for easy rendering in Markdown viewers
that support it (GitHub, GitLab, many IDEs).

## High-level architecture

```mermaid
flowchart TD
    CLI[CLI entry: main.rs] --> SIM[Simulation runner: sim.rs]
    SIM --> TQ[TaskQueue]
    SIM --> ZA[ZoneAccess]
    SIM --> HM[HealthMonitor]
    SIM --> LOG[Dev logging macro]

    TQ --> TQSYNC[Mutex + Condvar]
    ZA --> ZASYNC[Mutex + Condvar]
    HM --> HMSYNC[Mutex + HashMap/HashSet]
```

## Robot task flow + monitoring

```mermaid
sequenceDiagram
    participant R as Robot thread
    participant Q as TaskQueue
    participant Z as ZoneAccess
    participant H as HealthMonitor
    participant M as Monitor thread

    R->>Q: pop_blocking()
    Q-->>R: Task
    R->>Z: acquire(zone)
    Z-->>R: granted
    R->>H: heartbeat()
    R->>Z: release(zone)
    M->>H: detect_offline(timeout)
    H-->>M: offline set
```

## Zone access control logic

```mermaid
flowchart TD
    A[Robot requests zone] --> B{Zone occupied?}
    B -- no --> C[Insert owner and enter zone]
    B -- yes --> D[Wait on condvar]
    D --> B
    C --> E[Robot completes work]
    E --> F[Release zone]
    F --> G[Notify all waiters]
```
