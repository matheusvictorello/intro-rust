fn main() {
    let mut u0: u32 = 42;
    let u1: *mut u32 = &mut u0;

    unsafe {
        *u1 += 1;
    }

    println!("{}", u0);
}