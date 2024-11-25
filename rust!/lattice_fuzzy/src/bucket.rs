use std::collections::HashMap;
use crate::lattice::Lattice;
use crate::fuzzy_extractor::Fuzzy;
use rand::Rng;

pub struct Bucket {
    pub bucket: HashMap<String, Vec<Vec<f64>>>,
    lattice: Lattice,
    pub param: i32,
    pub scale : f64,
    mult: i32,
    fuzzy_extractor: Fuzzy,
    add: i32
}

impl Bucket {
    pub fn new(param: i32, lattice: Lattice) -> Self {
        Bucket {
            bucket: HashMap::new(),
            param: param,
            lattice: lattice.clone(),
            scale: lattice.scale,
            fuzzy_extractor: Fuzzy::new(lattice.clone()),
            add: rand::thread_rng().gen_range(0..65536),
            mult: rand::thread_rng().gen_range(0..65536)
            // Notes: param = 2 seems to strike a good balance between
            // having too many buckets vs. having large buckets
            // - First risks violating security
            // - Second risks the algorithm being slow, again
        }
    }

    pub fn get_bucket_id(&mut self, vec: Vec<f64>) -> String {
        if self.param as usize > vec.len() {
            panic!("Paramater fail: param > length of vector");
        }
        let closest = self.lattice.closest(vec);
        let mut id_vec = vec![0.0; self.param as usize];
        
        for i in 0..self.param {
            id_vec[i as usize] = closest[((self.mult * i + self.add) % closest.len() as i32) as usize];
        }
        return self.fuzzy_extractor.hash_vector(id_vec);
    }

    pub fn get_bucket_id_with_slack(&mut self, vec: Vec<f64>) -> Vec<String> {
        if self.param as usize > vec.len() {
            panic!("Paramater fail: param > length of vector");
        }
        let closest = self.lattice.closest(vec.clone());
        let mut id_vec = vec![0.0; self.param as usize];
        
        for i in 0..self.param {
            id_vec[i as usize] = closest[((self.mult * i + self.add) % closest.len() as i32) as usize];
        }
        
        let mut out = vec![];

        let vars = self.create_variants(id_vec.clone());

        for v in vars {
            out.push(self.fuzzy_extractor.hash_vector(v));
        }


        let closest = self.lattice.closest(vec);
        let mut id_vec = vec![0.0; self.param as usize];
    
        for i in 0..self.param {
            id_vec[i as usize] = closest[((self.mult * i + self.add) % closest.len() as i32) as usize];
        }
        out.push(self.fuzzy_extractor.hash_vector(id_vec));

        return out;
    }

    pub fn add(&mut self, vec: Vec<f64>) -> bool {
        let id = self.get_bucket_id(vec.clone());

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

    pub fn add_with_slack(&mut self, vec: Vec<f64>) {
        let ids = self.get_bucket_id_with_slack(vec.clone());
        
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

    pub fn display_bucket(&mut self) {
        for (key, arr) in &self.bucket {
            println!("{key} {} -> {:?}", arr.len(), arr);
            println!("");
        }
    }

    pub fn get_bucket_size(&mut self) -> i32 {
        let mut count = 0;
        for (_key, arr) in &self.bucket {
            count +=  arr.len();
        }
        return count as i32;
    }

    pub fn get_bucket_count(&mut self) -> i32 {
        let mut count = 0;
        for (_key, _arr) in &self.bucket {
            count +=  1;
        }
        return count as i32;
    }


    pub fn get_candidates(&mut self, vec: Vec<f64>) -> Vec<Vec<f64>> {
        let id = self.get_bucket_id(vec.clone());
        
        if self.bucket.contains_key(&id) {
            if let Some(x) = self.bucket.get_mut(&id) {
                return x.clone();
            } 
        }

        return vec![];
    }

    pub fn get_candidates_with_slack(&mut self, vec: Vec<f64>) -> Vec<Vec<f64>> {
        let ids = self.get_bucket_id_with_slack(vec.clone());
        
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

    pub fn get_candidates_from_id(&mut self, id: String) -> Vec<Vec<f64>> {
        if self.bucket.contains_key(&id) {
            if let Some(x) = self.bucket.get_mut(&id) {
                return x.clone();
            } 
        }

        return vec![];
    }

    pub fn create_variants_thin(&mut self, vec: Vec<f64>) -> Vec<Vec<f64>> {
        fn helper(vec: &Vec<f64>, index: usize, current: &mut Vec<f64>, result: &mut Vec<Vec<f64>>, scale: f64) {
            if index == vec.len() {
                result.push(current.clone());
            } else {
                current.push(vec[index] + scale);
                helper(vec, index + 1, current, result, scale);
                current.pop();
                
                current.push(vec[index] - scale);
                helper(vec, index + 1, current, result, scale);
                current.pop();
            }
        }
    
        let mut result = Vec::new();
        let mut current = Vec::new();
        helper(&vec, 0, &mut current, &mut result, self.scale as f64);
        return result;
    }


    pub fn create_variants(&mut self, vec: Vec<f64>) -> Vec<Vec<f64>> {
        fn helper(vec: &Vec<f64>, index: usize, current: &mut Vec<f64>, result: &mut Vec<Vec<f64>>, scale: f64) {
            if index == vec.len() {
                result.push(current.clone());
            } else {
                current.push(vec[index] + scale);
                helper(vec, index + 1, current, result, scale);
                current.pop();
                
                current.push(vec[index] - scale);
                helper(vec, index + 1, current, result, scale);
                current.pop();

                current.push(vec[index]);
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

impl Clone for Bucket {
    fn clone(&self) -> Bucket {
        Bucket {
            bucket: self.bucket.clone(),
            param: self.param,
            scale: self.scale,
            lattice: self.lattice.clone(),
            fuzzy_extractor: self.fuzzy_extractor.clone(),
            add: self.add,
            mult: self.mult
        }
    }
}
