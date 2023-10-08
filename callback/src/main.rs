fn main() {
    let mut i = 1;
    let incr = || {
        i += 1;
        println!("i = {}", i);
    };
    call_twice(|| i += 1);
    assert_eq!(i, 3);

}

fn call_twice<F>(mut closure: F) where F: FnMut() {
    closure();
    closure();
}


