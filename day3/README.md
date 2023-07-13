## Rust Variables and Mutability

### Variables

In Rust, variables are created using the `let` keyword. The general syntax for declaring a variable is:

```rust
let variable_name: data_type = value;
```

Here, `variable_name` is the name of the variable, `data_type` is the type of data that the variable can hold, and `value` is the initial value assigned to the variable.

### Mutability

By default, variables in Rust are immutable, meaning their values cannot be changed once assigned. However, you can make a variable mutable by using the `mut` keyword.

```rust
let mut variable_name: data_type = value;
```

With the `mut` keyword, you can modify the value of the variable later in the code.

### Examples

```rust
// Immutable variable
let x: i32 = 5;
println!("The value of x is: {}", x);

// Mutable variable
let mut y: i32 = 10;
println!("The value of y is: {}", y);

// Modifying the value of a mutable variable
y = 15;
println!("The new value of y is: {}", y);
```

Output:

```
The value of x is: 5
The value of y is: 10
The new value of y is: 15
```
