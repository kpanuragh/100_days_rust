# Understanding Ownership
## The Stack and the Heap
![HEADSTACK](https://os.phil-opp.com/heap-allocation/call-stack.svg)  

## Ownership Rules
- Each value in Rust has an owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped
## Variable Scope
```rust
{ //a is not available here . its not declared
    let a = "hello"; //a is avaialbe from here 
} //a scope is over here 
```
Its similar to all other language

Normal  `Data Type` have no problem with ownership  due to it has fixed size limit so it can easily push and pop from stack memory. But in case of `String` like data type it's bit complecated we can use `String` as an example

we can use `str` for most cases but still it's not suitable for all application like if we need to get string from users input that case we need to use `String` type we can also create String from string literal
```rust
let s_string = String::from("hello world");
```
This kind of string can be mutate like
```rust
 let mut s = String::from("hello");
s.push_str(", world!"); // push_str() appends a literal to a String
println!("{}", s); // This will print `hello, world!`
```
we know that `str` string literal are imuatable so it's very effective for programming its memory is always hardcoded in prgramme binary. But for string it's littile bit difficult the `String.form` function request the memory from heap and it's not efficient when we use heap memeory. most of the garbage collecter(gc) language the gc will check if the memory need to be freed or not without gc we need to manually allocate and free the memory. In  case of rust it has special function drop that automatically call when the variable  is out of scope

```rust
let x = 5;
let y = x;
```
Here both x and y is integer and both are fixed size both are puhed to stack and both are diffrent memeory

```rust
let s1 = String::from("hello");
let s2 = s1;
```
But in this the s1 pointer and length and capcity are stored in stack all other values are stored in heap pointer point to the memory location of heap. when we are copy s1 and s2 it will actully copy s1 stack to s2 stack memory but heap is same 
![STRING](https://doc.rust-lang.org/book/img/trpl04-02.svg)  
But here the problem like if s2 or s1 is out of scope rust call drop function and it will also clear both of the memory it will case `double free ` error 
```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```
When you compile the programme we get an error Rust prevents you from using the invalidated reference.

Rust not automatically clone s1 to s2 we can use `s1.clone()` to make a deep copy of s1
