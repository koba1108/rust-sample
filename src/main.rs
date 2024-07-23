extern crate core;

use chrono::NaiveDate;
use crate::models::user::User;

mod models;
mod utils;

fn main() {
    let birthday= NaiveDate::from_ymd_opt(2000, 1, 1);
    let user = User::new("John", birthday.unwrap());
    match user {
        Ok(user) => {
            println!("User created: {:?}", user);
            println!("User age: {}", user.age());
            return;
        }
        Err(e) => {
            println!("Failed to create user: {}", e);
            return;
        }
    }
}
