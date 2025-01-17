# [🦀] Constants in rust 
--------------------------------------------------------------------------------------------------------------------------------- 
- (I know this doesn't really fit into the variable category like said in the book but its better placed in here for readability of the project)
---------------------------------------------------------------------------------------------------------------------------------
- First you **aren't** allowed to use *mut* with constants
  - constants are always immutable
  - u declare them by using the const keyword
  - you must define the type of the value said (in this example *String*)
  - constants are valid for the entire time the program runs
  - *This makes constants useful for values that you will repeatedly need in your program but dont need to modify*
```rust
const example: String = String::from("Hello world");
```
---------------------------------------------------------------------------------------------------------------------------------
