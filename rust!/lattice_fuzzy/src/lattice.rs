use nalgebra::{DMatrix, DVector};

pub struct Lattice {
    name: String,
    matrix: Vec<f64>,
    pub dim: i32,
    pub scale: f64,
    init: bool
}

impl Lattice {
    const LEACH: &'static str = "LEECH_24";
    const GAUSS: &'static str = "GAUSS_INF";
    const E8_LAT: &'static str = "E8";

    pub fn new(name: String, scale: f64) -> Self {
        let mut lattice = Lattice {
            name: name,
            matrix: vec![0.0],
            dim: 0,
            scale: scale,
            init: false,
        };
        lattice.init();
        lattice
    }

    fn check_init(&self) {
        if self.init == false {
            panic!("Lattice has to be initialised!");
        }
    }

    pub fn init(&mut self) {
        
        if self.name == Self::LEACH {
            self.dim = 24;
            self.matrix = vec![0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,-1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,-1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,-1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,-1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,-1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,-1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,-1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,-1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,-1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,-1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,];
            for i in 0..self.matrix.len() {
                self.matrix[i] *= self.scale;
            }
        }

        if self.name == Self::E8_LAT {
            self.dim = 8;
            self.matrix = vec![2.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 
                               0.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 
                               0.0, 0.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.5, 
                               0.0, 0.0, 0.0, 1.0, -1.0, 0.0, 0.0, 0.5, 
                               0.0, 0.0, 0.0, 0.0, 1.0, -1.0, 0.0, 0.5,
                               0.0, 0.0, 0.0, 0.0, 0.0, 1.0, -1.0, 0.5,
                               0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.5, 
                               0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5]; 
            for i in 0..self.matrix.len() {
                self.matrix[i] *= self.scale;
            }
        }

        if self.name == Self::GAUSS {
            self.dim = -1;
        }
        self.init = true;
    }
    
    fn round(&self, vec: Vec<f64>) -> Vec<f64> {
        vec.into_iter()
            .map(|x| (x / self.scale).round() * self.scale)
            .collect()
    }

    fn closest_gauss(&self, vec: Vec<f64>) -> Vec<f64> {
        return self.round(vec);
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
        let rounded = DVector::from_vec(self.round(coeff_vec));
        let almost_out = b * rounded;

        return almost_out.data.as_vec().clone();
    }

    fn closest_e8(&self, vec: Vec<f64>) -> Vec<f64> {
        let p = DVector::from_vec(vec);
        let b = DMatrix::from_vec(8, 8, self.matrix.clone());
        if  b.is_invertible() == false {
            panic!("Failed finding closest in the E8 Lattice");
        }
        let inv_b = b.clone().try_inverse().unwrap();
        let coeff = inv_b * p;
        let coeff_vec = coeff.data.as_vec().clone();
        let rounded = DVector::from_vec(self.round(coeff_vec));
        let almost_out = b * rounded;

        return almost_out.data.as_vec().clone();
    }

    pub fn split_and_pad(&self, vec: Vec<f64>, chunk_size: usize) -> Vec<Vec<f64>> {
        self.check_init();
        let mut result = vec![vec![0.0; chunk_size]; (vec.len() + chunk_size - 1) / chunk_size];
        for (i, &val) in vec.iter().enumerate() {
            result[i / chunk_size][i % chunk_size] = val;
        }
        return result;
    }

    pub fn closest(&self, vector: Vec<f64>) -> Vec<f64> {
        self.check_init();
        if self.name == Self::GAUSS {
            return self.closest_gauss(vector);
        }

        if self.name == Self::LEACH {
            return self.closest_leech(vector);
        }

        if self.name == Self::E8_LAT {
            return self.closest_e8(vector); 
        }

        return vec![0.0];
    }

}

impl Clone for Lattice {
    fn clone(&self) -> Lattice {
        let mut lattice = Lattice {
            name: self.name.clone(),
            matrix: self.matrix.clone(),
            dim: self.dim,
            scale: self.scale,
            init: false
        };
        lattice.init();
        lattice
    }
}