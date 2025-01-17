# [🦀] Concept of Mutabality and Immutability
```rust
let immutable = 2;
let mut mutable = 2;
```
- Now what does immutable and mutable have different?
  - immutable means that the value cannot be modified in any way
  - the mut keyword changes that behaviour and makes the value mutable (modifiable)

- Now lets see what happens when assinging a immutable variable a differnt value
```rust
let immutable = 2;
immutable = 3;
```
- [❌] This will error when compiling since the value immutable
  (^^^ cannot assign twice to immutable variable)


- Now lets see what happens when assinging a mutable variable a differnt value
```rust
let mut mutable = 2;
mutable = 3;
```

- [✅] This will compile without any errors
------------------------------------------------
Rust variables are immutable by default to ensure safety when writing your rust programm
