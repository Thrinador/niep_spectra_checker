use spectrum::Spectrum;
use polynomial::Polynomial;

pub mod spectrum;
pub mod polynomial;

pub fn mutate_spectrum(
    spectrum: Spectrum,
    spectra_mutations: usize,
    mutated_spectra_to_evaluate: usize,
) -> Vec<Spectrum> {
    vec![spectrum]
}

pub fn test_spectra(spectra: &Spectrum, powers: usize) -> bool {
    let moment = test_moment_condition(spectra, powers);
    let jll = test_jll_condition(spectra, powers);
    let taamp = test_taamp_condition(spectra);
    moment && jll && taamp
}

pub fn test_moment_condition(spectrum: &Spectrum, powers: usize) -> bool {
    for k in 1..powers {
        if moment(spectrum, k) <= -0.00001 {
            return false;
        }
    }
    true
}

pub fn test_jll_condition(spectrum: &Spectrum, powers: usize) -> bool {
    let n = spectrum.len();
    for m in 1..powers {
        for k in 1..powers {
            if moment(spectrum, k).powi(m as i32) > 
                (n.pow((m-1) as u32) as f64) * moment(spectrum, k*m) {
                return false;
            } 
        }
    }
    true
}

fn moment(spectrum: &Spectrum, k: usize) -> f64 {
    let mut sum = 0.0;
    for i in 0..spectrum.len() {
        sum += spectrum[i].powi(k as i32);
    }
    sum
} 

pub fn test_taamp_condition(spectrum: &Spectrum) -> bool {
    // The TAAMP condition only applies for 4x4 and above.
    if spectrum.len() < 4 {
        return true;
    }
    let poly = spectrum_to_polynomial(spectrum);
    let k_1 = poly[1];
    let k_2 = poly[2];
    let k_3 = poly[3];
    let n = poly.len() as f64;

    if k_1 > 0 {
        false
    } else if k_2 > ((n - 1) / (2*n)) * k_1.powi(2) {
        false
    } else if (((n-1)*(n-4)) / (2*(n-2).powi(2))) * k_1.powi(2) < k_2 {
        k_3 <= ((n-2) / n) * (k_1 * k_2 + ((n-1) / (3*n)) * ((k_1.powi(2) - ((2*n*k_2)/ (n-1)).powi(3/2) - k_1.powi(3))))
    } else {
        k_3 <= k_1 * k_2 - (((n-1) * (n-3)) / (3*(n-2).powi(2))) * k_1.powi(3)
    }
}

fn spectrum_to_polynomial(spectrum: &Spectrum) -> Polynomial {
    let mut poly = Polynomial::from_vec(vec![1.0]);
    for i in 0..spectrum.len() {
        poly = poly * Polynomial::from_vec(vec![1.0, -spectrum[i]]);
    }
    poly
}