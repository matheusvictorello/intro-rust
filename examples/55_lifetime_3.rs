fn main() {
    let r: &u32 = {
        static v: u32 = 35;

        &v
    };

    println!("{}", r);
}