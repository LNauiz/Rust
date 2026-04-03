use serde_json::to_string;
use clap::{Parser, Subcommand};
use serde::Serialize;
use serde::Deserialize;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
#[derive(Serialize, Deserialize)]
struct TodoItem{
    id:u32,
    title:String,
    done:bool,
}

#[derive(Serialize, Deserialize)]
struct TodoList{
    list:Vec<TodoItem>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { list: Vec::new() }
    }


    fn add(&mut self,title: String) {
        if self.list.len() == 0 {
            self.list.push(TodoItem { id: 0, title: title, done: false });
        } else {
            let idx = self.list.len() as u32;
            self.list.push(TodoItem { id:idx, title: title, done: false });
        }
    }

    fn list(&self)-> &[TodoItem] {
        &self.list
    }

    fn mark_done(&mut self, id:u32)-> Result<(), String> {
        for item in self.list.iter_mut() {
            if item.id == id {
                item.done=true;
                return Ok(())
            }
        }
        Err(format!("Process {} not found",id))
    }
    fn remove(&mut self, id:u32)-> Result<TodoItem, String> {
        for (i,item)in self.list.iter_mut().enumerate() {
            if item.id == id {
                return Ok(self.list.remove(i))
            }
        }
        Err(format!("Process {} not found",id))
    }

    fn display(&self){
        for item in self.list.iter() {
            if item.done {
                println!("[x] {} - {}", item.id ,item.title);
            }
            else{
            println!("[] {} - {}",item.id,item.title);
            }
        }

    }

    pub fn save_to_file(&self, path: &str) -> io::Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, &self)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(())
        }
        pub fn load_from_file(path: &str) -> io::Result<Self> {
            match File::open(path) {
                Ok(file) => {
                    let reader = BufReader::new(file);

                    let todo_list = serde_json::from_reader(reader)
                        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                    Ok(todo_list)
                }
                Err(_) => {
                    Ok(Self::new())
                }
            }
        }
    }
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {
    Add { title: String },
    List,
    Done { id: u32 },
    Remove { id: u32 },
}

fn main() {
    println!("Hello, world2!");
    let mut todo_list = TodoList::new();
    todo_list.add("Steve".to_string());
    todo_list.add("Doni".to_string());
    todo_list.add("Agathe".to_string());
    todo_list.display();
    todo_list.mark_done(0).unwrap();
    todo_list.mark_done(2).unwrap();
    todo_list.display();
    todo_list.remove(0).unwrap();
    todo_list.display();
}