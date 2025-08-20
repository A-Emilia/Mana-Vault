use std::str::FromStr;

use rocket::futures::future::ok;
use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: usize,
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
        id: usize,
        name: String,
        set_code: String,
        text: String,
        cost: ManaCost,
        supertype: Vec<SuperType>,
        card_type: Vec<CardType>,
        subtype: Vec<String>,
    ) -> Card {
        Card {id, name, set_code, text, cost, supertype, card_type, subtype}
    }
    
}

type ManaCost = Option<Vec<ManaPip>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ManaPip {
    Colored(ManaColor),
    Generic(u8),
    Phyrexian(ManaColor),
    Hybrid(Box<(ManaPip, ManaPip)>),
    Snow,
    Variable,
}

impl FromStr for ManaPip {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ManaPip::*;
        use ManaColor::*;
        let s = s.trim_matches(['{', '}']);

        let mut pips = s.split('/').collect::<Vec<&str>>();
        let phyrexian = pips.pop_if(|x| *x == "P").is_some();
        // If pips.len > 1 && phyrexian == false = hybrid
        // If pips.len > 1 && phyrexian == true = phyrexian hybrid

        fn make_pip(mana_pip: &str, is_phyrexian: bool) -> Result<ManaPip, ()> {

            match mana_pip {
                "X" => Variable,
                "Y" => Variable,
                "Z" => Variable,
                "S" => Snow,

                // Current progress
                x if {x.parse::<u8>().is_ok()} => Generic(x.parse::<u8>().unwrap()),



                _ => return Err(());
            };
            !unimplemented!()
        }
        

        let res = match s {
            "W" => Colored(White),
            "U" => Colored(Blue),
            "B" => Colored(Black),
            "R" => Colored(Red),
            "G" => Colored(Green),
            "C" => Colored(Colorless),

            // Rest of numbers
            "0" => Generic(0),

            "W/U" => Hybrid(Box::new((Colored(White), Colored(Blue)))),
            "W/B" => Hybrid(Box::new((Colored(White), Colored(Black)))),


            _ => return Err(())
        };
        Ok(res)
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuperType {
    Basic,
    Legendary,
    Snow,
    Host,
    Ongoing,
    World,
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
