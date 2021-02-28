use ansi_term::Color;
use std::str::FromStr;
use std::time::Duration;

use super::*;

pub fn parse_usize(s: &str) -> Result<usize, ParseTodoError> {
    let r = s.parse::<usize>();

    match r {
        Ok(v) => Ok(v),
        Err(err) => Err(ParseTodoError::new(&format!(
            "Error parsing {} to usize: {}",
            s, err
        ))),
    }
}
pub fn parse_u64(s: &str) -> Result<u64, ParseTodoError> {
    let r = s.parse::<u64>();

    match r {
        Ok(v) => Ok(v),
        Err(err) => Err(ParseTodoError::new(&format!(
            "Error parsing {} to u64: {}",
            s, err
        ))),
    }
}
pub fn parse_isize(s: &str) -> Result<isize, ParseTodoError> {
    let r = s.parse::<isize>();

    match r {
        Ok(v) => Ok(v),
        Err(err) => Err(ParseTodoError::new(&format!(
            "Error parsing {} to isize: {}",
            s, err
        ))),
    }
}

pub fn parse_string(s: &str) -> String {
    s.to_string().replace("\t", "    ")
}

pub fn parse_duration_result(s: &str) -> Result<Option<Duration>, ParseTodoError> {
    let u = parse_u64(s)?;
    Ok(Some(Duration::from_secs(u)))
}

impl FromStr for Todo {
    type Err = ParseTodoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split('\t').collect();
        if split.len() != 9 {
            return Err(ParseTodoError::new("Not seven fields long"));
        }

        let new_todo = Todo {
            id: parse_usize(split[0])?,
            priority: parse_isize(split[1])?,
            description: parse_string(split[2]),
            projects: parse_string(split[3]),
            categories: parse_string(split[4]),
            time_estimated: parse_duration_result(split[5])?,
            time_actual: parse_duration_result(split[6])?,
            status: split[7].parse()?,
            color: string_to_color_or_white(split[8]),
        };
        Ok(new_todo)
    }
}

impl FromStr for TodoStatus {
    type Err = ParseTodoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "New" => Ok(TodoStatus::New),
            "Backlog" => Ok(TodoStatus::Backlog),
            "Next" => Ok(TodoStatus::Next),
            "Planned" => Ok(TodoStatus::Planned),
            "Doing" => Ok(TodoStatus::Doing),
            "Review" => Ok(TodoStatus::Review),
            "Done" => Ok(TodoStatus::Done),
            "Deleted" => Ok(TodoStatus::Deleted),
            _ => Err(ParseTodoError::new("Todostatus did not match")),
        }
    }
}

pub fn color_to_string(c: Color) -> String {
    match c {
        Color::Black => String::from("Black"),
        Color::Red => String::from("Red"),
        Color::Green => String::from("Green"),
        Color::Yellow => String::from("Yellow"),
        Color::Blue => String::from("Blue"),
        Color::Purple => String::from("Purple"),
        Color::Cyan => String::from("Cyan"),
        Color::White => String::from("White"),
        _ => String::from("White"),
    }
}

pub fn string_to_color_or_white(s: &str) -> Color {
    match s {
        "Black" => Color::Black,
        "Red" => Color::Red,
        "Green" => Color::Green,
        "Yellow" => Color::Yellow,
        "Blue" => Color::Blue,
        "Purple" => Color::Purple,
        "Cyan" => Color::Cyan,
        "White" => Color::White,
        _ => Color::White,
    }
}
