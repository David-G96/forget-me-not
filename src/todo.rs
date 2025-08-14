use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime, naive};

#[derive(Debug)]
pub struct Todo {
    pub set_date: NaiveDateTime,
    pub todo_date: Option<NaiveDateTime>,
    pub tag: String,
    pub title: String,
    pub content: String,
}

impl Todo {
    pub fn new_now(title: String, content: String) -> Todo {
        let date = chrono::Local::now().date_naive();
        let time = chrono::Local::now().time();

        let date_time = NaiveDateTime::new(date, time);

        Todo {
            set_date: date_time,
            todo_date: None,
            tag: "".to_string(),
            title,
            content,
        }
    }
}
