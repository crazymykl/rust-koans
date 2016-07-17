// Structs are a convenient way to wrap up related data into one neatly packaged structure.
// Let's consider the example of a Struct that allows us to package together data about a person.
#[test]
fn our_first_struct() {
  struct Person {
    name: &'static str,
    age: u32
  }

  let jim = Person { name: "Jim", age: 57 };

  assert_eq!(jim.name, __);
  assert_eq!(jim.age, __);
}

// Let's try another example
#[test]
fn one_more_struct() {
  struct Movie {
    title: &'static str,
    runtime: u32
  }

  let movie = Movie { title: "Star Wars", runtime: __ };

  assert_eq!(movie.title, __;
  assert_eq!(movie.runtime, 121);
}

// Structs, like anything in Rust, are immutable by default.
// If we create a mutable instance of a Struct, we can reassign its attributes.
#[test]
fn mutable_structs() {
  struct Language {
    version: &'static str
  }

  let mut rust = Language { version: "1.3.0" };

  __ = "1.4.0";

  assert_eq!(rust.version, "1.4.0");
}

// We can also decide to temporarily allow a Struct to be mutable
#[test]
fn revoking_mutability() {
  struct Language {
    version: &'static str
  }

  let mut rust = Language { version: "1.3.0" };

  rust.version = "1.4.0";

  assert_eq!(rust.version, "1.4.0");

  let rust = rust;

  rust.version = "1.5.0";

  assert_eq!(rust.version, "1.5.0");
}

// There may be cases where you want to create a new instance of a Struct
// that is only slightly different from an existing one
#[test]
fn dot_merging() {
  struct Account {
    holder: &'static str,
    account_number: &'static str,
    balance: f64
  }

  let broke = Account { holder: "Morgan Stanley", account_number: "00021948523756312", balance: 0.00 };

  let rich = Account { balance: 1000000.00, .. broke };

  assert_eq!(rich.holder, __);
  assert_eq!(rich.balance, __);
}
