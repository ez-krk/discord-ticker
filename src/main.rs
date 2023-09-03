use anyhow::anyhow;
use num_format::{Locale, ToFormattedString};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::Activity;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tokio::time;
use tracing::{error, info};
struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                error!("Error sending message: {:?}", e);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let mut interval = time::interval(std::time::Duration::from_secs(600));

        let client = reqwest::Client::new();

        info!("{} is connected!", ready.user.name);

        let gecko_base = "https://api.coingecko.com/api/v3";

        // change this list
        let crypto = "bitcoin";
        let fiat = "usd";

        let gecko_price = format!(
            "{}/simple/price?ids={}&vs_currencies={}",
            gecko_base, crypto, fiat
        );

        tokio::spawn(async move {
            loop {
                interval.tick().await;
                let price = fetch(&client, &gecko_price, crypto, fiat).await;
                println!("{}", price);
                ctx.set_activity(Activity::watching(format!("$ {}", &price)))
                    .await;
            }
        });
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}

async fn fetch(client: &reqwest::Client, url: &String, crypto: &str, fiat: &str) -> String {
    let body = client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();
    let data = body.get(crypto).unwrap();
    data[format!("{}", fiat)]
        .as_u64()
        .unwrap()
        .to_formatted_string(&Locale::en)
}
