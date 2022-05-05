#[derive(Debug)]
struct Dados<'a, 'b> {
    altura:  &'a u32,
    largura: &'b u32,
}

fn main() {
    let dados = {
        let altura = 35;

        Dados {
            altura:  &altura.clone(),
            largura: &35,
        }
    };

    println!("{:#?}", dados);
}