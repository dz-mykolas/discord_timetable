# Timetable Bot
Discord bot written in pure rust that allows displaying/managing of university timetables.

# Important
Removed courses.db file in latest commit. So if needed create your own for now:
## Tables:
### Courses:
```sql
CREATE TABLE courses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    semester INTEGER NOT NULL,
    year INTEGER NOT NULL,
    credit REAL NOT NULL
);
```
### Assessments
```sql
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
```

# Quickstart
To be updated...

# Task list
### Essential
- [ ] Courses
  - [x] View
  - [x] Pages listing
  - [ ] Manage
  - [x] Authorization
- [ ] Assessments
  - [x] View
  - [x] Pages listing
  - [ ] Manage
  - [x] Authorization
  - [x] Color due dates accordingly
  - [ ] Auto-update in set channels

### Might consider
- [ ] Scripts
  - [ ] Setup database
  - [ ] Setup permissions

# Other notes
### a Guide to ANSI on Discord:
https://gist.github.com/kkrypt0nn/a02506f3712ff2d1c8ca7c9e0aed7c06
