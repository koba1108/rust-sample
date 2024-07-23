extern crate core;

use chrono::NaiveDate;
use crate::models::user::User;

mod models;
mod utils;

fn main() {
    let birthday= NaiveDate::from_ymd_opt(2000, 1, 1);
    let birthday2 =  NaiveDate::from_ymd_opt(2020, 1, 1);
    let user = User::new("John", birthday);
    let user2 = User::new("John", None);
    let not_valid_user = User::new("John", birthday2);
    let users = vec![user, user2, not_valid_user];

    for user in users {
        match user {
            Ok(user) => {
                println!("User created: {:?}", user);
                println!("User id: {:?}", user.id.to_string());
                println!("User age: {:?}", user.age());
            }
            Err(e) => {
                println!("Failed to create user: {}", e);
                return;
            }
        }
    }
}
