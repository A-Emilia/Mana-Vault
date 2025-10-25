use std::str::FromStr;

use rocket::form::{self, FromFormField, ValueField};
use serde::{Deserialize, Serialize};

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
