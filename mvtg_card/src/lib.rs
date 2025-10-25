use std::str::FromStr;

use rocket::{form::{self, FromFormField, ValueField}, futures::future::ok, FromForm};
use serde::{Serialize, Deserialize};

pub use crate::{card_types::CardType, mana_cost::ManaCost, supertypes::SuperType};
/// The main struct for representing Magic: The Gathering Cards used across Mana Vault.
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromForm)]
pub struct Card {
    pub id: String,
    pub oracle_id: String,
    pub name: String,
    pub set_code: String,
    pub text: String,
    pub cost: ManaCost,
    pub supertype: Vec<SuperType>,
    pub card_type: Vec<CardType>,
    pub subtype: Vec<String>,
}

impl Card {
    pub fn new(
        id: String,
        oracle_id: String,
        name: String,
        set_code: String,
        text: String,
        cost: ManaCost,
        supertype: Vec<SuperType>,
        card_type: Vec<CardType>,
        subtype: Vec<String>,
    ) -> Card {
        Card {id, oracle_id, name, set_code, text, cost, supertype, card_type, subtype}
    }
}

pub mod mana_cost;
pub mod supertypes;
pub mod card_types;