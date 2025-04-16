
use super::*;
use syn::{LitBool, LitFloat, LitInt, LitStr, parse_str};

// ---------------------- 基础值测试 ----------------------
#[test]
fn test_null() {
    let input = "null";
    let value: JsonValue = parse_str(input).unwrap();
    assert!(matches!(value, JsonValue::Null));
}

#[test]
fn test_bool() {
    let true_input = "true";
    let false_input = "false";

    let true_value: JsonValue = parse_str(true_input).unwrap();
    let false_value: JsonValue = parse_str(false_input).unwrap();

    assert!(matches!(true_value, JsonValue::Bool(b) if b.value()));
    assert!(matches!(false_value, JsonValue::Bool(b) if !b.value()));
}

#[test]
fn test_number() {
    let int_input = "42";
    let float_input = "3.14";

    let int_value: JsonValue = parse_str(int_input).unwrap();
    let float_value: JsonValue = parse_str(float_input).unwrap();

    if let JsonValue::Number(n) = int_value {
        assert_eq!(n.base10_digits(), "42");
    } else {
        panic!("Expected Number variant");
    }

    if let JsonValue::Float(f) = float_value {
        assert_eq!(f.base10_digits(), "3.14");
    } else {
        panic!("Expected Float variant");
    }
}

#[test]
fn test_string() {
    let input = "\"hello world\"";
    let value: JsonValue = parse_str(input).unwrap();

    if let JsonValue::String(s) = value {
        assert_eq!(s.value(), "hello world");
    } else {
        panic!("Expected String variant");
    }
}

// ---------------------- 复合结构测试 ----------------------
#[test]
fn test_array() {
    let input = "[1, true, \"test\"]";
    let value: JsonValue = parse_str(input).unwrap();

    if let JsonValue::Array(arr) = value {
        assert_eq!(arr.len(), 3);

        // 检查第一个元素（数字）
        if let JsonValue::Number(n) = &arr[0] {
            assert_eq!(n.base10_digits(), "1");
        } else {
            panic!("First element is not a number");
        }

        // 检查第二个元素（布尔值）
        if let JsonValue::Bool(b) = &arr[1] {
            assert!(b.value());
        } else {
            panic!("Second element is not a boolean");
        }

        // 检查第三个元素（字符串）
        if let JsonValue::String(s) = &arr[2] {
            assert_eq!(s.value(), "test");
        } else {
            panic!("Third element is not a string");
        }
    } else {
        panic!("Expected Array variant");
    }
}

#[test]
fn test_object() {
    let input = r#"{"name": "Alice", "age": 30, "is_student": false}"#;
    let value: JsonValue = parse_str(input).unwrap();

    if let JsonValue::Object(obj) = value {
        assert_eq!(obj.len(), 3);

        // 检查 "name" 键
        let name_entry = obj.iter().find(|(k, _)| k.value() == "name").unwrap();
        if let JsonValue::String(s) = &name_entry.1 {
            assert_eq!(s.value(), "Alice");
        } else {
            panic!("'name' value is not a string");
        }

        // 检查 "age" 键
        let age_entry = obj.iter().find(|(k, _)| k.value() == "age").unwrap();
        if let JsonValue::Number(n) = &age_entry.1 {
            assert_eq!(n.base10_digits(), "30");
        } else {
            panic!("'age' value is not a number");
        }

        // 检查 "is_student" 键
        let student_entry = obj.iter().find(|(k, _)| k.value() == "is_student").unwrap();
        if let JsonValue::Bool(b) = &student_entry.1 {
            assert!(!b.value());
        } else {
            panic!("'is_student' value is not a boolean");
        }
    } else {
        panic!("Expected Object variant");
    }
}

