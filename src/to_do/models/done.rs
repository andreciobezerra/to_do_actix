use crate::to_do::traits::{crud::Crud, task::Task};

use super::base::Base;

#[derive(Clone)]
pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: &str) -> Self {
        Done {
            super_struct: Base {
                title: input_title.to_string(),
                status: crate::to_do::enums::TaskStatus::DONE,
            },
        }
    }
}

impl Crud for Done {}

impl Task for Done {
    fn get_title(&self) -> &String {
        &self.super_struct.title
    }

    fn get_status(&self) -> &crate::to_do::enums::TaskStatus {
        &self.super_struct.status
    }
}
