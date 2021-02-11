// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!


fn main() {
    let a = [0; 100];
    //let v = Vec::with_capacity(100);
    //only useful to prevent allocations from overrunning. so a performant solution, if odd, up until it's 100, then it overruns.

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
