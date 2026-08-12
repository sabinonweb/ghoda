# ghoda

A Rust web application built by following [*Zero To Production In Rust*](https://www.zero2prod.com/) by Luca Palmieri — an email newsletter backend built with Actix-web, following the book's chapter-by-chapter approach to production-grade Rust.

## About

This project follows the zero2prod curriculum: building a newsletter subscription service from scratch, covering routing, database integration, error handling, telemetry, and deployment, while learning idiomatic Rust patterns for web backends along the way.

## Tech Stack

- **[Actix-web](https://actix.rs/)** — web framework
- **[sqlx](https://github.com/launchbadge/sqlx)** — async, compile-time checked SQL (PostgreSQL)
- **[secrecy](https://docs.rs/secrecy)** — wrapper types for sensitive data (API keys, credentials) to prevent accidental logging/exposure
- **[reqwest](https://docs.rs/reqwest)** — HTTP client (used for the outbound email client)
- **[tokio](https://tokio.rs/)** — async runtime

## Getting Started

### Prerequisites

- Rust (stable toolchain — see `rust-toolchain.toml` if present)
- PostgreSQL
- `sqlx-cli` — install with:
```bash
  cargo install sqlx-cli --no-default-features --features rustls,postgres
```

### Setup

1. Clone the repo:
```bash
   git clone <repo-url>
   cd ghoda
```

2. Set up the database:
```bash
   ./scripts/init_db.sh
```

3. Copy the example configuration and fill in secrets:
```bash
   cp configuration/base.yaml.example configuration/base.yaml
```

4. Run the app:
```bash
   cargo run
```

### Running tests

```bash
cargo test
```

> **Note:** the email client tests use a mock server (`wiremock`) to simulate the outbound email API — no real credentials or network calls needed for the test suite.

## Project Structure
src/
├── main.rs # entry point
├── startup.rs # app/server bootstrapping
├── configuration.rs # config loading (secrecy-wrapped values)
├── routes/ # HTTP handlers
├── domain/ # core types and validation
└── email_client.rs # outbound email integration
tests/
└── api/ # black-box integration tests

## Notable Implementation Details

- **Secrets handling**: sensitive config values (DB credentials, API keys) are wrapped in a custom `SecretString` newtype built on `secrecy::SecretBox`, ensuring they're redacted from `Debug`/logs and can't be accidentally leaked.
- **Error handling**: custom error types implement proper conversions (e.g. `reqwest::Error` handling in the email client) rather than relying on stringly-typed errors.

## Status

Work in progress — following the book chapter by chapter.

## Deployment
I didn't have the DigitalOcean pro pack. I tried to workaround but I have skipped that part for now.

## Acknowledgements

Built by following [*Zero To Production In Rust*](https://www.zero2prod.com/) by Luca Palmieri.
