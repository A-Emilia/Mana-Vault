use std::{fs::read_to_string, sync::LazyLock};

use sqlx::{self, mysql::MySqlPoolOptions, prelude::FromRow, MySql, Pool};


static DATABASE_URL: LazyLock<String> = LazyLock::new(|| {
    let env = read_to_string(".env").expect("No .env found!");
    let prefix = r#"DATABASE_URL=""#;
    env.lines()
        .filter(|line| line.starts_with(prefix))
        .flat_map(|line| line.strip_prefix(prefix)?.strip_suffix('"'))
        .nth(0)
        .expect("Database_URL not found in .env file")
        .to_owned()
});

pub async fn establish_db_pool() -> Result<Pool<MySql>, sqlx::Error> {
    MySqlPoolOptions::new()
        .max_connections(69)
        .connect(
            DATABASE_URL.as_str()
        )
        .await
}

 
#[test]
fn db_test() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .enable_io()
        .build()
        .unwrap();
    rt.block_on(async {
        establish_db_pool().await.unwrap();
        let card: Card2 = sqlx::query_as(
            "SELECT c.name, c.setCode as keyrune_code
                        FROM cards as c 
                        WHERE c.name = 'Jodah';"
        )
        .fetch_one(&establish_db_pool().await.unwrap())
        .await
        .unwrap();
        dbg!(card);
    });
    //sqlx::query_as!();
}

#[derive(Default, Debug, Clone, FromRow)]
struct Card2 {
    pub name: Option<String>,
    pub keyrune_code: Option<String>,
}


#[test]
fn test_static(){
    println!("{}", DATABASE_URL.as_str());
}