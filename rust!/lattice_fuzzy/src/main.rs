mod lattice;
mod fuzzy_extractor;
mod bucket;

static LATTICE_NAME: &str = "GAUSS_INF";

fn main() {
    println!("Hello, world!");
    let mut cow = lattice::Lattice::new(LATTICE_NAME.to_string(), 1.0, 0.5);
    cow.init();
    let vec = vec![0.3, 12.2, 4.1, 53.2, -1.5, 1.2, 0.3, 12.2, 4.1, 53.2, -1.5, 1.2, 0.3, 12.2, 4.1, 53.2, -1.5, 1.2, 0.3, 12.2, 4.1, 53.2, -1.5, 1.2];
    let v1 = cow.closest(vec);
    println!("{:?}", v1);
    let v2 = cow.closest(v1.clone());
    println!("{:?}", v2);
    println!("{:?}", v1 == v2);
    let mut fuzzy = fuzzy_extractor::Fuzzy::new(cow.clone());
    let mut bucket = bucket::Bucket::new(2, cow.clone());
}
