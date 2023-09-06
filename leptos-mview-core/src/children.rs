use core::slice;

use proc_macro_error::abort;
use quote::ToTokens;
use syn::parse::Parse;

use crate::{element::Element, value::Value};

/// Possible child nodes inside a component.
///
/// If the child is a `Value::Lit`, this lit must be a string.
/// Parsing will abort if the lit is not a string.
#[derive(Debug)]
pub enum Child {
    Value(Value),
    Element(Element),
}

impl Parse for Child {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Ok(value) = input.parse::<Value>() {
            // only allow literals if they are a string.
            if let Value::Lit(ref lit) = value {
                if let syn::Lit::Str(_) = lit {
                    Ok(Self::Value(value))
                } else {
                    abort!(lit.span(), "only string literals are allowed in children");
                }
            } else {
                Ok(Self::Value(value))
            }
        } else if let Ok(elem) = input.parse::<Element>() {
            Ok(Self::Element(elem))
        } else {
            Err(input.error("invalid child"))
        }
    }
}

impl ToTokens for Child {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Value(v) => tokens.extend(v.into_token_stream()),
            Self::Element(e) => tokens.extend(e.into_token_stream()),
        }
    }
}

/// A space-separated series of children.
///
/// Parsing does not include the surrounding braces.
/// If no children are present, an empty vector will be stored.
///
/// There are two ways of passing children, so no `ToTokens` implementation
/// is provided. Use `to_child_methods` or `to_fragment` instead.
#[derive(Debug)]
pub struct Children(Vec<Child>);

impl Children {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn into_vec(self) -> Vec<Child> {
        self.0
    }

    pub fn iter(&self) -> slice::Iter<'_, Child> {
        self.0.iter()
    }
}

impl Parse for Children {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut vec = Vec::new();
        while let Ok(child) = input.parse::<Child>() {
            vec.push(child);
        }
        Ok(Self(vec))
    }
}
