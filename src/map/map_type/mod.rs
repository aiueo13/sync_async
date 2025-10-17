use std::borrow::Cow;
use quote::quote;
use syn::{Result, Type};
use crate::map::replace_item_name::{replace_item_name_in_type, ReplaceItemNameFromTo};


pub struct SyncAsyncType {
    pub sync_ty: Type,
    pub async_ty: Type,
}

pub struct SyncAsyncIdentOfType {
    pub sync_ident: String,
    pub async_ident: String,
    pub ident: String,
}

pub fn map_type_to_ident(
    ty: &Type
) -> Result<SyncAsyncIdentOfType> {

    let ident = {
        let ty_str = quote! { #ty }.to_string().replace(" ", "");
        ty_str
            .split_once('<')
            .map(|(i, _)| i.to_string())
            .unwrap_or(ty_str)
    };
    let sync_ident = format!("Sync{ident}");
    let async_ident = format!("Async{ident}");

    Ok(SyncAsyncIdentOfType { sync_ident, async_ident, ident })
}

pub fn map_type(
    ty: Type
) -> Result<SyncAsyncType> {

    let SyncAsyncIdentOfType { sync_ident, async_ident, ident } = map_type_to_ident(&ty)?;

    let replaces = |from: &str, to: &str| -> Result<Vec<ReplaceItemNameFromTo<'_>>> {
        Ok(vec![ReplaceItemNameFromTo { 
            from: Cow::Owned(syn::parse_str(from)?), 
            to: Cow::Owned(syn::parse_str(to)?) 
        }])
    };

    let mut sync_ty = ty.clone();
    let mut async_ty = ty;
    replace_item_name_in_type(&mut sync_ty, &replaces(&ident, &sync_ident)?);
    replace_item_name_in_type(&mut async_ty, &replaces(&ident, &async_ident)?);
   
    Ok(SyncAsyncType { sync_ty, async_ty })
}