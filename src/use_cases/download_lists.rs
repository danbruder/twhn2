use crate::{
    capabilities::{FetchItems, FetchList, StoreItems, StoreList},
    domain::ListCategory,
};
use anyhow::Result;
use strum::IntoEnumIterator;

pub fn run(app: &(impl StoreList + StoreItems + FetchItems + FetchList)) -> Result<()> {
    for category in ListCategory::iter() {
        let ids = app.fetch_list(category.clone())?;
        fetch_and_store(app, ids, category)?;
    }

    Ok(())
}

fn fetch_and_store(
    app: &(impl StoreList + StoreItems + FetchItems),
    ids: Vec<u32>,
    category: ListCategory,
) -> Result<()> {
    let ids = ids.into_iter().take(30).collect::<Vec<_>>();
    let items = app.fetch_items(ids.clone())?;

    // Store top items by rank
    app.store_list(category, &ids)?;

    // Store change in item
    app.store_items(items)?;

    Ok(())
}
