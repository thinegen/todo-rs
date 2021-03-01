use ansi_term::Color;
use std::cmp::Ordering;
use std::fmt;
use std::time::Duration;

pub mod parse;
use parse::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TodoStatus {
    Done,
    Backlog,
    Next,
    Planned,
    Doing,
    Review,
    New,
    Deleted,
}

impl fmt::Display for TodoStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TodoStatus::New => write!(f, "New"),
            TodoStatus::Backlog => write!(f, "Backlog"),
            TodoStatus::Next => write!(f, "Next"),
            TodoStatus::Planned => write!(f, "Planned"),
            TodoStatus::Doing => write!(f, "Doing"),
            TodoStatus::Review => write!(f, "Review"),
            TodoStatus::Done => write!(f, "Done"),
            TodoStatus::Deleted => write!(f, "Deleted"),
        }
    }
}

#[derive(Debug)]
pub struct ParseTodoError {
    msg: String,
}
impl fmt::Display for ParseTodoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parsing error: {}", self.msg)
    }
}
impl ParseTodoError {
    fn new(s: &str) -> ParseTodoError {
        ParseTodoError { msg: s.to_string() }
    }
}

#[derive(Debug)]
pub struct TodoIOError {
    msg: String,
}
impl fmt::Display for TodoIOError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Todo IO Error: {}", self.msg)
    }
}
impl TodoIOError {
    pub fn new(s: &str) -> TodoIOError {
        TodoIOError { msg: s.to_string() }
    }
}

#[derive(Debug)]
pub struct Todo {
    id: usize,
    priority: isize,
    description: String,
    projects: String,
    categories: String,
    time_estimated: Option<Duration>,
    time_actual: Option<Duration>,
    status: TodoStatus,
    color: Color,
}

impl Default for Todo {
    fn default() -> Todo {
        Todo {
            id: 0,
            priority: 0,
            description: "".to_string(),
            projects: "".to_string(),
            categories: "".to_string(),
            time_estimated: None,
            time_actual: None,
            status: TodoStatus::New,
            color: Color::White,
        }
    }
}

impl Todo {
    #[allow(dead_code)]
    pub fn new() -> Todo {
        Todo::default()
    }
    pub fn new_with_id(id: usize) -> Todo {
        let mut new_todo = Todo::default();
        new_todo.set_id(id);
        new_todo
    }
    pub fn to_file(&self) -> String {
        let time_estimated = match self.time_estimated {
            Some(v) => v.as_secs(),
            None => 0,
        };
        let time_actual = match self.time_actual {
            Some(v) => v.as_secs(),
            None => 0,
        };

        format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            self.id,
            self.priority,
            self.description,
            self.projects,
            self.categories,
            time_estimated,
            time_actual,
            self.status,
            color_to_string(self.color),
        )
    }
    pub fn filter(&self, needle: &str) -> bool {
        if let Ok(v) = needle.parse::<TodoStatus>() {
            if self.status == v {
                return true;
            }
        }

        let needle = needle.to_uppercase();

        if self.description.to_uppercase().contains(&needle)
            || self.projects.to_uppercase().contains(&needle)
            || self.categories.to_uppercase().contains(&needle)
        {
            return true;
        }
        false
    }
    pub fn set_deleted(&mut self) {
        self.status = TodoStatus::Deleted;
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let time_estimated = self.time_estimated.unwrap_or_default();
        let time_actual = self.time_actual.unwrap_or_default();

        let mut time_prop: f64 = 0.0;
        let time_delta = match (time_estimated.as_secs(), time_actual.as_secs()) {
            (0, _) | (_, 0) => Duration::from_secs(0),
            (_, _) => {
                time_prop = time_actual.as_secs() as f64 / time_estimated.as_secs() as f64;
                if time_estimated < time_actual {
                    time_actual - time_estimated
                } else {
                    time_estimated - time_actual
                }
            }
        };

        let time_estimated = duration_to_human_string(time_estimated);
        let time_actual = duration_to_human_string(time_actual);
        let time_delta = duration_to_human_string(time_delta);

        let time_prop_str = if time_prop == 0.0 {
            String::from("")
        } else {
            let mut x = "/".to_string();
            x.push_str(&format!("{:.2}", time_prop));
            x
        };

        let print_color = if self.status == TodoStatus::Done {
            Color::White
        } else {
            self.color
        };

        write!(
            f,
            "{}",
            print_color.paint(format!(
                "{}\t{:02}\t{}\t{}\t{}\t{}\t{}\t{}\t{}{}",
                self.id,
                self.status,
                self.priority,
                self.description,
                self.projects,
                self.categories,
                time_estimated,
                time_actual,
                time_delta,
                time_prop_str,
            ))
        )
    }
}

