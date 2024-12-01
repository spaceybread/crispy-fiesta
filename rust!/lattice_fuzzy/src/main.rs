mod lattice;
mod fuzzy_extractor;
mod bucket;
mod file_loader;

static LATTICE_NAME: &str = "GAUSS_INF";

fn _rehersal() {
    let test_data = file_loader::get_vectors_from_file("../../test_data/embeddings.txt");
    println!("Size: {}", test_data.len());
    let mut lat = lattice::Lattice::new(LATTICE_NAME.to_string(), 2.0 * 0.06225556);
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

fn viktor_nation() {
    let lat = lattice::Lattice::new(LATTICE_NAME.to_string(), 999.0);
    // 2.0 * 0.55
    let fuzzy = fuzzy_extractor::Fuzzy::new(lat);

    let data_size = 5000;
    let jaybe = file_loader::get_vectors_from_file("../../test_data/matches/v1.txt");
    let jaybe_not = file_loader::get_vectors_from_file("../../test_data/matches/v2.txt");
    
    println!("data loaded!");
    
    let mut matches = vec![];
    let mut count = 0;

    for i in 0..data_size {
        let v1 = jaybe[i].clone();
        let v2 = jaybe_not[i].clone();

        let res = fuzzy.gen(v1.clone());
        let rec = fuzzy.recov(res.0.clone(), v2);

        if rec == res.1 {
            matches.push(1);
            count += 1;
        } else {
            matches.push(0);
        }
    }
    println!("{}/{}", count, data_size);
    file_loader::make_file_from_i32_vec(matches, "../../test_data/matches/pairs.txt");
}

fn main() {
    viktor_nation();
}
