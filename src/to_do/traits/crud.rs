use crate::to_do::enums::TaskStatus;

pub trait Crud {
    fn get(&self, title: &str) {
        println!("{} is being fetched", title);
    }

    fn create(&self, title: &str) {
        println!("{} is being created", title);
    }

    fn update(&self, title: &str, status: TaskStatus) {
        println!("{} is beiong set to {}", title, status);
    }

    fn delete(&self, title: &str) {
        println!("{} is being deleted", title);
    }
}
