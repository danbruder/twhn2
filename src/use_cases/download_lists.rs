use crate::{
    capabilities::*,
    domain::{ItemRank, ListCategory},
};
use anyhow::Result;
use chrono::{DateTime, Utc};
use strum::IntoEnumIterator;

pub fn run(
    app: &(impl StoreList
          + StoreItems
          + FetchItems
          + FetchList
          + StoreItemRanks
          + LoadLatestItemRank
          + ReplaceList),
) -> Result<()> {
    for category in ListCategory::iter() {
        let ids = app.fetch_list(category.clone())?;
        fetch_and_store(app, ids, category)?;
    }

    Ok(())
}

fn fetch_and_store(
    app: &(impl StoreList + ReplaceList + StoreItems + FetchItems + StoreItemRanks + LoadLatestItemRank),
    ids: Vec<u32>,
    category: ListCategory,
) -> Result<()> {
    let ids = ids.into_iter().take(30).collect::<Vec<_>>();
    let item_ranks = item_ranks_from_vec(app, &category, &ids, &Utc::now())?;

    // Fetch from HN API
    let items = app.fetch_items(ids.clone())?;

    // Replaces list
    app.replace_list(category, &ids)?;

    // Store change in item
    app.store_items(items)?;

    // Now store the item ranks
    app.store_item_ranks(item_ranks)?;

    Ok(())
}

fn item_ranks_from_vec(
    app: &impl LoadLatestItemRank,
    category: &ListCategory,
    input: &[u32],
    ts: &DateTime<Utc>,
) -> Result<Vec<ItemRank>> {
    let mut results = vec![];

    for (rank, id) in input.iter().enumerate() {
        let maybe_item_rank = app.load_latest_item_rank(*id, category.clone())?;
        let rank = rank as u32 + 1; // Take care of the off by one nature here

        // If the rank has changed, include it in the results
        match maybe_item_rank {
            Some(item_rank) if item_rank.rank != rank => {
                results.push(ItemRank {
                    id: *id,
                    rank,
                    category: category.clone(),
                    ts: ts.clone(),
                });
            }
            None => {
                results.push(ItemRank {
                    id: *id,
                    rank,
                    category: category.clone(),
                    ts: ts.clone(),
                });
            }
            _ => (),
        };
    }

    Ok(results)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn item_ranks_from_vec_when_rank_changed() {
        let mut mock = MockLoadLatestItemRank::new();
        let ts = Utc::now();

        mock.expect_load_latest_item_rank()
            .times(1)
            .returning(move |id, category| {
                Ok(Some(ItemRank {
                    id,
                    category,
                    rank: 2,
                    ts: ts.clone(),
                }))
            });

        let input = vec![100];

        let got = item_ranks_from_vec(&mock, &ListCategory::Top, &input, &ts).unwrap();
        let want = vec![ItemRank {
            id: 100,
            rank: 1, // New rank is 1
            category: ListCategory::Top,
            ts: ts.clone(),
        }];

        assert_eq!(got, want);
    }

    #[test]
    fn item_ranks_from_vec_when_none_exists() {
        let mut mock = MockLoadLatestItemRank::new();
        let ts = Utc::now();

        mock.expect_load_latest_item_rank()
            .times(1)
            .returning(move |_, _| Ok(None));

        let input = vec![100];

        let got = item_ranks_from_vec(&mock, &ListCategory::Top, &input, &ts).unwrap();
        let want = vec![ItemRank {
            id: 100,
            rank: 1,
            category: ListCategory::Top,
            ts: ts.clone(),
        }];

        assert_eq!(got, want);
    }

    #[test]
    fn item_ranks_from_vec_with_mixed_cases() {
        let mut mock = MockLoadLatestItemRank::new();
        let ts = Utc::now();

        mock.expect_load_latest_item_rank()
            .times(3)
            .returning(move |id, category| match id {
                // 100 has changed rank
                100 => Ok(Some(ItemRank {
                    id,
                    category,
                    rank: 2,
                    ts: ts.clone(),
                })),
                // 200 wasn't there
                // 300 stayed the same
                300 => Ok(Some(ItemRank {
                    id,
                    category,
                    rank: 3,
                    ts: ts.clone(),
                })),
                _ => Ok(None),
            });

        let input = vec![100, 200, 300];

        let got = item_ranks_from_vec(&mock, &ListCategory::Top, &input, &ts).unwrap();
        let want = vec![
            ItemRank {
                id: 100,
                rank: 1,
                category: ListCategory::Top,
                ts: ts.clone(),
            },
            ItemRank {
                id: 200,
                rank: 2,
                category: ListCategory::Top,
                ts: ts.clone(),
            },
            // 300 didn't change
        ];

        assert_eq!(got, want);
    }
}
