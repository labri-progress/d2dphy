//! extracting meaningful informations from plskg logs.
#![warn(clippy::all, clippy::pedantic, clippy::cargo, clippy::nursery)]
#![allow(clippy::cast_precision_loss)]
use std::{env, time::Instant};

use colored::Colorize;

use extractor::Extractor;

mod bindings;
mod extractor;
mod key_generations_metadatas;
mod rng_metadatas;

fn main() -> Result<(), String> {
    let start_time = Instant::now();
    let args: Vec<String> = env::args().collect();
    // TODO: better checks on args / file validity
    if args.len() != 2 && args.len() != 3 {
        eprintln!("wrong number of arguments given. arguments given:");
        for arg in &args {
            eprintln!("{arg}");
        }
    }
    // for now we just get the two files from argument.
    // in the future, we probably want to add a CLI/TUI interface for more flexibility and
    // functionalities
    if let Some(alice_logs_path) = args.get(1) {
        let extractor = Extractor::new().map_err(|e| e.to_string())?;
        if let Some(bob_logs_path) = args.get(2) {
            match extractor.extract_metadatas(alice_logs_path, bob_logs_path) {
                Ok(metadatas) => {
                    println!(
                        "=== Post Mortem Analysis ===\nGenerated Key Size: {} bits\n\n{metadatas}",
                        extractor.key_length
                    );
                    // just to get an idea of the whole process duration
                    let elapsed = start_time.elapsed();
                    println!(
                        "{}",
                        format!(
                            "\ntime taken to extract data from logs: {:.1}ms",
                            elapsed.as_micros() as f64 / 1000f64
                        )
                        .green()
                    );
                    Ok(())
                }
                Err(e) => Err(e.to_string()),
            }
        } else {
            match extractor.extract_metadatas_single(alice_logs_path) {
                Ok(metadatas) => {
                    println!(
                        "=== Post Mortem Analysis ===\nGenerated Key Size: {} bits\n\n{metadatas}",
                        extractor.key_length
                    );
                    // just to get an idea of the whole process duration
                    let elapsed = start_time.elapsed();
                    println!(
                        "{}",
                        format!(
                            "\ntime taken to extract data from logs: {:.1}ms",
                            elapsed.as_micros() as f64 / 1000f64
                        )
                        .green()
                    );
                    Ok(())
                }
                Err(e) => Err(e.to_string()),
            }
        }
    } else {
        panic!("unexpected error") // this is rather unexpected as we should at least get those
                                   // two args...
    }
}
