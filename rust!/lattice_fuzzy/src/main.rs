mod lattice;
mod fuzzy_extractor;
mod bucket;
mod file_loader;

static GAUSS_LATTICE_NAME: &str = "GAUSS_INF";
static LEECH_24_LATTICE_NAME: &str = "LEECH_24";

fn _rehersal() {
    let test_data = file_loader::get_vectors_from_file("../../test_data/embeddings.txt");
    println!("Size: {}", test_data.len());
    let mut lat = lattice::Lattice::new(GAUSS_LATTICE_NAME.to_string(), 2.0 * 0.06);
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

fn _viktor_nation() {
    let mut lat = lattice::Lattice::new(GAUSS_LATTICE_NAME.to_string(), 0.3);
    lat.init();
    // 2.0 * 0.55
    let fuzzy = fuzzy_extractor::Fuzzy::new(lat);

    let data_size = 200;
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
    println!("Gauss: {}/{}", count, data_size);
    // let _ = file_loader::make_file_from_i32_vec(matches, "../../test_data/matches/pairs.txt");
}

fn _heimerdinger_fan() {
    let mut lat = lattice::Lattice::new(LEECH_24_LATTICE_NAME.to_string(), 2.0 * 0.6175562);
    lat.init();
    // 2.0 * 0.55
    let fuzzy = fuzzy_extractor::Fuzzy::new(lat);

    let data_size = 400;
    let jaybe = file_loader::get_vectors_from_file("../../test_data/matches/v1.txt");
    let jaybe_not = file_loader::get_vectors_from_file("../../test_data/matches/v2.txt");
    
    println!("data loaded!");
    
    let mut matches = vec![];
    let mut count = 0;

    for i in 0..data_size {
        if i % 10 == 0 {println!("At {} with {}", i, count);}
        let v1 = jaybe[i].clone();
        let v2 = jaybe_not[i].clone();

        let res = fuzzy.gen_oversize(v1.clone());
        let rec = fuzzy.recov_oversize(res.0.clone(), v2);

        let mut flag = true;
        for x in 0..rec.len() {
            if rec[x] != res.1[x] {
                matches.push(0);
                flag = false; 
                break;
            }
        }

        if flag {
            matches.push(1);
            count += 1;
        }
    }
    println!("Leech: {}/{}", count, data_size);
    let _ = file_loader::make_file_from_i32_vec(matches, "../../test_data/matches/pairs_leech.txt");
}

fn _debug() {
    let mut lat = lattice::Lattice::new(GAUSS_LATTICE_NAME.to_string(), 1.0);
    lat.init();
    let fuzzy = fuzzy_extractor::Fuzzy::new(lat);

    let a = vec![1.0; 24];
    let b = vec![1.0; 24];

    let res = fuzzy.gen(a.clone());
    let rec = fuzzy.recov(res.0.clone(), b);

    if rec == res.1 {
        println!("match!");
    } else {
        println!("no match!");
    }


}

fn sweep_func(jaybe: Vec<Vec<f64>>, jaybe_not: Vec<Vec<f64>>, scale: f64) -> (f64, f64, f64) {
    let mut lat = lattice::Lattice::new(GAUSS_LATTICE_NAME.to_string(), scale);
    lat.init();
    // 2.0 * 0.55
    let fuzzy = fuzzy_extractor::Fuzzy::new(lat);

    let data_size = 400;
    
    // println!("data loaded!");
    let mut true_count = 0;
    let mut false_count = 0;

    for i in 0..data_size {
        let v1 = jaybe[i].clone();
        let v2 = jaybe_not[i].clone();

        let res = fuzzy.gen(v1.clone());
        let rec = fuzzy.recov(res.0.clone(), v2);

        if rec == res.1 {
            if i < 200 {
                true_count += 1;
            } else {
                false_count += 1;
            }
        }
    }
    // println!("Gauss: {}/{} and {}/{}", true_count, data_size / 2, false_count, data_size / 2);

    return (scale, 2.0 * true_count as f64 / data_size as f64, 2.0 * false_count as f64 / data_size as f64)
}

fn sweep() {
    let jaybe = file_loader::get_vectors_from_file("../../test_data/matches/v1.txt");
    let jaybe_not = file_loader::get_vectors_from_file("../../test_data/matches/v2.txt");
    let mut results = vec![];
    for i in 0..10000 {
        if i % 10 == 0 {println!("{}", i);}
        results.push(sweep_func(jaybe.clone(), jaybe_not.clone(), i as f64 / 10000.0));
    }
    let _ = file_loader::write_tuples_to_file(results, "../../test_data/matches/pairs.txt");
}
fn sweep_3() {
    let jaybe = file_loader::get_vectors_from_file("../../test_data/matches/v1.txt");
    let jaybe_not = file_loader::get_vectors_from_file("../../test_data/matches/v2.txt");
    let mut results = vec![];
    results.push(sweep_func(jaybe.clone(), jaybe_not.clone(), 0.3));

    println!("{:?}", results);
}

fn main() {
    sweep_3();
    // sweep();
    // viktor_nation();
    // _heimerdinger_fan();
    // debug();
}
