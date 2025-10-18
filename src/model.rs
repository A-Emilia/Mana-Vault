use std::str::FromStr;

use rocket::{form::{self, FromFormField, ValueField}, futures::future::ok, FromForm};
use serde::{Serialize, Deserialize};
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

/// Constructor.
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

/// Wrapper type for representing the full mana cost of a `Card` struct.
pub type ManaCost = Option<Vec<ManaPip>>;

/// Enum for representing each of Magic: The Gatherings five colors along with Colorless.
/// This enum should never be used on its own, but should rather be wrapped in a `ManaPip`.
#[derive(Debug, Clone, Serialize, Deserialize, FromFormField)]
pub enum ManaColor {
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
}

impl FromStr for ManaColor {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ManaColor::*;

        let res = match s.trim_matches(['{', '}']) {
            "W" => White,
            "U" => Blue,
            "B" => Black,
            "R" => Red,
            "G" => Green,
            "C" => Colorless,

            _ => return Err(())
        };
        Ok(res)
    }
}

/// Enum representing an individual pip of Magic: The Gathering mana.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ManaPip {
    Colored(ManaColor),
    Generic(u8),
    Phyrexian(ManaColor),
    Hybrid(Box<(ManaPip, ManaPip)>),
    Snow,
    Variable,
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for ManaPip {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        ManaPip::from_str(field.value).map_err(|_| form::Error::validation("Unable to parse mana pip.").into())
    }
}

impl FromStr for ManaPip {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ManaPip::*;

        let mut pips = s.trim_matches(['{', '}']).split('/').collect::<Vec<&str>>();
        let phyrexian = pips.pop_if(|x| *x == "P").is_some();

        match pips.len() {
            1 => return make_pip(pips[0], phyrexian),
            2 => return {
                let x = make_pip(pips[0], phyrexian)?;
                let y = make_pip(pips[1], phyrexian)?;
                Ok(Hybrid(Box::new((x,y))))
            },
            _ => return Err(()),
        }

        fn make_pip(mana_pip: &str, is_phyrexian: bool) -> Result<ManaPip, ()> {
            match mana_pip {
                "X"|"Y"|"Z" => return Ok(Variable),
                "S" => return Ok(Snow),

                "W"|"U"|"B"|"R"|"G" if !is_phyrexian => return Ok(Colored(ManaColor::from_str(mana_pip)?)),
                "W"|"U"|"B"|"R"|"G" if is_phyrexian => return Ok(Phyrexian(ManaColor::from_str(mana_pip)?)),
                x if {x.parse::<u8>().is_ok()} => return Ok(Generic(x.parse::<u8>().unwrap())),
                _ => return Err(()),
            };
        }
    }
}

// Add more functionality to this. Maybe split Card-Types into permanet and non-permanent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CardType {
    Artifact,
    Creature,
    Enchantment,
    Instant,
    Land,
    Planeswalker,
    Sorcery,
    Battle,
    Kindred,
    Conspiracy,
    Dungeon,
    Eaturecray,
    Phenomenon,
    Plane,
    Scheme,
    Summon,
    Vanguard,
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for CardType {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        CardType::from_str(field.value).map_err(|_| form::Error::validation("Unable to parse card type.").into())
    }
}

impl FromStr for CardType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Artifact" => Ok(CardType::Artifact),
            "Creature" => Ok(CardType::Creature),
            "Enchantment" => Ok(CardType::Enchantment),
            "Instant" => Ok(CardType::Instant),
            "Land" => Ok(CardType::Land),
            "Planeswalker" => Ok(CardType::Planeswalker),
            "Sorcery" => Ok(CardType::Sorcery),
            "Battle" => Ok(CardType::Battle),
            "Kindred" => Ok(CardType::Kindred),
            "Conspiracy" => Ok(CardType::Conspiracy),
            "Dungeon" => Ok(CardType::Dungeon),
            "Eaturecray" => Ok(CardType::Eaturecray),
            "Phenomenon" => Ok(CardType::Phenomenon),
            "Plane" => Ok(CardType::Plane),
            "Scheme" => Ok(CardType::Scheme),
            "Summon" => Ok(CardType::Summon),
            "Vanguard" => Ok(CardType::Vanguard),
            _ => Err(()),
        }
    }
}

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

/* COOL SOLUTION; NOT SUPER PORTABLE:

struct CostElement(u8);
impl CostElement {
    const WHITE     = 1 << 0;
    const BLUE      = 1 << 1;
    const BLACK     = 1 << 2;
    const RED       = 1 << 3;
    const GREEN     = 1 << 4;
    const COLORLESS = 1 << 5;

    c = WHITE | BLUE;
    contains_blue = c & BLUE;
}
*/
