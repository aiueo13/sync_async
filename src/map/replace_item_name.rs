use crate::{ItemImport, PathWithoutArgs};
use std::borrow::Cow;
use syn::{
    punctuated::Punctuated, Field, Fields, FnArg, GenericArgument, GenericParam, Generics, Ident, 
    Macro, Pat, Path, PathArguments, PathSegment, ReturnType, Signature, Type, TypeParamBound, 
    WherePredicate
};


pub struct ReplaceItemNameFromTo<'a> {
    pub from: Cow<'a, Ident>,
    pub to: Cow<'a, PathWithoutArgs>,
}

pub fn replaces_from_item_imports<'a>(items: &'a [ItemImport]) -> Vec<ReplaceItemNameFromTo<'a>> {
    let mut buf = Vec::with_capacity(items.len());
    for item in items {
        let Some(from) = item.alias.as_ref().or_else(|| item.path.last_ident()) else {
            continue;
        };

        let from = Cow::Borrowed(from);
        let to = Cow::Borrowed(&item.path);
        buf.push(ReplaceItemNameFromTo { from, to });
    }
    buf
}


pub fn replace_item_name_in_signature<'a>(
    sig: &mut Signature, 
    replaces: &[ReplaceItemNameFromTo<'a>]
) {

    replace_item_name_in_return_type(&mut sig.output, replaces);
    replace_item_name_in_generics(&mut sig.generics, replaces);
    for input in &mut sig.inputs {
        replace_item_name_in_fn_arg(input, replaces);
    }
}

pub fn replace_item_name_in_fn_arg<'a>(
    arg: &mut FnArg, 
    replaces: &[ReplaceItemNameFromTo<'a>]
) {
    match arg {
        FnArg::Receiver(t) => replace_item_name_in_type(&mut t.ty, replaces),
        FnArg::Typed(t) => {
            replace_item_name_in_type(&mut t.ty, replaces);
            replace_item_name_in_pat(&mut t.pat, replaces);
        },
    }
}

pub fn replace_item_name_in_pat<'a>(
    pat: &mut Pat, 
    replaces: &[ReplaceItemNameFromTo<'a>]
) {

    match pat {
        Pat::Ident(t) => {
            if let Some(t) = &mut t.subpat {
                replace_item_name_in_pat(&mut *t.1, replaces);
            }
        },
        Pat::Macro(t) => replace_item_name_in_macro(&mut t.mac, replaces),
        Pat::Paren(t) => replace_item_name_in_pat(&mut t.pat, replaces),
        Pat::Path(t) => {
            if let Some(qself) = &mut t.qself {
                replace_item_name_in_type(&mut *qself.ty, replaces);
            }
            replace_item_name_in_path(&mut t.path, replaces)
        },
        Pat::Reference(t) => replace_item_name_in_pat(&mut t.pat, replaces),
        Pat::Slice(t) => {
            for elem in &mut t.elems {
                replace_item_name_in_pat(elem, replaces);
            }
        },
        Pat::Struct(t) => {
            if let Some(q) = &mut t.qself {
                replace_item_name_in_type(&mut q.ty, replaces);
            }
            for f in &mut t.fields {
                replace_item_name_in_pat(&mut f.pat, replaces);
            }
            replace_item_name_in_path(&mut t.path, replaces);
        },
        Pat::Tuple(t) => {
            for elem in &mut t.elems {
                replace_item_name_in_pat(elem, replaces);
            }
        },
        Pat::TupleStruct(t) => {
            if let Some(q) = &mut t.qself {
                replace_item_name_in_type(&mut q.ty, replaces);
            }
            for elem in &mut t.elems {
                replace_item_name_in_pat(elem, replaces);
            }
            replace_item_name_in_path(&mut t.path, replaces);
        },
        Pat::Type(t) => {
            replace_item_name_in_pat(&mut t.pat, replaces);
            replace_item_name_in_type(&mut t.ty, replaces);
        },
        Pat::Wild(_) => (),
        Pat::Or(_) => (),
        Pat::Range(_) => (),
        Pat::Rest(_) => (),
        Pat::Lit(_) => (),
        Pat::Const(_) => (),
        Pat::Verbatim(_) => (),
        _ => (),
    }
}

