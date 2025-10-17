use super::replace_item_name::*;
use syn::{Generics, Result};
use crate::ItemImport;


pub struct SyncAsyncGenerics {
    pub sync_generics: Generics,
    pub async_generics: Generics
}

pub fn map_generics(
    generics: Generics,
    item_imports_for_sync: &[ItemImport],
    item_imports_for_async: &[ItemImport]
) -> Result<SyncAsyncGenerics> {

    let mut sync_generics = generics.clone();
    let mut async_generics = generics;
    replace_item_name_in_generics(
        &mut sync_generics, 
        &replaces_from_item_imports(&item_imports_for_sync)
    );
    replace_item_name_in_generics(
        &mut async_generics, 
        &replaces_from_item_imports(&item_imports_for_async)
    );
        
    Ok(SyncAsyncGenerics { sync_generics, async_generics })
}