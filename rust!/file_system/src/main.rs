use std::collections::HashMap;

extern crate fuzzy;
mod bucket_loader;
use fuzzy::bucket::GaussBucket;
use fuzzy::fuzzyImpl;
use fuzzy::gaussFuzzy;


fn basic_test() {
    // HAS TO BE RUN FROM src
    let scale = 1;
    let queries = vec![vec![1.6, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0]];
    let mut db = vec![vec![1.45, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0], vec![1.55, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0], vec![1.55, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0]];

    let lat = fuzzyImpl::getLeechLattice();
    let mut bct = GaussBucket::new(scale, 3);
    
    for vec in &db {
        bct.add(vec.clone());
    }
    bucket_loader::make_files_from_bucket(bct.clone());
    
    let mut slack = (0, 0);
    for vec in &queries {    
        let cands = bucket_loader::handle_queries(bct.clone(), vec.clone());
        let res = gaussFuzzy::gen(vec.clone(), scale);
        println!("Query: {}: {:?}", vec[0], res.1);
        for can in cands {
            let rec = gaussFuzzy::recov(res.0.clone(), can.clone(), scale);
            println!("{:?}: {:?}", can[0], rec);
            slack.1 += 1;
            if rec == res.1 {
                slack.0 += 1
            }
        }
    }
    println!("{:?}/{:?} returned with slack", slack.0, slack.1);
}

fn main() {
    basic_test();
}