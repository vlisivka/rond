use rond::value::Number;

#[test]
fn test_large_number() {
    use rond::value::Value;
    let test_var = Value::Number(Number::new(10000000000000000000000.0f64));
    let test_ser = rond::ser::to_string(&test_var).unwrap();
    let test_deser = rond::de::from_str::<Value>(&test_ser);

    assert_eq!(
        test_deser.unwrap(),
        Value::Number(Number::new(10000000000000000000000.0))
    );
}
