use proc_macro2::TokenStream;
use syn::{
    braced, bracketed, parse::Parse, parse_macro_input, punctuated::Punctuated, token::{self, Brace, Bracket}, Ident, Lit, LitBool, LitFloat, LitInt, LitStr, Token
};
use proc_macro::TokenStream as Tok1;
use quote::{quote, ToTokens};

enum JsonValue {
    Null,
    Bool(LitBool),
    Number(LitInt),
    Float(LitFloat),
    String(LitStr),
    Array(Vec<JsonValue>),
    Object(Vec<(LitStr, JsonValue)>),
}

struct KeyValue{
    k: LitStr,
    v: JsonValue,
}

impl Parse for KeyValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let k:LitStr = input.parse()?;
        input.parse::<Token![:]>()?;
        let v:JsonValue = input.parse()?;
        Ok(KeyValue { k, v })
    }
}

impl Parse for JsonValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(JsonValue::Null);
        } else if input.peek(Brace) {
            let content;
            braced!(content in input);
            let fields = Punctuated::<KeyValue, Token![,]>::parse_terminated(&content)?;
            Ok(JsonValue::Object(
                fields.into_iter().map(|kv| (kv.k, kv.v)).collect(),
            ))
        } else if input.peek(Bracket) {
            let content;
            bracketed!(content in input);
            let arr = Punctuated::<JsonValue, token::Comma>::parse_terminated(&content)?;
            Ok(JsonValue::Array(arr.into_iter().collect()))
        } else if input.peek(LitStr) {
            Ok(JsonValue::String(input.parse()?))
        } else if input.peek(LitInt) {
            Ok(JsonValue::Number(input.parse()?))
        } else if input.peek(LitFloat) {
            Ok(JsonValue::Float(input.parse()?))
        } else if input.peek(LitBool) {
            Ok(JsonValue::Bool(input.parse()?))
        } else if input.peek(Ident){
            let ident = input.parse::<Ident>()?;
            if ident == "null" {
                Ok(JsonValue::Null)
            }else{
                Err(input.error("not json"))
            }
        } else {
            return Err(input.error("not json"));
        } 
    }
}

impl ToTokens for JsonValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            JsonValue::Null => tokens.extend(quote!(JsonValue::Null)),
            JsonValue::Bool(b) => b.to_tokens(tokens),
            JsonValue::Number(n) => n.to_tokens(tokens),
            JsonValue::Float(f) => f.to_tokens(tokens),
            JsonValue::String(s) => s.to_tokens(tokens),
            JsonValue::Array(arr) => {
                let elements = arr.iter().map(|e| {
                    let mut t = TokenStream::new();
                    e.to_tokens(&mut t);
                    t
                });
                tokens.extend(quote!([#( #elements ),*]))
            }
            JsonValue::Object(obj) => {
                let entries = obj.iter().map(|(k, v)| {
                    let mut t = TokenStream::new();
                    k.to_tokens(&mut t);
                    t.extend(quote!(:));
                    v.to_tokens(&mut t);
                    t
                });
                tokens.extend(quote!({ #( #entries ),* }))
            }
        }
    }
}

#[proc_macro]
pub fn json(input: Tok1) -> Tok1 {
    let i = input.clone();
    let json_value = parse_macro_input!(input as JsonValue);
    i
}


mod test;