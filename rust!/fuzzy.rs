fn makeLattice(scale: i32, dim: usize) -> Vec<Vec<f64>> {
    let mut lattice = vec![vec![0.0; dim]; dim];

    for i in 0..dim {
        for j in 0..dim {
            if i == j {
                lattice[i][j] = scale as f64;
            }
        }
    }
    return lattice;
}

fn printLattice(lattice: Vec<Vec<f64>>) {
    for row in lattice {
        println!("{:?}", row);
    }
}


fn main() {
    printLattice(makeLattice(4, 16));
}
