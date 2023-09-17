//! A collection of structs and functions for parsing attributes.

use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream},
    token::{Brace, CustomToken},
    Token,
};

use crate::{error_ext::ResultExt, ident::KebabIdent, value::Value};

/// Parsing function for attributes that can accept:
/// - Normal `key={value}` pairs
/// - Shorthand attributes like `{class}` to `class={class}`
/// - The above can also be kebab-case idents.
///
/// For use with `on` directives and key-value attributes.
pub fn parse_braced_bool(input: ParseStream) -> syn::Result<(KebabIdent, Value)> {
    if input.peek(syn::token::Brace) {
        let braced_ident = input.parse::<BracedKebabIdent>()?;
        Ok((
            braced_ident.ident().clone(),
            braced_ident.into_block_value(),
        ))
    } else {
        let fork = input.fork();
        let ident = fork.parse::<KebabIdent>()?;
        fork.parse::<Token![=]>()?;
        let value = fork.parse::<Value>()?;
        input.advance_to(&fork);
        Ok((ident, value))
    }
}

/// Parsing function for attributes that can accept:
/// - Normal `key={value}` pairs
/// - Keys that are str literals `"something"={value}`
/// - Shorthand attributes like `{class}` to `class={class}`.
/// - The above can also be kebab-case idents.
///
/// # Errors
/// Returns `Err`s if the input cannot be parsed. Does not advance the
/// token stream if so.
pub fn parse_str_braced(input: ParseStream) -> syn::Result<(syn::LitStr, Value)> {
    // either a shorthand `{class}` or key-value pair `class={class}`.
    if input.peek(syn::token::Brace) {
        let braced_ident = input.parse::<BracedKebabIdent>()?;
        Ok((
            braced_ident.ident().to_lit_str(),
            braced_ident.into_block_value(),
        ))
    } else {
        let fork = input.fork();
        let class = fork.parse::<KebabIdentOrStr>()?.into_lit_str();
        fork.parse::<Token![=]>()?;
        let value = fork.parse::<Value>()?;
        input.advance_to(&fork);
        Ok((class, value))
    }
}

/// Parsing functions for attributes that can accept:
/// - Normal `key={value}` pairs
/// - Shorthand attributes like `{class}` to `class={class}`
/// - All idents must be a regular ident, cannot be a keyword.
///
/// # Errors
/// Returns `Err`s if the input cannot be parsed. Does not advance the
/// token stream if so.
pub fn parse_ident_braced(input: ParseStream) -> syn::Result<(syn::Ident, Value)> {
    if input.peek(syn::token::Brace) {
        // TODO: give these better errors
        let ident = input.parse::<BracedIdent>().unwrap_or_abort();
        Ok((ident.ident().clone(), ident.into_block_value()))
    } else {
        let ident = input.parse::<syn::Ident>().unwrap_or_abort();
        input.parse::<Token![=]>().unwrap_or_abort();
        let value = input.parse::<Value>().unwrap_or_abort();
        Ok((ident, value))
    }
}

/// Generic parsing function for directives.
///
/// Tries the parse the `Kw` and colon, then parses the `next` function.
///
/// # Aborts
/// An `Err` is returned if the keyword is not found or a colon is not found
/// after the keyword. Otherwise, this function will abort.
///
/// Input stream will not be advanced if unable to parse.
pub fn parse_dir_then<Kw: CustomToken + Parse, R>(
    input: ParseStream,
    next: fn(ParseStream) -> syn::Result<R>,
) -> syn::Result<(Kw, R)> {
    if !input.peek2(Token![:]) {
        return Err(input.error("expected colon after directive"));
    }

    let dir = input.parse::<Kw>()?; // should not advance if no match
    input.parse::<Token![:]>().expect("peeked for token");
    Ok((dir, next(input).unwrap_or_abort()))
}

// Parse either a kebab-case ident or a str literal.
pub enum KebabIdentOrStr {
    KebabIdent(KebabIdent),
    Str(syn::LitStr),
}

impl KebabIdentOrStr {
    pub fn into_lit_str(self) -> syn::LitStr {
        match self {
            Self::KebabIdent(ident) => ident.to_lit_str(),
            Self::Str(s) => s,
        }
    }
}

impl Parse for KebabIdentOrStr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Ok(str) = input.parse::<syn::LitStr>() {
            Ok(Self::Str(str))
        } else {
            Ok(Self::KebabIdent(input.parse()?))
        }
    }
}

/// Parses a braced kebab-cased ident like `{abc-123}`
///
/// Does not advance the token stream if it cannot be parsed.
pub struct BracedKebabIdent {
    brace_token: Brace,
    ident: KebabIdent,
}

impl BracedKebabIdent {
    pub fn new(brace: Brace, ident: KebabIdent) -> Self {
        Self {
            brace_token: brace,
            ident,
        }
    }

    pub fn ident(&self) -> &KebabIdent {
        &self.ident
    }

    pub fn into_block_value(self) -> Value {
        let ident = self.ident();
        Value::Block(syn::parse_quote_spanned!(self.brace_token.span=> {#ident}))
    }
}

impl Parse for BracedKebabIdent {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let (brace, ident) = parse_braced::<KebabIdent>(input)?;
        Ok(Self::new(brace, ident))
    }
}

/// Parses a braced ident like `{abc_123}`
///
/// Does not advance the token stream if it cannot be parsed.
///
/// Does not parse kebab-case identifiers - see [`BracedKebabIdent`] instead.
pub struct BracedIdent {
    brace_token: Brace,
    ident: syn::Ident,
}

impl BracedIdent {
    pub fn new(brace: Brace, ident: syn::Ident) -> Self {
        Self {
            brace_token: brace,
            ident,
        }
    }

    pub fn ident(&self) -> &syn::Ident {
        &self.ident
    }

    pub fn into_block_value(self) -> Value {
        let ident = self.ident();
        Value::Block(syn::parse_quote_spanned!(self.brace_token.span=> {#ident}))
    }
}

impl Parse for BracedIdent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (brace, ident) = parse_braced::<syn::Ident>(input)?;
        Ok(Self::new(brace, ident))
    }
}

/// Parses an AST wrapped in braces.
/// Does not advance the token stream if unable to match.
fn parse_braced<T: syn::parse::Parse>(input: ParseStream) -> syn::Result<(Brace, T)> {
    let fork = input.fork();
    if fork.peek(Brace) {
        let inner;
        let brace_token = syn::braced!(inner in fork);
        let ast = inner.parse::<T>()?;
        input.advance_to(&fork);
        Ok((brace_token, ast))
    } else {
        Err(input.error("no brace found"))
    }
}
