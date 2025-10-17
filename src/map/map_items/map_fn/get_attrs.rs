use syn::{Attribute, Expr, Item, Stmt};

pub fn get_attrs_mut_from_stmt(stmt: &mut Stmt) -> Option<&mut Vec<Attribute>> {
    match stmt {
        Stmt::Local(local) => Some(&mut local.attrs),
        Stmt::Item(item) => get_attrs_mut_from_item(item),
        Stmt::Expr(expr, _) => get_attrs_mut_from_expr(expr),
        Stmt::Macro(stmt_macro) => Some(&mut stmt_macro.attrs),
    }
}

pub fn get_attrs_mut_from_item(item: &mut Item) -> Option<&mut Vec<Attribute>> {
    match item {
        Item::Const(i) => Some(&mut i.attrs),
        Item::Enum(i) => Some(&mut i.attrs),
        Item::ExternCrate(i) => Some(&mut i.attrs),
        Item::Fn(i) => Some(&mut i.attrs),
        Item::ForeignMod(i) => Some(&mut i.attrs),
        Item::Impl(i) => Some(&mut i.attrs),
        Item::Macro(i) => Some(&mut i.attrs),
        Item::Mod(i) => Some(&mut i.attrs),
        Item::Static(i) => Some(&mut i.attrs),
        Item::Struct(i) => Some(&mut i.attrs),
        Item::Trait(i) => Some(&mut i.attrs),
        Item::TraitAlias(i) => Some(&mut i.attrs),
        Item::Type(i) => Some(&mut i.attrs),
        Item::Union(i) => Some(&mut i.attrs),
        Item::Use(i) => Some(&mut i.attrs),
        Item::Verbatim(_) => None,
        _ => None,
    }
}

pub fn get_attrs_mut_from_expr(expr: &mut Expr) -> Option<&mut Vec<Attribute>> {
    match expr {
        Expr::Array(e) => Some(&mut e.attrs),
        Expr::Assign(e) => Some(&mut e.attrs),
        Expr::Async(e) => Some(&mut e.attrs),
        Expr::Await(e) => Some(&mut e.attrs),
        Expr::Binary(e) => Some(&mut e.attrs),
        Expr::Block(e) => Some(&mut e.attrs),
        Expr::Break(e) => Some(&mut e.attrs),
        Expr::Call(e) => Some(&mut e.attrs),
        Expr::Cast(e) => Some(&mut e.attrs),
        Expr::Closure(e) => Some(&mut e.attrs),
        Expr::Const(e) => Some(&mut e.attrs),
        Expr::Continue(e) => Some(&mut e.attrs),
        Expr::Field(e) => Some(&mut e.attrs),
        Expr::ForLoop(e) => Some(&mut e.attrs),
        Expr::Group(e) => Some(&mut e.attrs),
        Expr::If(e) => Some(&mut e.attrs),
        Expr::Index(e) => Some(&mut e.attrs),
        Expr::Infer(e) => Some(&mut e.attrs),
        Expr::Let(e) => Some(&mut e.attrs),
        Expr::Lit(e) => Some(&mut e.attrs),
        Expr::Loop(e) => Some(&mut e.attrs),
        Expr::Macro(e) => Some(&mut e.attrs),
        Expr::Match(e) => Some(&mut e.attrs),
        Expr::MethodCall(e) => Some(&mut e.attrs),
        Expr::Paren(e) => Some(&mut e.attrs),
        Expr::Path(e) => Some(&mut e.attrs),
        Expr::Range(e) => Some(&mut e.attrs),
        Expr::RawAddr(e) => Some(&mut e.attrs),
        Expr::Reference(e) => Some(&mut e.attrs),
        Expr::Repeat(e) => Some(&mut e.attrs),
        Expr::Return(e) => Some(&mut e.attrs),
        Expr::Struct(e) => Some(&mut e.attrs),
        Expr::Try(e) => Some(&mut e.attrs),
        Expr::TryBlock(e) => Some(&mut e.attrs),
        Expr::Tuple(e) => Some(&mut e.attrs),
        Expr::Unary(e) => Some(&mut e.attrs),
        Expr::Unsafe(e) => Some(&mut e.attrs),
        Expr::Verbatim(_) => None,
        Expr::While(e) => Some(&mut e.attrs),
        Expr::Yield(e) => Some(&mut e.attrs),
        _ => None,
    }
}