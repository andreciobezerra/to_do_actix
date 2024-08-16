use crate::to_do::traits::crud::Crud;

use super::base::Base;

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
