mod map_fn;

use map_fn::*;
use syn::{ImplItem, Item, Result};

use crate::ItemImport;


pub struct SyncAsyncItems<T> {
    pub sync_items: Vec<T>,
    pub async_items: Vec<T>
}

pub fn map_mod_items(
    items: Vec<Item>,
    item_imports_for_sync: &[ItemImport],
    item_imports_for_async: &[ItemImport]
) -> Result<SyncAsyncItems<Item>> {

    let mut sync_items = Vec::new();
    let mut async_items = Vec::new();

    for item in items {
        match item {
            Item::Fn(item_fn) => {
                sync_items.push(map_mod_fn(item_fn.clone(), false, item_imports_for_sync)?);
                async_items.push(map_mod_fn(item_fn, true, item_imports_for_async)?);
            }
            _ => {
                sync_items.push(item.clone());
                async_items.push(item);
            }
        }
    }

    Ok(SyncAsyncItems { sync_items, async_items })
}

pub fn map_impl_items(
    items: Vec<ImplItem>,
    item_imports_for_sync: &[ItemImport],
    item_imports_for_async: &[ItemImport],
) -> Result<SyncAsyncItems<ImplItem>> {

    let mut sync_items = Vec::new();
    let mut async_items = Vec::new();

    for item in items {
        match item {
            ImplItem::Fn(item_fn) => {
                sync_items.push(map_impl_fn(item_fn.clone(), false, item_imports_for_sync)?);
                async_items.push(map_impl_fn(item_fn, true, item_imports_for_async)?);
            }
            _ => {
                sync_items.push(item.clone());
                async_items.push(item);
            }
        }
    }

    Ok(SyncAsyncItems { sync_items, async_items })
}