fn main() {
    let o0: Option<u32> = None;
    let o1: Option<u32> = Some(42);

    let r0: Result<u32, &str> = Err("Deu errado");
    let r1: Result<u32, &str> = Ok(35);

    println!("{:?}", o0);
    println!("{:?}", o1);
    println!("{:?}", r0);
    println!("{:?}", r1);
}