use actix_web::HttpRequest;
use serde_json::{Map, Value};

use crate::{
    processes::process_input,
    state::read_file,
    to_do::{enums::TaskStatus, to_do_factory},
};

pub async fn create(req: HttpRequest) -> String {
    let mut state: Map<String, Value> = read_file("./state.json");
    let title = req.match_info().get("title").unwrap().to_string();
    let item = to_do_factory(&title.as_str(), TaskStatus::PENDING);

    process_input(item, crate::processes::Command::CREATE, &mut state);

    return format!("{} create", title);
}