impl Ord for Todo {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.id == other.id {
            return Ordering::Equal;
        }

        if self.status == TodoStatus::Done && other.status != TodoStatus::Done {
            return Ordering::Greater;
        } else if self.status != TodoStatus::Done && other.status == TodoStatus::Done {
            return Ordering::Less;
        }

        if self.priority == other.priority {
            other.status.cmp(&self.status)
        } else {
            other.priority.cmp(&self.priority)
        }
    }
}

impl PartialOrd for Todo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Todo {}
impl PartialEq for Todo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn duration_to_human_string(d: Duration) -> String {
    let s = d.as_secs();
    let seconds = s % 60;
    let minutes = (s / 60) % 60;
    let hours = (s / 60) / 60;

    let mut ret = String::from("");

    if hours != 0 {
        ret.push_str(&format!("{:02}h", hours));
    }
    if minutes != 0 {
        ret.push_str(&format!("{:02}m", minutes));
    }
    if seconds != 0 {
        ret.push_str(&format!("{:02}s", seconds));
    }
    ret
}

impl Todo {
    pub fn get_id(&self) -> usize {
        self.id
    }
    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }
    #[allow(dead_code)]
    pub fn set_id_from_string(&mut self, id: &str) -> Result<usize, ParseTodoError> {
        self.id = parse_usize(id)?;
        Ok(self.get_id())
    }

    pub fn get_priority(&self) -> isize {
        self.priority
    }
    #[allow(dead_code)]
    pub fn set_priority(&mut self, priority: isize) {
        self.priority = priority;
    }
    pub fn set_priority_from_string(&mut self, priority: &str) -> Result<isize, ParseTodoError> {
        self.priority = parse_isize(priority)?;
        Ok(self.get_priority())
    }

    #[allow(dead_code)]
    pub fn get_description(&mut self) -> String {
        self.description.clone()
    }
    pub fn set_description(&mut self, description: &str) {
        self.description = parse_string(description);
    }

    #[allow(dead_code)]
    pub fn get_projects(&self) -> String {
        self.projects.clone()
    }
    pub fn set_projects(&mut self, projects: &str) {
        self.projects = parse_string(projects);
    }

    #[allow(dead_code)]
    pub fn get_categories(&self) -> String {
        self.categories.clone()
    }
    pub fn set_categories(&mut self, categories: &str) {
        self.categories = parse_string(categories);
    }

    pub fn get_time_estimated(&self) -> Option<Duration> {
        self.time_estimated
    }
    #[allow(dead_code)]
    pub fn set_time_estimated(&mut self, time_estimated: Option<Duration>) {
        self.time_estimated = time_estimated;
    }
    pub fn set_time_estimated_from_string(
        &mut self,
        time_estimated: &str,
    ) -> Result<Option<Duration>, ParseTodoError> {
        self.time_estimated = parse_duration_result(time_estimated)?;
        Ok(self.get_time_estimated())
    }

    #[allow(dead_code)]
    pub fn get_time_actual(&self) -> Option<Duration> {
        self.time_actual
    }
    #[allow(dead_code)]
    pub fn set_time_actual(&mut self, time_actual: Option<Duration>) {
        self.time_actual = time_actual;
    }
    pub fn set_time_actual_from_string(
        &mut self,
        time_actual: &str,
    ) -> Result<Option<Duration>, ParseTodoError> {
        self.time_actual = parse_duration_result(time_actual)?;
        Ok(self.get_time_estimated())
    }

    #[allow(dead_code)]
    pub fn get_status(&self) -> TodoStatus {
        self.status
    }
    pub fn done(&self) -> bool {
        self.status == TodoStatus::Done
    }
    #[allow(dead_code)]
    pub fn set_status(&mut self, status: TodoStatus) -> Result<(), ParseTodoError> {
        if status == TodoStatus::Deleted {
            return Err(ParseTodoError::new("Not allowed to set Deleted"));
        }
        self.status = status;

        Ok(())
    }
    #[allow(dead_code)]
    pub fn set_status_from_string(&mut self, status: &str) -> Result<TodoStatus, ParseTodoError> {
        self.status = status.parse()?;

        Ok(self.get_status())
    }

    #[allow(dead_code)]
    pub fn get_color(&mut self) -> Color {
        self.color
    }
    #[allow(dead_code)]
    pub fn set_color(&mut self, c: Color) {
        self.color = c;
    }
    #[allow(dead_code)]
    pub fn set_color_from_string(&mut self, s: &str) {
        self.color = string_to_color_or_white(s);
    }
}
