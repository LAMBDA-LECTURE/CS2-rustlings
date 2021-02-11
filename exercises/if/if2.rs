// if2.rs

// Step 1: Make me compile!
// Step 2: Get the bar_for_fuzz and default_to_baz tests passing!
// Execute the command `rustlings hint if2` if you want a hint :)


pub fn fizz_if_foo(fizzish: &str) -> &str {
    if fizzish == "fizz" {
        "foo"
    }else if fizzish == "fuzz" {
        "bar"
    } else {
        "baz"
    }
    // alt way of using implicit returns, note, lines 19 & 21 require the explicit return
    // if fizzish == "fizz" {
    //     return "foo";
    // }else if fizzish == "fuzz" {
    //     return "bar";
    // }
    // "baz"
}



// No test changes needed!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        assert_eq!(fizz_if_foo("fizz"), "foo")
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(fizz_if_foo("fuzz"), "bar")
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(fizz_if_foo("literally anything"), "baz")
    }
}
