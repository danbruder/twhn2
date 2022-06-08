use std::str::FromStr;

use anyhow::Result;
use chrono::{DateTime, Utc};
use duckdb::{params, DropBehavior, OptionalExt};

use crate::{
    adapters::AppCapabilities,
    capabilities::{LoadItemRanks, LoadLatestItemRank, StoreItemRanks},
    domain::{ItemRank, ListCategory},
};

impl StoreItemRanks for AppCapabilities {
    fn store_item_ranks(&self, item_ranks: Vec<ItemRank>) -> Result<()> {
        let mut conn = self.db.get()?;
        let mut tx = conn.transaction()?;
        tx.set_drop_behavior(DropBehavior::Commit);

        for rank in item_ranks.into_iter() {
            tx.execute(
                "INSERT INTO item_rank (id, rank, category, ts) VALUES (?1, ?2, ?3, ?4)",
                params![rank.id, rank.rank, rank.category.to_string(), rank.ts],
            )?;
        }

        Ok(())
    }
}

impl LoadItemRanks for AppCapabilities {
    fn load_item_ranks(&self, id: u32, category: ListCategory) -> Result<Vec<ItemRank>> {
        let conn = self.db.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, rank, category, ts FROM item_rank WHERE id = ?1 AND category = ?2",
        )?;

        let results = stmt
            .query_map(params![id, category.to_string()], |row| {
                let cat: String = row.get(2)?;
                let ts: DateTime<Utc> = row.get(3)?;

                Ok(ItemRank {
                    id: row.get(0)?,
                    rank: row.get(1)?,
                    category: ListCategory::from_str(&cat)
                        .map_err(|_| duckdb::Error::InvalidQuery)?,
                    ts,
                })
            })?
            .filter_map(Result::ok)
            .collect();

        Ok(results)
    }
}

impl LoadLatestItemRank for AppCapabilities {
    fn load_latest_item_rank(&self, id: u32, category: ListCategory) -> Result<Option<ItemRank>> {
        let conn = self.db.get()?;
        let mut stmt = conn.prepare(
            r#"
            SELECT
                id, rank, category, ts 
            FROM 
                item_rank 
            WHERE 
                id = ?1 
            AND 
                list = ?2
            ORDER BY 
                ts DESC
            LIMIT 1
            "#,
        )?;

        let results = stmt
            .query_row(params![id, category.to_string()], |row| {
                let cat: String = row.get(2)?;
                let ts: DateTime<Utc> = row.get(3)?;

                Ok(ItemRank {
                    id: row.get(0)?,
                    rank: row.get(1)?,
                    category: ListCategory::from_str(&cat)
                        .map_err(|_| duckdb::Error::InvalidQuery)?,
                    ts,
                })
            })
            .optional()?;

        Ok(results)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn store_item_ranks() {
        let app = crate::adapters::test::setup();
        let ts = Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 1, 0);
        let ranks = vec![ItemRank {
            id: 1,
            rank: 1,
            ts,
            category: ListCategory::Top,
        }];
        let _ = app.store_item_ranks(ranks.clone()).unwrap();
        let got = app.load_item_ranks(1, ListCategory::Top).unwrap();
        let want = ranks;

        assert_eq!(got, want);
    }
}
