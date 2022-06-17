use crate::capabilities::*;
use anyhow::Result;

pub fn run(
    app: &(impl StoreItems + FetchItems + LoadConfigValue + StoreConfigValue),
    fetch_count: u32,
) -> Result<()> {
    let key = "backfill_ptr";
    let backfill_ptr: u32 = app.load_config_value_as(key)?.unwrap_or(0);
    let ids = (backfill_ptr..=(backfill_ptr + fetch_count))
        .into_iter()
        .collect::<Vec<_>>();
    let new_backfill_ptr = ids.iter().max();

    // Fetch from HN API
    // here we have network items need to transform into domain items
    let items = app.fetch_items(ids.clone())?;

    // Store change in item
    app.store_items(items)?;

    // Store our offset
    app.store_config_value(key, new_backfill_ptr)?;

    Ok(())
}
