use crate::{
    capabilities::{
        FetchItems, FetchList, LoadLatestItemRank, StoreItemRanks, StoreItems, StoreList,
    },
    domain::{ItemRank, ListCategory},
};
use anyhow::Result;
use chrono::Utc;
use strum::IntoEnumIterator;

pub fn run(
    app: &(impl StoreList + StoreItems + FetchItems + FetchList + StoreItemRanks + LoadLatestItemRank),
) -> Result<()> {
    for category in ListCategory::iter() {
        let ids = app.fetch_list(category.clone())?;
        fetch_and_store(app, ids, category)?;
    }

    Ok(())
}

fn fetch_and_store(
    app: &(impl StoreList + StoreItems + FetchItems + StoreItemRanks + LoadLatestItemRank),
    ids: Vec<u32>,
    category: ListCategory,
) -> Result<()> {
    let ids = ids.into_iter().take(30).collect::<Vec<_>>();
    let item_ranks = item_ranks_from_vec(app, &category, &ids)?;

    // Fetch from HN API
    let items = app.fetch_items(ids.clone())?;

    app.delete_list(category, &ids)?;
    app.store_list(category, &ids)?;

    // Store change in item
    app.store_items(items)?;

    // Now store the item ranks
    app.store_item_ranks(item_ranks)?;

    Ok(())
}

fn item_ranks_from_vec(
    app: &(impl StoreItemRanks + LoadLatestItemRank),
    category: &ListCategory,
    input: &[u32],
) -> Result<Vec<ItemRank>> {
    let ts = Utc::now();
    let mut results = vec![];

    for (rank, id) in input.iter().enumerate() {
        let maybe_item_rank = app.load_latest_item_rank(*id, category.clone())?;
        let rank = rank as u32;

        // If the rank has changed, include it in the results
        match maybe_item_rank {
            Some(item_rank) if item_rank.rank != rank => {
                results.push(ItemRank {
                    id: *id,
                    rank,
                    category: category.clone(),
                    ts,
                });
            }
            _ => (),
        };
    }

    Ok(results)
}
