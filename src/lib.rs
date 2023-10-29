use spectrum::Spectrum;
use polynomial::Polynomial;
use std::collections::HashMap;

pub mod spectrum;
pub mod polynomial;

pub fn mutate_spectrum(
    spectrum: Spectrum,
    spectra_mutations: usize,
    mutated_spectra_to_evaluate: usize,
) -> Vec<Spectrum> {
    let mut mapped_spectra = HashMap::new();
    let mut hash_code = vec![0;spectrum.len()];
    apply_mutations(&mut mapped_spectra, &mut spectrum.clone(), 0, spectra_mutations, 0.1, hash_code);
    let mut vec: Vec<Spectrum> = Vec::new();
    for element in mapped_spectra.into_iter() {
        vec.push(element.1);
    }
    vec
}

fn apply_mutations(
    mapped_spectra: &mut HashMap<Vec<i32>, Spectrum>, 
    current_spectrum: &mut Spectrum, 
    steps_taken: usize, 
    total_steps: usize,
    mutation_amount: f64, 
    hash_code: Vec<i32>,
) {
    if steps_taken != total_steps && !mapped_spectra.contains_key(&hash_code) {
        if current_spectrum.test_spectra_or() {
            mapped_spectra.insert(hash_code.clone(), current_spectrum.clone());
            for i in 1..current_spectrum.len() {
                let mut new_hash_code = hash_code.clone();
                let mut spec_pos = current_spectrum.clone();
                let mut spec_neg = current_spectrum.clone();
                new_hash_code[i] += 1;
                if spec_pos.change_eigenvalue(i, mutation_amount) {
                    apply_mutations(mapped_spectra, &mut spec_pos, steps_taken+1, total_steps, mutation_amount, new_hash_code.clone());
                }
                new_hash_code[i] -= 2;
                spec_neg.change_eigenvalue(i, -1.0*mutation_amount);
                apply_mutations(mapped_spectra, &mut spec_neg, steps_taken+1, total_steps, mutation_amount, new_hash_code);
            }
        }
    }
}
