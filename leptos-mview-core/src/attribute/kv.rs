use syn::parse::Parse;

use crate::{ident::KebabIdent, value::Value};

use super::parsing::parse_braced_bool;

/// A `key = value` type of attribute.
///
/// This can either be a normal `key = value`, a shorthand `{key}`, or a
/// boolean attribute `checked`.
///
/// # Examples
/// ```ignore
/// input type="checkbox" data-index=1 checked;
///       ^^^^^^^^^^^^^^^ ^^^^^^^^^^^^ ^^^^^^^
/// ```
/// Directives are not included.
/// ```ignore
/// input on:input={handle_input} type="text";
///       ^^^not included^^^^^^^^ ^included^^
/// ```
///
/// # Parsing
/// If parsing fails, the input `ParseStream` will not be advanced.
///
/// If an identifier and equal sign is found but no value after,
/// the macro will abort.
#[derive(Debug, Clone)]
pub struct KvAttr {
    key: KebabIdent,
    value: Value,
}

impl KvAttr {
    pub const fn new(key: KebabIdent, value: Value) -> Self {
        Self { key, value }
    }

    pub const fn key(&self) -> &KebabIdent {
        &self.key
    }

    pub const fn value(&self) -> &Value {
        &self.value
    }
}

impl Parse for KvAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let (key, value) = parse_braced_bool(input)?;
        Ok(Self::new(key, value))
    }
}
