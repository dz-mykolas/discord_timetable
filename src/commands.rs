use std::result::Result;

use chrono::NaiveDate;

use crate::database_utils;
use crate::utils;

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
            let response = format!("Warning, invalid date format of take1: {}", take1);
            ctx.send(|m| m.content(response).ephemeral(true)).await?;
        }
    }

    match NaiveDate::parse_from_str(&retake1, "%Y-%m-%d") {
        Ok(_) => {}
        Err(_) => {
            let response = format!("Warning, invalid date format of retake1: {}", retake1);
            ctx.send(|m| m.content(response).ephemeral(true)).await?;
        }
    }

    match NaiveDate::parse_from_str(&retake2, "%Y-%m-%d") {
        Ok(_) => {}
        Err(_) => {
            let response = format!("Warning, invalid date format of retake2: {}", retake2);
            ctx.send(|m| m.content(response).ephemeral(true)).await?;
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
    ctx.send(|m| m.content(response).ephemeral(true)).await?;

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

    let content = utils::format_course_response(&courses, page)?;

    // If there are less than 5 courses, just send it
    if courses.len() <= utils::COURSES_PER_PAGE {
        ctx.say(content).await?;
        return Ok(());
    }

    // Adding buttons
    let (previous_button, next_button, refresh_button) =
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
    user_cooldown = "5"
)]
pub async fn list_assessments(
    ctx: Context<'_>,
    course_id: i64,
    page: Option<usize>,
) -> Result<(), Error> {
    let connection = database_utils::establish_connection().await?;
    let assessments = database_utils::get_course_assessments(&connection, course_id).await?;
    let courses = database_utils::get_all_courses(&connection).await?;

    let page = page.unwrap_or(1);

    if page > (assessments.len() / utils::ASSESSMENTS_PER_PAGE) + 1 {
        let response = format!("Page {} does not exist", page);
        ctx.say(response).await?;
        return Ok(());
    }

    let course_name = match courses.iter().find(|course| course.id == course_id) {
        Some(course) => course.name.clone(),
        None => "No course selected".to_string(),
    };

    let mut content = utils::format_assessment_response(&assessments, page, &course_name)?;

    let select_menu = match utils::create_courses_select_menu(&courses, course_id) {
        Ok(menu) => menu,
        Err(e) => {
            content = format!("Error creating select menu: Course not found. Please select a course from the list below.");
            e
        }
    };

    if assessments.len() <= utils::ASSESSMENTS_PER_PAGE {
        ctx.send(|m| {
            m.content(content)
                .components(|c| c.add_action_row(select_menu))
        })
        .await?;
        return Ok(());
    }

    Ok(())
}
