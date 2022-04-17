use proc_macro2::{Ident, Punct, Spacing, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use std::cmp::Ordering;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Token, Type};

pub enum TupDefault {
    None,
    Unfinished(Expr),
    Finished(Ident),
}

#[derive(Educe)]
#[educe(PartialEq, Eq, PartialOrd, Ord)]
pub struct TupElement {
    #[educe(Ord(method = "cmp"))]
    pub name: Ident,
    #[educe(Ord(ignore), PartialOrd(ignore), Eq(ignore), PartialEq(ignore))]
    pub value: Option<Expr>,
}

#[derive(Educe)]
#[educe(PartialEq, Eq, PartialOrd, Ord)]
pub struct TupType {
    #[educe(Ord(method = "cmp"))]
    pub name: Ident,
    #[educe(Ord(ignore), PartialOrd(ignore), Eq(ignore), PartialEq(ignore))]
    pub value: Type,
    #[educe(Ord(ignore), PartialOrd(ignore), Eq(ignore), PartialEq(ignore))]
    pub default: TupDefault,
}

fn cmp(a: &Ident, b: &Ident) -> Ordering {
    let a = a.to_string();
    let b = b.to_string();
    if a > b {
        Ordering::Less
    } else if a < b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

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
