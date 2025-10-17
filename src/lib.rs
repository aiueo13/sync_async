mod args;
mod map;
mod utils;

use crate::{args::*, map::*, utils::*};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, spanned::Spanned, Error, Item, ItemImpl, ItemMod, ItemStruct, Result};


#[proc_macro_attribute]
pub fn sync_async(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as Item);
    let args = parse_macro_input!(attr as Args);
    
    match item {
        Item::Mod(item) => expand_mod(item, args),
        Item::Impl(item) => expand_impl(item, args),
        Item::Struct(item) => expand_struct(item, args),
        item => Err(Error::new(item.span(), "expected one of: impl, mod, struct"))
    }
    .map(Into::into)
    .unwrap_or_else(|r| r.into_compile_error().into())
}


fn expand_mod(item: ItemMod, args: Args) -> Result<TokenStream> {
    let vis = item.vis;
    let _unsafety = item.unsafety;
    let (sync_imports, async_imports) = {
        let si = item_uses_from_item_imports(&args.item_imports_for_sync);
        let ai = item_uses_from_item_imports(&args.item_imports_for_async);
        (si, ai)
    };
    let (sync_ident, async_ident, item_imports_for_sync, item_imports_for_async) = {
        let i = &item.ident;
        let s = format_ident!("sync_{}", i);
        let a = format_ident!("async_{}", i);
    
        let sp = PathWithoutArgs::from_idents(vec![s.clone()], false);
        let ap = PathWithoutArgs::from_idents(vec![a.clone()], false);
        let mut si = args.item_imports_for_sync;
        let mut ai = args.item_imports_for_async;
        si.push(ItemImport { path: sp, alias: Some(i.clone()) });
        ai.push(ItemImport { path: ap, alias: Some(i.clone()) });
        
        (s, a, si, ai)
    };
    let (sync_attrs, async_attrs) = {
        let a = map_attrs(item.attrs, &item_imports_for_sync, &item_imports_for_async)?;
        (a.sync_attrs, a.async_attrs)
    };
    let (sync_items, async_items) = {
        let i = item.content.map(|i| i.1).unwrap_or_else(|| Vec::with_capacity(0));
        // use を使うので型の置換はなしでいい
        let i = map_mod_items(i, &[], &[])?;
        (i.sync_items, i.async_items)
    };
    let semi = item.semi;

    Ok(quote! {
        #(#sync_attrs)*
        #[allow(unused_imports)]
        #vis mod #sync_ident {
            #(#sync_imports)*
            #(#sync_items)*
        }#semi

        #(#async_attrs)*
        #[allow(unused_imports)]
        #vis mod #async_ident {
            #(#async_imports)*
            #(#async_items)*
        }#semi
    })
}

fn expand_impl(item: ItemImpl, args: Args) -> Result<TokenStream> {
    if let Some(trait_) = item.trait_ {
        return Err(Error::new(trait_.2.span(), "unsupported trait impl"))
    }

    let (item_imports_for_sync, item_imports_for_async) = {
        let mut si = args.item_imports_for_sync;
        let mut ai = args.item_imports_for_async;
        let i = map_type_to_ident(&item.self_ty)?;

        si.push(ItemImport { alias: Some(syn::parse_str(&i.ident)?), path: syn::parse_str(&i.sync_ident)? });
        ai.push(ItemImport { alias: Some(syn::parse_str(&i.ident)?), path: syn::parse_str(&i.async_ident)? });
        (si, ai)
    };

    let (sync_attrs, async_attrs) = {
        let a = map_attrs(item.attrs, &item_imports_for_sync, &item_imports_for_async)?;
        (a.sync_attrs, a.async_attrs)
    };
    let _defaultness = item.defaultness;
    let unsafety = item.unsafety;
    let (sync_generics, async_generics) = {
        let g = map_generics(item.generics, &item_imports_for_sync, &item_imports_for_async)?;
        (g.sync_generics, g.async_generics)
    };
    let (sync_self_ty, async_self_ty) = {
        let t = map_type(*item.self_ty)?;
        (t.sync_ty, t.async_ty)
    };
    let (sync_items, async_items) = {
        let i = map_impl_items(item.items, &item_imports_for_sync, &item_imports_for_async)?;
        (i.sync_items, i.async_items)
    };

    Ok(quote! {
        #(#sync_attrs)*
        #[allow(unused_imports)]
        #unsafety impl #sync_generics #sync_self_ty {
            #(#sync_items)*
        }

        #(#async_attrs)*
        #[allow(unused_imports)]
        #unsafety impl #async_generics #async_self_ty {
            #(#async_items)*
        }
    })
}

fn expand_struct(item: ItemStruct, args: Args) -> Result<TokenStream> {
    let vis = item.vis;
    let (sync_ident, async_ident, item_imports_for_sync, item_imports_for_async) = {
        let i = &item.ident;
        let s = format_ident!("Sync{}", i);
        let a = format_ident!("Async{}", i);
    
        let sp = PathWithoutArgs::from_idents(vec![s.clone()], false);
        let ap = PathWithoutArgs::from_idents(vec![a.clone()], false);
        let mut si = args.item_imports_for_sync;
        let mut ai = args.item_imports_for_async;
        si.push(ItemImport { path: sp, alias: Some(i.clone()) });
        ai.push(ItemImport { path: ap, alias: Some(i.clone()) });
        
        (s, a, si, ai)
    };
    let (sync_attrs, async_attrs) = {
        let a = map_attrs(item.attrs, &item_imports_for_sync, &item_imports_for_async)?;
        (a.sync_attrs, a.async_attrs)
    };
    let (sync_generics, async_generics) = {
        let g = map_generics(item.generics, &item_imports_for_sync, &item_imports_for_async)?;
        (g.sync_generics, g.async_generics)
    };
    let (sync_fields, async_fields) = {
        let f = map_fields(item.fields, &item_imports_for_sync, &item_imports_for_async)?;
        (f.sync_fields, f.async_fields)
    };
    let semi = item.semi_token;

    Ok(quote! {
        #(#sync_attrs)* 
        #vis struct #sync_ident #sync_generics #sync_fields #semi

        #(#async_attrs)*      
        #vis struct #async_ident #async_generics #async_fields #semi
    }.into())
}