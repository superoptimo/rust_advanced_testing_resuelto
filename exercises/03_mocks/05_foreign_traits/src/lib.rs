//! Use `mock!` to generate a mock type named `MockParsed` that implements the `FromStr` trait from the standard library.
use std::str::FromStr;

mockall::mock! {
    pub Parsed{}

    impl FromStr for Parsed {
        type Err = String;
        fn from_str(s: &str) -> Result<Self, String>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn implements() {
        static_assertions::assert_impl_one!(MockParsed: FromStr);
    }
}
