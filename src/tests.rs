use ::rowdata::ValueType;
use ::rowdata::RowData;
use ::rowdata::RowDataStruct;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

use MojoModel;
use ::ModelCategory;

#[test]
fn can_load() {
    let mm = MojoModel::load("test.mdl");
    let a = ModelCategory::Regression;
    let c = ModelCategory::Binomial;
}


#[test]
fn test_rowdata() {
    let data = RowDataStruct{
        xvalue: 1.2346,
    };


    data.set("a",ValueType::Text(String::from("Hello")));

    println!("value = {}", data.get("haha"));
}
