// Não compila
enum Cor {
    Vermelho,
    Verde,
    Azul,
}

fn main() {
    let c0 = Cor::Vermelho;
    let c1 = Cor::Verde;
    let c2 = Cor::Azul;

    println!("{:?} {:?} {:?}", c0, c1, c2);
}