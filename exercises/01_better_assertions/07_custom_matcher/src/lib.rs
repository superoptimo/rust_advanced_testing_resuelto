//! Write a custom `is_redirect` matcher that checks if a `StatusCode` is a redirect.
use googletest::matcher::Matcher;
use http::StatusCode;

pub fn is_redirect() -> impl Matcher<ActualT = StatusCode> {
    struct Redirect(StatusCode);

    impl Matcher for Redirect
    {
        type ActualT = StatusCode;
    
        fn matches(&self, actual: &Self::ActualT) -> googletest::matcher::MatcherResult {
            (self.0 == *actual).into()
        }
    
        fn describe(&self, matcher_result: googletest::matcher::MatcherResult) -> googletest::description::Description {
            match matcher_result {
                googletest::matcher::MatcherResult::Match => format!("is a redirection status code").into(),
                googletest::matcher::MatcherResult::NoMatch => format!("isn't a redirection status code").into(),
            }
        }
    }
    // return instance
    Redirect(StatusCode::MOVED_PERMANENTLY)
}

#[cfg(test)]
mod tests {
    use crate::is_redirect;
    use googletest::assert_that;
    use http::StatusCode;

    #[test]
    fn success() {
        assert_that!(StatusCode::MOVED_PERMANENTLY, is_redirect());
    }

    #[test]
    fn failure() {
        assert_that!(StatusCode::OK, is_redirect());
    }
}
