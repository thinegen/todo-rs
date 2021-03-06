use std::env;
use std::io::ErrorKind;
use std::io::Write;
use tabwriter::TabWriter;

mod helper;
mod structs;
mod todoio;
use self::helper::*;
use self::structs::*;
use self::todoio::*;

fn add_new_todo(args: Vec<String>, id_file_path: &str, todo_file_path: &str) {
    if args.len() < 3 {
        print_help();
        return;
    }

    let current_id = match get_current_id(id_file_path) {
        Ok(v) => v,
        Err(err) => {
            println!("Error getting new ID: {}", err);
            return;
        }
    };

    let mut new_todo = Todo::new_with_id(current_id);
    if args.len() > 3 && new_todo.set_priority_from_string(&args[2]).is_ok() {
        new_todo.set_description(&args[3..].join(" "));
    } else {
        new_todo.set_description(&args[2..].join(" "));
    }

    if let Err(err) = write_to_file(&new_todo.to_file(), todo_file_path) {
        println!("Failed writing Todo to file: {}", err);
    } else {
        let mut tw = TabWriter::new(std::io::stdout()).padding(2);
        tw.write_all("New Todo:\n".as_bytes()).unwrap();
        tw.write_all(print_todo_header().as_bytes()).unwrap();
        tw.write_all(format!("{}\n", new_todo).as_bytes()).unwrap();
        tw.flush().unwrap();
    }
}

fn list_all_todos(args: Vec<String>, show_done: bool, todo_file_path: &str) {
    let mut todos = match get_all_todos(todo_file_path) {
        Ok(v) => v,
        Err(err) => {
            println!("Reading Todos failed: {}", err);
            return;
        }
    };
    todos.sort();

    let mut search_string = String::from("");
    if args.len() > 2 {
        search_string = args[2..].join(" ");
    }

    let mut tw = TabWriter::new(std::io::stdout()).padding(2);
    tw.write_all(print_todo_header().as_bytes()).unwrap();
    for todo in todos.iter() {
        if (!todo.done() || todo.done() == show_done) && todo.filter(&search_string) {
            tw.write_all(format!("{}\n", todo).as_bytes()).unwrap();
        }
    }
    tw.flush().unwrap();
}

fn set_todo(args: Vec<String>, todo_file_path: &str) {
    let edit_id = match args[3].parse::<usize>() {
        Ok(v) => v,
        Err(err) => {
            println!("Error: {}", err);
            print_help();
            return;
        }
    };

    let mut todos = match get_all_todos(todo_file_path) {
        Ok(v) => v,
        Err(err) => {
            println!("Reading Todos failed: {}", err);
            return;
        }
    };

    let mut tw = TabWriter::new(std::io::stdout()).padding(2);
    tw.write_all(print_todo_header().as_bytes()).unwrap();

    if let Err(err) = truncate_file(todo_file_path) {
        println!("Writing Todo file failed: {}", err);
        return;
    }
    for todo in todos.iter_mut() {
        if todo.get_id() == edit_id {
            match &args[2][..] {
                "prio" => {
                    if let Err(err) = todo.set_priority_from_string(&args[4]) {
                        println!("Error setting priority: {}", err)
                    }
                }
                "desc" => todo.set_description(&args[4]),
                "proj" => todo.set_projects(&args[4]),
                "cat" => todo.set_categories(&args[4]),
                "est" => {
                    if let Err(err) = todo.set_time_estimated_from_string(&args[4]) {
                        println!("Error setting time estimated: {}", err)
                    }
                }
                "act" => {
                    if let Err(err) = todo.set_time_actual_from_string(&args[4]) {
                        println!("Error setting time actual: {}", err)
                    }
                }
                "stat" => {
                    if let Err(err) = todo.set_status_from_string(&args[4]) {
                        println!("Error setting status: {}", err)
                    }
                }
                "color" => todo.set_color_from_string(&args[4]),
                &_ => {
                    println!("No such attribute: {}", args[3]);
                }
            };
            tw.write_all(format!("{}\n", todo).as_bytes()).unwrap();
        }
        if let Err(err) = write_to_file(&todo.to_file(), todo_file_path) {
            println!("Failed: {}", err);
        }
    }
    tw.flush().unwrap();
}

