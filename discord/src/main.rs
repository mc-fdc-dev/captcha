use gen::Captcha;
use twilight_gateway::{Event, Intents, Shard, ShardId};
use twilight_http::Client as HttpClient;
use twilight_model::{
    application::{
        command::CommandType,
        interaction::{InteractionData, InteractionType},
    },
    http::{
        attachment::Attachment,
        interaction::{InteractionResponse, InteractionResponseType},
    },
    oauth::Application,
};
use twilight_util::builder::{command::CommandBuilder, InteractionResponseDataBuilder};

use std::env;
use std::sync::Arc;

struct AppState {
    http: HttpClient,
    application: Application,
}

async fn handle_event(event: Event, state: Arc<AppState>) -> anyhow::Result<()> {
    match event {
        Event::Ready(_) => {
            let interaction_http = state.http.interaction(state.application.id);
            interaction_http
                .set_global_commands(&[CommandBuilder::new(
                    "generate",
                    "Test generate image",
                    CommandType::ChatInput,
                )
                .build()])
                .await?;
            log::info!("Ready!");
        }
        Event::InteractionCreate(interaction) => {
            let interaction_http = state.http.interaction(state.application.id);
            match interaction.kind {
                InteractionType::ApplicationCommand => {
                    match interaction.data.clone().unwrap() {
                        InteractionData::ApplicationCommand(command) => {
                            match command.name.as_str() {
                                "generate" => {
                                    let mut captcha = Captcha::new();
                                    let (_text, data) = captcha.generate().unwrap();
                                    let attachment =
                                        Attachment::from_bytes("captcha.png".to_string(), data, 1);
                                    let response = InteractionResponseDataBuilder::new()
                                        .content("Here is your captcha")
                                        .attachments(vec![attachment])
                                        .build();
                                    interaction_http.create_response(interaction.id, &interaction.token, &InteractionResponse {
                                        kind: InteractionResponseType::ChannelMessageWithSource,
                                        data: Some(response),
                                    })
                                        .await?;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            };
        }
        _ => {}
    }
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    log::info!("Starting bot...");
    dotenvy::dotenv().ok();
    let token = env::var("DISCORD_TOKEN")?;
    let http = HttpClient::new(token.clone());
    let application = http.current_user_application().await?.model().await?;
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
