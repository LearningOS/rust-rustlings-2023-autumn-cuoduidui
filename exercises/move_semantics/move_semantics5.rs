// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

// Rust中使用&符号表示引用，也叫引用操作符。其使用场景是只使用类型的值但不获取其所有权，同时Rust的引用规则为：
// 1在作用域中的数据有且只能有一个可变引用；
// 可以有多个不可变引用；
// 不能同时拥有不可变引用和可变引用。
// 引用必须总是有效的
fn main() {
    //https://github.com/rust-lang/rustlings/issues/809
    // https://doc.rust-lang.org/nomicon/lifetimes.html#the-area-covered-by-a-lifetime
    // https://blog.51cto.com/u_14834727/3031717
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut *y;
    *z += 1000;
    assert_eq!(x, 1200);
}
