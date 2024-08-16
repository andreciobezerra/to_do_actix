use crate::to_do::enums::TaskStatus;

#[derive(Clone)]
pub struct Base {
    pub title: String,
    pub status: TaskStatus,
}
