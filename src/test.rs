use super::*;

#[test]
fn test_basic_types() {
    // 测试基本类型
    let null_val = json!(null);
    assert_eq!(null_val, Value::Null);

    let bool_true = json!(true);
    assert_eq!(bool_true, Value::Bool(true));

    let bool_false = json!(false);
    assert_eq!(bool_false, Value::Bool(false));

    let int_num = json!(42);
    assert_eq!(int_num, Value::Number(42.0));

    let float_num = json!(3.14);
    assert_eq!(float_num, Value::Number(3.14));

    let string_val = json!("hello world");
    assert_eq!(string_val, Value::String("hello world".to_string()));
}

#[test]
fn test_array_types() {
    // 测试数组类型
    let simple_array = json!([1, 2, 3]);
    assert_eq!(
        simple_array,
        Value::Array(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0)
        ])
    );

    let mixed_array = json!([true, "test", null, 42.5]);
    assert_eq!(
        mixed_array,
        Value::Array(vec![
            Value::Bool(true),
            Value::String("test".to_string()),
            Value::Null,
            Value::Number(42.5)
        ])
    );
}

#[test]
fn test_object_types() {
    // 测试对象类型
    let simple_object = json!({"name": "Alice", "age": 30});
    let mut expected_object = HashMap::new();
    expected_object.insert("name".to_string(), Value::String("Alice".to_string()));
    expected_object.insert("age".to_string(), Value::Number(30.0));
    assert_eq!(simple_object, Value::Object(expected_object));
}

#[test]
fn test_nested_structures() {
    // 测试嵌套结构
    let nested_structure = json!({
        "user": {
            "name": "Bob",
            "hobbies": ["reading", "coding"]
        },
        "is_active": true
    });

    let mut nested_object = HashMap::new();
    nested_object.insert(
        "user".to_string(),
        Value::Object({
            let mut user = HashMap::new();
            user.insert("name".to_string(), Value::String("Bob".to_string()));
            user.insert(
                "hobbies".to_string(),
                Value::Array(vec![
                    Value::String("reading".to_string()),
                    Value::String("coding".to_string()),
                ]),
            );
            user
        }),
    );
    nested_object.insert("is_active".to_string(), Value::Bool(true));
    assert_eq!(nested_structure, Value::Object(nested_object));
}
