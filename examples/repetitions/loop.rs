fn main() {
    let mut numero = 0;

    // loop infinito
    loop {
        numero += 1;
        println!("{}", numero);

        if numero >= 10 {
            // sai do loop
            break;
        }
    }
}
