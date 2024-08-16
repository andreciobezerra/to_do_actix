use crate::to_do::traits::{crud::Crud, task::Task};

use super::base::Base;

#[derive(Clone)]
pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(input_title: &str) -> Self {
        Pending {
            super_struct: Base {
                title: input_title.to_string(),
                status: crate::to_do::enums::TaskStatus::PENDING,
            },
        }
    }
}

impl Crud for Pending {}

impl Task for Pending {
    fn get_title(&self) -> &String {
        &self.super_struct.title
    }

    fn get_status(&self) -> &crate::to_do::enums::TaskStatus {
        &self.super_struct.status
    }
}
