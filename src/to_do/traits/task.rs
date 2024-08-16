use crate::to_do::enums::TaskStatus;

pub trait Task {
    fn get_title(&self) -> &String;
    fn get_status(&self) -> &TaskStatus;
}
