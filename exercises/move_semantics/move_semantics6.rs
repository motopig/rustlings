// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.

/*
    get_char 第一次使用data 需要引用data指针，不然 string_uppercase 再次使用data会报错 data已经所有权已经move 无法再次使用
    
*/

fn main() {
    let data = "Rust is great!".to_string();
    
    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
