use ::std::collections::HashMap;
use std::io::{stdin, Read};
use std::str::FromStr;

const QUESTION_WHAT_ACTION: &str =
    "What action to you want to perform? (add, complete, list, remove, end)";

fn main() {
    println!("Hello, User!");

    let mut action = String::new();

    println!("{}", QUESTION_WHAT_ACTION);

    stdin()
        .read_line(&mut action)
        .expect("Did not enter a correct string");
    action.pop();

    while action != "end" {
        let mut todo = Todo::new().expect("Initialization of db failed");
        let mut item: String = String::new();

        if action == "add" {
            stdin().read_line(&mut item).expect("Bad input");
            item.pop();
            todo.insert(item);
            match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error ocurred: {}", why),
            }
        } else if action == "complete" {
            stdin().read_line(&mut item).expect("Bad input");
            item.pop();

            match todo.complete(&item) {
                None => println!("'{}' is not present in the list", item),
                Some(_) => match todo.save() {
                    Ok(_) => println!("todo saved"),
                    Err(why) => println!("An error occurred: {}", why),
                },
            }
        } else if action == "remove" {
            stdin().read_line(&mut item).expect("Bad input");
            item.pop();
            match todo.remove(&item) {
                None => println!("Todo does not exist: {}", item),
                Some(v) => match todo.save() {
                    Ok(_) => println!("todo removed: {} {}", item, v),
                    Err(why) => println!("An error occurred: {}", why),
                },
            }
        } else if action == "list" {
            todo.print_all();
        }
        action = String::from("");
        println!("{}", QUESTION_WHAT_ACTION);
        stdin()
            .read_line(&mut action)
            .expect("Did not enter correct String!");
        action.pop();
    }
    println!("Good bye!");
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;

        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let map: HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect();
        Ok(Todo { map })
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, false);
    }

    fn print_all(self) {
        let mut completed_todos: String = String::new();
        let mut not_completed_todos: String = String::new();

        for (key, value) in self.map {
            if value {
                completed_todos = completed_todos + &key + ", ";
            } else {
                not_completed_todos = not_completed_todos + &key + ", ";
            }
        }
        println!("Not completed Todos!: {}", not_completed_todos);
        println!("Completed Todos!: {}", completed_todos);
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }

    fn remove(&mut self, key: &String) -> Option<bool> {
        match self.map.remove(key) {
            Some(v) => Some(v),
            None => None,
        }
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = true),
            None => None,
        }
    }
}
