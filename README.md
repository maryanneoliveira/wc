<h1>O que é o <code>wc</code>?</h1>
<p>
  O <code>wc</code> (word count) é um utilitário presente em sistemas Unix/Linux que fornece estatísticas básicas sobre arquivos de texto. Ele pode contar:
</p>
<ul>
  <li>Linhas (<code>-l</code>)</li>
  <li>Palavras (<code>-w</code>)</li>
  <li>Bytes (<code>-c</code>)</li>
  <li>Caracteres (<code>-m</code>)</li>
</ul>

<h1>Criando um projeto Rust:</h1>

<ol>
  <li><code>cargo new ccwc</code></li>
  <li><code>cd ccwc</code></li>
</ol>

<p>Isso irá gerar:</p>

<pre>
ccwc/
├── Cargo.toml         ← arquivo de configuração (dependências, nome, versão, etc.)
└── src/
    └── main.rs        ← arquivo principal com o código-fonte (ponto de entrada: <code>fn main</code>)
</pre>

<h2>Adicionando Clap ao <code>Cargo.toml</code>:</h2>

<pre>
[package]
name = "ccwc"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4", features = ["derive"] }
</pre>

<h2>Compilando e executando</h2>
<ol>
  <li><strong>Compilar:</strong> <code>cargo build</code></li>
  <li><strong>Executar com arquivo:</strong> <code>cargo run -- -l -w exemplo.txt</code></li>
  <li><strong>Executar com entrada do terminal:</strong> <code>echo "Olá mundo!" | cargo run -- -w -m</code></li>
</ol>
