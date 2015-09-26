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

// Arrays can be iterated over.
#[test]
fn array_iteration() {
  let arr: [u8; 3] = [3, 2, 1];
  let mut iterator = arr.iter();
  assert!(iterator.next().unwrap() == &__);
  assert!(iterator.next().unwrap() == &__);
  assert!(iterator.next().unwrap() == &__);
}

// Arrays can also be mutated during iteration
#[test]
fn array_map() {
  let arr: [u32; 4] = [2, 5, 7, 4];
  let mut iterator = arr.iter().map(__);
  assert!(iterator.next() == Some(4));
  assert!(iterator.next() == Some(10));
  assert!(iterator.next() == Some(14));
  assert!(iterator.next() == Some(8));
}

// You can filter an array for results that match a given condition
#[test]
fn array_filter() {
  let arr: [u16; 5] = [1, 2, 3, 4, 5];
  let mut iterator = arr.iter().filter(__);
  assert!(iterator.next().unwrap() == &2);
  assert!(iterator.next().unwrap() == &4);
  assert!(iterator.next().is_none());
}

// Filter and map can be combined to do both at once
#[test]
fn array_filter_map() {
  let arr: [u8; 5] = [2, 1, 2, 1, 2];
  let mut iterator = arr.iter().filter_map(|&x|
    if x == 1 {Some(__)} else {None}
  );
  assert!(iterator.next() == Some(3));
  assert!(iterator.next() == Some(3));
  assert!(iterator.next().is_none());
}

// This can be used for more complex logic as well
#[test]
fn complex_array_filter_map() {
  let arr: [u64; 4] = [4, 8, 16, 32];
  let mut iterator = arr.iter().filter_map(|&x|
    if (x as f64).sqrt().floor() == (x as f64).sqrt() {Some((x as f64).sqrt() as u64)} else {None}
  );
  assert!(iterator.next().unwrap() == __);
  assert!(iterator.next().unwrap() == __);
  assert!(iterator.next().is_none());
}

// Arrays can also be iterated through using a for loop
#[test]
fn for_loops() {
  let arr: [u64; 3] = [1, 2, 3];
  let mut y: u64 = 1;
  for x in &arr {
    assert!(*x == y);
  }
}

// Let's try iterating over an array of strings to build a sentence
#[test]
fn for_loops_two() {
  let words: [&'static str; 3] = ["I", "love", "Rust"];
  let mut sentence: String = String::new();
  for word in words.iter() {
    __
  }
  println!("{:?}", sentence);
  assert!(sentence == "I love Rust".to_string());
}
