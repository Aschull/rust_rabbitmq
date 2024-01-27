use std::env;

use diesel::{Connection, PgConnection};
use dotenv::dotenv;

pub struct ConfigDB {
    pub database_url: String,
    pub conn: PgConnection,
}

impl ConfigDB {
    pub fn new() -> ConfigDB {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", &database_url));

        return ConfigDB { database_url, conn };
    }
}
