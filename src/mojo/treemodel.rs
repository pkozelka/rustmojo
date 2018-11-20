use std::io;
use mojoreader::SubNode;
use std::io::Error;
use std::io::ErrorKind;

pub fn score_all_trees(row: &Vec<f64>) -> io::Result<Vec<f64>> {
    let mut preds = Vec::new();
    preds.insert(0, 0f64); // todo
    let ntpg = 1; // MojoModel::ntrees_per_group
    let ntg = 1; // MojoModel::ntrees_groups
    let nclasses = 1;
    for i in 0..ntpg {
        let k = {if nclasses == 1 { 0 } else { i + 1 }};
        for j in 0..ntg {
            let tree = get_tree(i,j);
            let pred = score_tree(&tree?, &row)?;
            preds[k as usize] = preds[k as usize] + pred;
        }
    }
    Ok(preds)
}

fn get_tree(_i: i32,_j: i32) -> io::Result<SubNode> {
    // todo
    Err(Error::new(ErrorKind::NotFound, "Not implemented: get_tree"))
}

fn score_tree(_tree: &SubNode, _row: &Vec<f64>) -> io::Result<f64> {
    Err(Error::new(ErrorKind::NotFound, "Not implemented: score_tree"))
}
