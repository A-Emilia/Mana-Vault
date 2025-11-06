# Mana Vault

Mana Vault is a Magic: The Gathering Card search engine and collection tracker developed in the Rust programming language.

## Communication Channels (REST)
A client can access the Mana Vault server through one of its HTTP endpoints.

TODO: TABLE WITH ENDPOINTS


## `mvtg_card` Crate
This crate provides the struct Mana Vault uses for handling Magic: The Gathering cards.

```Rust
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
```

TODO: DESCRIBE WHY RUST IS COOL

## Scryfall Data Source
The underlying data for Mana Vault comes from the public Scryfall API. This data is stored in a 3NF MySQL database. The raw JSON data that Scryfall provides cannot be natively stored in a relational database so that it follows higher normal forms. Thus, the data is serialized to the `Card` struct which is then saved to the database.


TODO: ADD EER DIAGRAM OF DATABASE


Magic: The Gathering divides the types of a card into three main parts: Supertypes, types, and subtypes. Scryfall's Card object provides a singular string for the entire type line (`type_line`) of a card. Mana Vault provides a functional approach to transforming many such Scryfall object fields into its composite parts.

```Rust
fn extract_supertypes(json: serde_json::Value) -> Option<Vec<String>> {
  let pattern = RegexBuilder::new(r"(?x)
  (\bBasic\b)
  |(\bLegendary\b)
  |(\bSnow\b)
  |(\bHost\b)
  |(\bOngoing\b)
  |(\bWorld\b)"
  ).build().unwrap();

  Some(
    pattern.captures_iter(json.get("type_line")?.as_str()?)
      .map(|x| x.extract())
      .map(|(_, [val])| val.to_string())
      .collect::<Vec<String>>()
  )
}
```

TODO: THIS SHOULD USE STRING SLICES TO AVOID ALLOCATING UNNECESSARY MEMORY. MAJOR FUTURE REFACTOR
