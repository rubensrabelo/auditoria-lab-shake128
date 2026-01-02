//! Desafio A – Quebra da resistência a colisões usando SHAKE128.
use sha3::{
    Shake128,
    digest::{Update, ExtendableOutput, XofReader},
};
use std::collections::HashMap;
use std::time::Instant;

/// Executa o ataque de colisão contra o SHAKE128.
///
/// O algoritmo gera mensagens sequenciais (`message_N`) e
/// calcula seus hashes de 4 bytes.
/// Cada hash é armazenado em um `HashMap`.
///
/// Quando um hash já existente é encontrado novamente,
/// temos uma colisão.
///
/// ## Conceito criptográfico
/// Quebra da **resistência a colisões**:
/// encontrar `x ≠ y` tal que `H(x) = H(y)`.
///
/// ## Complexidade esperada
/// Para um hash de *n* bits, a colisão ocorre em média após:
/// ```text
/// 2^(n/2)
/// ```
///
/// Neste caso:
/// ```text
/// n = 32 bits → ~2^16 ≈ 65 mil tentativas
/// ```
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

/// Calcula o hash SHAKE128 com saída fixa de 4 bytes.
///
/// ## Parâmetros
/// - `input`: bytes da mensagem de entrada
///
/// ## Retorno
/// - Array de 4 bytes representando o hash truncado
fn shake128_4bytes(input: &[u8]) -> [u8; 4] {
    let mut hasher = Shake128::default();
    hasher.update(input);

    let mut reader = hasher.finalize_xof();
    let mut output = [0u8; 4];
    reader.read(&mut output);

    output
}
