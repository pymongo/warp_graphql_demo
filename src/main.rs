/*!
QA: 为什么sqlx使用runtime-tokio就会panicked at 'can call blocking only when running on the multi-threaded runtime'?

原因1: actix_web::main用的是单线程的执行器Executor

原因2: actix-web-v3.0和sqlx的tokio版本不兼容

解决方案1: 换async-std::main或sqlx换成runtime-async-std

解决方案2: 不要在main函数作用域内初始化sqlx，初始化完sqlx之后用OnceCell包着
*/
use async_graphql::{Schema, Object};
struct Query;

/*
#[derive(Clone)]
struct Db { pool: sqlx::SqlitePool}

impl Db {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            pool: sqlx::SqlitePool::connect("sqlite://db.sqlite").await?
        })
    }

    async fn first(&self) -> u32 {
        todo!("sqlx::query")
    }
}
*/

#[Object]
impl Query {
    #[field(desc="Returns sum of a and b")]
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = sqlx::SqlitePool::connect("sqlite://db.sqlite").await?;
    // let db = Db::new().await?;
    // let schema = Schema::build(Query, async_graphql::EmptyMutation, async_graphql::EmptySubscription).data(db).finish();
    // let res = schema.execute("{add(a: 10, b: 20)}").await;
    // dbg!(&res);
    // let res_json_str = serde_json::to_string_pretty(&res).unwrap();
    // println!("{}", res_json_str);
    Ok(())
}