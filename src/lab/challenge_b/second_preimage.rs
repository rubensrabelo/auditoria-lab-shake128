use sha3::{
    Shake128,
    digest::{Update, ExtendableOutput, XofReader},
};
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU64, Ordering},
};
use std::thread;
use std::time::Instant;

pub fn run_challenge_b() {
    let start = Instant::now();

    let target_input = "Aluno: Rubens Rabelo Soares";
    let target_hash = shake128_4bytes(target_input.as_bytes());

    let found = Arc::new(AtomicBool::new(false));
    let attempts = Arc::new(AtomicU64::new(0));

    let threads = 8;

    println!("Iniciando busca paralela com {} threads...", threads);

    let mut handles = Vec::new();

    for thread_id in 0..threads {
        let found_flag = Arc::clone(&found);
        let attempts_counter = Arc::clone(&attempts);
        let target_hash = target_hash.clone();
        let target_input = target_input.to_string();

        let handle = thread::spawn(move || {
            let mut counter: u64 = thread_id;

            while !found_flag.load(Ordering::Relaxed) {
                let candidate = format!("candidate_{}", counter);

                if candidate != target_input {
                    let hash = shake128_4bytes(candidate.as_bytes());
                    attempts_counter.fetch_add(1, Ordering::Relaxed);

                    if hash == target_hash {
                        if !found_flag.swap(true, Ordering::Relaxed) {
                            let elapsed = start.elapsed().as_secs_f64();
                            let total_attempts =
                                attempts_counter.load(Ordering::Relaxed);

                            println!("Segunda pré-imagem encontrada!");
                            println!("Entrada alvo (x1): {}", target_input);
                            println!("Entrada encontrada (x2): {}", candidate);
                            println!("Hash (4 bytes): {:02x?}", hash);
                            println!("Tentativas totais: {}", total_attempts);
                            println!(
                                "Tempo total: {:.3} segundos",
                                elapsed
                            );
                            println!(
                                "Taxa aproximada: {:.0} hashes/segundo",
                                total_attempts as f64 / elapsed
                            );
                        }
                        break;
                    }
                }

                counter += threads as u64;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn shake128_4bytes(input: &[u8]) -> [u8; 4] {
    let mut hasher = Shake128::default();
    hasher.update(input);

    let mut reader = hasher.finalize_xof();
    let mut output = [0u8; 4];
    reader.read(&mut output);

    output
}
