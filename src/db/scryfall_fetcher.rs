
//! Utility for transforming Scryfall Card JSON objects into Card objects used by Mana Vault.
//! | Scryfall JSON Field         | Mana Vault Card Field       |
//! |:----------------------------|:----------------------------|
//! | id                          | `id: String`                |
//! | oracle_id                   | `oracle_id: String`         |
//! | name                        | `name: String`              |
//! | set                         | `set_code: String`          |
//! | oracle_text                 | `text: String`              |
//! | mana_cost                   | `cost: ManaCost`            |
//! | type_line                   | `supertype: Vec<SuperType>` |
//! | type_line                   | `card_type: Vec<CardType>`  |
//! | type_line                   | `subtype: Vec<String>`      |
//! 

use std::{iter, str::FromStr, vec};

use eframe::egui::TextBuffer;
use regex::{Regex, RegexBuilder};
use reqwest::{self, Client, Request, Response};
use rocket::{http::ext::IntoCollection, serde::json::{self, to_string}};
use sqlx::Value;

 /* Scryfall Json Fields Needed.
 card
  oracle_id     ->      general_id
  name          ->      name
  oracle_text   ->      text

 card_printing
  id            ->      id
  oracle_id     ->      card_id
  image_uris    ->      image_url
  rarity        ->      rarity

 card_mana_cost
  oracle_id     ->      card_id
  mana_cost     ->      cost_id
      Cost id is 
  
  
  
  */

use crate::model::Card;

pub fn try_update() -> Result<(),()> {
    // OK scenario is final. Error scenario TBA.
    !unimplemented!()
}

fn create_request() -> Request {
    !unimplemented!()
}

fn send_request(client: Client, request: Request) -> Response {
    Client::execute(&client, request);
    !unimplemented!()
}

fn process_card_data(response: &Response) -> impl Iterator<Item = Card> {
    !unimplemented!();
    std::iter::empty()
}

fn put_card_in_database(card: Card) -> Result<(),()> {
    !unimplemented!()
}

// list of functions for transforming data

/// Takes in a `serde_json::Value` enum of a Scryfall JSON Magic: The Gathering card and extracts the subtypes as `Vec<String>`.
/// ```
/// # use serde_json::Value;
/// let data = r#"{"type_line": "Legendary Creature — Human Wizard"}"#;
/// let json: Value = serde_json::from_str(data).unwrap();
/// let res = extract_subtypes(json).unwrap();
/// assert_eq!(res, ["Human", "Wizard"]);
/// ```
fn extract_subtypes(json: serde_json::Value) -> Option<Vec<String>> {
  Some(
    json.get("type_line")?
      .as_str()?
      .split_once('—')?.1
      .trim()
      .split(' ')
      .map(str::to_string)
      .collect::<Vec<String>>()
  )
}

fn extract_types(json: serde_json::Value) -> Option<Vec<String>> {
  let pattern = RegexBuilder::new(r"(?x)
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
  |(\bVanguard\b)"
  ).build().unwrap();

  Some(
    pattern.captures_iter(json.get("type_line")?.as_str()?)
      .map(|x| x.extract())
      .map(|(_, [val])| val.to_string())
      .collect::<Vec<String>>()
  )
}

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

fn extract_mana_cost(json: serde_json::Value) -> Option<Vec<String>> {

  let pattern = RegexBuilder::new(r"\{([WUBRGCXYSP0-9/]+)\}").build().unwrap();

  Some(
    pattern.captures_iter(json.get("mana_cost")?.as_str()?)
      .map(|x| x.extract())
      .map(|(_, [val])| val.to_string())
      .collect::<Vec<String>>()
  )
}

mod test {
    use std::str::FromStr;

    use eframe::egui::TextBuffer;
    use serde_json::Value;

    use crate::{db::scryfall_fetcher::{extract_mana_cost, extract_subtypes, extract_supertypes, extract_types}, model::ManaPip};

