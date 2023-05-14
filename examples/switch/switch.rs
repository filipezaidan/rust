// Utiliza uma expressão "match" para testar o valor de uma variável "valor" e executar diferentes blocos de código com base no valor correspondente.
fn main() {
    let valor = 4;
    match valor {
        1 => println!("O valor é um"),
        2..=5 => println!("O valor está entre 2 e 5"),
        6 | 7 => println!("O valor é 6 ou 7"),
        _ => println!("O valor é outro número"),
    }
}
