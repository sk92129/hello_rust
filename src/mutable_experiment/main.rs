fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // Immutable borrow
    let r2 = &s; // Fine: multiple readers allowed
    // let r3 = &mut s; // COMPILE ERROR: cannot borrow as mutable because it is also borrowed as immutable

    println!("{}, {}", r1, r2);
}