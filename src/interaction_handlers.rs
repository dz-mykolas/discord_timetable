use std::io::BufRead;
use std::sync::Arc;

use ::serenity::builder::CreateButton;
use ::serenity::model::application::interaction::message_component::MessageComponentInteraction;
use ::serenity::model::application::interaction::InteractionResponseType;
use poise::serenity_prelude as serenity;
use serenity::model::application::interaction;
use serenity::model::prelude::interaction::MessageFlags;

use crate::commands::Data;
use crate::{database_utils, utils};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub async fn check_interaction_caller(
    ctx: &serenity::Context,
    interaction: interaction::Interaction,
) -> Result<bool, Error> {
    let msg_component = interaction.as_message_component().unwrap().clone();
    let author = msg_component.message.clone().interaction.unwrap().user;
    let caller = msg_component.user.clone();

    if author.id != caller.id {
        msg_component
            .create_interaction_response(&ctx, |r| {
                r.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|d| {
                        d.content("This is not your interaction!")
                            .flags(MessageFlags::EPHEMERAL)
                    })
            })
            .await?;

        return Ok(false);
    }

    Ok(true)
}

pub async fn list_courses_handler(
    ctx: &serenity::Context,
    interaction: &serenity::model::application::interaction::Interaction,
) -> Result<(), Error> {
    //println!("Interaction: {:?}", interaction);

    let msg_component = match interaction {
        serenity::model::application::interaction::Interaction::MessageComponent(component) => {
            component.clone()
        }
        _ => return Err("Not a message component".into()),
    };

    if check_interaction_caller(&ctx, interaction.clone()).await? == false {
        // Send ephemeral message
        interaction.as_message_component().unwrap().create_interaction_response(&ctx, |r| {
            r.kind(serenity::model::application::interaction::InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|d| {
                    d.content("This is not your interaction!")
                        .flags(serenity::model::prelude::interaction::MessageFlags::EPHEMERAL)
                })
        }).await?;
    }

    let button_id = match msg_component.data.component_type {
        serenity::model::application::component::ComponentType::Button => {
            msg_component.data.custom_id
        }
        _ => return Err("Not a button".into()),
    };

    let mut split_id = button_id.split(";");

    let button_id = match split_id.next() {
        Some("next_page") => "next_page",
        Some("previous_page") => "previous_page",
        _ => return Err("nope".into()),
    };

    let current_page = match split_id.last() {
        Some(page) => page.parse::<usize>().unwrap(),
        None => return Err("No page number".into()),
    };

    let new_page = match button_id {
        "previous_page" => current_page - 1,
        "next_page" => current_page + 1,
        _ => current_page,
    };

    let connection = database_utils::establish_connection().await?;
    let courses = database_utils::get_all_courses(&connection).await?;

    if new_page > (courses.len() / utils::COURSES_PER_PAGE) + 1 {
        return Ok(());
    }

    let range = utils::calculate_range(new_page, utils::COURSES_PER_PAGE, courses.len());
    let courses_table = utils::build_courses_table(courses[range].to_vec());

    let content = match courses.len() <= utils::COURSES_PER_PAGE {
        true => format!("# Courses list\n{}", courses_table),
        false => format!(
            "# Courses list (Page {}/{})\n{}",
            new_page,
            (courses.len() / utils::COURSES_PER_PAGE) + 1,
            courses_table
        ),
    };

    if courses.len() <= utils::COURSES_PER_PAGE {
        interaction.as_message_component().unwrap().create_interaction_response(&ctx, |r| {
            r.kind(serenity::model::application::interaction::InteractionResponseType::UpdateMessage)
                .interaction_response_data(|d| {
                    d.content(content)
                })
        }).await?;
        return Ok(());
    }

    let (previous_button, next_button) =
        utils::create_buttons(new_page, courses.len() / utils::COURSES_PER_PAGE);

    interaction
        .as_message_component()
        .unwrap()
        .create_interaction_response(&ctx, |r| {
            r.kind(
                serenity::model::application::interaction::InteractionResponseType::UpdateMessage,
            )
            .interaction_response_data(|d| {
                d.content(content).components(|c| {
                    c.create_action_row(|row| {
                        row.add_button(previous_button.clone())
                            .add_button(next_button.clone())
                    })
                })
            })
        })
        .await?;

    Ok(())
}

