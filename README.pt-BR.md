# Motor de Verificação ZK-SNARK em Rust: Groth16 e Curva BN254

[![CI](https://github.com/FelipeMadson/rust-zk-snark-verifier-engine/actions/workflows/ci.yml/badge.svg)](https://github.com/FelipeMadson/rust-zk-snark-verifier-engine/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust 1.80+](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)
[![Groth16](https://img.shields.io/badge/Zero--Knowledge-Groth16%20SNARK-blueviolet.svg)](https://eprint.iacr.org/2016/260)
[![BN254 / Alt-bn128](https://img.shields.io/badge/Curve-BN254%20Pairing-success.svg)](https://eips.ethereum.org/EIPS/eip-197)

> **Motor Criptográfico de Conhecimento Zero para Atestação de Credenciais com Preservação de Privacidade e Verificação On-Chain (RNP TRUST-ID e Observatório de Blockchain).**

---

## 1. Visão Geral

Argumentos Sucintos de Conhecimento Não-Interativos de Conhecimento Zero (zk-SNARKs) permitem que um provador demonstre matematicamente que possui uma testemunha privada válida que satisfaz um conjunto arbitrário de restrições algébricas, sem revelar nenhum dado sensível.

O **rust-zk-snark-verifier-engine** provê uma infraestrutura de verificação de conhecimento zero em nível industrial:
1. **Emparelhamentos Bilineares em Curva BN254 (Alt-bn128)**: Aritmética de curvas elípticas sobre $\mathbb{G}_1$, $\mathbb{G}_2$ e $\mathbb{G}_T$ com mapa de emparelhamento $e(P, Q)$.
2. **Sistema de Restrições Rank-1 (R1CS)**: Compilador determinístico de restrições algébricas $\langle A, z \rangle \cdot \langle B, z \rangle = \langle C, z \rangle$.
3. **Verificador Groth16**: Validação de provas sucintas de tamanho constante (128 bytes) em tempo sub-milissegundo com 3 checagens de emparelhamento:
   $$e(A, B) = e(\alpha, \beta) \cdot e\left(IC_0 + \sum x_i IC_i, \gamma\right) \cdot e(C, \delta)$$
4. **Circuito de Credencial Científica com Privacidade**: Prova que um agente de IA possui autorização com escore de confiança suficiente e quota não expirada sem revelar o DID, identidade ou chaves privadas do agente.

---

## 2. Execução Local

```bash
git clone https://github.com/FelipeMadson/rust-zk-snark-verifier-engine.git
cd rust-zk-snark-verifier-engine
cargo test --verbose
cargo run
```

## Licença
Licença MIT (c) 2026 Felipe Madison.
