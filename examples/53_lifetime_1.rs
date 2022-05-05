fn foo(v: &mut [u32]) {
    println!("{:?}", v);
}

fn main() {
    let mut v = vec![0, 1, 2, 3, 4, 5]; // 'v' nasceu

    let r0 = &mut v; // 'r0' nasceu
    foo(r0);         // 'r0' agora é de 'foo'
    
    let r1 = &mut v; // 'r1' nasceu
    foo(r1);         // 'r1' agora é de 'foo'

    // 'v' morreu
}