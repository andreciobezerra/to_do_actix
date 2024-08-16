use crate::to_do::traits::crud::Crud;

use super::base::Base;

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
