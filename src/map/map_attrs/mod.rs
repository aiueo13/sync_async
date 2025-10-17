use syn::{Attribute, Result};
use crate::{replace_item_name_in_doc, replaces_from_item_imports, ItemImport};


pub struct SyncAsyncAttrs {
    pub sync_attrs: Vec<Attribute>,
    pub async_attrs: Vec<Attribute>
}

pub fn map_attrs(
    attrs: Vec<Attribute>,
    item_imports_for_sync: &[ItemImport],
    item_imports_for_async: &[ItemImport]
) -> Result<SyncAsyncAttrs> {

    let mut sync_attrs = attrs.clone();
    let mut async_attrs = attrs;
    replace_item_name_in_doc(
        sync_attrs.iter_mut(), 
        &replaces_from_item_imports(item_imports_for_sync)
    );
    replace_item_name_in_doc(
        async_attrs.iter_mut(), 
        &replaces_from_item_imports(item_imports_for_async)
    );

    Ok(SyncAsyncAttrs { sync_attrs, async_attrs })
}
