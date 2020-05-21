fn add_two_numbers(arg1: i32, arg2: i32) -> i32 {
    return arg1 + arg2;
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_numbers() {
        assert_eq!(6, add_two_numbers(2, 4));
    }
}
