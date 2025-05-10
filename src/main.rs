use reqwest::get;
use scryfall::card::Card;
use std::fs::File;
use std::io::{Cursor, copy};

#[tokio::main]
async fn main() {
    match Card::named("Lightning Bolt").await {
        Ok(card) => {
            println!("Card name: {}", card.name);
            println!("Card set: {}", card.set);
            println!("Card images: {:?}", card.image_uris);
            let card_image_url = card.image_uris.unwrap().normal.unwrap();
            println!("Card image URL: {:?}", card_image_url);
            let response = get(card_image_url).await.unwrap();
            let mut file = File::create("lightning_bolt.jpg").unwrap();
            let mut content = Cursor::new(response.bytes().await.unwrap());
            copy(&mut content, &mut file).unwrap();
        }
        Err(e) => panic!("{e:?}"),
    }
}
