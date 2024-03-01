use ::serenity::model::application::interaction::InteractionResponseType;
use poise::serenity_prelude as serenity;
use serenity::model::application::interaction;
use serenity::model::prelude::interaction::MessageFlags;

use crate::{database_utils, utils};

type Error = Box<dyn std::error::Error + Send + Sync>;

pub fn parse_select_menu_course_id(select_menu_id: &str) -> Result<i64, Error> {
    println!("Select menu id: {}", select_menu_id);
    let mut split_id = select_menu_id.split(";");

    let course_id = match split_id.next() {
        Some("select_course") => match split_id.last() {
            Some(id) => id.parse::<i64>().unwrap(),
            None => return Err("No course id".into()),
        },
        _ => return Err("Wrong value".into()),
    };

    Ok(course_id)
}

pub fn parse_button_page(button_id: &str) -> Result<usize, Error> {
    let mut split_id = button_id.split(";");

    let button_id = match split_id.next() {
        Some("next_page") => "next_page",
        Some("previous_page") => "previous_page",
        Some("refresh_page") => "refresh_page",
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

    Ok(new_page)
}

pub async fn check_interaction_caller(
    ctx: &serenity::Context,
    interaction: interaction::Interaction,
) -> Result<bool, Error> {
    let msg_component = interaction.as_message_component().unwrap();
    let author = (msg_component.message.interaction).clone().unwrap().user;
    let caller = (msg_component.user).clone();

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
        return Ok(());
    }

    let button_id = match msg_component.data.component_type {
        serenity::model::application::component::ComponentType::Button => {
            msg_component.data.custom_id
        }
        _ => return Err("Not a button".into()),
    };

    let connection = database_utils::establish_connection().await?;
    let courses = database_utils::get_all_courses(&connection).await?;

    let mut new_page = parse_button_page(&button_id)?;
    if new_page > (courses.len() / utils::COURSES_PER_PAGE) + 1 {
        new_page = 1;
    }

    let content = utils::format_course_response(&courses, new_page)?;

    let (previous_button, next_button, refresh_button) =
        utils::create_buttons(new_page, courses.len() / utils::COURSES_PER_PAGE);

    if courses.is_empty() {
        interaction.as_message_component().unwrap().create_interaction_response(&ctx, |r| {
            r.kind(serenity::model::application::interaction::InteractionResponseType::UpdateMessage)
                .interaction_response_data(|d| {
                    d.content("No courses found").components(|c| {
                        c.create_action_row(|row| {
                            row
                                .add_button(previous_button.clone())
                                .add_button(next_button.clone())
                                .add_button(refresh_button.clone())
                        })
                    })
                })
        }).await?;
        return Ok(());
    }

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
                            .add_button(refresh_button.clone())
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
        return Ok(());
    }

    let course_id;
    let mut new_page = 1;

    match msg_component.data.component_type {
        serenity::model::application::component::ComponentType::Button => {
            let button_id = msg_component.data.custom_id.clone();
            new_page = parse_button_page(&button_id)?;

            let select_menu = msg_component.message.components.clone()[1].clone();
            let select_menu_id = match select_menu.components[0].clone() {
                serenity::model::application::component::ActionRowComponent::SelectMenu(menu) => {
                    menu.custom_id.unwrap_or("".to_string())
                }
                _ => return Err("Not a select menu".into()),
            };
            course_id = match parse_select_menu_course_id(&select_menu_id) {
                Ok(id) => id,
                Err(e) => {
                    println!("Error parsing select menu id: {}", e);
                    return Ok(());
                }
            };
        }
        serenity::model::application::component::ComponentType::SelectMenu => {
            course_id = msg_component.data.values[0].clone().parse::<i64>().unwrap();
        }
        _ => return Err("Wrong component type".into()),
    };

    let connection = database_utils::establish_connection().await?;
    let courses = database_utils::get_all_courses(&connection).await?;
    let assessments = database_utils::get_course_assessments(&connection, course_id).await?;

    if new_page > (assessments.len() / utils::ASSESSMENTS_PER_PAGE) + 1 {
        return Ok(());
    }

    let course_name = match courses.iter().find(|course| course.id == course_id) {
        Some(course) => course.name.clone(),
        None => "No course selected".to_string(),
    };

    let mut content = utils::format_assessment_response(&assessments, new_page, &course_name)?;
    let select_menu = match utils::create_courses_select_menu(&courses, course_id) {
        Ok(menu) => menu,
        Err(e) => {
            content = format!("Error creating select menu: Course not found. Please select a course from the list below.");
            e
        }
    };

    if assessments.len() <= utils::ASSESSMENTS_PER_PAGE {
        interaction.as_message_component().unwrap().create_interaction_response(&ctx, |r| {
            r.kind(serenity::model::application::interaction::InteractionResponseType::UpdateMessage)
                .interaction_response_data(|d| {
                    d.content(content)
                        .components(|c| c.add_action_row(select_menu))
                })
        }).await?;
        return Ok(());
    }

    let (previous_button, next_button, refresh_button) =
        utils::create_buttons(new_page, assessments.len() / utils::ASSESSMENTS_PER_PAGE);

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
                    .add_action_row(select_menu)
                })
            })
        })
        .await?;

    Ok(())
}
