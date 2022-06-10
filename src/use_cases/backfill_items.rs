use crate::capabilities::*;
use anyhow::Result;

pub fn run(
    app: &(impl StoreItems + FetchItems + LoadConfigValue + StoreConfigValue),
    fetch_count: u32,
) -> Result<()> {
    let key = "backfill_ptr";
    let backfill_ptr: u32 = app.load_config_value_as(key)?.unwrap_or(0);
    let ids = (backfill_ptr..(backfill_ptr + fetch_count))
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

#[cfg(test)]
mod test {
    use super::*;

    mockall::mock! {
        App {}

        impl StoreItems for App {
            fn store_items(&self, items: Vec<Item>) -> Result<()>;
        }
        impl FetchItems for App {
            fn fetch_items(&self, ids: Vec<u32>) -> Result<Vec<Item>>;
        }
        impl StoreConfigValue for App {
            fn store_config_value<T: Serialize>(&self, key: &str, value: T) -> Result<()>;
        }

        impl LoadConfigValue for App {
            fn load_config_value_as<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>>;
        }
    }

    #[test]
    fn backfill_ptr_increases_by_fetch_count() {
        let mut mock = MockApp::new();
        mock.expect_fetch_items()
            .times(1)
            .returning(move |_| Ok(vec![]));

        assert_eq!(got, want);
    }
}
