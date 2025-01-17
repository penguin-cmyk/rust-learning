# [🦀] Data Types in rust 
-------------------------------------
- Scalar types
  - Scalar types represent a single value 
  - Rust has 4 primary scalar types
    - integers
    - floating-point numbers
    - booleans
    - characters
-------------------------------------      
-> Integer types in rust
|  Length | Signed | Unsigned |    
| ------- | -------| -------- |    
| 8-bit   |  i8    | u8       |   
| 16-bit  |  i16   | u16      |     
| 32-bit  |  i32   | u32      |  
| 64-bit  |  i64   | u64      |  
| 128-bit |  i128  | u128     |  
| arch    |  isize | usize    |  

- Signed and unsigned just refers to wether the number can be negative
  - signed: number can be negative (*aka have a sign*)
  - unsigned: number is expected to only be positive (*aka represented without a sign*)
- isize and usize is dependend on the architecture your program is running on
  - isize and usize in generally used when indexing some sort of collection
-------------------------------------
-> Integer literals
| Number literals  |    Example   | 
| ---------------- | ------------ | 
| Decimal          | 98_222       |
| Hex              | 0xff         |     
| Octal            | 0o77         |
| Binary           | 0b1111_0000  |
| Byte (u8 only)   | b'A'         | 

- *b represents the string as a byte so e.g b'Hello world'*
------------------------------------- 
-> floating-point numbers 
- Floating point numbers are numbers with decimal points
  - ```rust
    let x = 2.0; // f64
    let y: f32 = 3.0 // f32 ```
