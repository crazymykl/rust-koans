// Traits in Rust are a way of guaranteeing particular functionality for a type.
// They let the compiler know that a type is capable of necessary functions to
// help it ensure safety.
#[test]
fn implementing_traits() {
  struct Person {
    first_name: &'static str,
    last_name: &'static str
  }

  // Any type that implements the HasName trait is guaranteed to have a
  // function called full_name() that returns a String
  trait HasName {
    fn full_name(&self) -> String;
  }

  impl Person {
    fn full_name(&self) -> String {
      format!("{} {}", self.first_name, self.last_name)
    }
  }

  let person = Person {
    first_name: "Chris",
    last_name: "Cerami"
  };

  // The assert_full_name function needs to know that its argument can call
  // full_name(). In order to guarantee this, it is cast to receive any
  // argument type that has implemented the HasName trait.
  fn assert_full_name<T: HasName>(person: T) {
    assert_eq!(person.full_name(), "Chris Cerami");
  }

  assert_full_name(person);
}

// In order to implement a trait, a type must implement ALL of its functions.
// This is required in order to guarantee that any type that shares that trait
// will be able to respond to those functions.
#[test]
fn implementing_traits2() {
  struct Character {
    name: &'static str,
    level: u16
  }

  trait HasLevel {
    fn level_up(&mut self) -> u16;

    fn print_level(&self);
  }

  impl HasLevel for Character {
    fn level_up(&mut self) -> u16 {
      self.level += 1;
      self.level
    }
  }

  let mut durz = Character {
    name: "Durz",
    level: 2
  };

  fn test_level_up<T: HasLevel>(character: &mut T) {
    assert_eq!(character.level_up(), 3);
  }

  test_level_up(&mut durz);
}

// Now let's try creating a trait an implementing it for an existing type.
#[test]
fn creating_traits() {
  let num_one: u16 = 3;
  let num_two: u16 = 4;

  assert!(!num_one.is_even());
  assert!(num_two.is_even());
}
