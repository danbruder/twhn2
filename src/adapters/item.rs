use anyhow::Result;
use chrono::Utc;
use duckdb::{params, OptionalExt};

use crate::{adapters::AppCapabilities, capabilities::*, infra::hn::types::Item};

impl FetchUpdates for AppCapabilities {
    fn fetch_updates(&self) -> Result<Vec<u32>> {
        Ok(self.client.get_updates()?.items)
    }
}

impl FetchItems for AppCapabilities {
    fn fetch_items(&self, ids: Vec<u32>) -> Result<Vec<Item>> {
        Ok(self.client.get_items(ids)?)
    }
}

impl FetchItem for AppCapabilities {
    fn fetch_item(&self, id: u32) -> Result<Option<Item>> {
        Ok(self.client.get_item(id)?)
    }
}

impl StoreItems for AppCapabilities {
    fn store_items(&self, items: Vec<Item>) -> Result<()> {
        let mut conn = self.db.get()?;
        let tx = conn.transaction()?;

        let ts = Utc::now();
        for item in items {
            let original = serde_json::to_string(&item)?;
            let _ = tx
            .execute(
                r#"
                    INSERT INTO item (id, original, descendants, username, score, title, url, body, ts)
                    VALUES 
                    (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
                "#,
                params![
                    item.id(),
                    original,
                    item.descendants(),
                    item.username(),
                    item.score(),
                    item.title(),
                    item.url(),
                    item.body(),
                    ts.clone(),
                ]
            )?;
        }
        tx.commit()?;

        Ok(())
    }
}

impl StoreItem for AppCapabilities {
    fn store_item(&self, item: Item) -> Result<()> {
        let conn = self.db.get()?;

        let ts = Utc::now();
        let original = serde_json::to_string(&item)?;
        let _ = conn
            .execute(
                r#"
                    INSERT INTO item (id, original, descendants, username, score, title, url, body, ts)
                    VALUES 
                    (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
                "#,
                params![
                    item.id(),
                    original,
                    item.descendants(),
                    item.username(),
                    item.score(),
                    item.title(),
                    item.url(),
                    item.body(),
                    ts.clone(),
                ]
            )?;

        Ok(())
    }
}

impl LoadItems for AppCapabilities {
    fn load_items(&self, ids: Vec<u32>) -> Result<Vec<Item>> {
        let conn = self.db.get()?;

        let mut stmt = conn.prepare("SELECT original FROM item WHERE id = ?1")?;

        let mut results = vec![];
        for id in ids {
            if let Some(item) = stmt
                .query_row([id], |row| {
                    let blob: String = row.get(0)?;
                    let item: Item = serde_json::from_str(&blob).unwrap();
                    Ok(item)
                })
                .optional()?
            {
                results.push(item);
            }
        }

        Ok(results)
    }
}

impl LoadItem for AppCapabilities {
    fn load_item(&self, id: u32) -> Result<Option<Item>> {
        let conn = self.db.get()?;
        let mut stmt = conn.prepare("SELECT original FROM item WHERE id = ?1")?;

        Ok(stmt
            .query_row([id], |row| {
                let blob: String = row.get(0)?;
                let item: Item = serde_json::from_str(&blob).unwrap();
                Ok(item)
            })
            .optional()?)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::infra::hn::types::tests::sample_items;

    #[test]
    fn store_items() {
        let app = crate::adapters::test::setup();
        let items = sample_items();
        let ids = items.iter().map(|i| i.id()).collect::<Vec<_>>();
        let _ = app.store_items(items.clone()).unwrap();
        let got = app.load_items(ids).unwrap();
        let want = items;

        assert_eq!(got, want);
    }

    #[test]
    fn store_item() {
        let app = crate::adapters::test::setup();
        let items = sample_items();
        let item = &items[0];
        let _ = app.store_item(item.clone()).unwrap();
        let got = app.load_item(item.id()).unwrap();
        let want = Some(item.clone());

        assert_eq!(got, want);
    }
}
