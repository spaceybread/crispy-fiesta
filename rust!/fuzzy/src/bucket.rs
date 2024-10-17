use std::collections::HashMap;
use std::collections::HashSet;
use sha2::{Sha256, Digest};
use crate::fuzzyImpl;
use crate::gaussFuzzy;

pub struct Bucket {
    bucket: HashMap<String, Vec<Vec<f64>>>,
    lattice: Vec<f64>,
    param: i32,
    dim:i32,
}

impl Bucket {
    pub fn new(lattice: Vec<f64>, dim:i32, param: i32) -> Self {
        Bucket {
            bucket: HashMap::new(),
            lattice: lattice,
            dim: dim,
            param: param,
            // Notes: param = 2 seems to strike a good balance between
            // having too many buckets vs. having large buckets
            // - First risks violating security
            // - Second risks the algorithm being slow, again
        }
    }

    pub fn getBucketID(&mut self, vec: Vec<f64>) -> String {
        if self.param as usize > vec.len() {
            panic!("Paramater fail: param > length of vector");
        }
        let closest = fuzzyImpl::closest1D(vec, self.lattice.clone(), self.dim);
        let mut idVec = vec![0.0; self.param as usize];
    
        for i in 0..self.param {
            idVec[i as usize] = closest[((3 * i) % closest.len() as i32) as usize];
        }
        return fuzzyImpl::hashVector(idVec);
    }

    pub fn add(&mut self, vec: Vec<f64>) -> bool {
        let id = self.getBucketID(vec.clone());

        if self.bucket.contains_key(&id) {
            if let Some(x) = self.bucket.get_mut(&id) {
                x.push(vec);
                return true;
            } 
        } else {
            self.bucket.insert(id, vec![vec]);
            return true;
        }
        
        return false;
    }

    pub fn displayBucket(&mut self) {
        for (key, arr) in &self.bucket {
            println!("{key} {} -> {:?}", arr.len(), arr);
            println!("");
        }
    }

    pub fn getBucketSize(&mut self) -> i32 {
        let mut count = 0;
        for (key, arr) in &self.bucket {
            count +=  arr.len();
        }
        return count as i32;
    }

    pub fn getBucketCount(&mut self) -> i32 {
        let mut count = 0;
        for (key, arr) in &self.bucket {
            count +=  1;
        }
        return count as i32;
    }


    pub fn getCandidates(&mut self, vec: Vec<f64>) -> Vec<Vec<f64>> {
        let id = self.getBucketID(vec.clone());
        
        if self.bucket.contains_key(&id) {
            if let Some(x) = self.bucket.get_mut(&id) {
                return x.clone();
            } 
        }

        panic!("ID not in the bucket");
    }

    pub fn getCandidatesFromID(&mut self, id: String) -> Vec<Vec<f64>> {
        if self.bucket.contains_key(&id) {
            if let Some(x) = self.bucket.get_mut(&id) {
                return x.clone();
            } 
        }

        panic!("ID not in the bucket");
    }

}

pub struct GaussBucket {
    bucket: HashMap<String, Vec<Vec<f64>>>,
    param: i32,
    pub scale : i32,
}

impl GaussBucket {
    pub fn new(scale: i32, param: i32) -> Self {
        GaussBucket {
            bucket: HashMap::new(),
            scale: scale,
            param: param,
            // Notes: param = 2 seems to strike a good balance between
            // having too many buckets vs. having large buckets
            // - First risks violating security
            // - Second risks the algorithm being slow, again
        }
    }

    pub fn getBucketID(&mut self, vec: Vec<f64>) -> String {
        if self.param as usize > vec.len() {
            panic!("Paramater fail: param > length of vector");
        }
        let closest = gaussFuzzy::closest(vec, self.scale);
        let mut idVec = vec![0.0; self.param as usize];
    
        for i in 0..self.param {
            idVec[i as usize] = closest[((3 * i) % closest.len() as i32) as usize];
        }
        return gaussFuzzy::hashVector(idVec);
    }

