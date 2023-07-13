# Rust Variables and Mutability

In Rust, variables are immutable by default. This means that once a value is assigned to a variable, it cannot be changed. However, Rust also provides the `mut` keyword to declare mutable variables.

Here is an example of declaring an immutable variable:

```rust
let x = 5;
```

And here is an example of declaring a mutable variable:

```rust
let mut y = 10;
```

With mutable variables, you can change the value assigned to them:

```rust
y = 15;
```

# Rust Constants

In addition to variables, Rust also has constants. Constants are similar to immutable variables, but they have a few differences. Constants are always immutable and must be annotated with a type. They can be declared in any scope, including the global scope.

Here is an example of declaring a constant:

```rust
const MAX_POINTS: u32 = 100;
```

Constants are useful when you have a value that should never change throughout the execution of your program.

# Rust Shadowing

Shadowing is the process of declaring a new variable with the same name as an existing variable. This allows you to reuse the variable name while changing its type or value.

Here is an example of shadowing a variable:

```rust
let x = 5;
let x = x + 1;
```

In this example, the second `x` shadows the first `x`, allowing us to change its value. Shadowing is different from mutability because it creates a new variable with the same name, while mutability allows you to change the value of the existing variable.

Shadowing can be useful when you want to transform a variable into a different type or when you want to reuse a variable name for a different purpose.
