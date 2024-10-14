use sha2::{Sha256, Digest};
use nalgebra::{DMatrix, DVector};
use std::env;
use rand::Rng;
use std::time::Instant;
use crate::fuzzyImpl;
use crate::bucket::GaussBucket;
use crate::bucket;
use crate::gaussFuzzy;

pub fn makeHelpersGauss(queries: Vec<Vec<f64>>, mut bucket: GaussBucket) -> (Vec<String>, Vec<Vec<f64>>, Vec<String>) {
    let mut keys = vec![];
    let mut helpers = vec![];
    let mut buckets = vec![];

    for qu in queries {
        let res = gaussFuzzy::gen(qu.clone(), bucket.scale);
        let bid = bucket.getBucketID(qu.clone());
        keys.push(res.1);
        helpers.push(res.0);
        buckets.push(bid);
    }
    return (keys, helpers, buckets);
}

pub fn attemptMatchingGauss(set: Vec<Vec<f64>>, mut bucket: GaussBucket, helpers: Vec<Vec<f64>>, buckets: Vec<String>) -> Vec<Vec<String>> {
    let mut out: Vec<Vec<String>> = vec![];

    for i in 0..helpers.len() {
        let mut tmp: Vec<String> = vec![];
        let cands = bucket.getCandidatesFromID(buckets[i].clone());
        
        for can in cands {
            let rec = gaussFuzzy::recov(helpers[i].clone(), can, bucket.scale);
            tmp.push(rec);
        }
        out.push(tmp);
    } 

    return out; 
}

pub fn returnMatches(cands: Vec<Vec<String>>, keys: Vec<String>, queries: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut out: Vec<Vec<f64>> = vec![]; 

    for i in 0..queries.len() {
        for j in 0..cands[i].len() {
            if keys[i] == cands[i][j] {
                out.push(queries[i].clone());
            }
        }
    }

    return out;

}