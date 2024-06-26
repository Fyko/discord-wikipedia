use anyhow::Result;
use time::format_description::well_known::Iso8601;
use time::OffsetDateTime;
use twilight_model::application::interaction::application_command::CommandOptionValue;
use twilight_model::http::interaction::InteractionResponseData;
use twilight_model::{
    application::{
        command::{CommandOptionChoice, CommandOptionChoiceValue, CommandOptionType},
        interaction::application_command::CommandData,
    },
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::embed::{
    EmbedBuilder, EmbedFieldBuilder, EmbedFooterBuilder, ImageSource,
};

use crate::commands::text_response;
use crate::wikipedia::{fetch_page_summary, fetch_search_results};

use super::embed_response;

pub async fn chat_input(data: Box<CommandData>) -> Result<InteractionResponse> {
    let title = data
        .options
        .iter()
        .find(|opt| opt.name == "title")
        .map(|opt| match &opt.value {
            CommandOptionValue::String(value) => value,
            _ => panic!("WITCHCRAFT"),
        });
    let Some(title) = title else {
        return Ok(text_response("You need to provide a title!", true));
    };

    let plaintext = data
        .options
        .iter()
        .find(|opt| opt.name == "plaintext")
        .map_or(false, |opt| match opt.value {
            CommandOptionValue::Boolean(value) => value,
            _ => false,
        });

    let summary = match fetch_page_summary(&title).await {
        Ok(summary) => summary,
        Err(e) => {
            tracing::warn!("{e}");
            return Ok(text_response(e, true));
        }
    };

    let title = summary.title;
    let url = summary.content_urls.desktop.page;
    let description = html2md::parse_html(&summary.description);
    let excerpt = html2md::parse_html(&summary.extract_html);
    let mut last_updated = String::new();
    if let Ok(timestamp) = OffsetDateTime::parse(&summary.timestamp, &Iso8601::DEFAULT) {
        last_updated = format!(
            "Last updated <t:{timestamp}:F> (<t:{timestamp}:R>)",
            timestamp = timestamp.unix_timestamp()
        );
    }

    if plaintext {
        let content = indoc::formatdoc! {r#"
            ## [{title}](<{url}>)
            {description}
            ### Excerpt
            {excerpt}

            {last_updated}
        "#};

        Ok(text_response(content, false))
    } else {
        let mut embed = EmbedBuilder::new()
            .title(&title)
            .url(&url)
            .field(EmbedFieldBuilder::new("Description", &description).build())
            .field(EmbedFieldBuilder::new("Excerpt", &excerpt).build())
            .footer(EmbedFooterBuilder::new("Powered by Wikipedia").build());

        if !last_updated.is_empty() {
            embed = embed.description(last_updated);
        }

        if let Some(thumbnail) = summary.thumbnail {
            if let Ok(source) = ImageSource::url(thumbnail.source) {
                embed = embed.thumbnail(source);
            }
        }

        let built = embed.build();

        Ok(embed_response(vec![built], false))
    }
}

pub async fn autocomplete(data: Box<CommandData>) -> anyhow::Result<InteractionResponse> {
    let title =
        data.options
            .iter()
            .find(|opt| opt.name == "title")
            .map_or("Discord".to_string(), |opt| match opt.value {
                CommandOptionValue::Focused(ref value, CommandOptionType::String) => {
                    value.to_string()
                }
                _ => "Discord".to_string(),
            });

    let title = if title.is_empty() {
        "Discord".to_string()
    } else {
        title
    };

    let search_results = fetch_search_results(&title).await?;

    let choices = search_results
        .pages
        .into_iter()
        .map(|page| {
            let name = if let Some(description) = page.description {
                format!("{}: {description}", page.title)
            } else {
                page.title
            };

            // if the name is more than 100 characters, truncate it and add an ellipsis
            let name = if name.len() > 100 {
                format!("{}...", &name[..97])
            } else {
                name
            };

            CommandOptionChoice {
                name,
                name_localizations: None,
                value: CommandOptionChoiceValue::String(page.key),
            }
        })
        .collect::<Vec<_>>();
    tracing::debug!("choices: {choices:#?}");

    Ok(InteractionResponse {
        kind: InteractionResponseType::ApplicationCommandAutocompleteResult,
        data: Some(InteractionResponseData {
            choices: Some(choices),
            ..Default::default()
        }),
    })
}
