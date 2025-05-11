use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::get;
use ril::prelude::*;
use rust_embed::Embed;
use scryfall::card::{Card, ImageUris, Layout};

struct LoadedCard {
    name: String,
    image: Image<Rgb>,
}

impl LoadedCard {
    pub fn sanitized_name(self: &Self) -> String {
        static ALPHA_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)[^a-z]").unwrap());
        static UNDERSCORE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"_+").unwrap());
        format!(
            "{}.png",
            UNDERSCORE_PATTERN.replace_all(
                &ALPHA_PATTERN.replace_all(&self.name.to_lowercase(), "_"),
                "_",
            )
        )
    }

    pub fn add_text(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let font_file = Fonts::get("HomeVideo-BLG6G.ttf").expect("Font not found");
        let font = Font::from_bytes(&font_file.data, 128.0)?;
        let rectangle = Rectangle::at(0, 250)
            .with_size(self.image.width(), 200)
            .with_fill(Rgb::new(0, 0, 0));
        self.image.draw(&rectangle);
        TextLayout::new()
            .with_position(self.image.width() / 2, 250 + 36)
            .with_basic_text(&font, "PLAYTEST", Rgb::new(255, 0, 0))
            .with_horizontal_anchor(HorizontalAnchor::Center)
            .with_align(TextAlign::Center)
            .draw(&mut self.image);
        Ok(())
    }

    pub fn save(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let filename = &self.sanitized_name();
        self.add_text()?;
        self.image.save(ImageFormat::Png, filename)?;
        Ok(filename.to_string())
    }
}

#[derive(Embed)]
#[folder = "fonts"]
struct Fonts;

async fn load_card(
    cardname: String,
    image_uris: Option<ImageUris>,
) -> Result<LoadedCard, Box<dyn std::error::Error>> {
    if let Some(card_image_uris) = image_uris {
        let card_image_url = card_image_uris.png.unwrap();
        let response = get(card_image_url).await?;
        let image = Image::<Rgb>::from_bytes(ImageFormat::Png, response.bytes().await?)?;
        Ok(LoadedCard {
            name: cardname.to_string(),
            image,
        })
    } else {
        Err(format!("Card {} does not have image uris", cardname).into())
    }
}

async fn load_images(cardname: &str) -> Result<Vec<LoadedCard>, Box<dyn std::error::Error>> {
    let card = Card::named(cardname).await?;
    let has_faces = match card.layout {
        Layout::Normal => false,
        Layout::ModalDfc | Layout::Transform => true,
        _ => {
            return Err(format!(
                "Card {} has an unsupported layout: {:?}",
                card.name, card.layout
            )
            .into());
        }
    };
    let mut cards: Vec<LoadedCard> = Vec::new();
    if has_faces {
        if let Some(card_faces) = card.card_faces {
            for face in card_faces {
                cards.push(load_card(face.name, face.image_uris).await?);
            }
        }
    } else {
        cards.push(load_card(card.name, card.image_uris).await?);
    }
    Ok(cards)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let images = load_images("Delver of Secrets").await?;
    for mut image in images {
        image.save()?;
    }
    Ok(())
}
