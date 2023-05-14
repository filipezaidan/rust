//  Cria um loop for que percorre um range de números de 1 a 10, em ordem reversa, utilizando a função "rev()".
fn main() {
    for numero in (1..=10).rev() {
        println!("{}", numero);
    }
}
