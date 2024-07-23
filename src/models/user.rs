use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use validator::{Validate, ValidationError};
use crate::utils::{calc_age};


#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct User {
    pub id: Ulid,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(custom(function = "validate_birthday"))]
    pub birthday: NaiveDate,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

impl User {
    pub fn new(name: &str, birthday: NaiveDate) -> Result<Self, String> {
        let user = User {
            id: Ulid::new(),
            name: name.to_string(),
            birthday,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        Ok(user)
    }

    pub fn age(&self) -> i32 {
        calc_age(&self.birthday)
    }
}

fn validate_birthday(birthday: &NaiveDate) -> Result<(), ValidationError> {
    // is before today
    if birthday > &Utc::now().naive_utc().date() {
        return Err(ValidationError::new("birthday is must be before today"));
    }
    // is adult
    if calc_age(&birthday) < 20 {
        return Err(ValidationError::new("user must be adult"));
    }
    Ok(())
}
