use mojo::Mojo;
use std::collections::HashMap;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn test_binomial_easy() {
    let mojo = Mojo::load("data/gbm_v1.00_names.mojo").unwrap();
    let row: HashMap<&str,&str> = [
        ("AGE", "68"),
        ("RACE", "2"),
        ("DCAPS", "2"),
    ].iter().cloned().collect();

    let prediction = mojo.predict_binomial_easy(row).unwrap();
//    assert_eq!(0.5, *prediction.pred.get(0).unwrap());
//    assert_eq!(0.5, *prediction.pred.get(1).unwrap());
    assert_eq!("EASY_DUMMY", prediction.label);
}

#[test]
fn test_raw() {
    let mojo = Mojo::load("data/gbm_v1.00_names.mojo").unwrap();
    let row = vec![68f64, 2f64, 2f64];

    let prediction = mojo.gbm_predict(&row).unwrap();
//    assert_eq!(0.5, prediction.pred.get(0).unwrap());
//    assert_eq!(0.5, prediction.pred.get(1).unwrap());
    assert_eq!("RAW_DUMMY", prediction.label);
}
