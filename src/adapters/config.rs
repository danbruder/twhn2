use anyhow::Result;
use duckdb::{params, OptionalExt};
use serde::{de::DeserializeOwned, ser::Serialize};

use crate::{adapters::AppCapabilities, capabilities::*};

impl StoreConfigValue for AppCapabilities {
    fn store_config_value<T: Serialize>(&self, key: &str, value: T) -> Result<()> {
        let conn = self.db.get()?;

        let _ = conn.execute(
            r#"
                    DELETE FROM config 
                    WHERE key=?1
            "#,
            params![key],
        )?;

        let _ = conn.execute(
            r#"
                    INSERT INTO config (key, value)
                    VALUES (?1, ?2)
            "#,
            params![key, serde_json::to_string(&value)?],
        )?;

        Ok(())
    }
}

impl LoadConfigValue for AppCapabilities {
    fn load_config_value_as<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        let conn = self.db.get()?;

        let mut stmt = conn.prepare("SELECT value FROM config WHERE key = ?1")?;
        if let Some(value) = stmt
            .query_row([key], |row| {
                let value: String = row.get(0)?;
                let value: T =
                    serde_json::from_str(&value).map_err(|_| duckdb::Error::InvalidQuery)?;
                Ok(value)
            })
            .optional()?
        {
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn round_trip() {
        let app = crate::adapters::test::setup();
        let key = "foo";
        let value = 42;
        let _ = app.store_config_value(key, value).unwrap();
        let got: i32 = app.load_config_value_as(key).unwrap().unwrap();
        let want = value;

        assert_eq!(got, want);
    }
}
