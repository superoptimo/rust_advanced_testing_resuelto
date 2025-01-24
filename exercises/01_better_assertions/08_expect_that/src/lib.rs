//! Use `expect_that!` to simplify `assert_that!` invocations and to check multiple properties on the same object.
#[cfg(test)]
mod tests {
    use googletest::matchers::{all, each, empty, eq, ge, gt, lt, not};
    use googletest::{assert_that, expect_that, verify_current_test_outcome};

    #[googletest::test]
    fn multi_matcher() {
        let v: Vec<i32> = vec![1, 2, 3];
        // Convert the assertion below into two invocations of `expect_that!`.        
        //assert_that!(v, all!(each(lt(3)), each(gt(1))))
        expect_that!(v, each(lt(3)));
        expect_that!(v, each(gt(1)));
        // verify_current_test_outcome().unwrap()
    }

    #[googletest::test]
    fn multi_property() {
        struct Person {
            name: String,
            surname: String,
            age: u8,
        }

        let person = Person {
            name: "John".to_string(),
            surname: "".to_string(),
            age: 16,
        };

        expect_that!(person.name, not(eq("")));
        expect_that!(person.surname, not(eq("")));
        expect_that!(person.age,ge(18));
        // Check that name and surname are not empty and that age is greater or equal than 18.
    }

    #[googletest::test]
    // Notice that we are returning a `Result` from the test function.
    fn barrier() -> googletest::Result<()> {
        let v: Vec<i32> = vec![1, 2, 3];
        expect_that!(v, each(lt(3)));
        expect_that!(v, each(gt(1)));

        verify_current_test_outcome()?;

        // TODO: if any of the assertions above fails, we want to abort the test
        //   and return an error.
        //   Do it without changing the assertions above!
        //   Tip: search for a verification function in googletest's docs.

        expect_that!(v, empty());
        verify_current_test_outcome()
    }
}
