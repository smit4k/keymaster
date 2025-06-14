use poise::{serenity_prelude as serenity};
use serenity::builder::CreateEmbed;
use crate::{Context, Error};
use csv::Reader;
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Record {
    key: String,
    value: String,
    #[serde(flatten)]
    other_fields: std::collections::HashMap<String, String>,
}

/// Redeem a key
#[poise::command(slash_command, ephemeral)]
pub async fn redeem(
    ctx: Context<'_>,
    #[description = "The key to redeem"] key: String,
) -> Result<(), Error> {
    let exists = check_key_exists_deserialize("keys.csv", &key).unwrap();
    let key_value = check_key_value_deserialize("keys.csv", &key);

    let embed = if exists {
        CreateEmbed::default()
            .title("✅ Valid Key")
            .description(key_value.unwrap())
            .color(0x27ae60)
    } else {
        CreateEmbed::default()
            .title("❌ Invalid Key")
            .color(0xff4444)
    };
    let reply = poise::CreateReply::default().embed(embed);
    ctx.send(reply).await?;
    Ok(())
}

fn check_key_exists_deserialize(file_path: &str, target_key: &str) -> Result<bool, Error> {
    let file = File::open(file_path)?;
    let mut reader = Reader::from_reader(file);
    
    for result in reader.deserialize() {
        let record: Record = result?;
        if record.key == target_key {
            return Ok(true);
        }
    }
    
    Ok(false)
}

fn check_key_value_deserialize(file_path: &str, target_key: &str) -> Result<String, Error> {
    let file = File::open(file_path)?;
    let mut reader = Reader::from_reader(file);
    
    for result in reader.deserialize() {
        let record: Record = result?;
        if record.key == target_key {
            return Ok(record.value);
        }
    }
    
    Ok("".to_string())
}
