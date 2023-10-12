// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });
    // 取Option里面的结构体的时候,我们需要知道,结构体在option里面,
    // 所有权是归Option值的,后面又访问了一次Option值,所以说y还没有放弃所有权,只能通过引用去访问.(y没有放弃所有权,里面的结构体也是属于y的,y= Some(p)来取数据是不对的)
    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
