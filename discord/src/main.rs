use gen::Captcha;
use twilight_http::Client as HttpClient;
use twilight_gateway::{Intents, Shard, ShardId, Event};
use twilight_model::oauth::Application;

use std::env;
use std::sync::Arc;

struct AppState {
    http: HttpClient,
    application: Application,
}

async fn handle_event(event: Event, state: Arc<AppState>) -> anyhow::Result<()> {
    match event {
        Event::Ready(_) => {
            println!("Ready!");
            let interaction_http = state.http.interaction(state.application.id);
            
        }
        Event::InteractionCreate(interaction) => {

        }
        _ => {}
    }
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let token = env::var("DISCORD_TOKEN")?;
    let http = HttpClient::new(token.clone());
    let application = http.current_user_application()
        .await?
        .model()
        .await?;
    let intents = Intents::GUILDS | Intents::GUILD_MEMBERS;
    let mut shard = Shard::new(ShardId::ONE, token, intents);
    let state = Arc::new(AppState { http, application });
    loop {
        let event = match shard.next_event().await {
            Ok(event) => event,
            Err(e) => {
                if e.is_fatal() {
                    break;
                }

                continue;
            }
        };

        tokio::spawn(handle_event(event, Arc::clone(&state)));
    }
    Ok(())
}
