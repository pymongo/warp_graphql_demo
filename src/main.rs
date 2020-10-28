use async_graphql::{Object, Schema, EmptySubscription, EmptyMutation};
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter, Rejection};
use async_graphql_warp::{BadRequest, Response};
use warp::http::StatusCode;
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};

type DynError = Result<(), Box<dyn std::error::Error>>;

struct QueryRoot;

/// impl resolver function for QueryRoot
#[Object]
impl QueryRoot {
    /// {add(a:10, b:20)}: 如果没有携带变量参数，最外层的query关键字可以省略
    /// `query($a: Int!) {add(a: $a, b: 2)}` + {"a":123}: 如果含变量，必须要在最外层的花括号外的写上query()去「定义变量及其类型」
    /// curl -X POST -H "Content-Type: application/json" -d '{"query": "{add(a:1,b:1)}"}' http://localhost:8003
    /// curl -X POST -H "Content-Type: application/json" -d '{"query": "query{add(a:1,b:1)}", "variables": null}' http://localhost:8003
    /// curl -X POST -H "Content-Type: application/json" -d '{"query": "query($a: Int!){add(a:$a,b:1)}", "variables": {"a": 1}}' http://localhost:8003
    async fn add(&self, #[graphql(desc = "Returns the sum of a and b")] a: i32, b: i32) -> async_graphql::Result<i32> {
        Ok(a + b)
    }

    /// sign_up会转为js的驼峰命名，枚举命名FileA则会转为js风格例如FILE_A
    async fn sign_up(&self, _new_user: NewUser) -> User {
        // TODO
        User::default()
    }

    // FIXME panic at runtime
    // async fn profiles(&self, ctx: &Context<'_>) -> Vec<Profile> {
    //     let data = ctx.data::<Arc<Mutex<Vec<Profile>>>>().unwrap().lock().await;
    //     let res = data.clone();
    //     drop(data);
    //     res
    // }
}

#[derive(async_graphql::InputObject)]
struct NewUser {
    email: String,
    password: String
}

/// a SimpleObject can't have resolver function(因为自动生成了各字段的getter方法)
#[derive(async_graphql::SimpleObject, Default)]
struct User {
    id: u64,
    email: String,
    password: String
}

// struct MutationRoot;

// struct Subscription;
//
// #[async_graphql::Subscription]
// impl Subscription {
//     async fn integers(&self) -> impl futures::Stream<Item = i32> {
//         futures::stream::once(async move { 10 })
//         // tokio::time::interval(std::time::Duration::from_secs(1)).map(move |_| {
//         //     value += 1;
//         //     value
//         // })
//     }
// }

#[tokio::main]
async fn main() -> DynError {
    // let data = Arc::new(Mutex::new(profile_data));
    let schema = Schema::build(
        QueryRoot,
        EmptyMutation,
        EmptySubscription,
    )
    // .data(data)
    .finish();

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (
            Schema<QueryRoot, EmptyMutation, EmptySubscription>,
            async_graphql::Request,
        )| async move { Ok::<_, Infallible>(Response::from(schema.execute(request).await)) },
    );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    let routes = graphql_playground.or(graphql_post)
        .recover(|err: Rejection| async move {
            if let Some(BadRequest(err)) = err.find() {
                return Ok::<_, Infallible>(warp::reply::with_status(
                    err.to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }

            // hide server error to client
            Ok(warp::reply::with_status(
                "INTERNAL_SERVER_ERROR".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        });

    println!("http://localhost:8003");
    warp::serve(routes).run(([0, 0, 0, 0], 8003)).await;

    Ok(())
}
