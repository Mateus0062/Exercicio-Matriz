use std::fs::File;
use std::io::{BufReader, stdin};
use std::path::Path;

fn main() {
    let mut soma = 0;
    let mut o: String = String::new();
    let path = Path::new("./data/matriz.json");
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let matriz: Vec<Vec<i32>> = serde_json::from_reader(reader).unwrap();

    println!("Matriz: {:?}", matriz);

    println!("Digite a operação desejada: ");

    println!("Soma S ou Média M => ");
    stdin().read_line(&mut o).unwrap();

    let o = o.trim();

    for i in 0..matriz.len() {
        for j in 0..matriz.len() {
            if j > i && j > matriz.len() - i - 1 {
                print!("x ");
                soma += matriz[i][j];
            } else {
                print!(". ");
            }
        }
        println!();
    }

    let mut media:f64 = soma as f64 / 30.0;

    if o == "S" {
        println!("A soma é: {}", soma);
    } else if o == "M" {
        println!("A média é: {}", media);
    } else {
        println!("Opção inválida");
    }
}