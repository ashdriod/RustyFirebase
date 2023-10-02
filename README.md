# Firebase CRUD App in Rust

This application demonstrates basic CRUD (Create, Read, Update, Delete) operations on Firebase Realtime Database using Rust.

## Overview

The application defines a `User` struct, which will be used as the data model for interacting with Firebase. The operations available in this application include:

- **Create**: Add a new `User` to the database.
- **Read**: Fetch all users or a specific user by ID.
- **Update**: Modify details of an existing user.
- **Delete**: Remove a user from the database.

## Setup

### Prerequisites

- [Rust](https://www.rust-lang.org/): Ensure you have Rust installed.
- A Firebase Realtime Database: Ensure you have set up a [Firebase project](https://firebase.google.com/docs/database).

### Dependencies

This application uses the following Rust crates:

- `firebase_rs`: For Firebase operations.
- `serde`: For serialization and deserialization of data.
- `tokio`: An async runtime for Rust.

Make sure to add these to your `Cargo.toml`.
