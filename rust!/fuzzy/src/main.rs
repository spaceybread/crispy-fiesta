use sha2::{Sha256, Digest};
use nalgebra::{DMatrix, DVector};
use std::env;
use rand::Rng;
use std::time::Instant;

// Make the lattice given the constraints
fn makeLattice(scale: i32, dim: usize) -> Vec<Vec<f64>> {
    let mut lattice = vec![vec![0.0; dim]; dim]; // init the lattice

    for i in 0..dim {
        for j in 0..dim {
            if i == j {
                lattice[i][j] = scale as f64;
            }
        }
    }
    return lattice;
}

// Make the leech lattice
fn makeLeechLattice() -> Vec<Vec<f64>> {
    let out = vec![8.0,4.0,4.0,4.0,4.0,4.0,4.0,2.0,4.0,4.0,4.0,2.0,4.0,2.0,2.0,2.0,4.0,2.0,2.0,2.0,0.0,0.0,0.0,-3.0,4.0,4.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,1.0,2.0,1.0,1.0,2.0,1.0,0.0,0.0,-1.0,4.0,2.0,4.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,2.0,1.0,2.0,2.0,1.0,1.0,1.0,0.0,0.0,-1.0,4.0,2.0,2.0,4.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,1.0,2.0,2.0,1.0,2.0,1.0,1.0,0.0,0.0,-1.0,4.0,2.0,2.0,2.0,4.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,0.0,0.0,-1.0,4.0,2.0,2.0,2.0,2.0,4.0,2.0,2.0,2.0,2.0,2.0,1.0,2.0,2.0,1.0,1.0,2.0,1.0,2.0,1.0,0.0,0.0,0.0,-1.0,4.0,2.0,2.0,2.0,2.0,2.0,4.0,2.0,2.0,2.0,2.0,1.0,2.0,1.0,2.0,1.0,2.0,1.0,1.0,2.0,0.0,0.0,0.0,-1.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,4.0,1.0,1.0,1.0,2.0,1.0,2.0,2.0,2.0,1.0,2.0,2.0,2.0,2.0,0.0,0.0,1.0,4.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,4.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,1.0,1.0,-1.0,4.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,2.0,4.0,2.0,2.0,2.0,2.0,1.0,1.0,2.0,2.0,1.0,1.0,0.0,1.0,0.0,-1.0,4.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,2.0,2.0,4.0,2.0,2.0,1.0,2.0,1.0,2.0,1.0,2.0,1.0,0.0,0.0,1.0,-1.0,2.0,2.0,2.0,2.0,1.0,1.0,1.0,2.0,2.0,2.0,2.0,4.0,1.0,2.0,2.0,2.0,1.0,2.0,2.0,2.0,2.0,1.0,1.0,1.0,4.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,2.0,2.0,2.0,1.0,4.0,2.0,2.0,2.0,2.0,1.0,1.0,1.0,1.0,1.0,1.0,-1.0,2.0,2.0,1.0,1.0,2.0,2.0,1.0,2.0,2.0,2.0,1.0,2.0,2.0,4.0,2.0,2.0,1.0,2.0,2.0,2.0,2.0,2.0,1.0,1.0,2.0,1.0,2.0,1.0,2.0,1.0,2.0,2.0,2.0,1.0,2.0,2.0,2.0,2.0,4.0,2.0,1.0,2.0,2.0,2.0,2.0,1.0,2.0,1.0,2.0,1.0,1.0,2.0,2.0,1.0,1.0,2.0,2.0,1.0,1.0,2.0,2.0,2.0,2.0,4.0,1.0,2.0,2.0,2.0,2.0,1.0,1.0,1.0,4.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,2.0,2.0,2.0,1.0,2.0,1.0,1.0,1.0,4.0,2.0,2.0,2.0,1.0,1.0,1.0,-1.0,2.0,1.0,2.0,1.0,2.0,1.0,1.0,2.0,2.0,2.0,1.0,2.0,1.0,2.0,2.0,2.0,2.0,4.0,2.0,2.0,2.0,2.0,1.0,1.0,2.0,1.0,1.0,2.0,2.0,2.0,1.0,2.0,2.0,1.0,2.0,2.0,1.0,2.0,2.0,2.0,2.0,2.0,4.0,2.0,2.0,1.0,2.0,1.0,2.0,2.0,1.0,1.0,2.0,1.0,2.0,2.0,2.0,1.0,1.0,2.0,1.0,2.0,2.0,2.0,2.0,2.0,2.0,4.0,2.0,1.0,1.0,1.0,0.0,1.0,1.0,1.0,1.0,0.0,0.0,2.0,1.0,0.0,0.0,2.0,1.0,2.0,2.0,2.0,1.0,2.0,2.0,2.0,4.0,2.0,2.0,2.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,1.0,0.0,1.0,1.0,2.0,1.0,1.0,1.0,2.0,1.0,1.0,2.0,4.0,2.0,2.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,1.0,1.0,1.0,1.0,2.0,1.0,1.0,1.0,2.0,1.0,2.0,2.0,4.0,2.0,-3.0,-1.0,-1.0,-1.0,-1.0,-1.0,-1.0,1.0,-1.0,-1.0,-1.0,1.0,-1.0,1.0,1.0,1.0,-1.0,1.0,1.0,1.0,2.0,2.0,2.0,4.0];
    let mut lattice = vec![vec![0.0; 24]; 24];
    let mut c = 0;
    for i in 0..24 {
        for j in 0..24 {
            lattice[i][j] = out[c]; 
            c += 1;
        }
    }
    return lattice;
}

