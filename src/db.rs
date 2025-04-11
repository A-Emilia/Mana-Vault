use std::env;

use sqlx::{self, mysql::MySqlPoolOptions, MySql, Pool, Row};

use crate::model::Card;

pub async fn establish_db_pool() ->  Result<Pool<MySql>, sqlx::Error>{
    MySqlPoolOptions::new()
        .max_connections(69)
        .connect(env::var("DATABASE_URL").unwrap().as_str()).await
    }


#[test]
fn db_test(){
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().enable_io().build().unwrap();
    rt.block_on(async {
        establish_db_pool().await.unwrap();
        let card: Card2 =sqlx::query_as!(
                        Card2,
                        "SELECT c.name, c.setCode as keyruneCode
                        FROM cards as c 
                        WHERE c.name = 'Jodah';")
                        .fetch_one(&establish_db_pool().await.unwrap()).await.unwrap();
        dbg!(card);
    });
    //sqlx::query_as!();
}

#[derive(Default, Debug, Clone)]
struct Card2 {
    pub name: Option<String>,
    pub keyruneCode: Option<String>
}