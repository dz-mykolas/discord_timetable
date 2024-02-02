CREATE TABLE courses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    semester INTEGER NOT NULL,
    year INTEGER NOT NULL,
    credit REAL NOT NULL
);

CREATE TABLE assessments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    weight REAL NOT NULL,
    take1 TEXT NOT NULL,
    retake1 TEXT NOT NULL,
    retake2 TEXT NOT NULL,
    fk_course_id INTEGER NOT NULL,
    FOREIGN KEY (fk_course_id) REFERENCES courses(id)
);

