#[derive(Debug)]
struct Dados<'a, 'b> {
    altura:  &'a u32,
    largura: &'b u32,
}

fn main() {
    let dados = Dados {
        altura:  &35,
        largura: &35,
    };

    println!("{:#?}", dados);
}