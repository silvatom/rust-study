
# RUST STUDY

`$ cargo new hello_cargo --bin`
* cria um repositório com Cargo.toml

`Cargo.toml`
* Arquivo para gerenciamento de dependências

`rustc` 
* compilador
`println!`
* the '!' sign calls a macro

`termos`
* pacotes do rust: crates
* trait: have no idea yet

`basic stuff`
* creating variables
let foo = 5; // imutável
let mut bar = 5; // mutável

* &mut var // passing var as reference and mutable

`tipos`
* Result
	Tipo retornado por funções io (apenas io?)
	Variantes: Ok() ou Err()

`crate`
* pacote de códigos Rust. Dependências.
* cargo new project_name --bin
	cria um crate binário
* 'rand = "0.3.14"' depois [dependencies] no .toml são crates de biblioteca.

`comandos`
* cargo build compila o projeto, baixando as dependências definidas no .toml
* cargo check verifica o estado atual do projeto, verificando a possibilidade de compilação
* cargo run compila e executa o projeto
* cargo build --release compila o projeto com otimizações. (mais demorado)
* cargo doc --open constroi localmente a documentação fornecida por todas as dependencias do projeto e abrir no navegador.

## Ambiente online:
https://play.rust-lang.org/

## Study sessions ministred by vcwild based on the book:
https://doc.rust-lang.org/book/title-page.html