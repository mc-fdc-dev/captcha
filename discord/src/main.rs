use gen::Captcha;
use image::{codecs::png::PngEncoder, ColorType, ImageEncoder};
use twilight_gateway::{Event, Intents, Shard, ShardId};
use twilight_http::Client as HttpClient;
use twilight_model::application::command::CommandType;
use twilight_model::application::interaction::{InteractionData, InteractionType};
use twilight_model::http::attachment::Attachment;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_model::oauth::Application;
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
            println!("Ready!");
            let interaction_http = state.http.interaction(state.application.id);
            interaction_http
                .set_global_commands(&[CommandBuilder::new(
                    "generate",
                    "Test generate image",
                    CommandType::ChatInput,
                )
                .build()])
                .await?;
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
                                    let (_text, image) = captcha.generate().unwrap();
                                    let data = {
                                        let mut png_data = Vec::new();
                                        PngEncoder::new(&mut png_data).write_image(
                                            &image,
                                            image.width(),
                                            image.height(),
                                            ColorType::Rgba8,
                                        )?;
                                        png_data
                                    };
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
