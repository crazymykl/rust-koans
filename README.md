# Rust Koans

## The Structure

The koans are broken out into areas by file, arrays are covered in arrays.rs, booleans are introduced in boolean.rs, etc. They are presented in order in the path_to_enlightenment.rs file.

Each koan builds up your knowledge of Rust and builds upon itself. By following the failures and errors presented, you will know what to work on next. As you finish one koan file, the next will be added to path_to_enlightenment.rs.

Some koans simply need to have the correct answer substituted for an incorrect one. Some, however, require you to supply your own answer. If you see the __ (a double underscore) listed, it is a hint to you to supply your own code in order to make it work correctly.

## Installing Rust

If you do not have Rust setup, please visit [rust-lang.org](https://www.rust-lang.org/) for operating specific instructions. In order to run the koans you need Rust installed. To check your installations simply type:

*nix platforms from any terminal window:

```
$ rustc --version
```
Currently, a Rust version of 1.3.0 or higher is recommended.

## The Path to Englightenment

In order to run the koans, simply run:

```
$ cargo run
```

This will generate the `path_to_enlightenment.rs` file and populate it with the first test in the list. After supplying an answer for the first test case, entering `cargo run` again will continue you on your path.

### Red, Green, Refactor

Red, Green, Refactor

In test-driven development the mantra has always been red, green, refactor. Write a failing test and run it (red), make the test pass (green), then look at the code and consider if you can make it any better (refactor).

While walking the path to Rust enlightenment you will need to run the koan and see it fail (red), make the test pass (green), then take a moment and reflect upon the test to see what it is teaching you and improve the code to better communicate its intent (refactor).

## Other Resources

The Rust Language    | https://www.rust-lang.org/
---------------------|----------------------------
Rust Playground      | https://play.rust-lang.org/
The Book (Rust docs) | https://doc.rust-lang.org/book/

