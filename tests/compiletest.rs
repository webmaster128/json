#[rustversion::attr(not(nightly), ignore)]
#[cfg(feature = "floats")]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
