use sha3::{
    Shake128,
    digest::{Update, ExtendableOutput, XofReader},
};
use std::collections::HashMap;
use std::time::Instant;

pub fn run_challenge_a() {
    let start = Instant::now();

    let mut seen_hashes: HashMap<[u8; 4], String> = HashMap::new();
    let mut counter: u64 = 0;

    loop {
        let message = format!("message_{}", counter);
        let hash = shake128_4bytes(message.as_bytes());

        if let Some(previous) = seen_hashes.get(&hash) {
            let elapsed = start.elapsed().as_secs_f64();

            println!("Colisão encontrada!");
            println!("Mensagem 1: {}", previous);
            println!("Mensagem 2: {}", message);
            println!("Hash (4 bytes): {:02x?}", hash);
            println!("Tentativas: {}", counter);
            println!("Tempo total: {:.3} segundos", elapsed);
            println!(
                "Taxa aproximada: {:.0} hashes/segundo",
                counter as f64 / elapsed
            );
            break;
        } else {
            seen_hashes.insert(hash, message);
        }

        counter += 1;
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
