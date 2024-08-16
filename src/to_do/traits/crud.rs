use serde_json::{json, Map, Value};

use crate::{state::write_to_file, to_do::enums::TaskStatus};

pub trait Crud {
    fn get(&self, title: &String, state: &Map<String, Value>) {
        match state.get(title) {
            Some(result) => {
                println!("\n\nItem: {}", title);
                println!("Status: {}\n\n", result);
            }
            None => {
                println!("item: {} was not found", title);
            }
        }
    }

    fn create(&self, title: &str, status: TaskStatus, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(status.stringfy()));
        write_to_file("./state.json", state);

        println!("\n\n{} is being created\n\n", title);
    }

    fn update(&self, title: &str, status: TaskStatus, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(status.stringfy()));
        write_to_file("./state.json", state);

        println!("\n\n{} is being set to done\n\n", title);
    }

    fn delete(&self, title: &String, state: &mut Map<String, Value>) {
        state.remove(title);
        write_to_file("./state.json", state);
        println!("\n\n{} is being deleted\n\n", title);
    }
}
