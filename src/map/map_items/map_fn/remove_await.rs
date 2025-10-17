use syn::{Block, Expr, Local, Stmt};
use crate::map::map_items::map_fn::get_attrs::get_attrs_mut_from_expr;


pub fn remove_await_from_stmt(stmt: &mut Stmt) {
    match stmt {
        Stmt::Local(i) => remove_await_from_local(i),
        Stmt::Expr(i, _) => remove_await_from_expr(i),
        Stmt::Macro(_) => (),
        Stmt::Item(_) => (),
    }
}

pub fn remove_await_from_local(local: &mut Local) {
    if let Some(init) = &mut local.init {
        remove_await_from_expr(&mut init.expr);
    }
}

pub fn remove_await_from_expr(expr: &mut Expr) {
    match expr {
        Expr::Array(i) => {
            for e in &mut i.elems {
                remove_await_from_expr(e);
            }
        },
        Expr::Assign(i) => {
            remove_await_from_expr(&mut i.left);
            remove_await_from_expr(&mut i.right);
        },
        Expr::Async(i) => {
            remove_await_from_block(&mut i.block);
        },
        Expr::Await(_) => {
            remove_await_from_expr_await(expr);
        },
        Expr::Binary(i) => {
            remove_await_from_expr(&mut i.left);
            remove_await_from_expr(&mut i.right);
        },
        Expr::Block(i) => {
            remove_await_from_block(&mut i.block);
        },
        Expr::Break(i) => {
            if let Some(e) = &mut i.expr {
                remove_await_from_expr(e);
            }
        },
        Expr::Call(i) => {
            remove_await_from_expr(&mut i.func);
            for e in &mut i.args {
                remove_await_from_expr(e);
            }
        },
        Expr::Cast(i) => {
            remove_await_from_expr(&mut i.expr);
        },
        Expr::Closure(i) => {
            remove_await_from_expr(&mut i.body);
        },
        Expr::Const(i) => {
            remove_await_from_block(&mut i.block);
        },
        Expr::Field(i) => {
            remove_await_from_expr(&mut i.base);
        },
        Expr::ForLoop(i) => {
            remove_await_from_expr(&mut i.expr);
            remove_await_from_block(&mut i.body);
        },
        Expr::Group(i) => {
            remove_await_from_expr(&mut i.expr);
        },
        Expr::If(i) => {
            remove_await_from_block(&mut i.then_branch);
            remove_await_from_expr(&mut i.cond);
            if let Some((_, e)) = &mut i.else_branch {
                remove_await_from_expr(e);
            }
        },
        Expr::Index(i) => {
            remove_await_from_expr(&mut i.expr);
            remove_await_from_expr(&mut i.index);
        },
        Expr::Let(i) => {
            remove_await_from_expr(&mut i.expr);
        },
        Expr::Loop(i) => {
            remove_await_from_block(&mut i.body);
        },
        Expr::Match(i) => {
            remove_await_from_expr(&mut i.expr);
            for a in &mut i.arms {
                remove_await_from_expr(&mut a.body);
                if let Some((_, g)) = &mut a.guard {
                    remove_await_from_expr(g);
                }
            }
        },
        Expr::MethodCall(i) => {
            remove_await_from_expr(&mut i.receiver);
            for a in &mut i.args {
                remove_await_from_expr(a);
            }
        },
        Expr::Paren(i) => {
            remove_await_from_expr(&mut i.expr);
        },
        Expr::Range(i) => {
            if let Some(s) = &mut i.start {
                remove_await_from_expr(s);
            }
            if let Some(e) = &mut i.end {
                remove_await_from_expr(e);
            }
        },
        Expr::RawAddr(i) => {
            remove_await_from_expr(&mut i.expr);
        },
        Expr::Reference(i) => {
            remove_await_from_expr(&mut i.expr);
        },
        Expr::Repeat(i) => {
            remove_await_from_expr(&mut i.expr);
            remove_await_from_expr(&mut i.len);
        },
        Expr::Return(i) => {
            if let Some(e) = &mut i.expr {
                remove_await_from_expr(e);
            }
        },
        Expr::Struct(i) => {
            for f in &mut i.fields {
                remove_await_from_expr(&mut f.expr);
            }
            if let Some(r) = &mut i.rest {
                remove_await_from_expr(r);
            }
        },
        Expr::Try(i) => {
            remove_await_from_expr(&mut i.expr);
        },
        Expr::TryBlock(i) => {
            remove_await_from_block(&mut i.block);
        },
        Expr::Tuple(i) => {
            for e in &mut i.elems {
                remove_await_from_expr(e);
            }
        },
        Expr::Unary(i) => {
            remove_await_from_expr(&mut i.expr);
        },
        Expr::Unsafe(i) => {
            remove_await_from_block(&mut i.block);
        },
        Expr::While(i) => {
            remove_await_from_expr(&mut i.cond);
            remove_await_from_block(&mut i.body);
        },
        Expr::Yield(i) => {
            if let Some(e) = &mut i.expr {
                remove_await_from_expr(e);
            }
        },
        Expr::Verbatim(_) => (),
        Expr::Path(_) => (),
        Expr::Macro(_) => (),
        Expr::Lit(_) => (),
        Expr::Infer(_) => (),
        Expr::Continue(_) => (),
        _ => (),
    }
}

pub fn remove_await_from_block(block: &mut Block) {
    for stmt in &mut block.stmts {
        remove_await_from_stmt(stmt);
    }
}

pub fn remove_await_from_expr_await(expr: &mut Expr) {
    if let Expr::Await(await_expr) = expr {
        let d = Expr::Verbatim(Default::default());
        let mut base_expr = std::mem::replace(&mut *await_expr.base, d);
        if let Some(attrs) = get_attrs_mut_from_expr(&mut base_expr) {
            attrs.append(&mut await_expr.attrs);
        }

        *expr = base_expr;
        remove_await_from_expr(expr);
    }
}