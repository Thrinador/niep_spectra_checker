use spectrum::Spectrum;

pub mod spectrum;

pub fn mutate_spectrum(
    spectrum: Spectrum,
    spectra_mutations: usize,
    mutated_spectra_to_evaluate: usize,
) -> Vec<Spectrum> {
    vec![spectrum]
}

pub fn test_spectra(spectra: &Spectrum, powers: usize) -> bool {
    test_moment_condition(spectra, powers) && 
    test_jll_condition(spectra, powers) &&
    test_taamp_condition(spectra)
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
    true
}