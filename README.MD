# Web App build in RUST 🦀

## Requirements

- rust lang
- postgres

## Backend SETUP

run the setup script from `bin/setup.sh`

.env file

```
DATABASE_URL=postgres://localhost/rust_web_app
RUST_LOG=trace
```

## Start the backend server

```
cd backend
cargo run
```
