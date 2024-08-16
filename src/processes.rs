use serde_json::{Map, Value};

use crate::to_do::{
    enums::TaskStatus,
    traits::{crud::Crud, task::Task},
    ItemTypes,
};

pub enum Command {
    CREATE,
    GET,
    UPDATE,
    DELETE,
}

pub fn process_input(item_type: ItemTypes, command: Command, state: &Map<String, Value>) {
    let (item, status): (Box<dyn Crud>, TaskStatus) = match item_type.clone() {
        ItemTypes::Pending(item) => (Box::new(item), TaskStatus::PENDING),
        ItemTypes::Done(item) => (Box::new(item), TaskStatus::DONE),
    };

    let task: Box<dyn Task> = match item_type {
        ItemTypes::Pending(item) => Box::new(item),
        ItemTypes::Done(item) => Box::new(item),
    };

    let mut state = state.clone();

    match command {
        Command::CREATE => item.create(task.get_title(), status, &mut state),
        Command::GET => item.get(task.get_title(), &state),
        Command::UPDATE => item.update(task.get_title(), status, &mut state),
        Command::DELETE => item.update(task.get_title(), status, &mut state),
    }
}
