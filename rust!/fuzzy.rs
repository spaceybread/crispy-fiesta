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

        if (val - val.floor() > threshold) {
            out[i] = val.ceil();
        } else {out[i] = val.floor();}
    }

    return out;
}


fn main() {
    let mut test = vec![0.5; 4];
    test = round(test, 0.5);

    for val in test {
        println!("{:?}", val);
    }
}