pub async fn list_assessments_handler(
    ctx: &serenity::Context,
    interaction: &serenity::model::application::interaction::Interaction,
) -> Result<(), Error> {
    let msg_component = match interaction {
        serenity::model::application::interaction::Interaction::MessageComponent(component) => {
            component.clone()
        }
        _ => return Err("Not a message component".into()),
    };

    if check_interaction_caller(&ctx, interaction.clone()).await? == false {
        // Send ephemeral message
        interaction.as_message_component().unwrap().create_interaction_response(&ctx, |r| {
            r.kind(serenity::model::application::interaction::InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|d| {
                    d.content("This is not your interaction!")
                        .flags(serenity::model::prelude::interaction::MessageFlags::EPHEMERAL)
                })
        }).await?;
    }

    let button_id = match msg_component.data.component_type {
        serenity::model::application::component::ComponentType::Button => {
            msg_component.data.custom_id
        }
        _ => return Err("Not a button".into()),
    };

    let mut split_id = button_id.split(";");

    let button_id = match split_id.next() {
        Some("next_page") => "next_page",
        Some("previous_page") => "previous_page",
        _ => return Err("nope".into()),
    };

    let current_page = match split_id.last() {
        Some(page) => page.parse::<usize>().unwrap(),
        None => return Err("No page number".into()),
    };

    let new_page = match button_id {
        "previous_page" => current_page - 1,
        "next_page" => current_page + 1,
        _ => current_page,
    };

    // let connection = database_utils::establish_connection().await?;
    // let assessments = database_utils::get_all_assessments(&connection).await?;

    // if new_page > (assessments.len() / utils::ASSESSMENTS_PER_PAGE) + 1 {
    //     return Ok(());
    // }

    // let range = utils::calculate_range(new_page, utils::ASSESSMENTS_PER_PAGE, assessments.len());
    // let assessments_table = utils::build_assessments_table(assessments[range].to_vec());
    
    //     let content = match assessments.len() <= utils::ASSESSMENTS_PER_PAGE {
    //     true => format!("# Assessments list\n{}", assessments_table),
    //     false => format!(
    //         "# Assessments list (Page {}/{})\n{}",
    //         new_page,
    //         (assessments.len() / utils::ASSESSMENTS_PER_PAGE) + 1,
    //         assessments_table
    //     ),
    // };

    // if assessments.len() <= utils::ASSESSMENTS_PER_PAGE {
    //     interaction.as_message_component().unwrap().create_interaction_response(&ctx, |r| {
    //         r.kind(serenity::model::application::interaction::InteractionResponseType::UpdateMessage)
    //             .interaction_response_data(|d| {
    //                 d.content(content)
    //             })
    //     }).await?;
    //     return Ok(());
    // }

    // let (previous_button, next_button) =
    //     utils::create_buttons(new_page, assessments.len() / utils::ASSESSMENTS_PER_PAGE);

    // interaction
    //     .as_message_component()
    //     .unwrap()
    //     .create_interaction_response(&ctx, |r| {
    //         r.kind(
    //             serenity::model::application::interaction::InteractionResponseType::UpdateMessage,
    //         )
    //         .interaction_response_data(|d| {
    //             d.content(content).components(|c| {
    //                 c.create_action_row(|row| {
    //                     row.add_button(previous_button.clone())
    //                         .add_button(next_button.clone())
    //                 })
    //             })
    //         })
    //     })
    //     .await?;

    Ok(())
}