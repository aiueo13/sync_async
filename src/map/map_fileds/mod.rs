use syn::{Fields, Result};
use crate::{map::replace_item_name::{replace_item_name_in_fields, replaces_from_item_imports}, ItemImport};


pub struct SyncAsyncFields {
    pub sync_fields: Fields,
    pub async_fields: Fields
}

pub fn map_fields(
    fields: Fields,
    item_imports_for_sync: &[ItemImport],
    item_imports_for_async: &[ItemImport]
) -> Result<SyncAsyncFields> {

    let mut sync_fields = fields.clone();
    let mut async_fields = fields;
    replace_item_name_in_fields(
        &mut sync_fields, 
        &replaces_from_item_imports(item_imports_for_sync)
    );
    replace_item_name_in_fields(
        &mut async_fields, 
        &replaces_from_item_imports(item_imports_for_async)
    );

    Ok(SyncAsyncFields { sync_fields, async_fields })
}