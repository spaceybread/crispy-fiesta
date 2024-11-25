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
        
        return (helper, self.hash_vector(rdm))
    }
    
    pub fn recov(&self, helper: Vec<f64>, vec: Vec<f64>) -> String {
        let mut out = self.vector_addition(helper, vec);
        out = self.lattice.closest(out.clone());
        
        return self.hash_vector(out);
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
}

impl Clone for Fuzzy {
    fn clone(&self) -> Fuzzy {
        Fuzzy {
            lattice: self.lattice.clone()
        }
    }
}