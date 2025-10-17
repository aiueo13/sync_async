use std::ops::Range;
use syn::{Attribute, Expr, Lit, LitStr, Meta};
use crate::map::replace_item_name::ReplaceItemNameFromTo;


pub fn replace_item_name_in_doc<'a, 'b>(
    attrs: impl IntoIterator<Item = &'b mut Attribute>,
    replaces: &[ReplaceItemNameFromTo<'a>],
) {

    let mut buf = Vec::new();

    for attr in attrs {
        let Meta::NameValue(meta) = &mut attr.meta else { continue; };
        if !meta.path.is_ident("doc") { continue; }
        let Expr::Lit(lit) = &mut meta.value else { continue; };
        let Lit::Str(st) = &mut lit.lit else { continue; };

        assert!(buf.is_empty());

        let doc = st.value();
        find_type(&doc, &mut buf);
        if buf.is_empty() {
            continue;
        }

        let mut new_doc = doc.clone();

        // buf は start の大きい順に pop することでインデックスズレを防ぐ
        while let Some(i) = buf.pop() {
            let item_path = &doc[i.inner.clone()];
            let (item_name, rest) = item_path.split_once("::").unwrap_or((item_path, ""));
            
            // replaces に一致する場合に置換
            for replace in replaces {
                let from = replace.from.to_string();

                let f = from.split_once("::").map(|(s, _)| s).unwrap_or(&from);
                if f == item_name {
                    // [Type](SomeType)
                    // [`Type`][SomeType]
                    //
                    // (from, to) = (F, T)
                    // [Type](F) => [Type](T)
                    // [Type](F::A) => [Type](T::A)
                    if i.has_label {
                        let mut to = replace.to.to_string();
                        if !rest.is_empty() {
                            to.push_str("::");
                            to.push_str(rest);
                        }
                        new_doc.replace_range(i.inner.clone(), &to);
                    }
                    // [Type]
                    // [`Type`]
                    // 
                    // (from, to) = (F, T)
                    // [F] => [F](T)
                    // [`F()`] => [`F()`](`T()`)
                    else {
                        let prefix = &doc[i.outer.start..i.inner.start];
                        let suffix = &doc[i.inner.end..i.outer.end];
                        let mut to = String::new();
                        to.push('(');
                        to.push_str(prefix);
                        to.push_str(&replace.to.to_string());
                        if !rest.is_empty() {
                            to.push_str("::");
                            to.push_str(rest);
                        }
                        to.push_str(suffix);
                        to.push(')');
                        
                        // outer.end + 1 の位置には ] が必ず存在する
                        let t = i.outer.end + 1;
                        new_doc.replace_range(t..t, &to);
                    }

                    break;
                }
            }
        }

        // 新しい文字列で Literal を更新
        *st = LitStr::new(&new_doc, st.span());
    }
}

struct ItemInDoc {
    outer: Range<usize>,
    inner: Range<usize>,
    has_label: bool
}

fn find_type(
    text: &str,
    buffer: &mut Vec<ItemInDoc>
) {

    let bytes = text.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'[' {
            let start_label = i + 1;
            let Some(end_label) = find_closing(bytes, start_label, b'[', b']') else {
                return
            };
            i = end_label + 1;

            // target の確認
            let (range, has_label) = if i < bytes.len() && (bytes[i] == b'(' || bytes[i] == b'[') {
                let open = bytes[i];
                let close = if open == b'(' { b')' } else { b']' };
                let start_target = i + 1;
                let Some(end_target) = find_closing(bytes, start_target, open, close) else {
                    return
                };
                i = end_target + 1;
                (Range { start: start_target, end: end_target }, true)
            }
            else {
                // 単独の [] の場合は label を target として扱う
                (Range { start: start_label, end: end_label }, false)
            };

            let trimmed = trim_range(text, range.clone());
            buffer.push(ItemInDoc { 
                outer: range , 
                inner: trimmed, 
                has_label
            });
        }
        else {
            i += 1;
        }
    }
}

fn find_closing(bytes: &[u8], start: usize, open: u8, close: u8) -> Option<usize> {
    let mut depth = 1;
    let mut j = start;
    while j < bytes.len() && depth > 0 {
        if bytes[j] == open {
            depth += 1;
        } 
        else if bytes[j] == close {
            depth -= 1;
        }
        j += 1;
    }
    if depth == 0 {
        Some(j - 1)
    } 
    else {
        None
    }
}

fn trim_range(text: &str, range: Range<usize>) -> Range<usize> {
    let Range { start, end } = range;
    let s = &text[start..end];

    // 先頭の ` や空白をスキップ
    let mut offset_start = 0;
    while offset_start < s.len() && (s.as_bytes()[offset_start] == b'`' || s.as_bytes()[offset_start].is_ascii_whitespace()) {
        offset_start += 1;
    }

    // a@b であれば b にする
    if let Some(pos) = &s[offset_start..].find('@') {
        offset_start += pos + 1;
    }

    // 末尾の ` や () !() を削除
    let mut offset_end = s.len();
    while offset_end > offset_start && s.as_bytes()[offset_end - 1] == b'`' {
        offset_end -= 1;
    }

    // 末尾が () または !(), !{}, ![] の場合は削る
    if s[offset_start..offset_end].ends_with("()") {
        offset_end -= 2;
    }
    if s[offset_start..offset_end].ends_with("{}") {
        offset_end -= 2;
    }
    if s[offset_start..offset_end].ends_with("[]") {
        offset_end -= 2;
    } 
    if s[offset_start..offset_end].ends_with("!") {
        offset_end -= 1;
    }

    Range {
        start: start + offset_start,
        end: start + offset_end,
    }
}