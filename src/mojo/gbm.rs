use std::io;
use mojo::treemodel;

pub fn score(row: &Vec<f64>, _offset: f64) -> io::Result<Vec<f64>> {
    let preds = treemodel::score_all_trees(&row);
    // todo: add offset, correct probabilities etc
    preds
}
