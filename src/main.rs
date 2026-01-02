//! Programa principal do laboratório de segurança com SHAKE128.
mod lab;

use std::env;

/// Função principal do programa.
///
/// Lê os argumentos da linha de comando e decide
/// qual desafio criptográfico será executado.
///
/// ## Uso
/// ```bash
/// cargo run --release a
/// cargo run --release b
/// cargo run --release c
/// ```
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Uso:");
        println!("  cargo run --release a  (Desafio A - Colisão)");
        println!("  cargo run --release b  (Desafio B - Segunda Pré-imagem)");
        println!("  cargo run --release c  (Desafio C - Pré-imagem)");
        return;
    }

    match args[1].as_str() {
        "a" => {
            println!("--- Desafio A: Colisão ---");
            lab::challenge_a::collision::run_challenge_a();
        }
        "b" => {
            println!("--- Desafio B: Segunda Pré-imagem ---");
            lab::challenge_b::second_preimage::run_challenge_b();
        }
        "c" => {
            println!("--- Desafio C: Pré-imagem ---");
            lab::challenge_c::preimage::run_challenge_c();
        }
        _ => {
            println!("Opção inválida. Use a, b ou c.");
        }
    }
}
