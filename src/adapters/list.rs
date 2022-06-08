use anyhow::Result;

use crate::{
    adapters::AppCapabilities,
    capabilities::{FetchList, LoadList, StoreList},
    domain::ListCategory,
};
use chrono::Utc;
use duckdb::{params, OptionalExt};

impl FetchList for AppCapabilities {
    fn fetch_list(&self, category: ListCategory) -> Result<Vec<u32>> {
        Ok(self.client.get_stories_list(category)?)
    }
}

impl StoreList for AppCapabilities {
    fn store_list(&self, category: ListCategory, ids: &[u32]) -> Result<()> {
        let conn = self.db.get()?;
        let _ = conn.execute(
            r#"
                    INSERT INTO item_list (key, ids, ts)
                    VALUES 
                    (?1, ?2, ?3)
                "#,
            params![
                category.to_string(),
                serde_json::to_string(&ids)?,
                Utc::now()
            ],
        )?;

        Ok(())
    }
}

impl LoadList for AppCapabilities {
    fn load_list(&self, category: ListCategory) -> Result<Vec<u32>> {
        let conn = self.db.get()?;
        let mut stmt = conn.prepare("SELECT ids FROM item_list WHERE key=?1")?;
        let ids_str: Option<String> = stmt
            .query_row([category.to_string()], |row| Ok(row.get(0)?))
            .optional()?;
        let ids = ids_str
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();

        Ok(ids)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn item_list() {
        let app = crate::adapters::test::setup();
        let _ = app.store_list(ListCategory::Top, &[1, 2, 3]).unwrap();
        let got = app.load_list(ListCategory::Top).unwrap();
        let want = vec![1, 2, 3];

        assert_eq!(got, want);
    }
}
