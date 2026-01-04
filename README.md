# **Trabalho Extra de Criptografia**

**Nome:** Rubens Rabelo Soares

## 1. Introdução

Este trabalho tem como objetivo analisar, na prática, as propriedades de segurança de funções hash criptográficas utilizando o algoritmo SHAKE128. Por meio de experimentos controlados, são demonstradas as quebras de resistência à colisão, segunda pré-imagem e pré-imagem quando o tamanho do digest é reduzido. 

## **2. Pré-requisitos**

Para compilar e executar o projeto, é necessário possuir:

* **Rust** (toolchain oficial, incluindo `cargo`)


## **3. Estrutura do Projeto**

O projeto foi organizado de forma **modular**, seguindo boas práticas da linguagem Rust e facilitando a manutenção, leitura e análise de cada desafio de forma independente.

```bash
src/
├── main.rs
└── lab/
    ├── mod.rs
    ├── desafio_a/
    │   ├── mod.rs
    │   └── collision.rs
    ├── desafio_b/
    │   ├── mod.rs
    │   └── second_preimage.rs
    └── desafio_c/
        ├── mod.rs
        └── preimage.rs
```

Cada desafio está isolado em seu próprio módulo, permitindo avaliar separadamente as técnicas utilizadas para quebrar cada propriedade criptográfica.


## **3. Como Executar o Programa**

Com o Rust devidamente instalado, execute o comando abaixo para compilar o projeto em modo otimizado:

```bash
cargo build --release
```

O programa é controlado via linha de comando, permitindo selecionar diretamente qual desafio será executado.

### 3.1. **Compilação e execução otimizada**

```bash
cargo run --release a
cargo run --release b
cargo run --release c
```

Onde:

* `a` → Desafio A (Colisão)
* `b` → Desafio B (Segunda Pré-imagem)
* `c` → Desafio C (Pré-imagem)


## **4. Desafio A – Quebra da Resistência a Colisões**

### **4.1. Metodologia**

* Geração sequencial de mensagens (`message_n`)
* Cálculo do hash SHAKE128 com saída truncada
* Armazenamento dos hashes em uma estrutura de dados (`HashMap`)
* Detecção de colisão quando um hash já existente é encontrado

### **4.2. Análise**

Devido ao **paradoxo do aniversário**, espera-se encontrar colisões após aproximadamente:

$$
2^{n/2} \Rightarrow 2^{16} \approx 65,536
$$

tentativas, o que torna esse ataque rápido e computacionalmente viável mesmo em máquinas comuns.

*Os resultados obtidos estão apresentados nas imagens anexadas.*

<p align="center">
  <img src="./docs/img/challenge_a.jpeg" alt="Resultado do Desafio A" width="500">
</p>


## **5. Desafio B – Quebra da Resistência à Segunda Pré-imagem**

### **5.1. Metodologia**

* Escolha da entrada fixa: `"Aluno: Rubens Rabelo Soares"`
* Cálculo prévio do hash da entrada alvo
* Busca por força bruta de uma segunda entrada distinta
* Utilização de múltiplas threads para paralelizar o ataque
* Uso de variáveis atômicas para controle seguro das tentativas
* Encerramento coordenado das threads ao encontrar uma solução válida

### **5.2. Análise**

A resistência à segunda pré-imagem exige, em média:

$$
2^n \Rightarrow 2^{32} \approx 4 \text{ bilhões}
$$

tentativas. Isso explica o tempo significativamente maior em comparação ao Desafio A, mesmo utilizando paralelismo.

*Os resultados obtidos estão apresentados nas imagens anexadas.*

<p align="center">
  <img src="./docs/img/challenge_b.jpeg" alt="Resultado do Desafio B" width="500">
</p>


## **6. Desafio C – Quebra da Resistência à Pré-imagem**

### **6.1. Metodologia**

* Escolha do Hash alvo utilizado: `79455269`
* Geração sequencial de senhas candidatas (`password_n`)
* Cálculo do SHAKE128 com saída de 5 bytes
* Comparação parcial dos **34 bits** exigidos
* Execução paralela para acelerar a busca

### **6.2. Análise**

A resistência à pré-imagem é a propriedade mais forte entre as três e exige, em média:

$$
2^{34} \approx 17 \text{ bilhões}
$$

tentativas no pior caso. Mesmo com paralelismo, o ataque é computacionalmente caro, evidenciando como o aumento do tamanho do digest impacta diretamente na segurança do sistema.

*Os resultados obtidos estão apresentados nas imagens anexadas.*

<p align="center">
  <img src="./docs/img/challenge_c.jpeg" alt="Resultado do Desafio C" width="500">
</p>


## **7. Decisões de Projeto, Dificuldades e Soluções**
### **7.1 Decisões de Projeto**
* A linguagem Rust foi escolhida devido ao seu alto desempenho, segurança de memória e suporte nativo a concorrência.
* O projeto foi estruturado de forma modular, facilitando a organização, manutenção e compreensão do código.
* A adoção de execução paralela com múltiplas threads foi uma escolha necessária para tornar viáveis os desafios B e C dentro de um tempo aceitável de execução.
* O controle de execução e a escolha do desafio via linha de comando foram implementados para simplificar o uso do programa e permitir testes independentes de cada ataque.

### **7.2 Dificuldades Encontradas**
* **Dificuldade 1:** Alto tempo de execução nos desafios de segunda pré-imagem e pré-imagem, devido ao grande espaço de busca exigido matematicamente.
* **Dificuldade 2:** Compreensão prática das diferenças entre colisão, segunda pré-imagem e pré-imagem, especialmente na tradução desses conceitos teóricos para implementações concretas.
* **Dificuldade 3:** Implementação correta de paralelismo, incluindo divisão de trabalho entre threads e sincronização segura de variáveis compartilhadas.
* **Dificuldade 4:** Gerenciamento de contadores e sinais de parada utilizando variáveis atômicas, evitando condições de corrida e resultados inconsistentes.

### **7.3 Soluções Adotadas**

* **Solução 1:** Uso de múltiplas threads para dividir o espaço de busca entre os núcleos disponíveis do processador, reduzindo significativamente o tempo total de execução.
* **Solução 2:** Implementação de variáveis atômicas para controle seguro do número de tentativas e detecção do sucesso do ataque.
* **Solução 3:** Separação clara das funções de hash, verificação de alvos e controle de execução, aumentando a legibilidade e reduzindo a complexidade do código.
* **Solução 4:** Testes individuais de cada desafio, permitindo validar o funcionamento correto antes da execução completa em modo otimizado (--release).
