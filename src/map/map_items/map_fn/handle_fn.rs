use std::{collections::VecDeque, fmt::Display};
use syn::{spanned::Spanned, Attribute, Block, Item, Result, Signature, Stmt};
use crate::{item_uses_from_item_imports, map::{map_items::map_fn::{get_attrs::get_attrs_mut_from_stmt, remove_await::remove_await_from_stmt}, replace_item_name_in_signature, replaces_from_item_imports}, ItemImport};
use crate::map::replace_item_name_in_doc;

pub fn handle_fn(
    attrs: Vec<Attribute>, 
    sig: Signature,
    block: Option<Block>,
    asyncness: bool,
    item_imports: &[ItemImport],
) -> Result<(Vec<Attribute>, Signature, Option<Block>)> {

    if sig.asyncness.is_some() {
        return Err(syn::Error::new(
            sig.asyncness.span(), 
            "Instead of the async keyword, use the attribute #[always_async] on the function."
        ))
    }

    let (sig_asyncness, attrs) = take_once_or_none_with_filter_map_from_attrs(
        attrs, 
        |attr| {
            if is_always_async_attr(&attr) {
                Some(Some(syn::token::Async::default()))
            }
            else if is_always_sync_attr(&attr) {
                Some(None)
            }
            else if is_maybe_async_attr(&attr) {
                if asyncness {
                    Some(Some(syn::token::Async::default()))
                }
                else {
                    Some(None)
                }
            }
            else {
                None
            }
        }, 
        || "Use one of the attributes #[always_async], #[always_sync], or #[maybe_async] on the function."
    )?;

    let mut sig = sig;

    if let Some((_, sig_asyncness)) = sig_asyncness {
        sig.asyncness = sig_asyncness;
    }
    else {
        return Err(syn::Error::new(
            sig.fn_token.span(), 
            "Use only one of the attributes #[always_async], #[always_sync], or #[maybe_async] on the function."
        ))
    }

    let replaces = &replaces_from_item_imports(item_imports);
    replace_item_name_in_signature(&mut sig, replaces);

    let block = match block {
        None => None,
        Some(mut block) => {
            let mut new_stmts = Vec::with_capacity(block.stmts.len());

            for item_use in item_uses_from_item_imports(item_imports) {
                new_stmts.push(Stmt::Item(Item::Use(item_use)));
            }

            let mut buf = VecDeque::from_iter(block.stmts.into_iter());
            while let Some(mut stmt) = buf.pop_front() {
                if !asyncness {
                    remove_await_from_stmt(&mut stmt);
                }

                if let Some(attrs) = get_attrs_mut_from_stmt(&mut stmt) {

                    #[derive(PartialEq, Eq)]
                    enum Target {
                        IfSync,
                        IfAsync,
                    }

                    let (target, rest_attrs) = take_once_or_none_with_filter_map_from_attrs(
                        attrs.clone(), 
                        |attr| {
                            if is_if_async_attr(attr) {
                                Some(Target::IfAsync)
                            }
                            else if is_if_sync_attr(attr) {
                                Some(Target::IfSync)
                            }
                            else {
                                None
                            }
                        },
                        || "expected only one of: `#[if_async]`, `#[if_sync]`"
                    )?;

                    *attrs = rest_attrs;

                    if let Some((_, target)) = target {
                        if asyncness && target == Target::IfAsync {
                            new_stmts.push(stmt)
                        }
                        else if !asyncness && target == Target::IfSync {
                            new_stmts.push(stmt)
                        }
                    }
                    else {
                        new_stmts.push(stmt);
                    }
                }
                else {
                    new_stmts.push(stmt);
                }
            }

            block.stmts = new_stmts;
            Some(block)
        }
    };
    
    let attrs = {
        let mut attrs = attrs;
        replace_item_name_in_doc(&mut attrs, replaces);
        attrs
    };

    Ok((attrs, sig, block))
}

fn is_always_sync_attr(attr: &Attribute) -> bool {
    attr.path().is_ident("always_sync")
}

fn is_always_async_attr(attr: &Attribute) -> bool {
    attr.path().is_ident("always_async")
}

fn is_maybe_async_attr(attr: &Attribute) -> bool {
    attr.path().is_ident("maybe_async")
}

fn is_if_async_attr(attr: &Attribute) -> bool {
    attr.path().is_ident("if_async")
}

fn is_if_sync_attr(attr: &Attribute) -> bool {
    attr.path().is_ident("if_sync")
}

fn _take_once_or_none_from_attrs<E: Display>(
    attrs: Vec<Attribute>, 
    filter: impl Fn(&Attribute) -> bool,
    err_msg: impl FnOnce() -> E
) -> Result<(Option<Attribute>, Vec<Attribute>)> {

    take_once_or_none_with_filter_map_from_attrs(
        attrs,
        |attr| {
            match filter(attr) {
                true => Some(()),
                false => None
            }
        }, 
        err_msg
    ).map(|(t, r)| (t.map(|(a, _)| a), r))
}

fn take_once_or_none_with_filter_map_from_attrs<E: Display, T>(
    attrs: Vec<Attribute>, 
    filter_map: impl Fn(&Attribute) -> Option<T>,
    err_msg: impl FnOnce() -> E
) -> Result<(Option<(Attribute, T)>, Vec<Attribute>)> {

    let mut target = None;
    let mut rest = Vec::with_capacity(attrs.len());

    for attr in attrs {
        if let Some(t) = filter_map(&attr) {
            if target.is_some() {
                return Err(syn::Error::new(attr.span(), err_msg()))
            }
            target = Some((attr, t));
        }
        else {
            rest.push(attr);
        }
    }

    Ok((target, rest))
}