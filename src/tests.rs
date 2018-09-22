use ::rowdata::ValueType;
use ::rowdata::RowData;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_rowdata() {
    let mut data = RowData::new();
    data.set("a",ValueType::Text(String::from("Hello")));
    data.set("haha", ValueType::Number(6.2631));

    println!("haha = {}", data.get("haha"));
    println!("hoho = {}", data.get("hoho"));
}
