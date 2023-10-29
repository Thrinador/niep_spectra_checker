use log::{error, info};
use niep_spectra_checker::spectrum::Spectrum;
use niep_spectra_checker::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::time::Instant;
use toml;

// Top level struct to hold the TOML data.
#[derive(Deserialize)]
struct Data {
    config: Config,
}

// Config struct holds to data from the `[config]` section.
#[derive(Deserialize)]
struct Config {
    spectra_size: usize,
    spectra_mutations: usize,
    mutated_spectra_to_evaluate: usize,
    mode: usize,
    starting_spectra: Vec<f64>,
}

#[derive(Serialize, Deserialize)]
struct Output {
    interesting_spectra: Vec<Spectrum>,
}

enum SpectrumMode {
    TestSpectrum,
    MutateSpectrum,
    MapSpace,
    ReturnState,
    Error,
}

fn i32_to_spectra_mode(val: usize) -> SpectrumMode {
    match val {
        1 => SpectrumMode::TestSpectrum,
        2 => SpectrumMode::MutateSpectrum,
        3 => SpectrumMode::MapSpace,
        4 => SpectrumMode::ReturnState,
        _ => SpectrumMode::Error,
    }
}

fn read_user_from_file() -> Config {
    let file_contents = fs::read_to_string("startup.toml").expect("file should open read only");
    let data: Data = toml::from_str(&file_contents).expect("Unable to load data");
    data.config
}

fn print_spectra(spectra: Vec<Spectrum>) {
    info!(
        "Total number of interesting spectra found {}",
        spectra.len()
    );
    for i in 0..spectra.len() {
        println!("{}: {}", i, spectra[i].to_string());
    }

    let json_object = serde_json::to_string(&Output {
        interesting_spectra: spectra,
    })
    .expect("Object will be converted to JSON string");
    File::create("output.json").expect("file should open read only");
    fs::write("output.json", json_object).expect("file should open read only");
}

fn mode_test_spectrum(args: Config) {
    let start = Instant::now();
    let spectrum = if args.starting_spectra.len() == 0 {
        Spectrum::from_vec(vec![1.0, 1.0, 1.0, 1.0, 1.0], 3)
    } else {
        Spectrum::from_vec(args.starting_spectra, 3)
    };
    
    let verify_spectrum = test_spectra(&spectrum);
    let duration = start.elapsed();
    info!("Total time elapsed verifying spectrum {:?}", duration);
    if verify_spectrum {
        println!(
            "The spectrum {} satisfies the spectrum conditions",
            spectrum.to_string(),
        );
    } else {
        println!(
            "The spectrum {} does not satisfy the spectrum conditions",
            spectrum.to_string(),
        );
    }
}

fn mode_mutate_spectra(args: Config) {
    let start = Instant::now();
    let spectrum = if args.starting_spectra.len() == 0 {
        Spectrum::from_vec(vec![1.0, 1.0, 1.0, 1.0, 1.0], 3)
    } else {
        Spectrum::from_vec(args.starting_spectra, 3)
    };
    let mut interesting_spectra = mutate_spectrum(
        spectrum,
        args.spectra_mutations,
        args.mutated_spectra_to_evaluate,
    );
    interesting_spectra.sort();
    let duration = start.elapsed();
    info!("Total time elapsed generating spectra {:?}", duration);
    print_spectra(interesting_spectra);
}

fn mode_map_space(args: Config) {
    
}

fn mode_return_state(args: Config) {
    
}

fn main() {
    let args = read_user_from_file();
    //env_logger::init();
    match i32_to_spectra_mode(args.mode) {
        SpectrumMode::TestSpectrum => mode_test_spectrum(args),
        SpectrumMode::MutateSpectrum => mode_mutate_spectra(args),
        SpectrumMode::MapSpace => mode_map_space(args),
        SpectrumMode::ReturnState => mode_return_state(args),
        SpectrumMode::Error => error!("mode must be set to 1,2, or 3"),
    }
}
