use async_graphql::{Object, Schema, Context, EmptySubscription, EmptyMutation};
use std::sync::Arc;
// use tokio::sync::Mutex;
use futures::lock::Mutex;
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter, Rejection};
use async_graphql_warp::{BadRequest, Response};
use warp::http::StatusCode;
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};

type DynError = Result<(), Box<dyn std::error::Error>>;

#[derive(Default)]
struct MyFirstQuery;

/// impl resolver function for QueryRoot. a SimpleObject can't have resolver function(因为自动生成了各字段的getter方法)
#[Object]
impl MyFirstQuery {
    /**
    {add(a: 10, b: 20)}
    */
    async fn add(&self, #[graphql(desc = "Returns the sum of a and b")] a: i32, b: i32) -> async_graphql::Result<i32> {
        Ok(a + b)
    }

    /**
    {
      signUp(newUser: { email: "w@w.w", password: "1234" }) {
        id
        email
      }
    }
    */
    async fn sign_up(&self, new_user: NewUser) -> User {
        User {
            id: 0,
            email: new_user.email.clone(),
            password: new_user.password.clone()
        }
    }
}

#[derive(Clone, Default)]
struct Profile {
    profile_views: u32,
    website_clicks: u32,
}

#[Object]
impl Profile {
    #[graphql(name = "profile_views")]
    async fn profile_views(&self) -> u32 {
        self.profile_views
    }

    /// would convert to camcel case websiteClicks in graphql query
    /// 对于枚举类型，graphql会转为js风格例如FILE_A
    async fn website_clicks(&self) -> u32 {
        self.website_clicks
    }


    // FIXME panic at runtime
    /*
    {
        {
            profiles {
                profile_views,
                # 注意struct中的字段会自动转为驼峰
                websiteClicks
        }
    }
    */
    // async fn profiles(&self, ctx: &Context<'_>) -> Vec<Profile> {
    //     let data = ctx.data::<Arc<Mutex<Vec<Profile>>>>().unwrap().lock().await;
    //     let res = data.clone();
    //     drop(data);
    //     res
    // }
}

#[derive(async_graphql::MergedObject, Default)]
struct QueryRoot(MyFirstQuery, Profile);

type Data = Arc<Mutex<Vec<Profile>>>;

#[derive(async_graphql::InputObject)]
struct NewUser {
    email: String,
    password: String
}

#[derive(async_graphql::SimpleObject)]
struct User {
    id: u64,
    email: String,
    password: String
}

// struct MutationRoot;

#[tokio::main]
async fn main() -> DynError {
    let profile_data = vec![Profile {
        profile_views: 12389,
        website_clicks: 322193,
    }];
    let data: Data = Arc::new(Mutex::new(profile_data));
    let schema = Schema::build(
        QueryRoot::default(),
        async_graphql::EmptyMutation,
        async_graphql::EmptySubscription,
    )
    .data(data)
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

    let routes = graphql_playground
        .or(graphql_post)
        .recover(|err: Rejection| async move {
            if let Some(BadRequest(err)) = err.find() {
                return Ok::<_, Infallible>(warp::reply::with_status(
                    err.to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }

            Ok(warp::reply::with_status(
                "INTERNAL_SERVER_ERROR".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        });

    warp::serve(routes).run(([0, 0, 0, 0], 8003)).await;




    Ok(())
}
