use async_graphql::{Object, Schema};
use std::sync::Arc;
use tokio::sync::Mutex;

type DynError = Result<(), Box<dyn std::error::Error>>;

struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Returns the sum of a and b
    async fn add(&self, #[graphql(desc = "Returns the sum of a and b")] a: i32, b: i32) -> i32 {
        a + b
    }
}

struct ProfileClicksAndViewsPeriod {
    profile_views: u32,
    website_clicks: u32,
}

type Data = Arc<Mutex<Vec<ProfileClicksAndViewsPeriod>>>;

#[tokio::main]
async fn main() -> DynError {
    let profile_data = vec![ProfileClicksAndViewsPeriod {
        profile_views: 12389,
        website_clicks: 322193,
    }];
    let data = Arc::new(Mutex::new(profile_data));
    let schema = Schema::build(
        QueryRoot,
        async_graphql::EmptyMutation,
        async_graphql::EmptySubscription,
    )
    .data(data)
    .finish();
    let res = schema.execute("{add(a: 10, b: 20)}").await;
    dbg!(&res);
    Ok(())
}
