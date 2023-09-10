pub mod math;
pub mod arr;

fn main() {
    println!("Hello, codecov!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
