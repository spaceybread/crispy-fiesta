use sha2::{Sha256, Digest};
use nalgebra::{DMatrix, DVector};
use std::env;
use rand::Rng;
use std::time::Instant;
mod fuzzyImpl;

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

// (Demo) Runs demo2 ite times and times it 
fn timedDemo(ite: i32, mode: i8) {
    let now = Instant::now();
    for i in 0..ite {
        if mode == 0 {
            // with flattening
            demo2();
        } else {
            // without flattening
            demo3();
        } 
        
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn main() {
    timedDemo(100, 0);
}