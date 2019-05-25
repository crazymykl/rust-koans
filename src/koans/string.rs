// Rust has string literals called string slices (&str)
// String slices are static and cannot be mutated
// They are functionally a pointer with an unchangeable length
#[test]
fn string_literals() {
    let string_slice = "Hello World";
    assert!(string_slice == __);
}

// A String is a heap-allocated string in Rust.
// It is mutable and growable
#[test]
fn growable_strings() {
    let mut string = String::new();
    string.push_str("Hello");
    string.push_str(" World");
    assert!(string == __);
}

// A string slice can be converted to a String using to_string
// But since converting &str to a String involved memory allocation
// It is cheaper to keep them as &strs if possible
#[test]
fn growable_string_literals() {
    let mut mutable = "Foo".to_string();
    mutable.push_str("Bar");
    assert!(mutable == __);
}

// A String can be coerced into a slice by prefacing it with a &
#[test]
fn string_to_slice() {
    let string = "Can't stop me now".to_string();
    let slice: &str = __;
    assert!(slice == "Can't stop me now");
}

// You can concat a &str to a String at the end
#[test]
fn strings_with_strs() {
    let hello = "Hello ".to_string();
    let world = "World";

    assert!(hello + world == __);
}

// But two Strings require a & to coerce the second String
#[test]
fn strings_with_strings() {
    let hello = "Hello ".to_string();
    let world = "World!".to_string();

    let hello_world = __ + __;
    assert!(hello_world == "Hello World!")
}

// Strings cannot be indexed as they are UTF-8 encoded
// Some UTF-8 characters can be multiple bytes long
// But you can access the string as chars and iterate from there using nth
#[test]
fn using_chars() {
    let string = "Anybody hungry?";
    assert!(string.chars().nth(2) == __.chars().nth(0));
}

// You can get a slice of a string using slicing syntax
#[test]
fn slicing_the_string() {
    let string = "Boom";
    assert!(&string[0..2] == __);
}

// However these are BYTE offsets not character offsets -
// rust will panic if you try and slice within a character boundary
#[test]
#[should_panic]
fn slicing_the_byte() {
    let dog = "忠犬ハチ公";
    let slice = &dog[0..2];
}

// You can truncate Strings
#[test]
fn truncate_string() {
    let mut string = String::from("Hello World!");
    string.truncate(5);
    assert!(string == __);
}

// You can pop Strings
#[test]
fn pop_string() {
    let mut string = String::from("Hello");
    string.pop();
    assert!(string == __);
}

// You can insert and remove from a String (at byte position)
#[test]
fn insert_and_remove_into_string() {
    let mut string = String::from("Hello");
    string.insert(2, 'e');
    assert!(string == __);

    string.remove(2);
    assert!(string == __);
}
