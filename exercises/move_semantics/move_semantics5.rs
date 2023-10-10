// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    //https://github.com/rust-lang/rustlings/issues/809
    // https://doc.rust-lang.org/nomicon/lifetimes.html#the-area-covered-by-a-lifetime
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut *y;
    *z += 1000;
    assert_eq!(x, 1200);
}
