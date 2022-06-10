#![feature(proc_macro_hygiene, decl_macro)]

mod adapters;
mod capabilities;
mod domain;
mod infra;
mod use_cases;

use adapters::AppCapabilities;
use anyhow::Result;
use infra::{db::Duck, hn::HnClient};
use std::thread;
use std::time::Duration;

#[macro_use]
extern crate rocket;

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

fn main() {
    let client = HnClient::init().unwrap();
    let duck = Duck::setup("data.db").expect("Could not connect to database");
    duck.migrate().expect("Failed to migrate the database");
    let app = AppCapabilities::new(duck, client);
    thread::spawn(move || {
        cron(app);
    });

    rocket::ignite().mount("/", routes![hello]).launch();
}

// Tasks
// ---
// [X] Download and save each list
// [X] Track change in rank
// [ ] Backfill all items
// [ ] Poll for updates
// [ ] Store valid HTML
// [ ] GraphQL api
// --------
// Future
// --------
// Track change in score
// Track change in comment count
// Process special fields
//   extract URLs
//   Parse body into valid HTML
//   Create Markdown from HTML
//   Create Plaintext from HTML
// Search index
fn cron(app: AppCapabilities) {
    let do_work = || -> Result<()> {
        use_cases::download_lists::run(&app)?;
        use_cases::backfill_items::run(&app, 10_000)?;
        Ok(())
    };

    loop {
        let result = do_work();
        println!("Done");

        if result.is_err() {
            println!("{:?}", result);
        }

        thread::sleep(Duration::from_secs(3));
    }
}
