// Traits in Rust are a way of guaranteeing particular functionality for a type.
// They let the compiler know that a type is capable of necessary functions to
// help it ensure safety. For more information, check the book:
// https://doc.rust-lang.org/nightly/book/traits.html
#[test]
fn implementing_traits() {
    struct Person {
        first_name: &'static str,
        last_name: &'static str,
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
        last_name: "Cerami",
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
        level: u16,
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
        level: 2,
    };

    fn test_level_up<T: HasLevel>(character: &mut T) {
        assert_eq!(character.level_up(), 3);
    }

    test_level_up(&mut durz);
}

// Now let's try creating a trait and implementing it for an existing type.
#[test]
fn creating_traits() {
    let num_one: u16 = 3;
    let num_two: u16 = 4;

    fn asserts<T: IsEvenOrOdd>(x: T, y: T) {
        assert!(!x.is_even());
        assert!(y.is_even());
    }

    asserts(num_one, num_two);
}

// We can also add trait constraints, or "bounds", to structs that we create.
// Using this pattern, we can use generic types and still ensure type safety.
#[test]
fn trait_constraints_on_structs() {
    struct Language<T> {
        stable_version: T,
        latest_version: T,
    }

    impl<__> Language<T> {
        fn is_stable(&self) -> bool {
            self.latest_version >= self.stable_version
        }
    }

    let rust = Language {
        stable_version: "1.3.0",
        latest_version: "1.5.0",
    };

    assert!(rust.is_stable());
}

// There is an alternate syntax for placing trait bounds on a function, the
// where clause. Let's revisit a previous example, this time using 'where'.
#[test]
fn where_clause() {
    let num_one: u16 = 3;
    let num_two: u16 = 4;

    trait IsEvenOrOdd {
        fn is_even(&self) -> bool;
    }

    impl IsEvenOrOdd for u16 {
        fn is_even(&self) -> bool {
            self % 2 == 0
        }
    }

    fn asserts<T>(x: T, y: T) {
        assert!(!x.is_even());
        assert!(y.is_even());
    }

    asserts(num_one, num_two);
}

// While you can always allow the implementor of a trait declare its functions,
// you can also supply default functionality. Let's revisit IsEvenOrOdd.
#[test]
fn default_functions() {
    let num_one: u16 = 3;
    let num_two: u16 = 4;

    trait IsEvenOrOdd {
        fn is_even(&self) -> bool;
        fn is_odd(&self) -> bool {
            __
        }
    }

    impl IsEvenOrOdd for u16 {
        fn is_even(&self) -> bool {
            self % 2 == 0
        }
    }

    fn asserts<T: IsEvenOrOdd>(x: T, y: T) {
        assert!(x.is_odd());
        assert!(y.is_even());
    }

    asserts(num_one, num_two);
}

// You can also create traits that inherit from other traits.
// In order to implement a child trait, you must first implement its parent.
// In this example, Bawks doesn't implement PartialOrd, so it fails to
// meet the requirements for the Ordered trait.
#[test]
fn inheritance() {
    use std::cmp::Ordering;

    #[derive(PartialEq)]
    struct Bawks<T> {
        thingy: T
    }

    impl<T: PartialOrd> PartialOrd for Bawks<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            __
        }
    }

    trait Ordered: PartialOrd {
        fn is_before(&self, other: &Self) -> bool;
    }

    impl<T: PartialOrd> Ordered for Bawks<T> {
        fn is_before(&self, other: &Self) -> bool {
            self < other
        }
    }

    let a = Bawks { thingy: 5.0 };
    let b = Bawks { thingy: 7.0 };

    assert!(a.is_before(&b));
}
