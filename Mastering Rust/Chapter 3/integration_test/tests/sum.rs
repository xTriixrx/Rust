// integration_test/tests/sum.rs

mod common;
use integration_test::add;

use common::{setup, teardown};

#[test]
fn add_test()
{
    assert_eq!(add(6, 8), 14);
}

#[test]
fn test_with_fixture()
{
    setup();
    assert_eq!(add(7, 14), 21);
    teardown();
}