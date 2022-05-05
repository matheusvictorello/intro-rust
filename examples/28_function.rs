fn foo() -> u32 {
    42
}

fn main() -> () {
    let u0 = foo();

    println!("{:?}", u0);
}