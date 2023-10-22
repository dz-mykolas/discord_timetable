use sqlx::{Result, SqlitePool, pool};

#[derive(sqlx::FromRow, Clone)]
pub struct Course {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub semester: i64,
    pub year: i64,
    pub credit: f64,
}

#[derive(sqlx::FromRow)]
pub struct Assessment {
    pub id: i64,
    pub name: String,
    pub weight: f64,
    pub take1: String,
    pub retake1: String,
    pub retake2: String,
    pub fk_course_id: i64,
}

pub async fn establish_connection() -> Result<SqlitePool> {
    let options = sqlx::sqlite::SqliteConnectOptions::new()
        .filename("./courses.db")
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;
    Ok(pool)
}

pub async fn get_all_courses(pool: &SqlitePool) -> Result<Vec<Course>, sqlx::Error> {
    let courses = sqlx::query_as::<_, Course>("SELECT * FROM courses")
        .fetch_all(pool)
        .await?;

    Ok(courses)
}

pub async fn insert_course(pool: &SqlitePool, course: &Course) -> Result<u64, sqlx::Error> {
    let rows_affected = sqlx::query!(
        r#"
        INSERT INTO courses (name, code, semester, year, credit)
        VALUES (?, ?, ?, ?, ?)
        "#,
        course.name,
        course.code,
        course.semester,
        course.year,
        course.credit
    )
    .execute(pool)
    .await?;

    Ok(rows_affected.rows_affected())
}

pub async fn delete_course(pool: &SqlitePool, id: i32) -> Result<Option<Course>, sqlx::Error> {
    let course: Option<Course> = sqlx::query_as!(
        Course,
        r#"
        DELETE FROM courses
        WHERE id = ?
        RETURNING *
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(course)
}

pub async fn get_all_assessments(pool: &SqlitePool) -> Result<Vec<Assessment>, sqlx::Error> {
    let assessments = sqlx::query_as::<_, Assessment>("SELECT * FROM assessments")
        .fetch_all(pool)
        .await?;

    Ok(assessments)
}

pub async fn get_course_assessments(pool: &SqlitePool, fk_course_id: i32) -> Result<Vec<Assessment>, sqlx::Error> {
    let assessments = sqlx::query_as::<_, Assessment>("SELECT * FROM assessments WHERE fk_course_id = ?")
        .bind(fk_course_id)
        .fetch_all(pool)
        .await?;

    Ok(assessments)
}

pub async fn insert_assessment(pool: &SqlitePool, assessment: &Assessment) -> Result<u64, sqlx::Error> {
    let rows_affected = sqlx::query!(
        r#"
        INSERT INTO assessments (name, weight, take1, retake1, retake2, fk_course_id)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        assessment.name,
        assessment.weight,
        assessment.take1,
        assessment.retake1,
        assessment.retake2,
        assessment.fk_course_id
    )
    .execute(pool)
    .await?;

    Ok(rows_affected.rows_affected())
}

pub async fn delete_assessment(pool: &SqlitePool, id: i32) -> Result<Option<Assessment>, sqlx::Error> {
    let assessment: Option<Assessment> = sqlx::query_as!(
        Assessment,
        r#"
        DELETE FROM assessments
        WHERE id = ?
        RETURNING *
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(assessment)
}


/* UNUSED FUNCTIONS */

// async fn create_table_courses(pool: &SqlitePool) -> Result<()> {
//     sqlx::query(
//         r#"
//         CREATE TABLE IF NOT EXISTS courses (
//             id INTEGER PRIMARY KEY AUTOINCREMENT,
//             name TEXT NOT NULL,
//             code TEXT NOT NULL,
//             semester INTEGER NOT NULL,
//             year INTEGER NOT NULL,
//             credit REAL NOT NULL
//         )
//         "#,
//     )
//     .execute(pool)
//     .await?;

//     Ok(())
// }


// async fn create_table_assessment(pool: &SqlitePool) -> Result<()> {
//     sqlx::query(
//         r#"
//         CREATE TABLE IF NOT EXISTS assessments (
//             id INTEGER PRIMARY KEY AUTOINCREMENT,
//             name TEXT NOT NULL,
//             weight REAL NOT NULL,
//             take1 TEXT NOT NULL,
//             retake1 TEXT NOT NULL,
//             retake2 TEXT NOT NULL,
//             fk_course_id INTEGER NOT NULL,
//             FOREIGN KEY (fk_course_id) REFERENCES courses(id)
//         )
//         "#,
//     )
//     .execute(pool)
//     .await?;

//     Ok(())
// }


// async fn insert_courses(pool: &SqlitePool) -> Result<(), sqlx::Error> {
//     let course1 = Course {
//         id: 1,
//         name: "Course 1".to_string(),
//         code: "CSE101".to_string(),
//         semester: 1,
//         year: 2023,
//         credit: 3.0,
//     };

//     let course2 = Course {
//         id: 2,
//         name: "Course 2".to_string(),
//         code: "CSE102".to_string(),
//         semester: 1,
//         year: 2023,
//         credit: 3.0,
//     };

//     insert_course(pool, &course1).await?;
//     insert_course(pool, &course2).await?;

//     Ok(())
// }
