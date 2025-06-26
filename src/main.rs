use clap::{Arg, ArgAction, Command}; //importa o clap que permite usar os argumentos 'l','m'..
use std::fs::File; //Permite abrir e ler arquivos
use std::io::{self, BufReader, Read}; //importa funcionalidades de leitura de arquivos e atraves do terminal

fn main() {
    let matches = Command::new("ccwc") //cria uma instancia que irá receber o que o usuário digitar (texto ou arquivo)
        .version("1.0")
        .author("Mary")
        .about("Contador de linhas, palavras, caracteres e bytes (tipo 'wc')")
        //INICIO DAS FLAGS;
          //-l
        .arg(
            Arg::new("lines")
                .short('l')
                .action(ArgAction::SetTrue) //se for digitado -l, essa flag se torna verdadeira e é executada
                .help("Conta linhas"),
        )
         
        //-w
        .arg(
            Arg::new("words")
                .short('w')
                .action(ArgAction::SetTrue)  //se for digitado -w, essa flag se torna verdadeira e é executada
                .help("Conta palavras"),
        )
        //-m
        .arg(
            Arg::new("chars")
                .short('m')
                .action(ArgAction::SetTrue)
                .help("Conta caracteres"),
        )
        //-c
        .arg(
            Arg::new("bytes")
                .short('c')
                .action(ArgAction::SetTrue)
                .help("Conta bytes"),
        )
        //define o nome da flag para o usuario usar para acessar o arquivo
        //Se o usuário não passar nenhum arquivo, seu programa saberá que deve ler do terminal (stdin).

        .arg(
            Arg::new("files")
                .help("Arquivos de entrada")
                .num_args(0..),
        )
        .get_matches();
        //verificação de qual/is flag/gs foram ativadas:
    let flags = (
        matches.contains_id("lines"),
        matches.contains_id("words"),
        matches.contains_id("chars"),
        matches.contains_id("bytes"),
    );

    let use_default = !flags.0 && !flags.1 && !flags.2 && !flags.3; //caso nenhuma flag tenha passado para true, o programa irá seguir o comportamento default

    //analisa os arquivos disponibilizados (se houver) e retorna uma lista deles.
    let files: Vec<String> = matches
        .get_many::<String>("files")
        .map(|vals| vals.map(|s| s.to_string()).collect())
        .unwrap_or_else(|| vec![]);

    if files.is_empty() { // se tiver vazio
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input) //leia do terminal e coloque em input
            .expect("Erro ao ler do stdin");
        process_input("<stdin>", &input); //diz que o conteúdo foi disponibilizado pelo teclado
    } else {
        for file_name in files { // caso haja arquivos, leia e exiba um por um
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
//mostra o conteúdo do arquivo ou texto do terminal:
fn process_input(filename: &str, text: &str) {
    println!("Arquivo: {}\n", filename);
    println!("{}", text);
}
