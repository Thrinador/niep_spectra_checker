use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::{Index, IndexMut};
use crate::Polynomial;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spectrum {
    eigenvalues: Vec<f64>,
    conditions: Vec<bool>,
}

impl Spectrum {
    pub fn to_string(&self) -> String {
        let mut out: String = String::new();
        out = format!("Eigenvalues: {}", self.eigenvalues[0]);
        for i in 1..self.eigenvalues.len() {
            if self.eigenvalues[i] >= 0.0 {
                out = format!("{}, +{:.7}", out, self.eigenvalues[i]);
            } else {
                out = format!("{}, -{:.7}", out, self.eigenvalues[i].abs());
            }
        }
        out = format!("{}; Tests: {}", out, self.conditions[0]);
        for i in 1..self.conditions.len() {
            out = format!("{}, {}", out, self.conditions[i]);
        }
        out
    }

    pub fn from_element(length: usize, element: f64, num_conditions: usize) -> Spectrum {
        Spectrum {
            eigenvalues: vec![element; length],
            conditions: vec![false; num_conditions],
        }
    }

    pub fn from_vec(eigenvalues: Vec<f64>, num_conditions: usize) -> Spectrum {
        Spectrum {
            eigenvalues: eigenvalues,
            conditions: vec![false; num_conditions],
        }
    }

    pub fn len(&self) -> usize {
        self.eigenvalues.len()
    }

    pub fn change_eigenvalue(&mut self, i: usize, amount: f64) -> bool {
        if self[i] + amount <= 1.0 {
            self[i] += amount;
            true
        } else {
            false
        }
    }

    pub fn test_spectra_or(&mut self) -> bool {
        self.conditions[0] = self.test_moment_condition(10);
        self.conditions[1] = self.test_jll_condition(10);
        self.conditions[2] = self.test_taamp_condition();

        self.conditions[0] || self.conditions[1] || self.conditions[2]
    }
    
    pub fn test_moment_condition(&self, powers: usize) -> bool {
        for k in 1..powers {
            if self.moment(k) <= -0.00001 {
                return false;
            }
        }
        true
    }
    
    pub fn test_jll_condition(&self, powers: usize) -> bool {
        let n = self.len() as f64;
        for m in 1..powers {
            for k in 1..powers {
                if self.moment(k).powi(m as i32) > 
                    n.powf((m-1) as f64) * self.moment(k*m) {
                    return false;
                } 
            }
        }
        true
    }
    
    fn moment(&self, k: usize) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.len() {
            sum += self[i].powi(k as i32);
        }
        sum
    } 
    
    pub fn test_taamp_condition(&self) -> bool {
        // The TAAMP condition only applies for 4x4 and above.
        if self.len() < 4 {
            return false;
        }
        let poly = Polynomial::from_spectrum(self);
        let k_1 = poly[1];
        let k_2 = poly[2];
        let k_3 = poly[3];
        let n = poly.len() as f64;
    
        if k_1 > 0.0 {
            false
        } else if k_2 > ((n - 1.0) / (2.0*n)) * k_1.powi(2) {
            false
        } else if (((n-1.0)*(n-4.0)) / (2.0*(n-2.0).powi(2))) * k_1.powi(2) < k_2 {
            k_3 <= ((n-2.0) / n) * (k_1 * k_2 + ((n-1.0) / (3.0*n)) * ((k_1.powi(2) - ((2.0*n*k_2)/ (n-1.0)).powf(1.5) - k_1.powi(3))))
        } else {
            k_3 <= k_1 * k_2 - (((n-1.0) * (n-3.0)) / (3.0*(n-2.0).powi(2))) * k_1.powi(3)
        }
    }
}

pub fn approx_equal(term1: f64, term2: f64) -> bool {
    term1 - term2 < 0.00001
}

impl PartialEq for Spectrum {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..self.eigenvalues.len() {
            if ! approx_equal(self[i],other[i]) {
                return false;
            }
        }
        true
    }
}

impl Eq for Spectrum {}

impl PartialOrd for Spectrum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for i in 0..self.eigenvalues.len() {
            if self[i] > other[i] {
                return Some(Ordering::Greater);
            } else if self[i] < other[i] {
                return Some(Ordering::Less);
            }
        }
        Some(Ordering::Equal)
    }
}

impl Ord for Spectrum {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in 0..self.eigenvalues.len() {
            if self[i] > other[i] {
                return Ordering::Greater;
            } else if self[i] < other[i] {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }
}

impl Index<usize> for Spectrum {
    type Output = f64;
    fn index<'a>(&'a self, i: usize) -> &'a f64 {
        &self.eigenvalues[i]
    }
}

impl IndexMut<usize> for Spectrum {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut f64 {
        &mut self.eigenvalues[i]
    }
}