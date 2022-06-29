use anyhow::Result;
use std::thread;
use std::time::Duration;

use crate::{adapters::AppCapabilities, use_cases};

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
pub fn run(app: AppCapabilities) {
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
