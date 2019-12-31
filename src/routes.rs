use rocket_contrib::json::{Json, JsonValue};
use mysql::Pool;
use rocket::State;
mod model;
mod action_service;

#[post("/action", format = "json", data = "<message>")]
pub fn new_action(conn: State<'_, Pool>, message: Json<model::action>) -> JsonValue {
    let result = action_service::insert(conn, message.0);
    match result {
        Ok(res)=>{
            json!({ "status": 202 , "info": "new action was created"})
        },
        Err(msg)=>{
            json!({ "status": 404 , "info": msg})
        }
    }
}