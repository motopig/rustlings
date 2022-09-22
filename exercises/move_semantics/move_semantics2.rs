// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.



fn main() {

    let mut v: Vec<i32> = Vec::new();
    v.push(200);
    // v is first moved into print_len's v1
    // and then moved into v2 when print_len returns it
    let v2 = print_len(v.clone());
    println!("{}",v[0]);
    fn print_len(v1:Vec<i32>) ->Vec<i32> {
        println!("v1's length is {}", v1.len());
        return v1;     // v1 is moved out of the function
    }

    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0.clone());

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
