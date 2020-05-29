use std::fs::read_to_string;
use std::path::Path;
use std::error::Error;

mod error;
use error::ParseErr;
use error::ReadErr;

#[derive(Debug)]
pub struct TodoList{
    tasks: Vec<String>,
}

impl TodoList {
    pub fn get_todos<P> (path: P) -> Result<TodoList, Box<dyn Error>>
    where P: AsRef<Path> {
        let read_todos = read_todos(path);
        let parsed_todos = parse_todos(&read_todos?)?;
        Ok(parsed_todos)
    }
}

pub fn read_todos<P> (path: P) -> Result<String, Box<dyn Error>>
where P: AsRef<Path>{
    let raw_todos = read_to_string(path)
        .map_err (|e| ReadErr {
            child_err: Box::new(e)
        })?;
    Ok(raw_todos)
}

pub fn parse_todos(todos_str: &str) -> Result<TodoList, Box<dyn Error>> {
    let mut tasks: Vec<String> = vec![];
    for i in todos_str.lines(){
        tasks.push(i.to_string());
    }
    if tasks.is_empty() {
        Err(ParseErr::Empty.into())
    } else{
        Ok(TodoList{tasks})
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
