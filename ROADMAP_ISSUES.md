# Enterprise Product Roadmap & Issue Specifications

This document outlines the strategic product gap analysis and production-grade issue specifications for **YouRust Lab**. Each issue is structured according to industrial open-source standards to guide progressive implementation toward a robust systems development platform.

---

## Table of Contents
1. [Issue #1: [FEAT] Unified Interactive CLI Runner & Argument Parser (`clap` + Subcommands)](#issue-1-feat-unified-interactive-cli-runner--argument-parser-clap--subcommands)
2. [Issue #2: [FEAT] Terminal User Interface (TUI) Dashboard with `ratatui` & `crossterm`](#issue-2-feat-terminal-user-interface-tui-dashboard-with-ratatui--crossterm)
3. [Issue #3: [FEAT] Asynchronous HTTP Benchmarking & Stress-Testing Engine](#issue-3-feat-asynchronous-http-benchmarking--stress-testing-engine)
4. [Issue #4: [FEAT] Centralized Error Handling & Diagnostics Architecture (`thiserror` + `anyhow`)](#issue-4-feat-centralized-error-handling--diagnostics-architecture-thiserror--anyhow)
5. [Issue #5: [FEAT] Structured Telemetry, Logging, & Tracing System (`tracing` + `tracing-subscriber`)](#issue-5-feat-structured-telemetry-logging--tracing-system-tracing--tracing-subscriber)
6. [Issue #6: [FEAT] Comprehensive Automated Test Suite & Micro-benchmarks (`criterion` + Unit/Integration Tests)](#issue-6-feat-comprehensive-automated-test-suite--micro-benchmarks-criterion--unitintegration-tests)
7. [Issue #7: [FEAT] Enterprise CI/CD Pipeline via GitHub Actions (Linting, Formatting, Multi-OS Matrix)](#issue-7-feat-enterprise-cicd-pipeline-via-github-actions-linting-formatting-multi-os-matrix)
8. [Issue #8: [FEAT] Data Persistence & State Storage Layer (Embedded SQLite / `redb`)](#issue-8-feat-data-persistence--state-storage-layer-embedded-sqlite--redb)

---

### Issue 1: [FEAT] Unified Interactive CLI Runner & Argument Parser (`clap` + Subcommands)

- **Labels**: `enhancement`, `priority: high`
- **Priority**: High (P1)
- **Target Version**: `v0.2.0`

#### Background & Problem Statement
Currently, `src/main.rs` hardcodes a direct execution of `basic::run()`. Developers must manually modify code in `src/main.rs` to run any of the other 12 modules (`ownership`, `structs`, `async_demo`, etc.). There is no unified mechanism to inspect available modules, pass dynamic arguments, or select individual runners from the terminal.

#### User Story
As a developer or contributor, I want to execute specific lab experiments directly from the terminal (e.g., `cargo run -- run ownership` or `cargo run -- list`) so that I can explore and test system primitives seamlessly without modifying source code.

#### Technical Specifications & Architecture
- Integrate `clap` with the `derive` feature.
- Define a central CLI parser in `src/cli.rs` with top-level subcommands:
  - `list`: Print all registered modules with brief descriptions and execution categories.
  - `run <MODULE>`: Execute a specific module by name.
  - `all`: Sequentially execute all available modules with runtime benchmarking.
- Support global flags: `--verbose` (`-v`), `--json`, and `--help`.

#### Implementation Tasks
- [ ] Add `clap = { version = "4", features = ["derive"] }` to `Cargo.toml`.
- [ ] Create `src/cli.rs` defining the `Cli` struct and `Commands` enum.
- [ ] Implement an executable `ModuleRunner` trait or registry linking string identifiers to module `run()` functions.
- [ ] Refactor `src/main.rs` to parse CLI input and dispatch to corresponding module executors.
- [ ] Add clear console output formatting for module boundaries.

#### Acceptance Criteria
- Running `cargo run -- --help` renders standard Unix-compliant CLI help text with available subcommands.
- Running `cargo run -- list` prints all 13 modules with their status and purpose.
- Running `cargo run -- run async-demo` launches the async HTTP demo without errors.
- Passing an invalid module name (e.g. `cargo run -- run nonexistent`) exits gracefully with an exit code of `1` and suggests valid options.

---

### Issue 2: [FEAT] Terminal User Interface (TUI) Dashboard with `ratatui` & `crossterm`

- **Labels**: `enhancement`, `priority: high`
- **Priority**: High (P1)
- **Target Version**: `v0.3.0`

#### Background & Problem Statement
All current output is dumped directly to stdout as raw terminal text. There is no interactive visual dashboard to navigate modules, view code syntax side-by-side, inspect memory allocation behavior, or monitor asynchronous task states in real-time.

#### User Story
As an engineer exploring Rust, I want an interactive terminal dashboard where I can navigate through lab modules using keyboard shortcuts, inspect execution logs, and monitor async thread activity visually.

#### Technical Specifications & Architecture
- Adopt `ratatui` (v0.28+) and `crossterm` (v0.28+) for cross-platform terminal rendering (Windows, Linux, macOS).
- Design a multi-pane layout:
  - Left pane: Module explorer (Categorized Tree: Fundamentals, Memory, Concurrency).
  - Center pane: Live execution log & captured stdout.
  - Right pane: Metadata inspector (Rust edition, execution duration, thread ID, memory footprint).
  - Footer pane: Status bar with keybindings (`[Q] Quit`, `[Enter] Run`, `[Tab] Switch Pane`).

#### Implementation Tasks
- [ ] Add `ratatui` and `crossterm` to `Cargo.toml`.
- [ ] Create `src/tui/` module containing `app.rs`, `ui.rs`, and `events.rs`.
- [ ] Implement raw terminal mode initialization and cleanup on exit/panic (graceful terminal restoration).
- [ ] Build key event listener loop with non-blocking event polling.
- [ ] Connect module execution triggers to background worker threads using `tokio::task::spawn_blocking` to prevent UI freezing.

#### Acceptance Criteria
- Running `cargo run -- tui` launches a full-screen interactive TUI.
- Navigation via arrow keys (`Up`/`Down`) smoothly highlights modules in the sidebar.
- Triggering execution with `Enter` updates the output pane without lagging or freezing the terminal.
- Terminal screen restores to its previous terminal buffer upon pressing `Esc` or `Q`.

---

### Issue 3: [FEAT] Asynchronous HTTP Benchmarking & Stress-Testing Engine

- **Labels**: `enhancement`, `priority: high`
- **Priority**: High (P1)
- **Target Version**: `v0.3.0`

#### Background & Problem Statement
`src/async_demo.rs` contains an elementary HTTP GET call to `httpbin.org` wrapped in a manual `tokio::runtime::Runtime::new()` instance. It lacks concurrent connection scaling, custom request payloads, throughput measurement, and latency percentile analytics.

#### User Story
As a systems engineer, I want to use YouRust Lab as a high-performance HTTP benchmarking utility to evaluate upstream service performance, measure requests-per-second (RPS), and compute latency distributions (p50, p95, p99).

#### Technical Specifications & Architecture
- Refactor the async runtime to leverage idiomatic `#[tokio::main]`.
- Build a configurable stress-testing engine:
  - Configurable parameters: target URL, concurrency limit, total requests, timeout duration.
  - Asynchronous worker pool spawning Tokio green tasks bounded by `tokio::sync::Semaphore`.
  - High-precision timestamp capture using `std::time::Instant`.
  - Statistical calculations: min, max, mean, standard deviation, p50, p90, p99 latency percentiles.

#### Implementation Tasks
- [ ] Upgrade `reqwest` to modern asynchronous connection pooling (reusing `reqwest::Client`).
- [ ] Add `hdrhistogram` or lightweight percentile calculation logic.
- [ ] Implement rate-limiting and connection concurrency control via `Semaphore`.
- [ ] Export benchmark summaries in formatted table and JSON formats.
- [ ] Provide graceful cancellation handling on `SIGINT` (Ctrl+C).

#### Acceptance Criteria
- CLI command `cargo run -- bench http --url <URL> -c <CONCURRENCY> -n <REQUESTS>` runs successfully.
- Latency percentiles (p50, p95, p99) and RPS are calculated and printed upon completion.
- Network errors (timeouts, DNS failures, connection drops) are tallied without crashing the process.

---

### Issue 4: [FEAT] Centralized Error Handling & Diagnostics Architecture (`thiserror` + `anyhow`)

- **Labels**: `enhancement`, `priority: medium`
- **Priority**: Medium (P2)
- **Target Version**: `v0.2.0`

#### Background & Problem Statement
Currently, error handling across the codebase relies on raw string errors (`Result<i32, String>`), `.unwrap()` panics in `async_demo.rs`, and inconsistent error logging. This leads to unpredictable runtime panics and poor error diagnostics.

#### User Story
As a developer, I want all library modules to return structured, typed errors and top-level applications to report rich contextual stack traces so that unexpected failures can be identified and debugged rapidly.

#### Technical Specifications & Architecture
- Adopt the industry-standard dual-error paradigm:
  - `thiserror`: For internal domain errors (`LabError`), providing typed, enumerated variants with custom display formatting.
  - `anyhow`: For application-level entrypoints (`main.rs`, CLI runners), providing effortless context chaining via `.context()`.
- Ensure zero unhandled `.unwrap()` calls in production-facing code paths.

#### Implementation Tasks
- [ ] Add `thiserror = "2"` and `anyhow = "1"` to `Cargo.toml`.
- [ ] Create `src/error.rs` defining `LabError` with variants: `NetworkError`, `IoError`, `ParseError`, `ModuleExecutionFailed`.
- [ ] Refactor `src/error_handling.rs` to demonstrate both low-level custom errors and high-level contextual errors.
- [ ] Refactor `src/async_demo.rs` to replace `.unwrap()` with `?` error propagation.
- [ ] Update `main()` signature to return `anyhow::Result<()>`.

#### Acceptance Criteria
- Zero explicit `.unwrap()` or `.expect()` calls in user-facing async or I/O workflows.
- Errors propagated from network or file operations include full cause chains and explanatory context.
- Running with `RUST_BACKTRACE=1` renders actionable stack traces for captured errors.

---

### Issue 5: [FEAT] Structured Telemetry, Logging, & Tracing System (`tracing` + `tracing-subscriber`)

- **Labels**: `enhancement`, `priority: medium`
- **Priority**: Medium (P2)
- **Target Version**: `v0.2.0`

#### Background & Problem Statement
Diagnostics throughout the project rely exclusively on `println!`. This creates inflexible terminal noise, prevents filtering by log severity levels, lacks timestamping, and makes it impossible to trace asynchronous execution flow across Tokio worker threads.

#### User Story
As an operator, I want structured, leveled logging (info, debug, warn, error) that can be filtered dynamically via environment variables and exported to files or JSON pipelines.

#### Technical Specifications & Architecture
- Integrate `tracing` and `tracing-subscriber`.
- Initialize logging subscriber at application startup with `EnvFilter` support (honoring `RUST_LOG`).
- Add span instrumentation (`#[tracing::instrument]`) on async network calls and heavy computational loops.
- Support both human-readable terminal output and structured JSON formatting via CLI flag (`--log-format=json`).

#### Implementation Tasks
- [ ] Add `tracing = "0.1"` and `tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }` to `Cargo.toml`.
- [ ] Create `src/telemetry.rs` with subscriber initialization logic.
- [ ] Replace `println!` statements in asynchronous and core workflows with appropriate tracing macros (`info!`, `debug!`, `warn!`, `error!`).
- [ ] Add execution duration tracking spans around module executions.

#### Acceptance Criteria
- Setting `RUST_LOG=debug cargo run` produces formatted debug logs with timestamps and module namespaces.
- Setting `RUST_LOG=warn cargo run` suppresses routine information logs.
- Asynchronous tasks display trace IDs / span metadata allowing correlation across threads.

---

### Issue 6: [FEAT] Comprehensive Automated Test Suite & Micro-benchmarks (`criterion` + Unit/Integration Tests)

- **Labels**: `enhancement`, `priority: high`
- **Priority**: High (P1)
- **Target Version**: `v0.2.0`

#### Background & Problem Statement
The codebase currently contains no automated unit tests, integration tests, or performance benchmarks. Any modification or dependency upgrade risks silent functional regressions without verification.

#### User Story
As a contributor, I want automated unit and integration tests and regression benchmarks so that I can refactor and optimize code with complete confidence.

#### Technical Specifications & Architecture
- Unit tests: Colocated in each module within `#[cfg(test)] mod tests` blocks.
- Integration tests: Dedicated `tests/` directory testing end-to-end CLI commands and module outputs.
- Benchmarks: Dedicated `benches/` directory leveraging `criterion` for statistical performance measurement (throughput, CPU cycles, and memory efficiency).

#### Implementation Tasks
- [ ] Add unit test suites for all 13 modules (testing edge cases, bounds, and error conditions).
- [ ] Add `criterion = { version = "0.5", features = ["html_reports"] }` under `[dev-dependencies]`.
- [ ] Create `tests/cli_integration.rs` testing CLI execution paths using `assert_cmd` and `predicates`.
- [ ] Create `benches/collections_bench.rs` comparing vector and hash map allocation latency.
- [ ] Create `benches/async_bench.rs` measuring Tokio task scheduling overhead.

#### Acceptance Criteria
- Running `cargo test` executes all unit and integration tests with 100% pass rate.
- Running `cargo bench` executes statistical benchmarks and generates HTML reports in `target/criterion/`.
- Edge cases (e.g. division by zero in `error_handling.rs`, empty slices in `generics.rs`) are explicitly covered by tests.

---

### Issue 7: [FEAT] Enterprise CI/CD Pipeline via GitHub Actions (Linting, Formatting, Multi-OS Matrix)

- **Labels**: `enhancement`, `priority: medium`
- **Priority**: Medium (P2)
- **Target Version**: `v0.2.0`

#### Background & Problem Statement
Pull requests and commits are not automatically verified. Code quality, formatting consistency, dependency security, and multi-platform compilation are prone to human oversight.

#### User Story
As a project maintainer, I want every pull request and commit to be automatically tested, formatted, linted, and audited across operating systems before merging.

#### Technical Specifications & Architecture
- Build a GitHub Actions workflow in `.github/workflows/ci.yml`.
- Multi-OS matrix: `ubuntu-latest`, `windows-latest`, `macos-latest`.
- Pipeline stages:
  1. **Formatting**: `cargo fmt -- --check`.
  2. **Linting**: `cargo clippy --all-targets --all-features -- -D warnings`.
  3. **Testing**: `cargo test --verbose`.
  4. **Security Audit**: `cargo audit` to scan for known crate vulnerabilities.
  5. **Documentation**: `cargo doc --no-deps`.

#### Implementation Tasks
- [ ] Create `.github/workflows/ci.yml`.
- [ ] Create `.github/workflows/audit.yml` for scheduled security auditing.
- [ ] Add rustfmt configuration file `.rustfmt.toml` to standardize coding conventions.
- [ ] Add Shields.io CI workflow badge to `README.md`.

#### Acceptance Criteria
- Workflow triggers automatically on push to `master` and pull requests.
- All matrix jobs pass across Linux, Windows, and macOS.
- Any formatting deviation or Clippy warning causes the CI pipeline to fail with detailed annotations.

---

### Issue 8: [FEAT] Data Persistence & State Storage Layer (Embedded SQLite / `redb`)

- **Labels**: `enhancement`, `priority: low`
- **Priority**: Low (P3)
- **Target Version**: `v0.4.0`

#### Background & Problem Statement
Currently, all laboratory runs, benchmark metrics, and experiment telemetry are ephemeral; all data vanishes as soon as the terminal session ends. There is no historical tracking of benchmark regressions or user execution records.

#### User Story
As a researcher or developer, I want the laboratory to persist benchmark results, execution history, and metrics into a lightweight local database so that I can compare performance over time and query past runs.

#### Technical Specifications & Architecture
- Implement an embedded persistence layer using either `rusqlite` (SQLite) or pure Rust `redb`.
- Schema design:
  - `experiments`: Record experiment ID, module name, timestamp, Rust version, and status.
  - `metrics`: Store execution duration (nanoseconds), CPU time, allocations, and custom parameters.
- Provide a CLI query command: `cargo run -- history --module async-demo`.

#### Implementation Tasks
- [ ] Add `rusqlite = { version = "0.32", features = ["bundled"] }` (or `redb`) to `Cargo.toml`.
- [ ] Create `src/storage/` module with connection pool and automatic schema migration.
- [ ] Implement CRUD repository pattern for recording experiment runs.
- [ ] Hook benchmark runs into the storage engine to automatically log telemetry upon completion.
- [ ] Add CLI subcommand `yourust history` to inspect previous runs in tabular format.

#### Acceptance Criteria
- Running benchmarks automatically records a timestamped entry in the local database (`.yourust_data.db`).
- Querying history via CLI displays past runs with durations, timestamps, and metric summaries.
- Database file is properly excluded in `.gitignore` to prevent committing local databases.
