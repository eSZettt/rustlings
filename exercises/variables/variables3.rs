// variables3.rs
// Make me compile! Execute the command `rustlings hint variables3` if you want a hint :)


fn main() {
    let mut x:Option<i32> = None;
    println!("Number {}", x.unwrap_or(0));
    x.replace(5);
    println!("Noumber {}", x.unwrap_or(0));
}
