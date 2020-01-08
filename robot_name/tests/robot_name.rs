use robot_name::Factory;

#[test]
fn test_different_names() {
    let f = Factory::new();
    let r1 = f.build().unwrap();
    let r2 = f.build().unwrap();

    assert_ne!(r1.name(), r2.name());
}