// ---------------------- 错误处理测试 ----------------------
#[test]
fn test_invalid_syntax() {
    let invalid_inputs = ["{", "1,2", r#""key": value"#, "true true"];

    for input in invalid_inputs {
        let result: syn::Result<JsonValue> = parse_str(input);
        assert!(result.is_err(), "Expected error for input: {}", input);
    }
}
// ---------------------- 混合测试 ----------------------
#[test]
fn test_nested_object_array() {
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

    let value: JsonValue = parse_str(input.trim()).unwrap();

    // 验证外层是对象
    if let JsonValue::Object(obj) = value {
        // 验证 "books" 数组字段
        let books_entry = obj.iter().find(|(k, _)| k.value() == "books").unwrap();
        if let JsonValue::Array(books_arr) = &books_entry.1 {
            assert_eq!(books_arr.len(), 2);

            // 第一个 book 对象
            if let JsonValue::Object(first_book) = &books_arr[0] {
                // 验证 title 字符串
                let title_entry = first_book.iter().find(|(k, _)| k.value() == "title").unwrap();
                if let JsonValue::String(s) = &title_entry.1 {
                    assert_eq!(s.value(), "The Rust Programming Language");
                } else {
                    panic!("First book title is not a string");
                }

                // 验证 details 对象中的 categories 数组
                let details_entry = first_book.iter().find(|(k, _)| k.value() == "details").unwrap();
                if let JsonValue::Object(details_obj) = &details_entry.1 {
                    let categories_entry = details_obj.iter().find(|(k, _)| k.value() == "categories").unwrap();
                    if let JsonValue::Array(categories_arr) = &categories_entry.1 {
                        assert_eq!(categories_arr.len(), 2);
                        if let JsonValue::String(s1) = &categories_arr[0] {
                            assert_eq!(s1.value(), "systems");
                        } else {
                            panic!("First category is not a string");
                        }
                        if let JsonValue::String(s2) = &categories_arr[1] {
                            assert_eq!(s2.value(), "programming");
                        } else {
                            panic!("Second category is not a string");
                        }
                    } else {
                        panic!("Categories is not an array");
                    }
                } else {
                    panic!("Details is not an object");
                }
            } else {
                panic!("First book is not an object");
            }

            // 第二个 book 对象（包含 null 值和空数组）
            if let JsonValue::Object(second_book) = &books_arr[1] {
                // 验证 author 为 null
                let author_entry = second_book.iter().find(|(k, _)| k.value() == "author").unwrap();
                assert!(matches!(author_entry.1, JsonValue::Null));

                // 验证 details 中的空数组
                let details_entry = second_book.iter().find(|(k, _)| k.value() == "details").unwrap();
                if let JsonValue::Object(details_obj) = &details_entry.1 {
                    let categories_entry = details_obj.iter().find(|(k, _)| k.value() == "categories").unwrap();
                    if let JsonValue::Array(categories_arr) = &categories_entry.1 {
                        assert!(categories_arr.is_empty());
                    } else {
                        panic!("Empty categories is not an array");
                    }
                } else {
                    panic!("Details is not an object");
                }
            } else {
                panic!("Second book is not an object");
            }
        } else {
            panic!("Books is not an array");
        }

        // 验证 "stats" 对象字段
        let stats_entry = obj.iter().find(|(k, _)| k.value() == "stats").unwrap();
        if let JsonValue::Object(stats_obj) = &stats_entry.1 {
            // 验证数值和布尔值
            let total_entry = stats_obj.iter().find(|(k, _)| k.value() == "total_books").unwrap();
            if let JsonValue::Number(n) = &total_entry.1 {
                assert_eq!(n.base10_digits(), "2");
            } else {
                panic!("Total books is not a number");
            }

            let has_data_entry = stats_obj.iter().find(|(k, _)| k.value() == "has_missing_data").unwrap();
            if let JsonValue::Bool(b) = &has_data_entry.1 {
                assert!(b.value());
            } else {
                panic!("Has missing data is not a boolean");
            }
        } else {
            panic!("Stats is not an object");
        }
    } else {
        panic!("Root is not an object");
    }
}