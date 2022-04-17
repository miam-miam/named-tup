use std::cmp::Ordering;

use proc_macro2::{Ident, Punct, Spacing, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Token, Type};

#[allow(clippy::large_enum_variant)]
pub enum TupDefault {
    None,
    Unfinished(Expr),
    Finished(Ident),
}

pub struct TupElement {
    pub name: Ident,
    pub value: Option<Expr>,
}

impl Ord for TupElement {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for TupElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TupElement {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for TupElement {}

pub struct TupType {
    pub name: Ident,
    pub value: Type,
    pub default: TupDefault,
}

impl Ord for TupType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for TupType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TupType {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for TupType {}

impl Parse for TupElement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let value = match input.peek(Token![:]) {
            true => {
                input.parse::<Token![:]>()?;
                Some(input.parse()?)
            }
            false => None,
        };
        Ok(TupElement { name, value })
    }
}

impl Parse for TupType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![:]>()?;
        let value = input.parse()?;
        let default = match input.peek(Token![=]) {
            true => {
                input.parse::<Token![=]>()?;
                match input.peek(Token![=]) {
                    true => {
                        input.parse::<Token![=]>()?;
                        TupDefault::Finished(input.parse::<Ident>()?)
                    }
                    false => TupDefault::Unfinished(input.parse::<Expr>()?),
                }
            }
            false => TupDefault::None,
        };
        Ok(TupType {
            name,
            value,
            default,
        })
    }
}

impl ToTokens for TupType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.name.to_tokens(tokens);
        tokens.append(Punct::new(':', Spacing::Alone));
        self.value.to_tokens(tokens);
        match &self.default {
            TupDefault::Unfinished(expr) => {
                tokens.append(Punct::new('=', Spacing::Alone));
                expr.to_tokens(tokens)
            }
            TupDefault::Finished(ident) => {
                tokens.append(Punct::new('=', Spacing::Joint));
                tokens.append(Punct::new('=', Spacing::Alone));
                ident.to_tokens(tokens)
            }
            TupDefault::None => {}
        }
    }
}
