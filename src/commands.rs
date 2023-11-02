use poise::serenity_prelude as serenity;
use poise::ReplyHandle;
use serenity::model::application::interaction::message_component::MessageComponentInteraction;

use std::result::Result;
use std::sync::Arc;

use chrono::NaiveDate;

use crate::database_utils;
use crate::utils;

use serenity::builder::CreateButton;
use serenity::futures::StreamExt;
use serenity::model::application::component::ButtonStyle;
use serenity::model::application::interaction::{InteractionResponseType, MessageFlags};
use std::time::Duration;

pub struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, default_member_permissions = "ADMINISTRATOR")]
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

#[poise::command(slash_command, default_member_permissions = "ADMINISTRATOR")]
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

#[poise::command(slash_command, default_member_permissions = "ADMINISTRATOR")]
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

    #[description = "Date of the retake. Format: YYYY-MM-DD"] retake1: String,

    #[description = "Date of the retake. Format: YYYY-MM-DD"] retake2: String,

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

#[poise::command(slash_command, default_member_permissions = "ADMINISTRATOR")]
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

#[poise::command(
    slash_command,
    default_member_permissions = "SEND_MESSAGES",
    user_cooldown = "5"
)]
pub async fn list_courses(ctx: Context<'_>, page: Option<usize>) -> Result<(), Error> {
    let connection = database_utils::establish_connection().await?;
    let courses = database_utils::get_all_courses(&connection).await?;
    let page = page.unwrap_or(1);

    if page > (courses.len() / utils::COURSES_PER_PAGE) + 1 {
        let response = format!("Page {} does not exist", page);
        ctx.say(response).await?;
        return Ok(());
    }

    let range = utils::calculate_range(page, utils::COURSES_PER_PAGE, courses.len());
    let courses_table = utils::build_courses_table(courses[range].to_vec());

    // If there are less than 5 courses, don't add page number
    let content = match courses.len() <= utils::COURSES_PER_PAGE {
        true => format!("# Courses list\n{}", courses_table),
        false => format!(
            "# Courses list (Page {}/{})\n{}",
            page,
            (courses.len() / utils::COURSES_PER_PAGE) + 1,
            courses_table
        ),
    };

    // If there are less than 5 courses, just send it
    if courses.len() <= utils::COURSES_PER_PAGE {
        ctx.say(content).await?;
        return Ok(());
    }

    // Adding buttons
    let (previous_button, next_button) =
        utils::create_buttons(page, courses.len() / utils::COURSES_PER_PAGE);

    ctx.send(|m| {
        m.content(content).components(|c| {
            c.create_action_row(|row| row.add_button(previous_button).add_button(next_button))
        })
    })
    .await?;

    Ok(())
}

#[poise::command(
    slash_command,
    default_member_permissions = "SEND_MESSAGES",
    user_cooldown = "30"
)]
pub async fn list_assessments(
    ctx: Context<'_>,
    course_id: i32,
    page: Option<usize>,
) -> Result<(), Error> {
    let connection = database_utils::establish_connection().await?;
    let assessments = database_utils::get_course_assessments(&connection, course_id).await?;

    let page = page.unwrap_or(1);
    let assessments_per_page = 5;

    if page > (assessments.len() / assessments_per_page) + 1 {
        let response = format!("Page {} does not exist", page);
        ctx.say(response).await?;
        return Ok(());
    }

    let range = utils::calculate_range(page, assessments_per_page, assessments.len());
    let assessments_table = utils::build_assessments_table(assessments[range].to_vec());

    let content = match assessments.len() <= assessments_per_page {
        true => format!("# Assessments list\n{}", assessments_table),
        false => format!(
            "# Assessments list (Page {}/{})\n{}",
            page,
            (assessments.len() / assessments_per_page) + 1,
            assessments_table
        ),
    };

    if assessments.len() <= assessments_per_page {
        ctx.say(content).await?;
        return Ok(());
    }

    Ok(())
}
