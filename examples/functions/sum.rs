// Declaração de uma função simples que recebe dois argumentos `x` e `y` e retorna a sua soma.
fn soma(x: i32, y: i32) -> i32 {
    let resultado = x + y;
    // Rust é uma linguagem de programação "expression-based", então a última expressão é implicitamente retornada.
    resultado
}

fn main() {
    let a = 10;
    let b = 20;
    let resultado = soma(a, b);
    println!("{} + {} = {}", a, b, resultado); // saída: "10 + 20 = 30"
}
