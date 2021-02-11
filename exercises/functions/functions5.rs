// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)


fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
    //this broke bc the `;` made the line a statement, not an expression, and line 11 says we're expecting an expression (by putting the `-> i32 {` )
    // num * num;
    num * num
}
