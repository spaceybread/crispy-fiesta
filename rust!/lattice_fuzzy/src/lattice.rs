use nalgebra::{DMatrix, DVector};

pub struct Lattice {
    name: String,
    matrix: Vec<f64>,
    pub dim:i32,
    pub scale: f64,
    pub threshold: f64
}

impl Lattice {
    const LEACH: &'static str = "LEECH_24";
    const GAUSS: &'static str = "GAUSS_INF";

    pub fn new(name: String, scale: f64, threshold: f64) -> Self {
        Lattice {
            name: name,
            matrix: vec![0.0],
            dim: 0,
            scale: scale,
            threshold: threshold
        }
    }

    pub fn init(&mut self) {
        
        if self.name == Self::LEACH {
            self.dim = 24;
            self.matrix = vec![8.0,4.0,4.0,4.0,4.0,4.0,4.0,2.0,4.0,4.0,4.0,2.0,4.0,2.0,2.0,2.0,4.0,2.0,2.0,2.0,0.0,0.0,0.0,-3.0,4.0,4.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,1.0,2.0,1.0,1.0,2.0,1.0,0.0,0.0,-1.0,4.0,2.0,4.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,2.0,1.0,2.0,2.0,1.0,1.0,1.0,0.0,0.0,-1.0,4.0,2.0,2.0,4.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,1.0,2.0,2.0,1.0,2.0,1.0,1.0,0.0,0.0,-1.0,4.0,2.0,2.0,2.0,4.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,0.0,0.0,-1.0,4.0,2.0,2.0,2.0,2.0,4.0,2.0,2.0,2.0,2.0,2.0,1.0,2.0,2.0,1.0,1.0,2.0,1.0,2.0,1.0,0.0,0.0,0.0,-1.0,4.0,2.0,2.0,2.0,2.0,2.0,4.0,2.0,2.0,2.0,2.0,1.0,2.0,1.0,2.0,1.0,2.0,1.0,1.0,2.0,0.0,0.0,0.0,-1.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,4.0,1.0,1.0,1.0,2.0,1.0,2.0,2.0,2.0,1.0,2.0,2.0,2.0,2.0,0.0,0.0,1.0,4.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,4.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,1.0,1.0,-1.0,4.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,2.0,4.0,2.0,2.0,2.0,2.0,1.0,1.0,2.0,2.0,1.0,1.0,0.0,1.0,0.0,-1.0,4.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,2.0,2.0,4.0,2.0,2.0,1.0,2.0,1.0,2.0,1.0,2.0,1.0,0.0,0.0,1.0,-1.0,2.0,2.0,2.0,2.0,1.0,1.0,1.0,2.0,2.0,2.0,2.0,4.0,1.0,2.0,2.0,2.0,1.0,2.0,2.0,2.0,2.0,1.0,1.0,1.0,4.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,2.0,2.0,2.0,1.0,4.0,2.0,2.0,2.0,2.0,1.0,1.0,1.0,1.0,1.0,1.0,-1.0,2.0,2.0,1.0,1.0,2.0,2.0,1.0,2.0,2.0,2.0,1.0,2.0,2.0,4.0,2.0,2.0,1.0,2.0,2.0,2.0,2.0,2.0,1.0,1.0,2.0,1.0,2.0,1.0,2.0,1.0,2.0,2.0,2.0,1.0,2.0,2.0,2.0,2.0,4.0,2.0,1.0,2.0,2.0,2.0,2.0,1.0,2.0,1.0,2.0,1.0,1.0,2.0,2.0,1.0,1.0,2.0,2.0,1.0,1.0,2.0,2.0,2.0,2.0,4.0,1.0,2.0,2.0,2.0,2.0,1.0,1.0,1.0,4.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,2.0,2.0,2.0,1.0,2.0,1.0,1.0,1.0,4.0,2.0,2.0,2.0,1.0,1.0,1.0,-1.0,2.0,1.0,2.0,1.0,2.0,1.0,1.0,2.0,2.0,2.0,1.0,2.0,1.0,2.0,2.0,2.0,2.0,4.0,2.0,2.0,2.0,2.0,1.0,1.0,2.0,1.0,1.0,2.0,2.0,2.0,1.0,2.0,2.0,1.0,2.0,2.0,1.0,2.0,2.0,2.0,2.0,2.0,4.0,2.0,2.0,1.0,2.0,1.0,2.0,2.0,1.0,1.0,2.0,1.0,2.0,2.0,2.0,1.0,1.0,2.0,1.0,2.0,2.0,2.0,2.0,2.0,2.0,4.0,2.0,1.0,1.0,1.0,0.0,1.0,1.0,1.0,1.0,0.0,0.0,2.0,1.0,0.0,0.0,2.0,1.0,2.0,2.0,2.0,1.0,2.0,2.0,2.0,4.0,2.0,2.0,2.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,1.0,0.0,1.0,1.0,2.0,1.0,1.0,1.0,2.0,1.0,1.0,2.0,4.0,2.0,2.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,1.0,1.0,1.0,1.0,2.0,1.0,1.0,1.0,2.0,1.0,2.0,2.0,4.0,2.0,-3.0,-1.0,-1.0,-1.0,-1.0,-1.0,-1.0,1.0,-1.0,-1.0,-1.0,1.0,-1.0,1.0,1.0,1.0,-1.0,1.0,1.0,1.0,2.0,2.0,2.0,4.0];
            for i in 0..self.matrix.len() {
                self.matrix[i] *= self.scale;
            }
        }

        if self.name == Self::GAUSS {
            self.dim = -1;
        }
    }
    
