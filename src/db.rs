use mysql;
use mysql::Pool;
use dotenv::dotenv;
use std::env;

pub fn new_connection() -> Pool {
    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").expect("can't find DATABASE_URL env variable");
    return mysql::Pool::new(db_url)
        .expect("error while connecting to db");
}

pub fn create_table_action(conn: &mut Pool) {
    conn.prep_exec(r"CREATE TABLE IF NOT EXISTS action (
            id INT(6) UNSIGNED AUTO_INCREMENT PRIMARY KEY,
            userId INT(32) NOT NULL,
            gameId INT(32) NOT NULL,
            action VARCHAR(50) NOT NULL
        )", ()).expect("error while creating table action");
}