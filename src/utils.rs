use chrono::{NaiveDate, Utc};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use comfy_table::Cell;

use crate::database_utils::{Assessment, Course};

pub fn build_courses_table(courses: Vec<Course>) -> String {
    let mut table = Table::new();
    table
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(100)
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["ID", "Name", "Code", "Semester", "Year", "Credit"]);

    for course in courses {
        table.add_row(vec![
            Cell::new(&course.id.to_string()),
            Cell::new(&course.name),
            Cell::new(&course.code),
            Cell::new(&course.semester.to_string()),
            Cell::new(&course.year.to_string()),
            Cell::new(&course.credit.to_string()),
        ]);
    }

    String::from("```ansi\n") + &table.to_string() + "```"
}

pub fn build_assessments_table(assessments: Vec<Assessment>) -> String {
    let mut table = Table::new();

    table
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(100)
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            "ID",
            "Name",
            "Take 1",
            "Retake 1",
            "Retake 2",
            "Weight",
            "Course ID",
        ]);

    for assessment in assessments {
        let take1 = add_date_color(assessment.take1.clone());
        let retake1 = add_date_color(assessment.retake1.clone());
        let retake2 = add_date_color(assessment.retake2.clone());
        // let take1 = console::style(assessment.take1.clone()).red();

        table.add_row(vec![
            Cell::new(&assessment.id.to_string()),
            Cell::new(&assessment.name),
            Cell::new(&take1),
            Cell::new(&retake1),
            Cell::new(&retake2),
            Cell::new(&assessment.weight.to_string()),
            Cell::new(&assessment.fk_course_id.to_string()),
        ]);
    }

    String::from("```ansi\n") + &table.to_string() + "```"
}

pub fn calculate_range(
    current_page: usize,
    per_page: usize,
    total: usize,
) -> std::ops::Range<usize> {
    let start = (current_page - 1) * per_page;
    let end = if start + per_page > total {
        total
    } else {
        start + per_page
    };

    start..end
}

fn add_date_color(date: String) -> String {
    let today = Utc::now().naive_utc().date();
    let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap();

    let result = if date < today {
        format!("{}{}{}", "\u{001b}[31m", date.format("%Y-%m-%d").to_string(), "\u{001b}[0m")
    } else if date <= today + chrono::Duration::days(7) {
        format!("{}{}{}", "\u{001b}[33m", date.format("%Y-%m-%d").to_string(), "\u{001b}[0m")
    } else {
        format!("{}{}{}", "\u{001b}[32m", date.format("%Y-%m-%d").to_string(), "\u{001b}[0m")
    };
    
    result
}