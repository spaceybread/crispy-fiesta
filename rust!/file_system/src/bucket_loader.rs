extern crate fuzzy;
use npyz;
use fuzzy::bucket::GaussBucket;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use std::io::BufRead;
use std::fs::read_to_string;

fn make_specifier_file(scale: i32, param: i32) {
    let filename = format!("db/spec.txt");
    let data = scale.to_string() + "\n" + &param.to_string();
    fs::write(filename, data).expect("Could not make specifier file!");
}

fn make_bucket_files(map: HashMap<String, Vec<Vec<f64>>>) {
    for (key, value) in &map {
        let filename = format!("db/{}.txt", key);
        let data = get_vector_in_format(value);
        fs::write(filename, data).expect("Unable to make bucket file");
    }
}

pub fn make_data_file(vecs: Vec<Vec<f64>>, name: String) {
    let filename = format!("db/{}.txt", name);
    let ln = get_vector_in_format(&vecs);
    fs::write(filename, ln).expect("Unable to make full db file!");
}

fn get_vector_in_format(bucketData: &Vec<Vec<f64>>) -> String {
    bucketData.iter()
        .map(|inner_vec| inner_vec.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(", "))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn make_files_from_bucket(bucket: GaussBucket) {
    make_specifier_file(bucket.scale, bucket.param);
    make_bucket_files(bucket.bucket);
}

fn parse_vector(input: &str) -> Vec<f64> {
    input
        .split(',')
        .filter_map(|s| s.trim().parse::<f64>().ok())
        .collect()
}

pub fn read_file_to_vec(filename: &str) -> Vec<Vec<f64>>{
    let mut out: Vec<Vec<f64>> = vec![];
    let binding = match read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            return out;
        }
    };

    let file = binding.lines();

    for line in file {
        let x = parse_vector(line);
        out.push(x);
    }

    return out;
}

pub fn get_bucket_from_data() -> GaussBucket {
    let binding = match read_to_string("db/spec.txt") {
        Ok(content) => content,
        Err(e) => {
            panic!("Spec.txt could not be read!");
        }
    };

    let file = binding.lines();
    
    let mut out: Vec<i32> = vec![];
    for line in file {
        out.push(line.parse::<i32>().unwrap());
    }

    return GaussBucket::new(out[0], out[1]);
}

pub fn handle_queries(mut bucket: GaussBucket, vec: Vec<f64>) -> Vec<Vec<f64>> {
    let mut pout: Vec<Vec<f64>> = vec![];

    let ids = bucket.getBucketIDWithSlack(vec);

    for id in ids {
        let filename = format!("db/{}.txt", id);
        let mut x = read_file_to_vec(&filename);
        pout.append(&mut x);
    }

    let mut out: Vec<Vec<f64>> = vec![];
    
    for v in pout {
        if !out.contains(&v) {
            out.push(v.clone());
        }    
    }

    return out; 
}