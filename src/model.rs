#[derive(Default, Debug, Clone)]
pub struct Card {
    pub id: usize,
    pub name: String,
    pub set_code: String,
    pub text: String,
    pub cost: ManaCost,
    pub supertype: Option<SuperType>,
    pub card_type: Vec<CardType>,
    pub subtype: Vec<String>,
}

type ManaCost = Option<Vec<ManaPip>>;

#[derive(Debug, Clone)]
pub enum ManaColor {
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
  }
  

#[derive(Debug, Clone)]
pub enum ManaPip {
    Colored(ManaColor),
    Generic(u8),
    Phyrexian(ManaColor),
    Hybrid(Box<(ManaPip, ManaPip)>),
    Snow,
    Variable,
  }


  // Add more functionality to this. Maybe split Card-Types into permanet and non-permanent
  #[derive(Debug, Clone)]
  pub enum CardType {
    Land,
    Creature,
    Artifact,
    Enchantment,
    Planeswalker,
    Battle,
    // Non-Permanent Types
    Instant,
    Sorcery,
    Kindred,
  }

  #[derive(Debug, Clone)]
  pub enum SuperType {
    Basic,
    Legendary,
    // Ongoing, Archenemy cards will be cut from database
    Snow,
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