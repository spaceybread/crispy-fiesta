use sha2::{Sha256, Digest};
use nalgebra::{DMatrix, DVector};
use std::env;
use rand::Rng;
use std::time::Instant;
mod fuzzyImpl;
mod bucket;
mod gaussFuzzy;
mod gaussPSM;

// (Demo) Single encode and decode
fn demo1() {
    env::set_var("RUST_BACKTRACE", "1");
    let dim = 16;
    let lat = fuzzyImpl::makeLattice(5, dim);

    let mut vec = vec![0.0; dim];
    
    for i in 0..dim{
        vec[i] = (i as f64) * 1.5;
    }

    // println!("{:?}", vec.clone());

    let res = fuzzyImpl::gen(vec.clone(), lat.clone());

    // println!("{:?}", res.0);
    // println!("{:?}", res.1);

    let mut another = vec![0.0; dim];
    for i in 0..dim{
        another[i] = (i as f64) * 1.5;
    }

    another[0] = 1.0;
    another[1] = 3.0;
    let rec = fuzzyImpl::recov(res.0.clone(), another.clone(), lat.clone());

    println!("Are both vectors the same: {:?}", fuzzyImpl::compareVec(vec.clone(), another.clone()));
    println!("Do both get the same assigned point: {:?}", res.1 == rec);
    println!("{:?}", rec);
}

// (Demo) Single encode and multiple decodes
fn demo2() {
    env::set_var("RUST_BACKTRACE", "1");
    let dim = 24;
    // let lat = makeLattice(5, dim);
    let lat = fuzzyImpl::makeLeechLattice();

    let mut vec = fuzzyImpl::randomVector(dim as usize);
    
    for i in 0..dim{
        vec[i] = (i as f64) * 1.5;
    }
    let res = fuzzyImpl::gen(vec.clone(), lat.clone());
    println!("{:?}", res.1);

    for i in 0..32 {
        let mut another = fuzzyImpl::randomVector(dim as usize);
        let rec = fuzzyImpl::recov(res.0.clone(), another.clone(), lat.clone());
        println!("{:?}", rec);
    }
}

fn demo3() {
    env::set_var("RUST_BACKTRACE", "1");
    let dim = 24;
    // let lat = makeLattice(5, dim);
    let lat = fuzzyImpl::getLeechLattice();

    let mut vec = fuzzyImpl::randomVector(dim as usize);
    
    for i in 0..dim{
        vec[i] = (i as f64) * 1.5;
    }
    let res = fuzzyImpl::gen1D(vec.clone(), lat.clone(), 24);
    println!("{:?}", res.1);

    for i in 0..32 {
        let mut another = fuzzyImpl::randomVector(dim as usize);
        let rec = fuzzyImpl::recov1D(res.0.clone(), another.clone(), lat.clone(), 24);
        println!("{:?}", rec);
    }
}

fn gaussTest() {
    let dim = 24;
    let vec = fuzzyImpl::randomVector(dim as usize);
    // println!("{:?}", vec.clone());

    let res = gaussFuzzy::gen(vec.clone(), 4);
    println!("{:?}", res.1);

    for i in 0..32 {
        let mut another = gaussFuzzy::randomVector(dim as usize);
        let rec = gaussFuzzy::recov(res.0.clone(), another.clone(), 4);
        println!("{:?}", rec);
    }
}

