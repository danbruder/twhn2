use anyhow::Result;

use crate::{
    domain::{ItemRank, ListCategory},
    infra::hn::types::Item,
};

// LISTS
pub trait StoreList {
    fn store_list(&self, category: ListCategory, ids: &[u32]) -> Result<()>;
}

pub trait LoadList {
    fn load_list(&self, category: ListCategory) -> Result<Vec<u32>>;
}

pub trait FetchList {
    fn fetch_list(&self, category: ListCategory) -> Result<Vec<u32>>;
}

// ITEMS
pub trait LoadItems {
    fn load_items(&self, ids: Vec<u32>) -> Result<Vec<Item>>;
}

pub trait LoadItem {
    fn load_item(&self, ids: u32) -> Result<Option<Item>>;
}

pub trait StoreItems {
    fn store_items(&self, items: Vec<Item>) -> Result<()>;
}

pub trait StoreItem {
    fn store_item(&self, item: Item) -> Result<()>;
}

pub trait FetchItem {
    fn fetch_item(&self, id: u32) -> Result<Option<Item>>;
}

pub trait FetchItems {
    fn fetch_items(&self, ids: Vec<u32>) -> Result<Vec<Item>>;
}

// ITEM RANKS
pub trait StoreItemRanks {
    fn store_item_ranks(&self, item_ranks: Vec<ItemRank>) -> Result<()>;
}

pub trait LoadItemRanks {
    fn load_item_ranks(&self, id: u32, category: ListCategory) -> Result<Vec<ItemRank>>;
}
