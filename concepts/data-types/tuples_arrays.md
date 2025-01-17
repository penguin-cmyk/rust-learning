# [🦀] Tuples & Arrays in Rust
-----------------------------------
- A tuple is a general way of grouping a number of values together
  - tuples have a fixed length (*once defined they cannot shrink or grow*)
  - 
```rust
fn main() {
  let tuple: (i32,f64,u8) = (500,6.4,1); // optional type annotations
  let (x,y,z) = tuple;

  println!("The value of x is: {x}");
  /*
    Will print the same but access it from the tuple directly
    instead of storing it in a seperate variable

    As in most languages the index starts at 0 
  */ 
  println!("The value of x is: {tuple.0}"); 
}
```
-------------------------------------------
- In a array every type must have the same value
  - arrays are useful when you want your data locaed on the stack rather than the heap (*will discuss later what the heap & stack is*)
  - arrays aren't flexiable so thats why it's recommended to use vectors instead
  - vectors are flexiable 
```rust
let test_array = [1,2,3,4];
```  