// (Demo) Runs demo2 ite times and times it 
fn timedDemo(ite: i32, mode: i8) {
    let now = Instant::now();
    for i in 0..ite {
        if mode == 0 {
            // with flattening
            demo2();
        } else if mode == 1 {
            // without flattening
            demo3();
        } else {
            // guass lattice with scale 4
            gaussTest();
        }
        
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn bucketDemo() {
    let dim = 24;
    let lat = fuzzyImpl::getLeechLattice();
    let now = Instant::now();
    let mut bct = bucket::Bucket::new(lat.clone(), dim, 1);
    
    // bct.displayBucket();
    let mut test = vec![0.0];
    for i in 0..10000 {
        let mut another = fuzzyImpl::randomVector(dim as usize);
        bct.add(another.clone());
        test = another.clone();
    }

    // println!("{:?}", bct.getCandidates(vec));
    println!("{}", bct.getBucketSize());
    println!("{}", bct.getBucketCount());
    
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let mut vec = fuzzyImpl::randomVector(dim as usize);
    let mut cands = bct.getCandidates(vec.clone());

    let mut res = fuzzyImpl::gen1D(vec.clone(), lat.clone(), 24);

    for can in cands {
        let rec = fuzzyImpl::recov1D(res.0.clone(), can.clone(), lat.clone(), 24);
        println!("{}", res.1 == rec);
    }

    cands = bct.getCandidates(test.clone());

    res = fuzzyImpl::gen1D(test.clone(), lat.clone(), 24);

    for can in cands {
        let rec = fuzzyImpl::recov1D(res.0.clone(), can.clone(), lat.clone(), 24);
        println!("{}", res.1 == rec);
    }

}

fn dragRace(doNormal: bool, doGauss: bool, doLeechBucket: bool, doGaussBucket: bool, doGaussPSM: bool, doGaussBucketSlack: bool) {
    // Lattice set up
    let dim = 24;
    let lat = fuzzyImpl::getLeechLattice();
    
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

    if doNormal {
        // No bucketing + Leech
        let mut now = Instant::now();
        for vec in &testCases {
            let res = fuzzyImpl::gen1D(vec.clone(), lat.clone(), 24);
            for another in &all {
                let rec = fuzzyImpl::recov1D(res.0.clone(), another.clone(), lat.clone(), 24);
            }
        }
        let mut elapsed = now.elapsed();
        println!("No Bucket (Leech) Elapsed: {:.2?}", elapsed);
        println!("");
    }

    if doGauss {
        // No bucketing + Gauss
        let mut now = Instant::now();
        for vec in &testCases {
            let res = gaussFuzzy::gen(vec.clone(), 4);
            for another in &all {
                let rec = gaussFuzzy::recov(res.0.clone(), another.clone(), 4);
            }
        }
        let mut elapsed = now.elapsed();
        println!("No Bucket (Gauss) Elapsed: {:.2?}", elapsed);
        println!("");
    }

    if doLeechBucket {
        // Pre Bucketing + Leech
        let mut now = Instant::now();
        let mut bct = bucket::Bucket::new(lat.clone(), dim, 2);
        for vec in &all {
            bct.add(vec.clone());
        }
        let mut elapsed = now.elapsed();
        println!("Bucket Processing (Leech) Elapsed: {:.2?}", elapsed);
        println!("{} elements stored in {} buckets", bct.getBucketSize(), bct.getBucketCount());

        // Bucket
        let mut now = Instant::now();
        for vec in &testCases {
            let cands = bct.getCandidates(vec.clone());
            let res = fuzzyImpl::gen1D(vec.clone(), lat.clone(), 24);
            for can in cands {
                let rec = fuzzyImpl::recov1D(res.0.clone(), can.clone(), lat.clone(), 24);
            }
        }
        let mut elapsed = now.elapsed();
        println!("Bucket (Leech) Elapsed: {:.2?}", elapsed);
        println!("");
    }

    if doGaussBucket {
        // Pre Bucketing + Gauss
        let mut now = Instant::now();
        let mut bct = bucket::GaussBucket::new(3, 2);
        for vec in &all {
            bct.add(vec.clone());
        }
        let mut elapsed = now.elapsed();
        println!("Bucket Processing (Gauss) Elapsed: {:.2?}", elapsed);
        println!("{} elements stored in {} buckets", bct.getBucketSize(), bct.getBucketCount());

        // Bucket
        let mut now = Instant::now();
        for vec in &testCases {
            let cands = bct.getCandidates(vec.clone());
            let res = gaussFuzzy::gen(vec.clone(), 3);
            for can in cands {
                let rec = gaussFuzzy::recov(res.0.clone(), can.clone(), 2);
            }
        }
        let mut elapsed = now.elapsed();
        println!("Bucket (Gauss) Elapsed: {:.2?}", elapsed);
        println!("");
    }

    if doGaussPSM {
        // Pre Bucketing + Gauss
        let mut now = Instant::now();
        let mut bct = bucket::GaussBucket::new(3, 2);
        for vec in &all {
            bct.add(vec.clone());
        }
        let mut elapsed = now.elapsed();
        println!("Bucket Processing (Gauss) Elapsed: {:.2?}", elapsed);
        println!("{} elements stored in {} buckets", bct.getBucketSize(), bct.getBucketCount());
        now = Instant::now();
        // Bucket
        let res = gaussPSM::makeHelpersGauss(testCases.clone(), bct.clone());
        let rec = gaussPSM::attemptMatchingGauss(all.clone(), bct, res.1.clone(), res.2.clone());
        let out = gaussPSM::returnMatches(rec, res.0.clone(), testCases.clone());

        let mut elapsed = now.elapsed();
        println!("PSM Bucket (Gauss) Elapsed: {:.2?}", elapsed);
        println!("");
    }

    if doGaussBucketSlack {
        // Pre Bucketing + Gauss
        let mut now = Instant::now();
        let mut bct = bucket::GaussBucket::new(3, 2);
        
        for vec in &all {
            bct.addWithSlack(vec.clone());
        }
        let mut elapsed = now.elapsed();
        println!("Bucket Processing (Gauss) Elapsed: {:.2?}", elapsed);
        println!("{} elements stored in {} buckets", bct.getBucketSize(), bct.getBucketCount());
        
        // Bucket
        let mut now = Instant::now();
        for vec in &testCases {
            let cands = bct.getCandidatesWithSlack(vec.clone());
            let res = gaussFuzzy::gen(vec.clone(), 3);
            for can in cands {
                let rec = gaussFuzzy::recov(res.0.clone(), can.clone(), 2);
            }
        }
        let mut elapsed = now.elapsed();
        println!("Bucket With Slack (Gauss) Elapsed: {:.2?}", elapsed);
        println!("");
    }
}

fn gaussPSMTest() {
    let mut now = Instant::now();
    let S = vec![
        vec![3.0, 1.0, 4.0, 1.0, 3.0, 1.0, 4.0, 1.0, 3.0, 1.0, 4.0, 1.0, 3.0, 1.0, 4.0, 1.0, 3.0, 1.0, 4.0, 1.0, 3.0, 1.0, 4.0, 1.0],
        vec![99.0; 24],
        vec![22.0; 24],
    ];

    let Q = vec![
        vec![2.0, 1.0, 4.0, 1.0, 3.0, 1.0, 4.0, 1.0, 3.0, 1.0, 4.0, 1.0, 3.0, 1.0, 4.0, 1.0, 3.0, 2.0, 4.0, 1.0, 3.0, 1.0, 4.0, 1.0],
        vec![100.0; 24],
        vec![33.0; 24],
    ];

    let mut bct = bucket::GaussBucket::new(4, 2);
    for vec in &Q {
        bct.add(vec.clone());
    }

    let res = gaussPSM::makeHelpersGauss(S.clone(), bct.clone());
    let rec = gaussPSM::attemptMatchingGauss(Q, bct, res.1.clone(), res.2.clone());
    let out = gaussPSM::returnMatches(rec, res.0.clone(), S);

    println!("{:?}", out);
    let mut elapsed = now.elapsed();
    println!("Gauss PSM Test Elapsed: {:.2?}", elapsed);
}

fn slackTest () {
    // Test data
    let scale = 1;
    let queries = vec![vec![1.6, 2.0, 3.0]];
    let mut db = vec![vec![1.45, 2.0, 3.0], vec![1.55, 2.0, 3.0]];

    let lat = fuzzyImpl::getLeechLattice();
    let mut bct = bucket::GaussBucket::new(scale, 1);
    
    for vec in &db {
        bct.add(vec.clone());
    }
    println!("{} elements stored in {} buckets", bct.getBucketSize(), bct.getBucketCount());

    let mut noSlack = (0, 0);
    for vec in &queries {
        let cands = bct.getCandidates(vec.clone());
        let res = gaussFuzzy::gen(vec.clone(), scale);
        println!("Query: {}: {:?}", vec[0], res.1);
        for can in cands {
            let rec = gaussFuzzy::recov(res.0.clone(), can.clone(), scale);
            println!("{:?}: {:?}", can[0], rec);
            noSlack.1 += 1;
            if rec == res.1 {
                noSlack.0 += 1
            }
        }
    }
    println!("");
    let mut slack = (0, 0);
    for vec in &queries {
        let cands = bct.getCandidatesWithSlack(vec.clone());
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

    println!("{:?}/{:?} returned without slack and {:?}/{:?} returned with slack", noSlack.0, noSlack.1, slack.0, slack.1);
}

fn main() {
    slackTest();
    // dragRace(false, false, false, true, false, true);
    // timedDemo(100, 2);
}