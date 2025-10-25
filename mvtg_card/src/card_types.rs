use std::str::FromStr;

use rocket::form::{self, FromFormField, ValueField};
use serde::{Deserialize, Serialize};

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
impl CardType{
    const REGEX_PATTERN: &str = r"(?x)
  (\bArtifact\b)
  |(\bCreature\b)
  |(\bEnchantment\b)
  |(\bInstant\b)
  |(\bLand\b)
  |(\bPlaneswalker\b)
  |(\bSorcery\b)
  |(\bBattle\b)
  |(\bKindred\b)
  |(\bConspiracy\b)
  |(\bDungeon\b)
  |(\bEaturecray\b)
  |(\bPhenomenon\b)
  |(\bPlane\b)
  |(\bScheme\b)
  |(\bSummon\b)
  |(\bVanguard\b)";
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