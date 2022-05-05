// Não compila
fn main() {
    let r: &u32 = {
        let v: u32 = 35;

        &v
    };

    println!("{}", r);
}