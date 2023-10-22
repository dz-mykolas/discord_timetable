use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

use crate::database_utils::Course;

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