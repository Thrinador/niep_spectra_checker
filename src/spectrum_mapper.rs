use std::collections::HashMap;
use crate::Spectrum;

pub struct SpectrumMapper {
    mapped_spectra: HashMap<Vec<i32>, Spectrum>,
    total_steps: usize,
    mutation_amount: f64,
}

impl SpectrumMapper {
    pub fn new_mapper(total_steps: usize, mutation_amount: f64) -> SpectrumMapper {
        SpectrumMapper {
            mapped_spectra: HashMap::new(),
            total_steps: total_steps,
            mutation_amount: mutation_amount,
        }
    }

    pub fn map_spectrum_boundary(&mut self, spectrum: &Spectrum) -> Vec<Spectrum> {
        let vec = self.map_spectrum(spectrum);

        vec
    }

    pub fn map_spectrum(&mut self, spectrum: &Spectrum) -> Vec<Spectrum> {
        self.apply_mapping(&mut spectrum.clone(), 0, vec!(0; spectrum.len()));
        let mut vec: Vec<Spectrum> = Vec::new();
        // TODO this loop needs to be fixed. This clone call will be very expensive. 
        for element in self.mapped_spectra.clone().into_iter() {
            vec.push(element.1);
        }
        vec
    }

    fn apply_mapping(&mut self, current_spectrum: &mut Spectrum, steps_taken: usize, hash_code: Vec<i32>) {
        if steps_taken != self.total_steps && !self.mapped_spectra.contains_key(&hash_code) {
            if current_spectrum.test_spectra_or() {
                self.mapped_spectra.insert(hash_code.clone(), current_spectrum.clone());
                for i in 1..current_spectrum.len() {
                    let mut new_hash_code = hash_code.clone();
                    let mut spec_pos = current_spectrum.clone();
                    let mut spec_neg = current_spectrum.clone();
                    new_hash_code[i] += 1;
                    if spec_pos.change_eigenvalue(i, self.mutation_amount) {
                        self.apply_mapping(&mut spec_pos, steps_taken+1, new_hash_code.clone());
                    }
                    new_hash_code[i] -= 2;
                    spec_neg.change_eigenvalue(i, -1.0* self.mutation_amount);
                    self.apply_mapping(&mut spec_neg, steps_taken+1, new_hash_code);
                }
            }
        }
    }
}

