use proc_macro::TokenStream as Tok1;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Ident, Lit, LitBool, LitFloat, LitInt, LitStr, Token, braced, bracketed,
    parse::Parse,
    parse_macro_input,
    punctuated::Punctuated,
    token::{self, Brace, Bracket},
};

enum JsonValue {
    Null,
    Bool(LitBool),
    Number(LitInt),
    Float(LitFloat),
    String(LitStr),
    Array(Vec<JsonValue>),
    Object(Vec<(LitStr, JsonValue)>),
}

struct KeyValue {
    k: LitStr,
    v: JsonValue,
}

impl Parse for KeyValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let k: LitStr = input.parse()?;
        input.parse::<Token![:]>()?;
        let v: JsonValue = input.parse()?;
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
        } else if input.peek(Ident) {
            let ident = input.parse::<Ident>()?;
            if ident == "null" {
                Ok(JsonValue::Null)
            } else {
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
            JsonValue::Null => tokens.extend(quote!(Value::Null)),
            JsonValue::Bool(b) => tokens.extend(quote!(Value::Bool(#b))), // 直接使用布尔字面量
            JsonValue::Number(n) => {
                // 将整数转换为f64以统一数值表示（JSON数字支持浮点）
                tokens.extend(quote!(Value::Number(#n as f64)))
            }
            JsonValue::Float(f) => tokens.extend(quote!(Value::Number(#f))), // 浮点直接使用
            JsonValue::String(s) => tokens.extend(quote!(Value::String(#s.to_string()))), // 转换为String类型
            JsonValue::Array(arr) => {
                let elements = arr.iter().map(|e| {
                    let mut t = TokenStream::new();
                    e.to_tokens(&mut t);
                    t
                });
                tokens.extend(quote!(Value::Array(vec![#( #elements ),*])))
            }
            JsonValue::Object(obj) => {
                let entries = obj.iter().map(|(k, v)| {
                    let mut key_tok = TokenStream::new();
                    k.to_tokens(&mut key_tok); // 生成字符串字面量
                    let mut val_tok = TokenStream::new();
                    v.to_tokens(&mut val_tok);
                    quote!((#key_tok.to_string(), #val_tok)) // 键值对转换为String和Value
                });
                tokens.extend(quote!(Value::Object(HashMap::from([#( #entries ),*]))));
            }
        }
    }
}

#[proc_macro]
pub fn json(input: Tok1) -> Tok1 {
    let json_value = parse_macro_input!(input as JsonValue);
    let mut tokens = TokenStream::new();
    json_value.to_tokens(&mut tokens);
    tokens.into()
}

mod test;
