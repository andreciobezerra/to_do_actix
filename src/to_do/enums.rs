use std::fmt::{Display, Formatter, Result};

pub enum TaskStatus {
    DONE,
    PENDING,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self {
            &Self::DONE => {
                write!(f, "Done")
            }
            &Self::PENDING => {
                write!(f, "Pending")
            }
        }
    }
}

impl TaskStatus {
    pub fn stringfy(&self) -> String {
        match &self {
            &Self::DONE => "DONE".to_string(),
            &Self::PENDING => "PENDING".to_string(),
        }
    }
}
