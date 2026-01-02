//! Desafio C – Quebra da resistência à pré-imagem usando SHAKE128.
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

const TARGET_PREFIX: [u8; 4] = [0x79, 0x45, 0x52, 0x69];

const EXTRA_BITS: u8 = 2;
const EXTRA_BITS_VALUE: u8 = 0b01;

/// Executa o ataque de pré-imagem.
///
/// O algoritmo tenta encontrar qualquer entrada `x` tal que:
///
/// ```text
/// H(x) = h
/// ```
///
/// onde `h` é conhecido apenas parcialmente (34 bits).
///
/// ## Conceito criptográfico
/// Quebra da **resistência à pré-imagem**.
///
/// ## Complexidade esperada
/// Para um hash de *n* bits:
/// ```text
/// ~2^n tentativas
/// ```
///
/// Neste caso:
/// ```text
/// n = 34 bits → ~17 bilhões de tentativas (pior caso)
/// ```
pub fn run_challenge_c() {
    let start = Instant::now();

    let found = Arc::new(AtomicBool::new(false));
    let attempts = Arc::new(AtomicU64::new(0));

    let threads = 8;

    println!(
        "Iniciando ataque de pré-imagem (34 bits) com {} threads...",
        threads
    );

    let mut handles = Vec::new();

    for thread_id in 0..threads {
        let found_flag = Arc::clone(&found);
        let attempts_counter = Arc::clone(&attempts);

        let handle = thread::spawn(move || {
            let mut counter: u64 = thread_id;

            while !found_flag.load(Ordering::Relaxed) {
                let candidate = format!("password_{}", counter);
                let hash = shake128_5bytes(candidate.as_bytes());

                attempts_counter.fetch_add(1, Ordering::Relaxed);

                if hash_matches(&hash) {
                    if !found_flag.swap(true, Ordering::Relaxed) {
                        let elapsed = start.elapsed().as_secs_f64();
                        let total_attempts =
                            attempts_counter.load(Ordering::Relaxed);

                        println!("Pré-imagem encontrada!");
                        println!("Senha encontrada: {}", candidate);
                        println!("Hash (prefixo): {:02x?}", &hash[..5]);
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

                counter += threads as u64;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/// Calcula o hash SHAKE128 com saída fixa de 5 bytes.
///
/// ## Parâmetros
/// - `input`: bytes da senha candidata
///
/// ## Retorno
/// - Array de 5 bytes do hash SHAKE128
fn shake128_5bytes(input: &[u8]) -> [u8; 5] {
    let mut hasher = Shake128::default();
    hasher.update(input);

    let mut reader = hasher.finalize_xof();
    let mut output = [0u8; 5];
    reader.read(&mut output);

    output
}

/// Verifica se o hash corresponde ao alvo de 34 bits.
///
/// Condições:
/// 1. Os primeiros 4 bytes devem ser iguais ao prefixo alvo
/// 2. Os `EXTRA_BITS` mais significativos do quinto byte
///    devem ter o valor esperado
fn hash_matches(hash: &[u8; 5]) -> bool {
    if hash[..4] != TARGET_PREFIX {
        return false;
    }

    let extra = hash[4] >> (8 - EXTRA_BITS);
    extra == EXTRA_BITS_VALUE
}
