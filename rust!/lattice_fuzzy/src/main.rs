mod lattice;
mod fuzzy_extractor;
mod bucket;
mod file_loader;

static LATTICE_NAME: &str = "GAUSS_INF";

fn rehersal() {
    let test_data = file_loader::get_vectors_from_file("../../test_data/embeddings.txt");
    println!("Size: {}", test_data.len());
    let mut lat = lattice::Lattice::new(LATTICE_NAME.to_string(), 2.0 * 0.1);
    lat.init();
    let mut bucket = bucket::Bucket::new(2, lat.clone());
    let fuzzy = fuzzy_extractor::Fuzzy::new(lat);
    println!("Init completed");
    for vec in &test_data {
        bucket.add(vec.clone());
    }
    println!("DB built");

    let queries = vec![test_data[0].clone(), test_data[test_data.len() -1].clone()];

    for vec in &queries {
        let cands = bucket.get_candidates_with_slack(vec.clone());
        println!("Found {} vectors in the bucket", cands.len());
        let res = fuzzy.gen(vec.clone());
        let mut count = 0; 
        for cand in cands {
            let rec = fuzzy.recov(res.0.clone(), cand.clone());
            if rec == res.1 {
                count += 1;
            }
        }
        println!("Count: {count}");
    }
}

fn main() {
    rehersal();
}
