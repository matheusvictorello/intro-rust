fn main() {
    let u0 = 37;

    let s = if u0 > 42 {
        "maior que 42"
    } else if u0 > 35 {
        "maior que 35"
    } else {
        "não serve"
    }; // Obrigatório

    println!("{}", s);
}