pub fn replace_item_name_in_fields<'a>(f: &mut Fields, replaces: &[ReplaceItemNameFromTo<'a>]) {
    match f {
        Fields::Named(f) => {
            for f in &mut f.named {
                 replace_item_name_in_field(f, replaces);
            }
        },
        Fields::Unnamed(f) => {
            for f in &mut f.unnamed {
                 replace_item_name_in_field(f, replaces);
            }
        },
        Fields::Unit => (),
    }
}

pub fn replace_item_name_in_macro<'a>(
    m: &mut Macro, 
    replaces: &[ReplaceItemNameFromTo<'a>]
) {

    replace_item_name_in_path(&mut m.path, replaces);
}

pub fn replace_item_name_in_generics<'a>(
    g: &mut Generics, 
    replaces: &[ReplaceItemNameFromTo<'a>]
) {

    if let Some(where_clause) = &mut g.where_clause {
        for wp in &mut where_clause.predicates {
            replace_item_name_in_where_predicates(wp, replaces);
        }
    }

    for param in &mut g.params {
        match param {
            GenericParam::Lifetime(_) => (),
            GenericParam::Const(t) => replace_item_name_in_type(&mut t.ty, replaces),
            GenericParam::Type(t) => {
                if let Some(default) = &mut t.default {
                    replace_item_name_in_type(default, replaces);
                }
                for bound in &mut t.bounds {
                    replace_item_name_in_type_param_bound(bound, replaces);
                }
            }
        }
    }
}

pub fn replace_item_name_in_type<'a>(ty: &mut Type, replaces: &[ReplaceItemNameFromTo<'a>]) {
    match ty {
        Type::Array(type_array) => replace_item_name_in_type(&mut *type_array.elem, replaces),
        Type::BareFn(type_bare_fn) => {
            for input in &mut type_bare_fn.inputs {
                replace_item_name_in_type(&mut input.ty, replaces);
            }
            replace_item_name_in_return_type(&mut type_bare_fn.output, replaces);
        }
        Type::Group(type_group) => replace_item_name_in_type(&mut type_group.elem, replaces),
        Type::ImplTrait(type_impl_trait) => {
            for bound in &mut type_impl_trait.bounds {
                replace_item_name_in_type_param_bound(bound, replaces);
            }
        }
        Type::Paren(type_paren) => replace_item_name_in_type(&mut type_paren.elem, replaces),
        Type::Path(type_path) => {
            if let Some(qself) = &mut type_path.qself {
                replace_item_name_in_type(&mut *qself.ty, replaces);
            }
            replace_item_name_in_path(&mut type_path.path, replaces)
        }
        Type::Ptr(type_ptr) => replace_item_name_in_type(&mut *type_ptr.elem, replaces),
        Type::Reference(type_reference) => {
            replace_item_name_in_type(&mut *type_reference.elem, replaces)
        }
        Type::Slice(type_slice) => replace_item_name_in_type(&mut *type_slice.elem, replaces),
        Type::TraitObject(type_trait_object) => {
            for bound in &mut type_trait_object.bounds {
                replace_item_name_in_type_param_bound(bound, replaces);
            }
        }
        Type::Tuple(type_tuple) => {
            for elem in &mut type_tuple.elems {
                replace_item_name_in_type(elem, replaces);
            }
        }
        Type::Macro(m) => replace_item_name_in_macro(&mut m.mac, replaces),
        Type::Infer(_) => (),
        Type::Never(_) => (),
        Type::Verbatim(_) => (),
        _ => (),
    }
}

pub fn replace_item_name_in_field<'a>(f: &mut Field, replaces: &[ReplaceItemNameFromTo<'a>]) {
    replace_item_name_in_type(&mut f.ty, replaces);
}

