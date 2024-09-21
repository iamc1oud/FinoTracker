# FinoTracker
Rust powered Fino Tracker

## Database Setup

**Installing Diesel CLI**

Diesel provides a separate CLI tool to help manage your project. Since it’s a standalone binary, and doesn’t affect your project’s code directly, we don’t add it to Cargo.toml. Instead, we just install it on our system.

We provide pre-built binaries for diesel cli. You can install the command line tool via:

#### **Linux/MacOS**

```bash
cargo install diesel_cli --no-default-features --features postgres
```

For more info: https://diesel.rs/guides/getting-started
