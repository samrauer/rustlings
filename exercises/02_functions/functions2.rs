// TODO: Add the missing type of the argument `num` after the colon `:`.

fn main() {
    call_me(3);
}


fn call_me(num:u32) {
    let mut x = 0;
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
        x += i;
    }
}
