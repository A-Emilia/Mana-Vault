#[derive(Default, Debug, Clone)]
pub struct Card {
    pub id: usize,
    pub name: String,
    pub set_code: String,
    pub text: String,
    pub cost: Option<ManaCost>,
}


#[derive(Debug, Clone)]
pub enum ManaColor{
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
    /* Rare ones */
    Snow,
    Phyrexian,
}

#[derive(Debug, Clone)]
pub enum ManaCost{
    Colored(ManaColor),
    Generic(u8),
    Variadic,
    Either(Vec<ManaCost>),
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