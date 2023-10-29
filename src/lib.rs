use spectrum::Spectrum;
use polynomial::Polynomial;
use spectrum_mapper::SpectrumMapper;

pub mod polynomial;
pub mod spectrum;
pub mod spectrum_mapper;

pub fn mutate_spectrum(
    spectrum: Spectrum,
    spectra_mutations: usize,
    mutated_spectra_to_evaluate: usize,
) -> Vec<Spectrum> {
    SpectrumMapper::new_mapper(spectra_mutations, 0.1).map_spectrum_boundary(&spectrum)
}
