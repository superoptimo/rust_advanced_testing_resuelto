//! Refactor the `square` function to ask for a type that implements the `Logger` trait rather than the concrete
//! `PrintlnLogger` type.\
//! Then pass a `TestLogger` to `square` in the test. `TestLogger` should implement `Logger` and do nothing
//! when `log` is called.

pub fn square<TLog:Logger>(x: i32, logger: &TLog) -> i32 {
    let y = x * x;
    logger.log(&format!("{}^2 == {}", x, y));
    y
}

pub trait Logger
{
    fn log(&self, msg: &str);
}

pub struct PrintlnLogger;

impl PrintlnLogger {
    pub fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}

impl Logger for PrintlnLogger
{
    fn log(&self, msg: &str)
    {
        PrintlnLogger::log(self, msg);
    }
}

#[cfg(test)]
mod tests {
    use super::square;
    use super::Logger;
    use googletest::assert_that;
    use googletest::matchers::eq;

    #[test]
    fn square_works() {
        struct TestLogger;
        impl Logger for TestLogger{
            fn log(&self, _msg: &str) {
                
            }
        }
        assert_that!(square(2, &TestLogger), eq(4));
    }
}