fn rm_todo(args: Vec<String>, todo_file_path: &str, id_file_path: &str) {
    if args[2] == "all" {
        if let Err(err) = truncate_file(todo_file_path) {
            println!("Writing Todo file failed: {}", err);
            return;
        }
        let _ = set_current_id(0, id_file_path);
        return;
    }
    let edit_id = match args[2].parse::<usize>() {
        Ok(v) => v,
        Err(err) => {
            println!("Error: {}", err);
            print_help();
            return;
        }
    };

    let mut todos = match get_all_todos(todo_file_path) {
        Ok(v) => v,
        Err(err) => {
            println!("Reading Todos failed: {}", err);
            return;
        }
    };

    let mut tw = TabWriter::new(std::io::stdout()).padding(2);
    tw.write_all(print_todo_header().as_bytes()).unwrap();

    if let Err(err) = truncate_file(todo_file_path) {
        println!("Writing Todo file failed: {}", err);
        return;
    }
    for todo in todos.iter_mut() {
        if todo.get_id() != edit_id {
            if let Err(err) = write_to_file(&todo.to_file(), todo_file_path) {
                println!("Failed: {}", err);
            }
        } else {
            todo.set_deleted();
            tw.write_all(format!("{}\n", todo).as_bytes()).unwrap();
        }
    }
    tw.flush().unwrap();
}

fn clean(todo_file_path: &str, id_file: &str) {
    let mut todos = match get_all_todos(todo_file_path) {
        Ok(v) => v,
        Err(err) => {
            println!("Reading Todos failed: {}", err);
            return;
        }
    };

    let mut tw = TabWriter::new(std::io::stdout()).padding(2);
    tw.write_all(print_todo_header().as_bytes()).unwrap();

    if let Err(err) = truncate_file(todo_file_path) {
        println!("Writing Todo file failed: {}", err);
        return;
    }

    todos.sort();
    let mut new_id: usize = 0;
    for todo in todos.iter_mut() {
        todo.set_id(new_id);
        new_id += 1;

        if let Err(err) = write_to_file(&todo.to_file(), todo_file_path) {
            println!("Failed: {}", err);
        }
        tw.write_all(format!("{}\n", todo).as_bytes()).unwrap();
    }
    tw.flush().unwrap();

    if let Err(err) = set_current_id(new_id, id_file) {
        println!("Error resetting ID: {}", err);
    }
}

fn do_task(args: Vec<String>, todo_file_path: &str) {
    let edit_id = match args[2].parse::<usize>() {
        Ok(v) => v,
        Err(err) => {
            println!("Error: {}", err);
            print_help();
            return;
        }
    };

    let mut todos = match get_all_todos(todo_file_path) {
        Ok(v) => v,
        Err(err) => {
            println!("Reading Todos failed: {}", err);
            return;
        }
    };

    let mut tw = TabWriter::new(std::io::stdout()).padding(2);
    tw.write_all(print_todo_header().as_bytes()).unwrap();

    if let Err(err) = truncate_file(todo_file_path) {
        println!("Writing Todo file failed: {}", err);
        return;
    }
    for todo in todos.iter_mut() {
        if todo.get_id() == edit_id {
            todo.set_status(TodoStatus::Done).unwrap();
            tw.write_all(format!("{}\n", todo).as_bytes()).unwrap();
        }
        if let Err(err) = write_to_file(&todo.to_file(), todo_file_path) {
            println!("Failed: {}", err);
        }
    }
    tw.flush().unwrap();
}

fn print_help() {
    println!(
        r#"usage:
t new [Prio] <description>
t set (prio|desc|proj|cat|est|act|stat|color) <id> <value>
t do  <id>
t rm  <id>|all
t ls  [searchterm]
t lsa [searchterm]
t clean # resets the ids

Possible status:
Open
Backlog
Next
Planned
Doing
Review
Done

Colors:
Black
Red
Green
Yellow
Blue
Purple
Cyan
White
"#
    );
}

fn main() {
    let user_home_dir = match home::home_dir() {
        Some(v) => v,
        None => {
            println!("Error. No home dir found. Exiting");
            return;
        }
    };
    let user_home_dir = match user_home_dir.to_str() {
        Some(v) => v,
        None => {
            println!("Error. No home dir found. Exiting");
            return;
        }
    };

    let todo_dir = ".todo";

    if let Err(err) = std::fs::create_dir([user_home_dir, todo_dir].join("/")) {
        if err.kind() != ErrorKind::AlreadyExists {
            println!("Error. Todo directory could not be created: {}", err);
            return;
        }
    }

    let todo_file_path = [user_home_dir, todo_dir, "todo.txt"].join("/");
    let id_file_path = [user_home_dir, todo_dir, "id.txt"].join("/");

    let args: Vec<String> = env::args().collect();

    let first_arg: String;
    if args.len() > 1 {
        first_arg = args[1].clone();
    } else {
        first_arg = String::from("");
    }

    match &first_arg[..] {
        "ls" => list_all_todos(args, false, &todo_file_path),
        "lsa" => list_all_todos(args, true, &todo_file_path),
        "new" => add_new_todo(args, &id_file_path, &todo_file_path),
        "set" => set_todo(args, &todo_file_path),
        "rm" => rm_todo(args, &todo_file_path, &id_file_path),
        "do" => do_task(args, &todo_file_path),
        "clean" => clean(&todo_file_path, &id_file_path),
        _ => print_help(),
    }
}