    pub fn round(&self, val: f64) -> f64 {
        let val_thresh = (val % self.scale) / self.scale;
        if val_thresh >= self.threshold {
            return val - (val % self.scale) + self.scale;
        }
        return val - (val % self.scale);
    }

    pub fn round_vector_for_leech(&self, vec: Vec<f64>) -> Vec<f64> {
        let mut out = vec![0.0; 24];

        for i in 0..vec.len() {
            let val = vec[i];
            let val_thresh = val % 1.0;
            if val_thresh >= self.threshold {
                out[i] = val.ceil();
            } else {
                out[i] = val.floor();
            }
        }

        return out;
    }

    fn closest_gauss(&self, mut vec: Vec<f64>) -> Vec<f64> {
        for i in 0..vec.len() {
            vec[i] = self.round(vec[i]);
        }
        return vec;
    }

    fn closest_leech(&self, vec: Vec<f64>) -> Vec<f64> {
        let p = DVector::from_vec(vec);
        let b = DMatrix::from_vec(24, 24, self.matrix.clone());
        if  b.is_invertible() == false {
            panic!("Failed finding closest in the Leech Lattice");
        }
        let inv_b = b.clone().try_inverse().unwrap();
        let coeff = inv_b * p;
        let coeff_vec = coeff.data.as_vec().clone();
        let rounded = DVector::from_vec(self.round_vector_for_leech(coeff_vec));
        let almost_out = b * rounded;

        return almost_out.data.as_vec().clone();
    }

    pub fn split_and_pad(&self, vec: Vec<f64>, chunk_size: usize) -> Vec<Vec<f64>> {
        let mut result = vec![vec![0.0; chunk_size]; (vec.len() + chunk_size - 1) / chunk_size];
        for (i, &val) in vec.iter().enumerate() {
            result[i / chunk_size][i % chunk_size] = val;
        }
        return result;
    }

    pub fn closest(&self, vector: Vec<f64>) -> Vec<f64> {

        if self.name == Self::GAUSS {
            return self.closest_gauss(vector);
        }

        if self.name == Self::LEACH {
            return self.closest_leech(vector);
        }

        return vec![0.0];
    }

}

impl Clone for Lattice {
    fn clone(&self) -> Lattice {
        Lattice {
            name: self.name.clone(),
            matrix: self.matrix.clone(),
            dim: self.dim,
            scale: self.scale,
            threshold: self.threshold
        }
    }
}