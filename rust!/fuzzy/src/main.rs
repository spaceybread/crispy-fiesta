use sha2::{Sha256, Digest};
use nalgebra::{DMatrix, DVector};
use std::env;

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


fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let dim = 16;
    let lat = makeLattice(3, dim);

    let mut vec = vec![0.0; dim];
    
    for i in 0..dim{
        vec[i] = (i as f64) * 1.5;
    }
    println!("{:?}", vec.clone());

    let close = closest(vec.clone(), lat);

    println!("{:?}", close.clone());

}
