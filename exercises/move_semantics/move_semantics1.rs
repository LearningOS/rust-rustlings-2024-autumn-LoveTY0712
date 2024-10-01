// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let vec0: Vec<i32> = Vec::new();
    let mut vec2=vec0.clone();
    let mut vec1 = fill_vec(vec0);
    // let x=String::from("hello");
    // let y=x.clone();
    // println!("{x}");
    vec2.push(22);

    println!("{} has length {} content `{:?}`", "vec2", vec2.len(), vec2);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec:Vec<i32> = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
