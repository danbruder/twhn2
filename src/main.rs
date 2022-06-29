#[macro_use]
extern crate rocket;

mod adapters;
mod capabilities;
mod cron;
mod domain;
mod graphql;
mod infra;
mod use_cases;

use adapters::AppCapabilities;
use infra::{db::Duck, hn::HnClient};
use std::thread;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use graphql::QueryRoot;
use rocket::{response::content, routes, State};

pub type TwhnSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[rocket::get("/")]
fn graphql_playground() -> content::RawHtml<String> {
    content::RawHtml(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[rocket::get("/graphql?<query..>")]
async fn graphql_query(schema: &State<TwhnSchema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(schema: &State<TwhnSchema>, request: GraphQLRequest) -> GraphQLResponse {
    request.execute(schema).await
}

#[launch]
fn rocket() -> _ {
    let client = HnClient::init().unwrap();
    let duck = Duck::setup("data.db").expect("Could not connect to database");
    duck.migrate().expect("Failed to migrate the database");
    let app = AppCapabilities::new(duck, client);
    thread::spawn(move || {
        cron::run(app);
    });

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    rocket::build().manage(schema).mount(
        "/",
        routes![graphql_query, graphql_request, graphql_playground],
    )
}
