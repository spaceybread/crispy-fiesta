use std::collections::HashMap;

extern crate fuzzy;
mod bucket_loader;
use fuzzy::bucket::GaussBucket;
use fuzzy::fuzzyImpl;

fn main() {
    let scale = 1;
    let queries = vec![vec![1.6, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0]];
    let mut db = vec![vec![1.45, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0], vec![1.55, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0], vec![1.55, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0]];

    let lat = fuzzyImpl::getLeechLattice();
    let mut bct = GaussBucket::new(scale, 3);
    
    for vec in &db {
        bct.add(vec.clone());
    }
    bucket_loader::make_files_from_bucket(bct);

}