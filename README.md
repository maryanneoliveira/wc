<h1>O que é o wc?</h1>
<p>O wc (word count) é um utilitário presente em sistemas Unix/Linux que fornece estatísticas básicas sobre arquivos de texto. Ele pode contar:</p>

-Linhas (-l)
-Palavras (-w)
-Bytes (-c)
-Caracteres (-m)

<h1>Criando um projeto: </h1>
<p>1) cargo new ccwc</p>
<p>2) cd ccwc</p>

<p>Isso irá gerar:
meu_projeto/
├── Cargo.toml         ← arquivo de configuração (dependências, nome, versão, etc.)
└── src/
    └── main.rs        ← arquivo principal com o código-fonte (ponto de entrada: fn main) </p>


3) Adicionar Clap no toml:
   
[package]
name = "ccwc"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4", features = ["derive"] }

4) Compilar: cargo build
5) executar (lendo o arquivo): cargo run -- -l -w exemplo.txt
6) digitando no terminal: echo "Olá mundo!" | cargo run -- -w -m

