use syn::{parenthesized, parse::{Parse, ParseStream}, spanned::Spanned, token, Error, Ident, Result, Token, UseTree};
use std::collections::VecDeque;
use crate::PathWithoutArgs;


pub struct Args {
    pub item_imports_for_async: Vec<ItemImport>,
    pub item_imports_for_sync: Vec<ItemImport>
}

pub struct ItemImport {
    pub path: PathWithoutArgs,
    pub alias: Option<Ident>,
}

impl Parse for Args {

    fn parse(input: ParseStream) -> Result<Self> {
        let mut item_imports_for_async = Vec::new();
        let mut item_imports_for_sync = Vec::new();

        while !input.is_empty() {
            if input.peek(Token![use]) {
                let i = parse_item_import(input)?;
                item_imports_for_async.extend(i.item_imports_for_async);
                item_imports_for_sync.extend(i.item_imports_for_sync);
            } 
            else {
                return Err(input.error("expected `use`"))
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
            else if input.peek(Token![;]) {
                input.parse::<Token![;]>()?;
            }
        }

        Ok(Self { item_imports_for_async, item_imports_for_sync })
    }
}


struct ItemImports {
    item_imports_for_async: Vec<ItemImport>,
    item_imports_for_sync: Vec<ItemImport>,
}

fn parse_item_import(input: ParseStream) -> Result<ItemImports> {
    input.parse::<Token![use]>()?;

    let mut available_for_async = true;
    let mut available_for_sync = true;

    if input.peek(token::Paren) {
        let content;
        parenthesized!(content in input);
        let attr = content.parse::<Ident>()?;

        if attr == "if_async" {
            available_for_sync = false;
        }
        else if attr == "if_sync" {
            available_for_async = false
        }
        else {
            return Err(Error::new(attr.span(), "expected none or one of: `if_async`, `if_sync`"))
        }
    }

    let is_absolute_paths = match input.peek(Token![::]) {
        true => {
            input.parse::<Token![::]>()?;
            true
        }
        false => false
    };
    let use_tree: UseTree = input.parse()?;
    let mut paths_and_aliases = Vec::new();

    let mut buf = VecDeque::new();
    buf.push_back((Vec::new(), use_tree));
    while let Some((mut parent, item)) = buf.pop_front() {
         match item {
            UseTree::Path(i) => {
                parent.push(i.ident);
                buf.push_back((parent, *i.tree));
            },
            UseTree::Group(i) => {
                for i in i.items {
                    buf.push_back((parent.clone(), i));
                }
            },
            UseTree::Name(i) => {
                parent.push(i.ident);
                paths_and_aliases.push((parent, None))
            },
            UseTree::Rename(i) => {
                parent.push(i.ident);
                paths_and_aliases.push((parent, Some(i.rename)))
            },
            UseTree::Glob(i) => return Err(Error::new(i.span(), "a glob import: `*` is not allowed")),
         }
    }

    let mut item_imports_for_async = Vec::new();
    let mut item_imports_for_sync = Vec::new();
    for (path, alias) in paths_and_aliases {
        let path = PathWithoutArgs::from_idents(path, is_absolute_paths);
        if available_for_async && available_for_sync {
            item_imports_for_async.push(ItemImport { path: path.clone(), alias: alias.clone()});
            item_imports_for_sync.push(ItemImport { path, alias });
        }
        else if available_for_async {
            item_imports_for_async.push(ItemImport { path, alias });
        }
        else if available_for_sync {
            item_imports_for_sync.push(ItemImport { path, alias });
        }
    }

    Ok(ItemImports { item_imports_for_async, item_imports_for_sync })
}