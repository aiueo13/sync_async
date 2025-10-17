mod get_attrs;
mod handle_fn;
mod remove_await;

use handle_fn::*;
use syn::{Block, ImplItem, ImplItemFn, Item, ItemFn, Result};
use crate::ItemImport;


pub fn map_impl_fn(
    item: ImplItemFn, 
    asyncness: bool,
    item_imports: &[ItemImport],
) -> Result<ImplItem> {

    let (attrs, sig, block) = handle_fn(
        item.attrs, 
        item.sig,
        Some(item.block), 
        asyncness,
        item_imports,
    )?;

    let block = block.unwrap();
    Ok(ImplItem::Fn(ImplItemFn { attrs, sig, block, ..item }))
}

pub fn map_mod_fn(
    item: ItemFn, 
    asyncness: bool,
    item_imports: &[ItemImport],
) -> Result<Item> {

    let (attrs, sig, block) = handle_fn(
        item.attrs, 
        item.sig, 
        Some(Block::clone(&item.block)), 
        asyncness,
        item_imports,
    )?;

    let block = Box::new(block.unwrap());
    Ok(Item::Fn(ItemFn { attrs, sig, block, ..item }))
}