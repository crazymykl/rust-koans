// The elements of an array can be accessed by their indices
// arr[4]
#[test]
fn array_index() {
  let arr: [i32; 5] = [1, 2, 3, 4, 5];
  assert!(arr[__] == 1);
}

// A new fixed size array can be created by declaring the type of its elements
// along with its length
// [i32; 0] = []
#[test]
fn array_empty() {
  let arr: __;
  assert!(arr.len() == 0);
}

// Attempting to access an array at an index that is
// out of its bounds will cause an error. Let's cause
// that error in this example.
#[test]
#[should_panic]
fn out_of_index() {
  let arr: [&'static str; 5] = ["rust", "is", "mostly", "for", "nerds"];
  arr[__];
}

// Elements can be replaced in an array at a certain index.
// hint: Without the 'mut' keyword, you won't be able to change data.
#[test]
fn insert_at_index() {
  let mut arr: [u8; 5] = [0, 1, 2, 3, 4];
  __ = 0;
  assert!(arr == [0, 1, 2, 3, 0]);
}
