extern crate enumflags;
#[macro_use]
extern crate enumflags_derive;

//use enumflags::BitFlags;

#[derive(EnumFlags, Copy, Clone, Debug)]
#[repr(u8)]
enum Test {
    A = 0b0001,
    B = 0b0010,
    C = 0b0100,
    D = 0b1000,
}

#[test]
fn test_foo() {
    use enumflags::BitFlags;
    assert_eq!(
        BitFlags::<Test>::all(),
        Test::A | Test::B | Test::C | Test::D
    );
    assert_eq!(BitFlags::<Test>::all() & Test::A, Test::A);
    assert_eq!(!Test::A, Test::B | Test::C | Test::D);
    assert_eq!((Test::A | Test::C) ^ (Test::C | Test::B), Test::A | Test::B);
    assert_eq!(BitFlags::<Test>::from_bits_truncate(4), Test::C);
    assert_eq!(BitFlags::<Test>::from_bits_truncate(5), Test::A | Test::C);
    assert_eq!(
        BitFlags::<Test>::from_bits_truncate(16),
        BitFlags::<Test>::empty()
    );
    assert_eq!(BitFlags::<Test>::from_bits_truncate(17), Test::A);
    assert_eq!(BitFlags::<Test>::from_bits(17), None);
    assert_eq!(
        BitFlags::<Test>::from_bits(15),
        Some(BitFlags::<Test>::all())
    );
    {
        let mut b = (Test::A | Test::B);
        b.insert(Test::C);
        assert_eq!(b, Test::A | Test::B | Test::C);
    }
    assert!((Test::A | Test::B).intersects(Test::B));
    assert!(!(Test::A | Test::B).intersects(Test::C));
    assert!((Test::A | Test::B | Test::C).contains(Test::A | Test::B));
    assert!(!(Test::A | Test::B | Test::C).contains(Test::A | Test::D));
    assert_eq!(!(Test::A | Test::B), Test::C | Test::D);
    assert_eq!((Test::A | Test::B).bits(), 3);
    assert_eq!((Test::A | Test::B).not().bits(), 12);
    assert_eq!(BitFlags::<Test>::all().bits(), 15);
}
