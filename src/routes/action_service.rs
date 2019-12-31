use mysql::Pool;
use rocket::State;
use super::model;

pub fn insert(conn: State<Pool>, data: model::action) -> Result<bool,String> {
    let query = format!(r"INSERT INTO action (`userId`, `gameId`, `action`)
                                   VALUES ('{}', '{}', :action)", data.userId, data.gameId);
    for mut stmt in conn.prepare(query).into_iter() {
        stmt.execute(params! {
            "action" => &data.action
            }).expect("error while update table action");
    };
    return Ok(true);
}