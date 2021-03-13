use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};

use super::structs::*;

fn open_file(open_option_string: &str, path: &str) -> Result<File, std::io::Error> {
    let mut oo = OpenOptions::new();

    if open_option_string.contains('r') {
        oo.read(true);
    }
    if open_option_string.contains('w') {
        oo.write(true);
    }
    if open_option_string.contains('a') {
        oo.append(true);
    }
    if open_option_string.contains('c') {
        oo.create(true);
    }
    if open_option_string.contains('t') {
        oo.truncate(true);
    }

    oo.open(path)
}

pub fn write_id_to_id_file(id: usize, mut id_file: File) -> Result<(), std::io::Error> {
    id_file.seek(SeekFrom::Start(0))?;
    id_file.write_all(id.to_string().as_bytes())?;

    Ok(())
}

pub fn get_current_id(id_file_path: &str) -> Result<usize, std::io::Error> {
    let mut id_file = open_file("rwc", id_file_path)?;

    let mut buffer = String::new();
    id_file.read_to_string(&mut buffer).unwrap();

    let buffer = buffer.trim();

    let current_id = buffer.parse::<usize>().unwrap_or(0);

    write_id_to_id_file(current_id + 1, id_file)?;

    Ok(current_id)
}

pub fn set_current_id(new_id: usize, id_file_path: &str) -> Result<(), std::io::Error> {
    let id_file = open_file("wct", id_file_path)?;

    write_id_to_id_file(new_id, id_file)?;

    Ok(())
}

pub fn get_all_todos(todo_path: &str) -> Result<Vec<Todo>, TodoIOError> {
    let mut todos = Vec::new();

    let todo_file = match open_file("r", todo_path) {
        Ok(v) => v,
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            _ => {
                return Err(TodoIOError::new(&format!(
                    "Error opening todo file: {}",
                    err
                )))
            }
        },
    };
    let reader = BufReader::new(todo_file);
    for line in reader.lines() {
        let todo: Todo = match line.unwrap().parse() {
            Ok(v) => v,
            Err(err) => return Err(TodoIOError::new(&format!("Error Parsing Todo: {}", err))),
        };
        todos.push(todo);
    }
    Ok(todos)
}

pub fn write_to_file(s: &str, path: &str) -> Result<(), std::io::Error> {
    let mut file = open_file("ac", path)?;

    file.write_all(s.as_bytes())?;

    Ok(())
}

pub fn truncate_file(path: &str) -> Result<(), std::io::Error> {
    open_file("wt", path)?;
    Ok(())
}
