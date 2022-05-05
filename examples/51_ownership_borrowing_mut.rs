// Não compila
fn foo(v: &mut [u32]) {
    println!("{:?}", v);
}

fn main() {
    let mut v = vec![0, 1, 2, 3, 4, 5];

    let r0 = &mut v;
    let r1 = &mut v;

    foo(r0);
    foo(r1);
}