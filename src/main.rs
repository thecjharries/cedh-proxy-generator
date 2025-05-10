use reqwest::get;
use scryfall::card::Card;
use std::fs::File;
use std::io::{Cursor, copy};

use ril::prelude::*;

#[tokio::main]
async fn main() {
    match Card::named("Lightning Bolt").await {
        Ok(card) => {
            println!("Card name: {}", card.name);
            println!("Card set: {}", card.set);
            let card_image_url = card.image_uris.unwrap().png.unwrap();
            println!("Card image URL: {:?}", card_image_url);
            let response = get(card_image_url).await.unwrap();
            let mut file = File::create("lightning_bolt.png").unwrap();
            let mut content = Cursor::new(response.bytes().await.unwrap());
            copy(&mut content, &mut file).unwrap();
        }
        Err(e) => panic!("{e:?}"),
    }
    let mut image = Image::<Rgb>::open("lightning_bolt.png").unwrap();
    let font = Font::open("src/HomeVideo-BLG6G.ttf", 128.0).unwrap();
    let rectangle = Rectangle::at(0, 250)
        .with_size(image.width(), 200)
        .with_fill(Rgb::new(0, 0, 0));
    image.draw(&rectangle);
    // let text_for_image = TextSegment::new(&font, "PLAYTEST", Rgb::new(255, 0, 0));
    TextLayout::new()
        .with_position(image.width() / 2, 250 + 36)
        .with_basic_text(&font, "PLAYTEST", Rgb::new(255, 0, 0))
        .with_horizontal_anchor(HorizontalAnchor::Center)
        .with_align(TextAlign::Center)
        .draw(&mut image);
    image.save(ImageFormat::Png, "lightning_bolt.png").unwrap();
}
