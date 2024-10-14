use sha2::{Sha256, Digest};
use nalgebra::{DMatrix, DVector};
use std::env;
use rand::Rng;
use std::time::Instant;
mod fuzzyImpl;
mod bucket;
mod gaussFuzzy;

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

fn dragRace(doNormal: bool, doGauss: bool, doLeechBucket: bool, doGaussBucket: bool) {
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
    }

    if doGaussBucket {
        // Pre Bucketing + Gauss
        let mut now = Instant::now();
        let mut bct = bucket::GaussBucket::new(4, 2);
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
            let res = gaussFuzzy::gen(vec.clone(), 4);
            for can in cands {
                let rec = gaussFuzzy::recov(res.0.clone(), can.clone(), 4);
            }
        }
        let mut elapsed = now.elapsed();
        println!("Bucket (Gauss) Elapsed: {:.2?}", elapsed);
    }
}




fn main() {
    dragRace(true, true, true, true);
    // timedDemo(100, 2);
}