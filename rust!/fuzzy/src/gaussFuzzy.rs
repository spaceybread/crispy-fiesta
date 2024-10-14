use sha2::{Sha256, Digest};
use nalgebra::{DMatrix, DVector};
use std::env;
use rand::Rng;
use std::time::Instant;

pub fn closest(mut vec: Vec<f64>, scale: i32) -> Vec<f64>{
    for i in 0..vec.len() {
        let mut val = vec[i];
        let r = val.rem_euclid(scale as f64); 

        if 2.0 * r >= scale as f64 {
            vec[i] += scale as f64;    
        }

        vec[i] += -1.0 * r;
    }
    return vec;
     
}

pub fn hashVector(vec: Vec<f64>) -> String {
    let mut hash = Sha256::new();

    for &n in &vec {
        hash.update(n.to_le_bytes());
    }

    let result = hash.finalize();
    result.to_vec();
    hex::encode(result)
}

// (Helper) Generate a random vector
pub fn randomVector(dim: usize) -> Vec<f64> {
    let mut out = vec![0.0; dim];
    for i in 0..dim {
        out[i] = rand::thread_rng().gen_range(0..100) as f64;
    }
    return out
}

// Generate a helper point and hash point from a vector
pub fn gen(vec: Vec<f64>, scale: i32) -> (Vec<f64>, String) {
    let mut rdm = randomVector(vec.len() as usize);
    rdm = closest(rdm.clone(), scale);
    let helper = vectorSubtraction(rdm.clone(), vec.clone());
    
    return (helper, hashVector(rdm))
}

// Generate a hashed point given a helper and a vector
pub fn recov(helper: Vec<f64>, vec: Vec<f64>, scale: i32) -> String {
    let mut out = vectorAddition(helper, vec);
    out = closest(out.clone(), scale);
    
    return hashVector(out);
}

// (Helper) Subtract two vectors
pub fn vectorSubtraction(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
    let mut out = vec![0.0; a.len() as usize];
    for i in 0..(a.len() as usize) {
        out[i] = a[i] - b[i];
    }
    return out; 
}

// (Helper) Add two vectors
pub fn vectorAddition(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
    let mut out = vec![0.0; a.len() as usize];
    for i in 0..(a.len() as usize) {
        out[i] = a[i] + b[i];
    }
    return out; 
}
