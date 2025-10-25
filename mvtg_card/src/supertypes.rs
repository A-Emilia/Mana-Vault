use std::str::FromStr;

use rocket::form::{self, FromFormField, ValueField};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuperType {
    Basic,
    Legendary,
    Snow,
    Host,
    Ongoing,
    World,
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for SuperType {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        SuperType::from_str(field.value).map_err(|_| form::Error::validation("Unable to parse super type.").into())
    }
}

impl FromStr for SuperType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Basic" => Ok(SuperType::Basic),
            "Legendary" => Ok(SuperType::Legendary),
            "Snow" => Ok(SuperType::Snow),
            "Host" => Ok(SuperType::Host),
            "Ongoing" => Ok(SuperType::Ongoing),
            "World" => Ok(SuperType::World),
            _ => Err(()),
        }
    }
}