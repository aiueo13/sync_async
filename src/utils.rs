use crate::ItemImport;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::Parse,
    punctuated::Punctuated,
    token::{self, PathSep},
    Ident, ItemUse, Path, PathArguments, PathSegment, Token, UseName, UsePath, UseRename, UseTree,
};


pub fn item_use_from_item_import(item_import: &ItemImport) -> ItemUse {
    let use_tree = {
        let mut iter = item_import.path.segments.iter().rev();

        let top_ident = iter.next().unwrap().clone();
        let mut tree = match &item_import.alias {
            None => UseTree::Name(UseName { ident: top_ident }),
            Some(alias) => UseTree::Rename(UseRename {
                ident: top_ident,
                as_token: Default::default(),
                rename: alias.clone(),
            })
        };

        for ident in iter {
            tree = UseTree::Path(UsePath {
                ident: ident.clone(),
                colon2_token: Default::default(),
                tree: Box::new(tree),
            });
        }

        tree
    };

    ItemUse {
        attrs: Vec::new(),
        vis: syn::Visibility::Inherited,
        use_token: Default::default(),
        leading_colon: item_import.path.leading_colon.clone(),
        tree: use_tree,
        semi_token: Default::default(),
    }
}

pub fn item_uses_from_item_imports(item_imports: &[ItemImport]) -> Vec<ItemUse> {
    let mut buf = Vec::with_capacity(item_imports.len());
    for item_import in item_imports {
        buf.push(item_use_from_item_import(item_import));
    }
    buf
}

/// Arguments (std::io::BufReader<'a, T> の 'a, T など) がないパス
#[derive(Clone)]
pub struct PathWithoutArgs {
    pub leading_colon: Option<Token![::]>,
    pub segments: Punctuated<Ident, Token![::]>,
}

impl PathWithoutArgs {

    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        if self.leading_colon.is_some() {
            buf.push_str("::");
        }
        for (i, seg) in self.segments.iter().enumerate() {
            if i != 0 {
                buf.push_str("::");
            }
            buf.push_str(&seg.to_string());
        }
        buf
    }

    pub fn from_idents(idents: Vec<Ident>, is_absolute: bool) -> PathWithoutArgs {
        let mut segments = Punctuated::new();

        for ident in idents {
            segments.push(ident);
        }

        PathWithoutArgs {
            leading_colon: match is_absolute {
                true => Some(<token::PathSep>::default()),
                false => None,
            },
            segments,
        }
    }

    pub fn last_ident(&self) -> Option<&Ident> {
        self.segments.last()
    }
}

impl From<PathWithoutArgs> for Path {
    fn from(value: PathWithoutArgs) -> Self {
        let leading_colon = value.leading_colon;
        let segments: Punctuated<PathSegment, PathSep> = value
            .segments
            .into_iter()
            .map(|ident| PathSegment {
                ident,
                arguments: PathArguments::None,
            })
            .collect();

        Path {
            leading_colon,
            segments,
        }
    }
}

impl PartialEq<Path> for PathWithoutArgs {
    fn eq(&self, other: &Path) -> bool {
        if self.leading_colon.is_some() != other.leading_colon.is_some() {
            return false;
        }
        if self.segments.len() != other.segments.len() {
            return false;
        }

        self.segments
            .iter()
            .zip(other.segments.iter())
            .all(|(self_ident, other_seg)| self_ident == &other_seg.ident)
    }
}

impl Parse for PathWithoutArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let leading_colon = match input.peek(Token![::]) {
            false => None,
            true => Some(input.parse::<Token![::]>()?),
        };
        let mut segments = Punctuated::<Ident, Token![::]>::new();

        // 少なくとも1つは Ident が必要
        let first: Ident = input.parse()?;
        segments.push(first);

        while input.peek(Token![::]) {
            let _colon: Token![::] = input.parse()?;
            let ident: Ident = input.parse()?;
            segments.push(ident);
        }

        Ok(PathWithoutArgs {
            leading_colon,
            segments,
        })
    }
}

impl ToTokens for PathWithoutArgs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(colon2) = &self.leading_colon {
            colon2.to_tokens(tokens);
        }
        let mut first = true;
        for seg in &self.segments {
            if !first {
                Token![::](seg.span()).to_tokens(tokens);
            }
            seg.to_tokens(tokens);
            first = false;
        }
    }

    fn to_token_stream(&self) -> TokenStream {
        let mut ts = TokenStream::new();
        self.to_tokens(&mut ts);
        ts
    }

    fn into_token_stream(self) -> TokenStream {
        let mut ts = TokenStream::new();
        self.to_tokens(&mut ts);
        ts
    }
}
