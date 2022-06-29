#[macro_use]
extern crate rocket;

mod adapters;
mod capabilities;
mod domain;
mod graphql;
mod infra;
mod use_cases;

use adapters::AppCapabilities;
use anyhow::Result;
use infra::{db::Duck, hn::HnClient};
use std::thread;
use std::time::Duration;

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
        cron(app);
    });

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    rocket::build().manage(schema).mount(
        "/",
        routes![graphql_query, graphql_request, graphql_playground],
    )
}

// Tasks
// ---
// [X] Download and save each list
// [X] Track change in rank
// [X] Backfill all items
// [X] Poll for updates
// [ ] GraphQL api
// --------
// Future
// --------
// [ ] Search API
// [ ] Store valid HTML
// [ ] Track change in score, comment count, etc.
fn cron(app: AppCapabilities) {
    let do_work = || -> Result<()> {
        use_cases::download_lists::run(&app)?;
        use_cases::backfill_items::run(&app, 10)?;
        use_cases::poll_for_updates::run(&app)?;
        Ok(())
    };

    loop {
        let result = do_work();
        println!("Finished cron loop");

        if result.is_err() {
            println!("{:?}", result);
        }

        thread::sleep(Duration::from_secs(20));
    }
}
