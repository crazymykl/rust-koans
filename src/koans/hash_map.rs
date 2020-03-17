use std::collections::HashMap;

// A HashMap is a data structure that contains key-value pairs
#[test]
fn simple_hash_map() {
    let mut hm = HashMap::new();
    hm.insert("first", 1);
    hm.insert("second", 2);
    assert_eq!(hm.len(), __);
}

// You can access the values of a HashMap using the correlating key
#[test]
fn hash_map_get() {
    let mut map = HashMap::new();
    map.insert("Rust", "https://www.rust-lang.org/");
    map.insert("Ruby", "https://www.ruby-lang.org/");
    assert_eq!(map.get(__), Some(&"https://www.rust-lang.org/"));
}

// Attempting to retrieve a key that doesn't exist will return a None option
#[test]
fn its_not_there() {
    let map: HashMap<&str, &str> = HashMap::new();
    assert_eq!(map.get("Rust"), __);
}

// Instead of the get() method, values can also be retrieved using []
#[test]
fn brackets() {
    let mut map = HashMap::new();
    map.insert("iPhone", "Apple");
    map.insert("Galaxy", "Samsung");
    assert_eq!(map[&"iPhone"], "Apple");
    assert_eq!(__, "Samsung");
}

// Keys in HashMaps will always be unique
#[test]
fn duplicate_key() {
    let mut hm = HashMap::new();
    hm.insert("Harry Potter", "Sorcerer's Stone");
    hm.insert("Harry Potter", "Goblet of Fire");
    assert_eq!(hm[&"Harry Potter"], "Sorcerer's Stone");
}

// A HashMap's values, however, do not have this constraint
#[test]
fn duplicate_values() {
    let mut hm = HashMap::new();
    assert_eq!(hm[&"Sorcerer's Stone"], hm[&"Goblet of Fire"]);
}

// You can create an iterator of all of the keys in a HashMap
#[test]
fn just_the_keys() {
    let mut map = HashMap::new();
    map.insert("Episode IV", "A New Hope");
    map.insert("Episode V", "Empire Strikes Back");
    map.insert("Episode VI", "Return of the Jedi");
    let episodes = vec![__];
    for episode in map.keys() {
        assert!(episodes.contains(episode));
    }
}

// You can do the same for the values in a HashMap
#[test]
fn just_the_values() {
    let mut map = HashMap::new();
    map.insert("One", "Fish");
    map.insert("Two", "Fish");
    map.insert("Red", "Fish");
    map.insert("Blue", "Fish");
    for num in map.values() {
        assert_eq!(num, __);
    }
}

// You can also iterate through all of the key value pairs together
#[test]
fn iterating() {
    let mut map = HashMap::new();
    map.insert(1, 1);
    map.insert(2, 4);
    map.insert(3, 9);
    for (key, value) in map.iter() {
        assert_eq!(&(__), value);
    }
}

// Rather than calling .iter() on the HashMap, you can also iterate over a reference to it
#[test]
fn iterating_2() {
    let mut map = HashMap::new();
    map.insert(1, 1);
    map.insert(2, 4);
    map.insert(3, 9);
    for __ in &map {
        assert_eq!(__, value);
    }
}

// If we no longer need the content of a HashMap, it can be cleared and reused
#[test]
fn clearing() {
    let mut map = HashMap::new();
    map.insert("chairs", 30);
    map.insert("tables", 8);
    assert_eq!(map.get("chairs"), None);
}

// The entry method can be used to concisely insert a key only if it is missing.
#[test]
fn entry() {
    let mut map = HashMap::new();
    map.insert(2, 7);
    assert_eq!(*map.entry(3).or_insert(9), __);
    assert_eq!(*map.entry(2).or_insert(15), __);
    assert_eq!(map.get(&3), __);
}

// HashMaps can be extended by adding iterators over pairs whose types line up.
#[test]
fn extend() {
    let mut map = HashMap::new();
    map.insert("Apples", 1);
    map.extend(vec![("Oranges",2), __].into_iter());
    
    assert_eq!(map.get("Apples"), Some(&1));
    assert_eq!(map.get("Oranges"), Some(&2));
    assert_eq!(map.get("Celery"), Some(&-2));
}

// HashMaps can also be created from iterators.
#[test]
fn from_tuple_iter() {
    use std::collections::hash_map::RandomState;
    use std::iter::FromIterator;

    let tuples = vec![(1,4), (2,5), (3,6), (7,8)];
    // The state of a HashMap
    let map: HashMap<i32, i32, RandomState> = HashMap::from_iter(tuples.into_iter());
    
    assert_eq!(map.get(&2), __);
    assert_eq!(map.get(&3), __);
    assert_eq!(map.get(&5), __);
}