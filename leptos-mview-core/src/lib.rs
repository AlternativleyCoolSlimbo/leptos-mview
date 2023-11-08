#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::option_if_let_else,
    clippy::or_fun_call,
    clippy::module_name_repetitions
)]

mod attribute;
mod children;
mod element;
mod error_ext;
mod expand;
mod ident;
mod kw;
mod parse;
mod span;
mod tag;
mod value;

use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::spanned::Spanned;

use crate::{children::Children, expand::children_fragment_tokens};

#[must_use]
pub fn mview_impl(input: TokenStream) -> TokenStream {
    // return () in case of any errors, to avoid "unexpected end of macro
    // invocation" e.g. when assigning `let res = mview! { ... };`
    proc_macro_error::set_dummy(quote! { () });

    let children = match syn::parse2::<Children>(input) {
        Ok(tree) => tree,
        Err(e) => return e.to_compile_error(),
    };
    // If there's a single top level component, can just expand like
    // div().attr(...).child(...)...
    // If there are multiple top-level children, need to use the fragment.
    if children.len() == 1 {
        let child = children.into_vec().remove(0);
        quote! {
            {
                #[allow(unused_braces)]
                #child
            }
        }
    } else {
        // look for any slots
        if let Some(slot) = children.slot_children().next() {
            abort!(
                slot.slot_token().span(),
                "slots should be inside a parent that supports slots"
            );
        };

        let fragment = children_fragment_tokens(children.element_children());
        quote! {
            {
                #[allow(unused_braces)]
                #fragment
            }
        }
    }
}
