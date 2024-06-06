use syn::Attribute;

pub fn get_title_and_desc_from_doc(attrs: &[Attribute]) -> (Option<String>, Option<String>) {
    let doc = match get_doc(attrs) {
        None => return (None, None),
        Some(doc) => doc,
    };

    if doc.starts_with('#') {
        let mut split = doc.splitn(2, '\n');
        let title = split.next().unwrap().trim_start_matches('#').trim();
        let maybe_desc = split.next().and_then(none_if_empty);
        (none_if_empty(title), maybe_desc)
    } else {
        (None, none_if_empty(&doc))
    }
}

fn get_doc(attrs: &[Attribute]) -> Option<String> {
    let description = attrs
        .iter()
        .filter_map(|attr| {
            if !attr.path().is_ident("doc") {
                return None;
            }

            let meta = attr.meta.require_name_value().ok()?;
            if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(lit_str),
                ..
            }) = &meta.value
            {
                return Some(lit_str.value());
            }

            None
        })
        .collect::<Vec<_>>()
        .join("\n");

    none_if_empty(description.trim())
}

fn none_if_empty(s: &str) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s.into())
    }
}
