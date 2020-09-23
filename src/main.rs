use async_graphql::{Schema, Object};
struct Query;

#[Object]
impl Query {
    #[field(desc="Returns sum of a and b")]
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::new(Query, async_graphql::EmptyMutation, async_graphql::EmptySubscription);
    let res = schema.execute("{add(a: 10, b: 20)}").await;
    dbg!(&res);
    let res_json_str = serde_json::to_string_pretty(&res).unwrap();
    println!("{}", res_json_str);
    Ok(())
}