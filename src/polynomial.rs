use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::{Index, IndexMut, Mul};
use crate::Spectrum;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Polynomial {
    coefficients: Vec<f64>,
}

impl Polynomial {
    pub fn to_string(&self) -> String {
        let mut i = self.len();
        let mut out: String = String::new();
        for term in self.coefficients.iter() {
            i -= 1;
            if term >= &0.0 {
                out = format!("{}+ {:.7}x^{} ", out, term, i);
            } else {
                out = format!("{}- {:.7}x^{} ", out, term.abs(), i);
            }
        }
        out
    }

    pub fn is_polynomial_nonnegative(&self) -> bool {
        self.is_polynomial_nonnegative_with_threshold(0.0)
    }

    pub fn is_polynomial_nonnegative_with_threshold(&self, threshold: f64) -> bool {
        for value in self.coefficients.iter() {
            if value < &threshold {
                return false;
            }
        }
        true
    }

    pub fn from_element(polynomial_length: usize, element: f64) -> Polynomial {
        Polynomial {
            coefficients: vec![element; polynomial_length],
        }
    }

    pub fn from_vec(coefficients: Vec<f64>) -> Polynomial {
        Polynomial {
            coefficients: coefficients,
        }
    }

    pub fn from_spectrum(spectrum: &Spectrum) -> Polynomial {
        let mut poly = Polynomial::from_vec(vec![1.0]);
        for i in 0..spectrum.len() {
            poly = poly * Polynomial::from_vec(vec![1.0, -spectrum[i]]);
        }
        poly
    }

    pub fn len(&self) -> usize {
        self.coefficients.len()
    }

    pub fn min_term(&self) -> f64 {
        let mut min = self.coefficients[0].abs();
        for coefficient in &self.coefficients {
            if min > coefficient.abs() {
                min = coefficient.abs();
            }
        }
        min
    }

    pub fn max_term(&self) -> f64 {
        let mut max = self.coefficients[0].abs();
        for coefficient in &self.coefficients {
            if max < coefficient.abs() {
                max = coefficient.abs();
            }
        }
        max
    }

    pub fn derivative(&self) -> Polynomial {
        let mut derivative = Polynomial::from_element(self.len() - 1, 0.0);
        for i in 1..self.len() {
            derivative[i - 1] = (i as f64) * self[i];
        }
        derivative
    }
}

// TODO this is a pretty rough function, for now my percision caps at 3 decimals so it is sufficient.
pub fn approx_equal(term1: f64, term2: f64) -> bool {
    (term1 - term2).abs() < 0.00001
}

impl PartialEq for Polynomial {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..self.len() {
            if self[i] != other[i] {
                return false;
            }
        }
        true
    }
}

impl Eq for Polynomial {}

impl PartialOrd for Polynomial {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for i in 0..self.len() {
            if self[i] > other[i] {
                return Some(Ordering::Greater);
            } else if self[i] < other[i] {
                return Some(Ordering::Less);
            }
        }
        Some(Ordering::Equal)
    }
}

impl Ord for Polynomial {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in 0..self.len() {
            if self[i] > other[i] {
                return Ordering::Greater;
            } else if self[i] < other[i] {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }
}

impl Index<usize> for Polynomial {
    type Output = f64;
    fn index<'a>(&'a self, i: usize) -> &'a f64 {
        &self.coefficients[i]
    }
}

impl IndexMut<usize> for Polynomial {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut f64 {
        &mut self.coefficients[i]
    }
}

impl Mul<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn mul(self, other: Polynomial) -> Polynomial {
        let mut poly = Polynomial::from_element(self.len() + other.len() - 1, 0.0);
        for i in 0..self.len() {
            for j in 0..other.len() {
                println!("{} + {}, i={}, j={}", poly[i+j], self[i]*other[j],i,j);
                poly [i+j] = poly[i+j] + self[i]*other[j];
            }
        }
        println!("{}", poly.to_string());
        poly
    }
}