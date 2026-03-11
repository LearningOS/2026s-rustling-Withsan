// cow1.rs
//
// This exercise explores the Cow, or Clone-On-Write type. Cow is a
// clone-on-write smart pointer. It can enclose and provide immutable access to
// borrowed data, and clone the data lazily when mutation or ownership is
// required. The type is designed to work with general borrowed data via the
// Borrow trait.
//
// This exercise is meant to show you what to expect when passing data to Cow.
// Fix the unit tests by checking for Cow::Owned(_) and Cow::Borrowed(_) at the
// TODO markers.
//
// Execute `rustlings hint cow1` or use the `hint` watch subcommand for a hint.


use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Clone occurs because `input` needs to be mutated.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[derive(PartialEq, Debug)]
    pub enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    fn main() {
        println!("This is an empty cons list: {:?}", create_empty_list());
        println!(
            "This is a non-empty cons list: {:?}",
            create_non_empty_list()
        );
    }

    pub fn create_empty_list() -> List {
        List::Nil
    }

    pub fn create_non_empty_list() -> List {
        List::Cons(32, Box::new(List::Nil))
    }
}
