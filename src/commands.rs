use twilight_model::{
    application::interaction::application_command::CommandData,
    channel::message::{Embed, MessageFlags},
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};
use twilight_util::builder::InteractionResponseDataBuilder;

mod article;

/// Shorthand to creating a text response to an interaction.
pub fn text_response<Content>(content: Content, ephemeral: bool) -> InteractionResponse
where
    Content: ToString,
{
    InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionResponseData {
            content: Some(content.to_string()),
            flags: if ephemeral {
                Some(MessageFlags::EPHEMERAL)
            } else {
                None
            },
            ..Default::default()
        }),
    }
}

pub fn embed_response(embeds: Vec<Embed>, ephemeral: bool) -> InteractionResponse {
    InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionResponseData {
            embeds: Some(embeds),
            flags: if ephemeral {
                Some(MessageFlags::EPHEMERAL)
            } else {
                None
            },
            ..Default::default()
        }),
    }
}

pub async fn handle_command(data: Box<CommandData>) -> anyhow::Result<InteractionResponse> {
    match data.name.as_ref() {
        "article" => article::chat_input(data).await,
        _ => debug(data).await,
    }
}

pub async fn handle_auto_complete(data: Box<CommandData>) -> anyhow::Result<InteractionResponse> {
    match data.name.as_ref() {
        "article" => article::autocomplete(data).await,
        _ => debug(data).await,
    }
}

pub async fn debug(data: Box<CommandData>) -> anyhow::Result<InteractionResponse> {
    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(
            InteractionResponseDataBuilder::new()
                .content(format!("```rust\n{data:?}\n```"))
                .build(),
        ),
    })
}
