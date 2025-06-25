use clap::{Arg, ArgAction, Command};
use std::fs::File;
use std::io::{self, BufReader, Read};

fn main() {
    let matches = Command::new("ccwc")
        .version("1.0")
        .author("Seu Nome")
        .about("Contador de linhas, palavras, caracteres e bytes (tipo 'wc')")
        .arg(
            Arg::new("lines")
                .short('l')
                .action(ArgAction::SetTrue)
                .help("Conta linhas"),
        )
        .arg(
            Arg::new("words")
                .short('w')
                .action(ArgAction::SetTrue)
                .help("Conta palavras"),
        )
        .arg(
            Arg::new("chars")
                .short('m')
                .action(ArgAction::SetTrue)
                .help("Conta caracteres"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .action(ArgAction::SetTrue)
                .help("Conta bytes"),
        )
        .arg(
            Arg::new("files")
                .help("Arquivos de entrada")
                .num_args(0..),
        )
        .get_matches();

    let flags = (
        matches.contains_id("lines"),
        matches.contains_id("words"),
        matches.contains_id("chars"),
        matches.contains_id("bytes"),
    );

    let use_default = !flags.0 && !flags.1 && !flags.2 && !flags.3;

    let files: Vec<String> = matches
        .get_many::<String>("files")
        .map(|vals| vals.map(|s| s.to_string()).collect())
        .unwrap_or_else(|| vec![]);

    if files.is_empty() {
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .expect("Erro ao ler do stdin");
        process_input("<stdin>", &input);
    } else {
        for file_name in files {
            let file = File::open(&file_name).expect("Erro ao abrir arquivo");
            let mut reader = BufReader::new(file);
            let mut content = String::new();
            reader
                .read_to_string(&mut content)
                .expect("Erro ao ler o arquivo");
            process_input(&file_name, &content);
        }
    }
}

fn process_input(filename: &str, text: &str) {
    println!("Arquivo: {}\n", filename);
    println!("{}", text);
}
