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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ManaPip {
    Colored(ManaColor),
    Generic(u8),
    Phyrexian(ManaColor),
    Hybrid(Box<(ManaPip, ManaPip)>),
    Snow,
    Variable,
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
