// A solid understanding of Rust's ownership rules is vital to working effectively in Rust.
// When a value is bound to a variable, that variable is said to have ownership of it.
// We'll go through several examples to help explain how ownership works.

// A variable's ownership of a value will last until that variable is out of scope.
// For example, a variable bound inside a function goes out of scope when the function ends.
#[test]
fn owning_a_value() {
  fn assign_a_value() {
    let x = 10;
  }
  assign_a_value();
  assert_eq!(x, 10);
}

// When a variable goes out of scope, it is cleaned up by Rust and its memory is deallocated.
// This cleanup also applies to everything associated with the variable.
#[test]
fn owning_a_value_2() {
  fn assign_a_value() {
    let x = 10;
    let y = &x;
  }
  assign_a_value();
  assert_eq!(y, &10);
}

// Another example of a variables ownership ending is the concept of "moving" the value.
// When a variable's value is moved to a new variable, the original binding is cleaned up.
#[test]
fn moving_a_value() {
  let name = String::from("Chris");
  let first_name = name;
  assert_eq!(name, "Chris".to_string());
}

// Some confusion can arise with moving values, because certain data types aren't moved.
// Primitive types like &'static str are copied rather than being moved, which means ownership doesn't end.
#[test]
fn copying_a_value() {
  let name = "Chris";
  let first_name = name;
  assert_eq!(name, __);
}

// The same will happen with integer types like i32. These types contain no pointers to other data.
// It's value is entirely self-contained and can be safely copied.
#[test]
fn copying_a_value_2() {
  let num:i32 = 12;
  let x = num;
  assert_eq!(x, __);
}

// Now that we've explored the difference between what types get moved and what types get copied,
// let's try an example with a Vec. Because the Vec contains pointers to other data, it can't be copied.
// This means that like the String up above, it will be moved.
#[test]
fn rebinding_a_vec() {
  let list = vec!["Rust", "Go", "C++"];
  let languages = list;
  assert_eq!(list[0], "Rust");
}

// Now that you've learned a bit about ownership in Rust, it's time to look at borrowing.
// When a binding "borrows" a value, it creates a reference to that value.

// One way to denote a reference to a value in Rust is using the & operator.
#[test]
fn simple_borrowing() {
  let name = String::from("Chris");
  let first_name = &name;
  assert_eq!(__, "Chris".to_string());
  assert_eq!(__, &"Chris".to_string());
}
// Unlike our earlier example, name has not been deallocated, because first_name has created a reference to it.

// By default, references will be immutable unless you explicitly made mutable.
#[test]
fn mutable_borrowing() {
  let mut count = 10;
  {
    let new_count = &count;
    *new_count += 1;
    assert_eq!(new_count, &11);
  }
  assert_eq!(count, 11);
}

// Borrowing can also be used to pass values through functions without needing to rebind them.
#[test]
fn borrowing_through_functions() {
  let mut vector = vec![1, 2, 3];

  fn insert_next_number(v: Vec<i32>) {
    let x = v.last().unwrap() + 1;
    v.push(x);
  }

  insert_next_number(vector);

  assert_eq!(vector, vec![1, 2, 3, 4]);
}

// Up until now, we've talked about things being cleaned up or deallocated,
// but we've avoided discussing lifetimes explicitly. That's because in most cases,
// Rust takes care of lifetimes for us.
#[test]
fn implicit_lifetime() {
  let x = 10;
  let y = 10;

  fn add(a: i32, b: i32) -> i32 {
    a + b
  }

  let sum = add(x, y);

  assert_eq!(sum, 20);
  assert_eq!(a, 10);
}

// Let's look at a similar function, but with references passed as arguments instead of values themselves.
// Lifetimes will need to be made explicit sometimes when passing around references. Because this max() function
// returns a reference, we must explicitly declare the lifetime of that reference.
#[test]
fn explicit_lifetime() {
  let x = 10;
  let y = 20;

  fn max<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if a >= b {
      a
    } else {
      b
    }
  }

  let max = max(&x, &y);

  assert_eq!(max, 20);
}
// Here we're saying that the i32 we return will have a lifetime equal to that of the function max().
