# Banking System

This is a Rust-based banking system project demonstrating a modern module structure.

## Project Structure
```
banking_system/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── accounts.rs
│   ├── accounts/
│   │   ├── user.rs
│   │   ├── admin.rs
│   │   ├── user/
│   │   │   ├── profile.rs
│   │   │   ├── settings.rs
│   │   ├── admin/
│   │   │   ├── permissions.rs
│   │   │   ├── reports.rs
│   ├── transactions.rs
│   ├── transactions/
│   │   ├── deposit.rs
│   │   ├── withdrawal.rs
│   ├── security.rs
│   ├── security/
│   │   ├── auth.rs
│   │   ├── encryption.rs
│   │   ├── hashing.rs
│   ├── shared.rs
│   ├── shared/
│   │   ├── utils.rs
│   │   ├── utils/
│   │   │   ├── say_hi.rs
```

## Description
- `accounts/` - Manages user and admin accounts.
- `transactions/` - Handles deposits and withdrawals.
- `security/` - Implements authentication, encryption, and hashing.
- `shared/utils/` - Contains utility functions, including `say_hi.rs`.

## Running the Project
```sh
cargo run
```

## License
MIT

