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

## Usage

Run the application using:
cargo run

## The application will:

- Add a sample user to Firebase.
- Retrieve and display the added user.
- Retrieve and display all users.
- Update the email of the added user and display the updated user.
- Delete the user.

## Code Structure

### Structs:
- **User**: Represents the data model.
- **Response**: Represents the Firebase response after creating a user.

### Main Function:
- Sets up a sample user and demonstrates the CRUD operations.

### CRUD Functions:
- **set_user**: Adds a user to Firebase.
- **get_users**: Retrieves all users.
- **get_user**: Retrieves a user by ID.
- **update_user**: Modifies user details.
- **delete_user**: Removes a user.

### Utility Functions:
- **string_to_response**: Converts a JSON string to a Response object.
- **string_to_user**: Converts a JSON string to a User object.

## Contributions

Contributions are welcome! Feel free to submit a pull request.
