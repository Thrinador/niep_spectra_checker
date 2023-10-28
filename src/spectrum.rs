use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spectrum {
    eigenvalues: Vec<f64>,
}

impl Spectrum {
    pub fn to_string(&self) -> String {
        let mut out: String = String::new();
        for term in self.eigenvalues.iter() {
            out = format!("{}, {}", out, term);
        }
        out
    }

    pub fn from_element(length: usize, element: f64) -> Spectrum {
        Spectrum {
            eigenvalues: vec![element; length],
        }
    }

    pub fn from_vec(eigenvalues: Vec<f64>) -> Spectrum {
        Spectrum {
            eigenvalues: eigenvalues,
        }
    }

    pub fn len(&self) -> usize {
        self.eigenvalues.len()
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