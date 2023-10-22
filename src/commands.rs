use poise::{serenity_prelude as serenity, CreateReply};
use ::serenity::collector::component_interaction_collector;

use std::result::Result;

use chrono::NaiveDate;

use crate::database_utils;
use crate::utils;

pub struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn write_table(ctx: Context<'_>) -> Result<(), Error> {
    let _pool = database_utils::establish_connection().await?;

    // stop execution for 1sec
    std::thread::sleep(std::time::Duration::from_secs(1));

    let take1 = "\u{001b}[0;31m2021-01-01\u{001b}[0;0m";
    let retake1 = "\u{001b}[0;32m2021-01-02\u{001b}[0;0m";
    let retake2 = "\u{001b}[0;32m2021-01-03\u{001b}[0;0m";

    let mut table = String::new();
    table.push_str("# Test hehe\n");
    table.push_str("```ansi\n");
    table.push_str("+--------+------------+------------+------------+\n");
    table.push_str("| Type   | Take 1     | Retake 1   | Retake 2   |\n");
    table.push_str("+--------+------------+------------+------------+\n");
    table.push_str(&format!(
        "| Lab 0  | {:<10} | {:<10} | {:<10} |\n",
        take1, retake1, retake2
    ));
    table.push_str("+--------+------------+------------+------------+\n");
    table.push_str("```");

    let response = table;
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn courses(ctx: Context<'_>) -> Result<(), Error> {
    let pool = database_utils::establish_connection().await?;
    let courses = database_utils::get_all_courses(&pool).await?;
    
    let response = utils::build_courses_table(courses);

    ctx.say(response).await?;
    Ok(())
}

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!(
        "```ansi\n\u{001b}[0;31m{}'s\u{001b}[0;0m account was created at {}```",
        u.name,
        u.created_at()
    );
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn insert_course(
    ctx: Context<'_>,

    #[description = "Name of the course"]
    #[min_length = 3]
    #[max_length = 80]
    name: String,

    #[description = "Course code"]
    #[min_length = 3]
    #[max_length = 10]
    code: String,

    #[description = "Semester in which course is offered"]
    #[min = 1]
    #[max = 20]
    semester: i64,

    #[description = "Year in which course is offered"]
    #[min = 1000]
    #[max = 9999]
    year: i64,

    #[description = "Credits course provides"]
    #[min = 0.0]
    #[max = 999.0]
    credit: f64,
) -> Result<(), Error> {
    let pool = database_utils::establish_connection().await?;
    let course = database_utils::Course {
        id: 0,
        name,
        code,
        semester,
        year,
        credit,
    };

    let rows_affected = database_utils::insert_course(&pool, &course).await?;

    let response = format!("Inserted {} rows", rows_affected);
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn remove_course(ctx: Context<'_>, id: i32) -> Result<(), Error> {
    let pool = database_utils::establish_connection().await?;
    let course = database_utils::delete_course(&pool, id).await?;

    match course {
        Some(course) => {
            let response = format!("Deleted course: {}, ID: {}", course.name, course.id);
            ctx.say(response).await?;
        }
        None => {
            let response = format!("Course not found with id: {}", id);
            ctx.say(response).await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn insert_assessment(
    ctx: Context<'_>,

    #[description = "Name of the assessment"]
    #[min_length = 3]
    #[max_length = 80]
    name: String,

    #[description = "Weight of the assessment"]
    #[min = 0.0]
    #[max = 999.0]
    weight: f64,

    #[description = "Date of the assessment. Format: YYYY-MM-DD"]
    #[min_length = 3]
    #[max_length = 12]
    take1: String,

    #[description = "Date of the retake. Format: YYYY-MM-DD"]
    retake1: String,

    #[description = "Date of the retake. Format: YYYY-MM-DD"]
    retake2: String,

    #[description = "Course ID"]
    #[min = 0]
    #[max = 999]
    fk_course_id: i64,
) -> Result<(), Error> {
    let pool = database_utils::establish_connection().await?;

    match NaiveDate::parse_from_str(&take1, "%Y-%m-%d") {
        Ok(_) => {}
        Err(_) => {
            let response = format!("Invalid date format: {}", take1);
            ctx.say(response).await?;
            return Ok(());
        }
    }

    match NaiveDate::parse_from_str(&retake1, "%Y-%m-%d") {
        Ok(_) => {}
        Err(_) => {
            let response = format!("Invalid date format: {}", retake1);
            ctx.say(response).await?;
            return Ok(());
        }
    }

    match NaiveDate::parse_from_str(&retake2, "%Y-%m-%d") {
        Ok(_) => {}
        Err(_) => {
            let response = format!("Invalid date format: {}", retake2);
            ctx.say(response).await?;
            return Ok(());
        }
    }

    let assessment = database_utils::Assessment {
        id: 0,
        name,
        weight,
        take1: take1,
        retake1: retake1,
        retake2: retake2,
        fk_course_id,
    };

    let rows_affected = database_utils::insert_assessment(&pool, &assessment).await?;

    let response = format!("Inserted {} rows", rows_affected);
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn remove_assessment(ctx: Context<'_>, id: i32) -> Result<(), Error> {
    let pool = database_utils::establish_connection().await?;
    let assessment = database_utils::delete_assessment(&pool, id).await?;

    match assessment {
        Some(assessment) => {
            let response = format!(
                "Deleted assessment: {}, ID: {}",
                assessment.name, assessment.id
            );
            ctx.say(response).await?;
        }
        None => {
            let response = format!("Assessment not found with id: {}", id);
            ctx.say(response).await?;
        }
    }

    Ok(())
}

use serenity::builder::CreateButton;
use serenity::model::application::component::ButtonStyle;
use ::serenity::futures::StreamExt;
use std::time::Duration;

#[poise::command(slash_command)]
pub async fn list_courses(ctx: Context<'_>, page: Option<usize>) -> Result<(), Error> {
    let connection = database_utils::establish_connection().await?;
    let courses = database_utils::get_all_courses(&connection).await?;

    let page = page.unwrap_or(1);
    let courses_per_page = 5;

    if page > (courses.len() / courses_per_page) + 1 {
        let response = format!("Page {} does not exist", page);
        ctx.say(response).await?;
        return Ok(());
    }

    let range = utils::calculate_range(page, courses_per_page, courses.len());
    let courses_table = utils::build_courses_table(courses[range].to_vec());

    let previous_button = CreateButton::default()
        .style(ButtonStyle::Primary)
        .label("Previous")
        .custom_id("previous_page")
        .disabled(true)
        .clone();

    let next_button = CreateButton::default()
        .label("Next")
        .style(ButtonStyle::Primary)
        .custom_id("next_page")
        .disabled(true)
        .clone();

    let content = match courses.len() <= courses_per_page {
        true => format!("# Courses list\n{}", courses_table),
        false => format!("# Courses list (Page {}/{})\n{}", page, (courses.len() / courses_per_page) + 1, courses_table),
    };

    let response: poise::ReplyHandle<'_> = ctx.send(|m| {
        m.content(content).components(|c| {
            c.create_action_row(|row| {
                row.add_button(previous_button)
                    .add_button(next_button)
            })
        })
    }).await?;

    
    let m = response.message().await?;
    
    // Wait for multiple interactions
    let mut interaction_stream =
            m.await_component_interactions(&ctx).timeout(Duration::from_secs(10 * 2)).build();

    while let Some(interaction) = interaction_stream.next().await {
        let custom_id = interaction.data.custom_id.as_str();
        interaction.create_interaction_response(&ctx, |r| {
            r.kind(serenity::model::application::interaction::InteractionResponseType::DeferredUpdateMessage)
            .interaction_response_data(|d| d.content(custom_id))
        }).await?;
    }

    response.edit(ctx, |m| {
        m.components(|c| c)
    }).await?;

    Ok(())
}