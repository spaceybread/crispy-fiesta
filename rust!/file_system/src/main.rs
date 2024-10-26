use std::collections::HashMap;
extern crate fuzzy;
mod bucket_loader;
use fuzzy::bucket::GaussBucket;
use fuzzy::fuzzyImpl;
use fuzzy::gaussFuzzy;
use std::time::Instant;


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

    bct = bucket_loader::get_bucket_from_data();
    
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

fn brute_force() {
    let dim = 24;
    // Creating the test data 
    let mut all = vec![vec![0.0]; 100000];
    let mut testCases = vec![vec![0.0]; 5];
    let mut tc = 0;

    for i in 0..100000 {
        all[i] = fuzzyImpl::randomVector(dim as usize);

        if i % 20000 == 0 {
            testCases[tc] = all[i].clone();
            tc += 1
        }
    }

    // Pre Bucketing + Gauss
    let mut now = Instant::now();
    let mut bct = GaussBucket::new(3, 2);
            
    for vec in &all {
        bct.add(vec.clone());
    }
    let mut elapsed = now.elapsed();
    println!("Bucket Processing (Gauss) Elapsed: {:.2?}", elapsed);
    println!("{} elements stored in {} buckets", bct.getBucketSize(), bct.getBucketCount());
    
    bucket_loader::make_files_from_bucket(bct.clone());
    bct = bucket_loader::get_bucket_from_data();

    // Bucket
    let mut now = Instant::now();
    for vec in &testCases {
        let cands = bucket_loader::handle_queries(bct.clone(), vec.clone());
        let res = gaussFuzzy::gen(vec.clone(), 3);
        for can in cands {
            let rec = gaussFuzzy::recov(res.0.clone(), can.clone(), 3);
            if rec == res.1 {
                println!("works!");
            }
        }
    }
    let mut elapsed = now.elapsed();
    println!("Bucket With Slack (Gauss) Elapsed: {:.2?}", elapsed);
    println!("");
}

fn main() {
    // basic_test();
    brute_force();
}