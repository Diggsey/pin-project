#[cfg_attr(not(cargo_expand), ignore)]
#[rustversion::attr(not(nightly), ignore)]
#[test]
fn expandtest() {
    macrotest::expand("tests/enum/*.rs");
    macrotest::expand("tests/struct/*.rs");
}
