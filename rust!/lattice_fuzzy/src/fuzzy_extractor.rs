use crate::lattice::Lattice;
use sha2::{Sha256, Digest};
use rand::Rng;

pub struct Fuzzy {
    lattice: Lattice
}

impl Fuzzy {
    pub fn new(lattice: Lattice) -> Self {
        Fuzzy {
            lattice: lattice
        }
    }

    pub fn gen(&self, vec: Vec<f64>) -> (Vec<f64>, String) {
        let mut rdm = self.random_vector(vec.len() as usize);
        rdm = self.lattice.closest(rdm.clone());
        let helper = self.vector_subtraction(rdm.clone(), vec.clone());
        
        return (helper, self.hash_vector_2(rdm))
    }
    
    pub fn recov(&self, helper: Vec<f64>, vec: Vec<f64>) -> String {
        let mut out = self.vector_addition(helper, vec);
        out = self.lattice.closest(out.clone());
        
        return self.hash_vector_2(out);
    }

    fn vector_subtraction(&self, a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
        let mut out = vec![0.0; a.len() as usize];
        for i in 0..(a.len() as usize) {
            out[i] = a[i] - b[i];
        }
        return out; 
    }
    

    fn vector_addition(&self, a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
        let mut out = vec![0.0; a.len() as usize];
        for i in 0..(a.len() as usize) {
            out[i] = a[i] + b[i];
        }
        return out; 
    }

    fn random_vector(&self, dim: usize) -> Vec<f64> {
        let mut out = vec![0.0; dim];
        for i in 0..dim {
            out[i] = rand::thread_rng().gen_range(0..100) as f64;
        }
        return out
    }

    pub fn hash_vector(&self, vec: Vec<f64>) -> String {
        let mut hash = Sha256::new();
        for &n in &vec {
            hash.update(n.to_le_bytes());
        }
        let result = hash.finalize();
        result.to_vec();
        hex::encode(result)
    }

    pub fn hash_vector_2(&self, vec: Vec<f64>) -> String {
        let mut hash = Sha256::new();
        let mut weird_al = 0.0; 
        for &n in &vec {
            weird_al += n;
            hash.update(weird_al.to_le_bytes());
        }
        let result = hash.finalize();
        // return weird_al.to_string();
        hex::encode(result)
    }

    pub fn gen_oversize(&self, vecs: Vec<f64>) -> (Vec<Vec<f64>>, Vec<String>) {
        if self.lattice.dim < 1 {panic!("what are you doing");}
        let split = self.split_and_pad(vecs, self.lattice.dim as usize, 0.0);
        let mut helpers = vec![];
        let mut keys = vec![];

        for vec in split {
            let mut rdm = self.random_vector(vec.len() as usize);
            rdm = self.lattice.closest(rdm.clone());
            let helper = self.vector_subtraction(rdm.clone(), vec.clone());
            helpers.push(helper);
            keys.push(self.hash_vector(rdm));
        }
        
        return (helpers, keys);
    }
    
    pub fn recov_oversize(&self, helpers: Vec<Vec<f64>>, vec: Vec<f64>) -> Vec<String> {
        if self.lattice.dim < 1 {panic!("what are you doing");}
        let vecs = self.split_and_pad(vec, self.lattice.dim as usize, 0.0);

        let mut recovs = vec![];
        for i in 0..vecs.len() {
            let mut out = self.vector_addition(helpers[i].clone(), vecs[i].clone());
            out = self.lattice.closest(out.clone());
            recovs.push(self.hash_vector(out));
        }
        return recovs;
    }
    
    
    fn split_and_pad(&self, vec: Vec<f64>, chunk_size: usize, pad_value: f64) -> Vec<Vec<f64>> {
        let mut result = vec![vec![pad_value; chunk_size]; (vec.len() + chunk_size - 1) / chunk_size];
        for (i, &val) in vec.iter().enumerate() {
            result[i / chunk_size][i % chunk_size] = val;
        }
        return result;
    }
}

impl Clone for Fuzzy {
    fn clone(&self) -> Fuzzy {
        Fuzzy {
            lattice: self.lattice.clone()
        }
    }
}