// (from, to)
// (std, s): std::io::Read -> s::io::Read
// (B, std::io::BufReader): B<T> -> std::io::BufReader<T>
pub fn replace_item_name_in_path<'a>(path: &mut Path, replaces: &[ReplaceItemNameFromTo<'a>]) {
    for seg in &mut path.segments {
        replace_item_name_in_path_arguments(&mut seg.arguments, replaces);
    }

    if let Some(root_seg) = path.segments.get_mut(0) {
        for ReplaceItemNameFromTo { from, to } in replaces.iter() {
            if from.as_ref() == &root_seg.ident {
                let mut seg_buf = Punctuated::new();
                for ident in &to.segments {
                    seg_buf.push(PathSegment {
                        ident: ident.clone(),
                        arguments: PathArguments::None,
                    });
                }
                seg_buf.last_mut().map(|s| {
                    s.ident.set_span(root_seg.ident.span());
                    s.arguments = root_seg.arguments.clone();
                });

                for seg in path.segments.iter().skip(1) {
                    seg_buf.push(seg.clone());
                }

                path.leading_colon = to.leading_colon;
                path.segments = seg_buf;
                return;
            }
        }
    }
}

pub fn replace_item_name_in_return_type<'a>(
    ty: &mut ReturnType,
    replaces: &[ReplaceItemNameFromTo<'a>]
) {

    match ty {
        ReturnType::Type(_, i) => replace_item_name_in_type(&mut *i, replaces),
        ReturnType::Default => (),
    }
}

pub fn replace_item_name_in_where_predicates<'a>(
    wp: &mut WherePredicate,
    replaces: &[ReplaceItemNameFromTo<'a>]
) {

    match wp {
        WherePredicate::Type(t) => {
            replace_item_name_in_type(&mut t.bounded_ty, replaces);
            for bound in &mut t.bounds {
                replace_item_name_in_type_param_bound(bound, replaces);
            }
        },
        WherePredicate::Lifetime(_) => (),
        _ => (),
    }
}

pub fn replace_item_name_in_type_param_bound<'a>(
    bound: &mut TypeParamBound,
    replaces: &[ReplaceItemNameFromTo<'a>],
) {

    match bound {
        TypeParamBound::Trait(trait_bound) => {
            replace_item_name_in_path(&mut trait_bound.path, replaces);
        }
        TypeParamBound::Lifetime(_) => (),
        TypeParamBound::PreciseCapture(_) => (),
        TypeParamBound::Verbatim(_) => (),
        _ => (),
    }
}

pub fn replace_item_name_in_generic_arguments<'a>(
    arg: &mut GenericArgument,
    replaces: &[ReplaceItemNameFromTo<'a>],
) {

    match arg {
        GenericArgument::Type(t) => replace_item_name_in_type(t, replaces),
        GenericArgument::AssocType(t) => {
            replace_item_name_in_type(&mut t.ty, replaces);
            if let Some(g) = &mut t.generics {
                for arg in &mut g.args {
                    replace_item_name_in_generic_arguments(arg, replaces);
                }
            }
        }
        GenericArgument::AssocConst(t) => {
            if let Some(g) = &mut t.generics {
                for arg in &mut g.args {
                    replace_item_name_in_generic_arguments(arg, replaces);
                }
            }
        },
        GenericArgument::Constraint(t) => {
            for bound in &mut t.bounds {
                replace_item_name_in_type_param_bound(bound, replaces);
            }
        },
        GenericArgument::Lifetime(_) => (),
        GenericArgument::Const(_) => (),
        _ => (),
    }
}

pub fn replace_item_name_in_path_arguments<'a>(
    args: &mut PathArguments,
    replaces: &[ReplaceItemNameFromTo<'a>],
) {
    match args {
        PathArguments::None => (),
        PathArguments::AngleBracketed(i) => {
            for arg in &mut i.args {
                replace_item_name_in_generic_arguments(arg, replaces);
            }
        },
        PathArguments::Parenthesized(i) => {
            for input in &mut i.inputs {
                replace_item_name_in_type(input, replaces);
            }
            replace_item_name_in_return_type(&mut i.output, replaces);
        }
    }
}
