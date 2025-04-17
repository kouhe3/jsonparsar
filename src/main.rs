fn main() {
    let input = r#"
    {
        "books": [
            {
                "title": "The Rust Programming Language",
                "author": "Steve Klabnik",
                "details": {
                    "isbn": "978-1718500440",
                    "pages": 480,
                    "categories": ["systems", "programming"]
                }
            },
            {
                "title": "JSON at Work",
                "author": null,
                "details": {
                    "isbn": null,
                    "pages": 200,
                    "categories": []
                }
            }
        ],
        "stats": {
            "total_books": 2,
            "has_missing_data": true
        }
    }
    "#;
    let json: JsonValue = parse_str(input).unwrap();
    let json_str = json.to_json_str();
    println!("{}", json_str)
}



use syn::{
    braced, bracketed, parse::Parse, parse_macro_input, parse_str, punctuated::Punctuated, token::{self, Brace, Bracket}, Ident, Lit, LitBool, LitFloat, LitInt, LitStr, Token
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

impl JsonValue {
    fn to_json_str(&self) -> String {
         match self {
            JsonValue::Null => "null".to_string(),
            JsonValue::Bool(b) => b.value().to_string(), // true/false
            JsonValue::Number(n) => n.base10_digits().to_string(), // 整数转字符串
            JsonValue::Float(f) => f.base10_digits().to_string(), // 浮点数转字符串
            JsonValue::String(s) => {
                format!("\"{}\"", s.value()) // 字符串加双引号
            }
            JsonValue::Array(arr) => {
                let elements = arr.iter()
                    .map(|e| e.to_json_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("[{}]", elements) // 数组用方括号包裹
            }
            JsonValue::Object(obj) => {
                let entries = obj.iter()
                    .map(|(k, v)| {
                        format!(
                            "\"{}\": {}",
                            k.value(), // 键加双引号
                            v.to_json_str() // 值递归转换
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{{{}}}", entries) // 对象用花括号包裹
            }
        }
    }
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
