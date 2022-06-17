use crate::capabilities::*;
use anyhow::Result;

pub fn run(app: &(impl StoreItems + FetchItems + FetchUpdates)) -> Result<()> {
    let updated_item_ids = app.fetch_updates()?;
    let items = app.fetch_items(updated_item_ids)?;
    app.store_items(items)?;

    Ok(())
}
