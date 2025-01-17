# [🦀] Shadowing concept in rust
------------------------------------------
- Shadowing is when you declare a variable with the same name as another variable
  - meaning that the first variable is *shadowed* (aka overlooked by the rust compiler)
```rust
fn main() {
  let x = 1;
  let x = x + 2;

  println!("{x}"); // will output 3 since the first x is shadowed 
}
```
------------------------------------------
- In scope example
  - scope is just what happends in the brackets *{}* 
```rust
fn main() {
  let x = 1;
  let x = x + 2;
  {
    let x = x + 1;
    println!("{x}"); // will output 4
  }
  println!("{x}"); // will still output 3 since the other x is in a inner scope and not the main scope 
}
```
------------------------------------------
- Shadowing useful examples
  - imagine the program asks how many spaces (in length) the user want
 ```rust
fn main() {
  let spaces = "    ";
  let spaces = spaces.len(); // spares us from having to come up with a different name
}
```