    fn get_test_data() -> Value {
        let data = r#"{
  "object": "card",
  "id": "fff58d35-eb23-47ee-9b8c-6919ad1a413a",
  "oracle_id": "86187aed-77ce-4b2d-aab0-0a8f807a9451",
  "multiverse_ids": [
    16457
  ],
  "tcgplayer_id": 2716,
  "cardmarket_id": 11092,
  "name": "River Boa",
  "lang": "en",
  "released_at": "1999-04-21",
  "uri": "https://api.scryfall.com/cards/fff58d35-eb23-47ee-9b8c-6919ad1a413a",
  "scryfall_uri": "https://scryfall.com/card/6ed/249/river-boa?utm_source=api",
  "layout": "normal",
  "highres_image": true,
  "image_status": "highres_scan",
  "image_uris": {
    "small": "https://cards.scryfall.io/small/front/f/f/fff58d35-eb23-47ee-9b8c-6919ad1a413a.jpg?1562825095",
    "normal": "https://cards.scryfall.io/normal/front/f/f/fff58d35-eb23-47ee-9b8c-6919ad1a413a.jpg?1562825095",
    "large": "https://cards.scryfall.io/large/front/f/f/fff58d35-eb23-47ee-9b8c-6919ad1a413a.jpg?1562825095",
    "png": "https://cards.scryfall.io/png/front/f/f/fff58d35-eb23-47ee-9b8c-6919ad1a413a.png?1562825095",
    "art_crop": "https://cards.scryfall.io/art_crop/front/f/f/fff58d35-eb23-47ee-9b8c-6919ad1a413a.jpg?1562825095",
    "border_crop": "https://cards.scryfall.io/border_crop/front/f/f/fff58d35-eb23-47ee-9b8c-6919ad1a413a.jpg?1562825095"
  },
  "mana_cost": "{1}{G/P}{W/U/P}{W/U}",
  "cmc": 2.0,
  "type_line": "Legendary Artifact Creature — Cat Snake",
  "oracle_text": "Islandwalk (This creature can't be blocked as long as defending player controls an Island.)\n{G}: Regenerate this creature.",
  "power": "2",
  "toughness": "1",
  "colors": [
    "G"
  ],
  "color_identity": [
    "G"
  ],
  "keywords": [
    "Landwalk",
    "Islandwalk"
  ],
  "legalities": {
    "standard": "not_legal",
    "future": "not_legal",
    "historic": "not_legal",
    "timeless": "not_legal",
    "gladiator": "not_legal",
    "pioneer": "not_legal",
    "modern": "legal",
    "legacy": "legal",
    "pauper": "legal",
    "vintage": "legal",
    "penny": "legal",
    "commander": "legal",
    "oathbreaker": "legal",
    "standardbrawl": "not_legal",
    "brawl": "not_legal",
    "alchemy": "not_legal",
    "paupercommander": "legal",
    "duel": "legal",
    "oldschool": "not_legal",
    "premodern": "legal",
    "predh": "legal"
  },
  "games": [
    "paper"
  ],
  "reserved": false,
  "game_changer": false,
  "foil": false,
  "nonfoil": true,
  "finishes": [
    "nonfoil"
  ],
  "oversized": false,
  "promo": false,
  "reprint": true,
  "variation": false,
  "set_id": "78ee1957-d5d4-4551-acae-b1b418e8a50b",
  "set": "6ed",
  "set_name": "Classic Sixth Edition",
  "set_type": "core",
  "set_uri": "https://api.scryfall.com/sets/78ee1957-d5d4-4551-acae-b1b418e8a50b",
  "set_search_uri": "https://api.scryfall.com/cards/search?order=set&q=e%3A6ed&unique=prints",
  "scryfall_set_uri": "https://scryfall.com/sets/6ed?utm_source=api",
  "rulings_uri": "https://api.scryfall.com/cards/fff58d35-eb23-47ee-9b8c-6919ad1a413a/rulings",
  "prints_search_uri": "https://api.scryfall.com/cards/search?order=released&q=oracleid%3A86187aed-77ce-4b2d-aab0-0a8f807a9451&unique=prints",
  "collector_number": "249",
  "digital": false,
  "rarity": "uncommon",
  "flavor_text": "\"But no one heard the snake's gentle hiss for peace over the elephant's trumpeting of war.\"\n—Afari, *Tales*",
  "card_back_id": "0aeebaf5-8c7d-4636-9e82-8c27447861f7",
  "artist": "Steve White",
  "artist_ids": [
    "674a172f-e11a-458b-a681-964726a0c320"
  ],
  "illustration_id": "d41f9827-e2c5-49db-8ad5-9533fda38f68",
  "border_color": "white",
  "frame": "1997",
  "full_art": false,
  "textless": false,
  "booster": true,
  "story_spotlight": false,
  "edhrec_rank": 15411,
  "penny_rank": 14069,
  "prices": {
    "usd": "0.39",
    "usd_foil": null,
    "usd_etched": null,
    "eur": "0.40",
    "eur_foil": null,
    "tix": null
  },
  "related_uris": {
    "gatherer": "https://gatherer.wizards.com/Pages/Card/Details.aspx?multiverseid=16457&printed=false",
    "tcgplayer_infinite_articles": "https://partner.tcgplayer.com/c/4931599/1830156/21018?subId1=api&trafcat=tcgplayer.com%2Fsearch%2Farticles&u=https%3A%2F%2Fwww.tcgplayer.com%2Fsearch%2Farticles%3FproductLineName%3Dmagic%26q%3DRiver%2BBoa",
    "tcgplayer_infinite_decks": "https://partner.tcgplayer.com/c/4931599/1830156/21018?subId1=api&trafcat=tcgplayer.com%2Fsearch%2Fdecks&u=https%3A%2F%2Fwww.tcgplayer.com%2Fsearch%2Fdecks%3FproductLineName%3Dmagic%26q%3DRiver%2BBoa",
    "edhrec": "https://edhrec.com/route/?cc=River+Boa"
  },
  "purchase_uris": {
    "tcgplayer": "https://partner.tcgplayer.com/c/4931599/1830156/21018?subId1=api&u=https%3A%2F%2Fwww.tcgplayer.com%2Fproduct%2F2716%3Fpage%3D1",
    "cardmarket": "https://www.cardmarket.com/en/Magic/Products?idProduct=11092&referrer=scryfall&utm_campaign=card_prices&utm_medium=text&utm_source=scryfall",
    "cardhoarder": "https://www.cardhoarder.com/cards?affiliate_id=scryfall&data%5Bsearch%5D=River+Boa&ref=card-profile&utm_campaign=affiliate&utm_medium=card&utm_source=scryfall"
  }
}"#;

let v: Value = serde_json::from_str(data).unwrap();
return v
    }


    #[test]
    fn test() {
        let data: Value = get_test_data();

        let res = extract_mana_cost(data).unwrap();
        print!("Result: {:?}", res);

        let res2 = res.iter().map(String::as_str).map(ManaPip::from_str).collect::<Vec<Result<ManaPip, ()>>>();
        print!("\nResult 2: {:?}", res2);
    }
}
