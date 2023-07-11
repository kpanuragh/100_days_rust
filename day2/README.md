# 100 Days of Rust Challenge - Day 2

## Checklist

- [x] Learn about variables and data types in Rust
     - [x] Rust is a statically typed language
        - [x] A scalar type represents a single value
            - [x] Integer(i8,u8,i16,u16etc)
            - [x] Floating-Point Types f32 -> single precision float f64 double precision float
            - [x] Boolean (bool) Booleans are one byte in size
            - [x] Character Type (char) literals with single quotes 4bytes support unicde char emoji ,Chinese etc
        - [x] Compound Types -  Group of multiple value 
            - [x] The Tuple Type
                ```rust: 
                let tup: (i32, f64, u8) = (500, 6.4, 1);
                let (x, y, z) = tup;
                let x: (i32, f64, u8) = (500, 6.4, 1);
                let five_hundred = x.0;
             - [x] Array Tyep
                 ```rust:
                 let a = [1, 2, 3, 4, 5];
                 let months = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];
                 let a: [i32; 5] = [1, 2, 3, 4, 5];
                 let a = [3; 5]; //same value  like [3, 3, 3, 3, 3]
                 let first = a[0];
                 let second = a[1];
