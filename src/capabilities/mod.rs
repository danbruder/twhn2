use anyhow::Result;
use serde::{de::DeserializeOwned, ser::Serialize};

use crate::{
    domain::{ItemRank, ListCategory},
    infra::hn::types::Item,
};

// LISTS
#[mockall::automock]
pub trait StoreList {
    fn store_list(&self, category: ListCategory, ids: &[u32]) -> Result<()>;
}

#[mockall::automock]
pub trait ReplaceList {
    fn replace_list(&self, category: ListCategory, ids: &[u32]) -> Result<()>;
}

#[mockall::automock]
pub trait LoadList {
    fn load_list(&self, category: ListCategory) -> Result<Vec<u32>>;
}

#[mockall::automock]
pub trait FetchList {
    fn fetch_list(&self, category: ListCategory) -> Result<Vec<u32>>;
}

// ITEMS
#[mockall::automock]
pub trait LoadItems {
    fn load_items(&self, ids: Vec<u32>) -> Result<Vec<Item>>;
}

#[mockall::automock]
pub trait LoadItem {
    fn load_item(&self, ids: u32) -> Result<Option<Item>>;
}

#[mockall::automock]
pub trait StoreItems {
    fn store_items(&self, items: Vec<Item>) -> Result<()>;
}

#[mockall::automock]
pub trait StoreItem {
    fn store_item(&self, item: Item) -> Result<()>;
}

#[mockall::automock]
pub trait FetchItem {
    fn fetch_item(&self, id: u32) -> Result<Option<Item>>;
}

#[mockall::automock]
pub trait FetchItems {
    fn fetch_items(&self, ids: Vec<u32>) -> Result<Vec<Item>>;
}

#[mockall::automock]
pub trait FetchUpdates {
    fn fetch_updates(&self) -> Result<Vec<u32>>;
}

// ITEM RANKS
#[mockall::automock]
pub trait StoreItemRanks {
    fn store_item_ranks(&self, item_ranks: Vec<ItemRank>) -> Result<()>;
}

#[mockall::automock]
pub trait LoadItemRanks {
    fn load_item_ranks(&self, id: u32, category: ListCategory) -> Result<Vec<ItemRank>>;
}

#[mockall::automock]
pub trait LoadLatestItemRank {
    fn load_latest_item_rank(&self, id: u32, category: ListCategory) -> Result<Option<ItemRank>>;
}

// CONFIG
pub trait StoreConfigValue {
    fn store_config_value<T: Serialize>(&self, key: &str, value: T) -> Result<()>;
}
pub trait LoadConfigValue {
    fn load_config_value_as<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>>;
}
