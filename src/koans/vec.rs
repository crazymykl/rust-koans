// Vecs act sort of like arrays, but allow more flexibility
#[test]
fn making_an_empty_vec() {
  let vector: Vec<()> = __
  assert!(vector.len() == 0);
}

// The vec! macro makes it easier to instantiate a vec
#[test]
fn vec_macro() {
  let vector = __
  assert!(vector.len() == 4);
}

// Elements can be added to mutable Vecs
#[test]
fn adding_to_vecs() {
  let mut vector = vec![0, 1, 2];
  vector.__(3);
  assert!(vector.len() == 4);
}

// Elements can also be remove from mutable Vecs
#[test]
fn removing_from_end_of_vecs() {
  let mut vector = vec![0, 1, 2, 3];
  vector.__();
  assert!(vector.len() == 3);
}

// The elements removed can also be stored to another variable
#[test]
fn storing_vec_elements(){
  let mut vector = vec![0, 1, 2, 3];
  let num = vector.__();
  assert!(num == 3);
}

// Vecs can change size to fit their contents
// Vecs do still need to be declared as mutable in order to change
#[test]
fn changing_size_of_vecs() {
  let mut vector = vec![1, 2, 3];
  __
  assert!(vector.len() == 4);
  __
  assert!(vector.len() == 3);
}

// Vecs have a certain maximum capacity at any given point
// When this capacity is reached, they will allocate more memory
#[test]
fn capacity() {
  let mut vector = vec![1, 2, 3, 4];
  assert_eq!(vector.capacity(), 4);
  vector.__;
  assert_eq!(vector.capacity(), 8);
  vector.pop();
  assert_eq!(vector.capacity(), __);
}

// This extra memory can also be deallocated when its no longer needed
#[test]
fn shrink_vecs() {
  let mut vector = vec![1, 2, 3, 4, 5];
  assert_eq!(vector.capacity(), 5);
  vector.pop();
  assert_eq!(vector.capacity(), __);
  vector.shrink_to_fit();
  assert_eq!(vector.capacity(), __);
}

// Vecs can reserve more space in order to prevent allocating several times
#[test]
fn reserve() {
  let mut vector = vec![1];
  vector.reserve(__);
  assert_eq!(vector.capacity(), 8);
}

// You can also cut Vecs down to size
#[test]
fn truncate() {
  let mut vector = vec![1, 2, 3, 4, 5];
  vector.__;
  assert_eq!(vector, vec![1, 2]);
}

// New elements can be stuffed into mutable Vecors
#[test]
fn insert() {
  let mut vector = vec![1, 2, 3, 4, 5];
  vector.insert(2, 6);
  assert_eq!(vector, __);
}

// Elements can also be deleted a particular position in a Vector
#[test]
fn remove() {
  let mut vector = vec![1, 2, 3, 4, 5];
  vector.remove(__);
  assert_eq!(vector, vec![2, 3, 4, 5])
}
