extern crate test;

// `test_case` causes ambiguous resolution errors with `pretty_assertions`:
// https://github.com/frondeus/test-case/issues/109
pub use pretty_assertions::{
    assert_eq as assert_eqp, assert_ne as assert_nep, assert_str_eq as assert_str_eqp,
};
pub use test::Bencher;
// `test_case::test_case` conflicts with `test::test_case`:
// https://doc.rust-lang.org/beta/unstable-book/language-features/custom-test-frameworks.html
pub use test_case::test_case as case;