// (Helper) Flatten the matrix
fn makeLatticeAs1DVector(lat: Vec<Vec<f64>>, dim: usize) -> Vec<f64> {
    let mut vec = vec![0.0; dim * dim];
    let mut c = 0;
    for i in 0..dim {
        for j in 0..dim {
            vec[c] = lat[i][j]; 
            c += 1;
        }
    }
    return vec;
}


// (Helper) To visualise the lattice and check for correctness
fn printLattice(lattice: Vec<Vec<f64>>) {
    for row in lattice {
        println!("{:?}", row);
    }
}

// (Helper) To round a vector given a custom threshold
fn round(vec: Vec<f64>, threshold: f64) -> Vec<f64> {
    let mut out = vec![0.0; vec.len()];

    for i in 0..vec.len() {
        let val = vec[i];

        if val - val.floor() > threshold {
            out[i] = val.ceil();
        } else {out[i] = val.floor();}
    }

    return out;
}

// (Helper) To hash a vector
fn hashVector(vec: Vec<f64>) -> String {
    let mut hash = Sha256::new();

    for &n in &vec {
        hash.update(n.to_le_bytes());
    }

    let result = hash.finalize();
    result.to_vec();
    hex::encode(result)
}

// (Helper) Find the closest point in the lattice
fn closest(vec: Vec<f64>, lattice: Vec<Vec<f64>>) -> Vec<f64> {
    let p = DVector::from_vec(vec);
    let flatLattice = makeLatticeAs1DVector(lattice.clone(), lattice.len() as usize);
    let B = DMatrix::from_vec(lattice.len(), lattice.len(), flatLattice);
    
    if  B.is_invertible() == false {
        return vec![0.0];
    }

    let inv_B = B.clone().try_inverse().unwrap();
    let coeff = inv_B * p;
    let coeffVec = coeff.data.as_vec().clone();
    let rounded = DVector::from_vec(round(coeffVec, 0.51));
    let almostOut = B * rounded;

    return almostOut.data.as_vec().clone();
}

// (Helper) Subtract two vectors
fn vectorSubtraction(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
    let mut out = vec![0.0; a.len() as usize];
    for i in 0..(a.len() as usize) {
        out[i] = a[i] - b[i];
    }
    return out; 
}

// (Helper) Add two vectors
fn vectorAddition(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
    let mut out = vec![0.0; a.len() as usize];
    for i in 0..(a.len() as usize) {
        out[i] = a[i] + b[i];
    }
    return out; 
}

// (Helper) Generate a random vector
fn randomVector(dim: usize) -> Vec<f64> {
    let mut out = vec![0.0; dim];
    for i in 0..dim {
        out[i] = rand::thread_rng().gen_range(-1000..1000) as f64;
    }
    return out
}

// Generate a helper point and hash point from a vector
fn gen(vec: Vec<f64>, lat: Vec<Vec<f64>>) -> (Vec<f64>, String) {
    let mut rdm = randomVector(vec.len() as usize);
    rdm = closest(rdm.clone(), lat);
    let helper = vectorSubtraction(rdm.clone(), vec.clone());
    
    return (helper, hashVector(rdm))
}

// Generate a hashed point given a helper and a vector
fn recov(helper: Vec<f64>, vec: Vec<f64>, lat: Vec<Vec<f64>>) -> String {
    let mut out = vectorAddition(helper, vec);
    out = closest(out.clone(), lat);
    
    return hashVector(out);
}


// (Helper) Check if two Vec<f64> are the same
fn compareVec(a: Vec<f64>, b: Vec<f64>) -> bool {
    
    for i in 0..(a.len() as usize) {
        if a[i] != b[i] {
            return false;
        }
    }

    return true;
}

// (Demo) Single encode and decode
fn demo1() {
    env::set_var("RUST_BACKTRACE", "1");
    let dim = 16;
    let lat = makeLattice(5, dim);

    let mut vec = vec![0.0; dim];
    
    for i in 0..dim{
        vec[i] = (i as f64) * 1.5;
    }

    // println!("{:?}", vec.clone());

    let res = gen(vec.clone(), lat.clone());

    // println!("{:?}", res.0);
    // println!("{:?}", res.1);

    let mut another = vec![0.0; dim];
    for i in 0..dim{
        another[i] = (i as f64) * 1.5;
    }

    another[0] = 1.0;
    another[1] = 3.0;
    let rec = recov(res.0.clone(), another.clone(), lat.clone());

    println!("Are both vectors the same: {:?}", compareVec(vec.clone(), another.clone()));
    println!("Do both get the same assigned point: {:?}", res.1 == rec);
    println!("{:?}", rec);
}

// (Demo) Single encode and multiple decodes
fn demo2() {
    env::set_var("RUST_BACKTRACE", "1");
    let dim = 24;
    // let lat = makeLattice(5, dim);
    let lat = makeLeechLattice();

    let mut vec = randomVector(dim as usize);
    
    for i in 0..dim{
        vec[i] = (i as f64) * 1.5;
    }
    let res = gen(vec.clone(), lat.clone());
    println!("{:?}", res.1);

    for i in 0..32 {
        let mut another = randomVector(dim as usize);
        let rec = recov(res.0.clone(), another.clone(), lat.clone());
        println!("{:?}", rec);
    }
}

// (Demo) Runs demo2 ite times and times it 
fn timedDemo(ite: i32) {
    let now = Instant::now();
    for i in 0..ite {
        demo2();
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn main() {
    timedDemo(32000);
}