    pub fn getBucketIDWithSlack(&mut self, vec: Vec<f64>) -> Vec<String> {
        if self.param as usize > vec.len() {
            panic!("Paramater fail: param > length of vector");
        }
        let closest = gaussFuzzy::closest(vec.clone(), self.scale);
        let mut idVec = vec![0.0; self.param as usize];
        
        for i in 0..self.param {
            idVec[i as usize] = closest[((3 * i) % closest.len() as i32) as usize];
        }
        
        let mut out = vec![];

        let vars = self.createVariants(idVec.clone());

        for v in vars {
            out.push(gaussFuzzy::hashVector(v));
        }


        let closest = gaussFuzzy::closest(vec, self.scale);
        let mut idVec = vec![0.0; self.param as usize];
    
        for i in 0..self.param {
            idVec[i as usize] = closest[((3 * i) % closest.len() as i32) as usize];
        }
        out.push(gaussFuzzy::hashVector(idVec));

        return out;
    }

    pub fn add(&mut self, vec: Vec<f64>) -> bool {
        let id = self.getBucketID(vec.clone());

        if self.bucket.contains_key(&id) {
            if let Some(x) = self.bucket.get_mut(&id) {
                x.push(vec);
                return true;
            } 
        } else {
            self.bucket.insert(id, vec![vec]);
            return true;
        }
        
        return false;
    }

    pub fn addWithSlack(&mut self, vec: Vec<f64>) {
        let ids = self.getBucketIDWithSlack(vec.clone());
        
        for id in ids {
            if self.bucket.contains_key(&id) {
                if let Some(x) = self.bucket.get_mut(&id) {
                    x.push(vec.clone());
                } 
            } else {
                self.bucket.insert(id, vec![vec.clone()]);
            }
        }
    }

    pub fn displayBucket(&mut self) {
        for (key, arr) in &self.bucket {
            println!("{key} {} -> {:?}", arr.len(), arr);
            println!("");
        }
    }

    pub fn getBucketSize(&mut self) -> i32 {
        let mut count = 0;
        for (key, arr) in &self.bucket {
            count +=  arr.len();
        }
        return count as i32;
    }

    pub fn getBucketCount(&mut self) -> i32 {
        let mut count = 0;
        for (key, arr) in &self.bucket {
            count +=  1;
        }
        return count as i32;
    }


    pub fn getCandidates(&mut self, vec: Vec<f64>) -> Vec<Vec<f64>> {
        let id = self.getBucketID(vec.clone());
        
        if self.bucket.contains_key(&id) {
            if let Some(x) = self.bucket.get_mut(&id) {
                return x.clone();
            } 
        }

        return vec![];
    }

    pub fn getCandidatesWithSlack(&mut self, vec: Vec<f64>) -> Vec<Vec<f64>> {
        let ids = self.getBucketIDWithSlack(vec.clone());
        
        let mut out = vec![];

        for id in ids {
            if self.bucket.contains_key(&id) {
                if let Some(x) = self.bucket.get_mut(&id) {
                    for v in x {
                        if !out.contains(v) {
                            out.push(v.clone());
                        }    
                    }
                } 
            }
        }
        return out;
    }

    pub fn getCandidatesFromID(&mut self, id: String) -> Vec<Vec<f64>> {
        if self.bucket.contains_key(&id) {
            if let Some(x) = self.bucket.get_mut(&id) {
                return x.clone();
            } 
        }

        return vec![];
    }

    pub fn createVariants(&mut self, vec: Vec<f64>) -> Vec<Vec<f64>> {
        fn helper(vec: &Vec<f64>, index: usize, current: &mut Vec<f64>, result: &mut Vec<Vec<f64>>, scale: f64) {
            if index == vec.len() {
                result.push(current.clone());
            } else {
                current.push(vec[index] + 0.5 * scale);
                helper(vec, index + 1, current, result, scale);
                current.pop();
                
                current.push(vec[index] - 0.5 * scale);
                helper(vec, index + 1, current, result, scale);
                current.pop();
            }
        }
    
        let mut result = Vec::new();
        let mut current = Vec::new();
        helper(&vec, 0, &mut current, &mut result, self.scale as f64);
        return result;
    }

}

impl Clone for GaussBucket {
    fn clone(&self) -> GaussBucket {
        GaussBucket {
            bucket: self.bucket.clone(),
            param: self.param,
            scale: self.scale,
        }
    }
}

impl Clone for Bucket {
    fn clone(&self) -> Bucket {
        Bucket {
            bucket: self.bucket.clone(),
            param: self.param,
            lattice: self.lattice.clone(),
            dim: self.dim,
        }
    }
}








