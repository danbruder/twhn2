mod item;
mod item_rank;
mod list;

use crate::infra::{db::Duck, hn::HnClient};

pub struct AppCapabilities {
    db: Duck,
    client: HnClient,
}

impl AppCapabilities {
    pub fn new(db: Duck, client: HnClient) -> Self {
        Self { db, client }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    pub fn setup() -> AppCapabilities {
        let db = crate::infra::db::test::setup();
        let client = HnClient::init().unwrap();
        AppCapabilities::new(db, client)
    }
}
