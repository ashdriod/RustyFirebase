// Importing necessary libraries and modules
use std::collections::HashMap;
use firebase_rs::*;
use serde::{Deserialize, Serialize};

// Define a struct `User` with fields name, age, and email
#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

// Define a struct `Response` with a single field name
#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
}

#[tokio::main] // This is a macro that designates the entry point of a Tokio-based async application
async fn main() {
    // Create a sample user instance
    let user = User {
        name: "Ashwin Vazhappilly".to_string(),
        age: 25,
        email: "ashwinvazhappilly@gmail.com".to_string(),
    };

    // Create a Firebase client for a given URL
    let firebase = Firebase::new("https://ashrusty-debe1-default-rtdb.europe-west1.firebasedatabase.app/").unwrap();

    // Add user to Firebase and get a response
    let response = set_user(&firebase, &user).await;

    // Retrieve the user we just added
    let mut user = get_user(&firebase, &response.name).await;
    println!("{:?}", user);

    // Retrieve all users
    let users = get_users(&firebase).await;
    println!("{:?}", users);

    // Update the email of the user and print the updated user
    user.email = "updated.mail@gmail.com".to_string();
    let updated_user = update_user(&firebase, &response.name, &user).await;
    println!("{:?}", updated_user);

    // Delete the user from Firebase
    delete_user(&firebase, &response.name).await;
    println!("User deleted!");
}

// Function to set (add) a user in Firebase
async fn set_user(firebase_client: &Firebase, user: &User) -> Response {
    let firebase = firebase_client.at("users");
    let _users = firebase.set::<User>(&user).await;
    return string_to_response(&_users.unwrap().data);
}

// Function to retrieve all users from Firebase
async fn get_users(firebase_client: &Firebase) -> HashMap<String, User> {
    let firebase = firebase_client.at("users");
    let users = firebase.get::<HashMap<String, User>>().await;
    println!("{:?}", users);
    return users.unwrap();
}

// Function to retrieve a specific user by ID from Firebase
async fn get_user(firebase_client: &Firebase, id: &String) -> User {
    let firebase = firebase_client.at("users").at(&id);
    let user = firebase.get::<User>().await;
    return user.unwrap();
}

// Function to update a specific user by ID in Firebase
async fn update_user(firebase_client: &Firebase, id: &String, user: &User) -> User {
    let firebase = firebase_client.at("users").at(&id);
    let _user = firebase.update::<User>(&user).await;
    return string_to_user(&_user.unwrap().data);
}

// Function to delete a specific user by ID from Firebase
async fn delete_user(firebase_client: &Firebase, id: &String) {
    let firebase = firebase_client.at("users").at(&id);
    let _result = firebase.delete().await;
}

// Helper function to convert a JSON string to `Response` struct
fn string_to_response(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}

// Helper function to convert a JSON string to `User` struct
fn string_to_user(s: &str) -> User {
    serde_json::from_str(s).unwrap()
}
