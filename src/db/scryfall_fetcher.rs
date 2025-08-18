use std::iter;

use eframe::egui::TextBuffer;
use reqwest::{self, Client, Request, Response};
use rocket::{http::ext::IntoCollection, serde::json};
use sqlx::Value;

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

fn extract_subtypes(json: serde_json::Value) -> Vec<String> {
  
  let mut res = json.get("type_line")
  .expect("No type_line field found.")
  .to_string()
  .trim_matches('"')
  .split_once('—')
  .expect("No subtypes found.")
  .1
  .trim()
  .take();

  if res.split_once(' ').is_some() {
    return res.split(' ').map(|x| x.to_string()).collect::<Vec<String>>()
  }

  vec![res]
}

mod test {
    use eframe::egui::TextBuffer;
    use serde_json::Value;

    use crate::db::scryfall_fetcher::extract_subtypes;

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
  "mana_cost": "{1}{G}",
  "cmc": 2.0,
  "type_line": "Creature — Cat Snake",
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

        let res = extract_subtypes(data);
        print!("Result: {:?}", res);

    }
}
