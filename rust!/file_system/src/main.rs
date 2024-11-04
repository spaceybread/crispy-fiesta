use std::collections::HashMap;
extern crate fuzzy;
mod bucket_loader;
use fuzzy::bucket::GaussBucket;
use fuzzy::fuzzyImpl;
use fuzzy::gaussFuzzy;
use std::time::Instant;
use fuzzy::bucket;


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
    let scale = 3;
    let p = 5;
    let dim = 24;
    // Creating the test data 
    let mut all = vec![vec![0.0]; 10000];
    let mut testCases = vec![vec![0.0]; 5];
    let mut tc = 0;

    for i in 0..10000 {
        all[i] = fuzzyImpl::randomVector(dim as usize);

        if i % 2000 == 0 {
            testCases[tc] = all[i].clone();
            tc += 1
        }
    }

    // Pre Bucketing + Gauss
    let mut now = Instant::now();
    let mut bct = GaussBucket::new(scale, p);
            
    // for vec in &all {
    //    bct.add(vec.clone());
    // }
    
    bucket_loader::make_files_from_bucket(bct.clone());
    let mut elapsed = now.elapsed();
    //println!("Bucket Processing (Gauss) Elapsed: {:.2?}", elapsed);
    // println!("{} elements stored in {} buckets", bct.getBucketSize(), bct.getBucketCount());

    bct = bucket_loader::get_bucket_from_data();

    // Bucket
    let mut now = Instant::now();
    for vec in &testCases {
        let cands = bucket_loader::handle_queries(bct.clone(), vec.clone());
        let res = gaussFuzzy::gen(vec.clone(), scale);
        for can in cands {
            let rec = gaussFuzzy::recov(res.0.clone(), can.clone(), scale);
            if rec == res.1 {
                println!("works!");
            }
        }
    }
    let mut elapsed1 = now.elapsed();
    //println!("Bucket With Slack (Gauss) Elapsed: {:.2?}", elapsed1);
    println!("{:.2?} ({:.2?})", elapsed1, elapsed);
}

fn make_db(scale: i32, p: i32) {
    let dim = 24;
    // Creating the test data 
    let mut all = vec![vec![0.0]; 10000000];
    let mut testCases = vec![vec![0.0]; 5];
    let mut tc = 0;

    for i in 0..10000000 {
        all[i] = fuzzyImpl::randomVector(dim as usize);

        if i % 2000000 == 0 {
            testCases[tc] = all[i].clone();
            tc += 1
        }
    }

    bucket_loader::make_data_file(all, "db".to_string());
    bucket_loader::make_data_file(testCases, "test".to_string());
}

fn loaded_test(scale: i32, p: i32) {
    let mut all = bucket_loader::read_file_to_vec("db/db.txt");
    let mut testCases = bucket_loader::read_file_to_vec("db/test.txt");

    // Pre Bucketing + Gauss
    let mut now = Instant::now();
    let mut bct = bucket::GaussBucket::new(scale, p);
    
    for vec in &all {
        bct.add(vec.clone());
    }
    let mut elapsed = now.elapsed();
    // Bucket
    let mut now = Instant::now();
    for vec in &testCases {
        let cands = bct.getCandidatesWithSlack(vec.clone());
        let res = gaussFuzzy::gen(vec.clone(), scale);
        for can in cands {
            let rec = gaussFuzzy::recov(res.0.clone(), can.clone(), scale);
            if rec == res.1 {
                // println!("works!");
            }
        }
    }
    let mut elapsed1 = now.elapsed();
    // println!("Bucket With Slack (Gauss) Elapsed: {:.2?}", elapsed1);
    println!("{:.2?} {:.2?}", elapsed, elapsed1);
}

fn timed_runs() {
    let scale = 3;
    let p = 2;
    
    make_db(scale, p);
    println!{"p = 2"};
    for i in 0..5 {
        loaded_test(scale, p);
    }
    println!{"p = 1"};
    for i in 0..5 {
        loaded_test(scale, 1);
    }
}


fn actual_data() {
    let scale = 1;
    let p = 2;

    let mut all = bucket_loader::read_file_to_vec("../../test_data/embeddings.txt");
    println!("Loading done!");
    let mut test_cases = vec![all[15].clone(), all[75].clone(), all[314].clone(), all[1618].clone(), all[9999].clone()];

    let mut now = Instant::now();
    let mut bct = bucket::GaussBucket::new(scale, p);
    
    for vec in &all {
        bct.add(vec.clone());
    }
    let mut elapsed = now.elapsed();
    println!("{:.2?}", elapsed);

    let mut elapsed = now.elapsed();
    // Bucket
    let mut now = Instant::now();
    for vec in &test_cases {
        let cands = bct.getCandidatesWithSlack(vec.clone());
        let res = gaussFuzzy::gen(vec.clone(), scale);
        let mut i = 0;
        for can in cands {
            let rec = gaussFuzzy::recov(res.0.clone(), can.clone(), scale);
            if rec == res.1 {
                i += 1;
                // println!("works!");
            }
        }
        println!("{} <- {:?}", i, res.1);
    }
    let mut elapsed1 = now.elapsed();
    // println!("Bucket With Slack (Gauss) Elapsed: {:.2?}", elapsed1);
    println!("{:.2?} {:.2?}", elapsed, elapsed1);

    

}

fn main() {
    actual_data();
}