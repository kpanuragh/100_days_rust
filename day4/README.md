# Rust Functions

## Introduction
In Rust, functions are used to group together a set of statements that perform a specific task. They allow code reusability and help in organizing the codebase. Functions in Rust are defined using the `fn` keyword followed by the function name, parameters, return type, and the function body.

## Syntax
```rust
fn function_name(parameter1: Type1, parameter2: Type2) -> ReturnType {
    // Function body
    // Statements
    // Optional return statement
}
```

## Parameters
- Parameters are the inputs to a function.
- They are defined inside the parentheses after the function name.
- Multiple parameters are separated by commas.
- Each parameter has a name followed by a colon and the parameter type.

## Return Type
- The return type specifies the type of value that the function will return.
- It is specified after the parameter list using the `->` arrow syntax.
- If a function does not return a value, the return type is specified as `()`, which is the unit type.

## Function Body
- The function body contains the statements that are executed when the function is called.
- It is enclosed in curly braces `{}`.
- Statements are executed in the order they appear in the function body.

## Calling a Function
- To call a function, use the function name followed by parentheses.
- If the function has parameters, provide the values for the parameters inside the parentheses.

## Example
```rust
fn add_numbers(a: i32, b: i32) -> i32 {
    let sum = a + b;
    sum // Implicit return statement
}

fn main() {
    let result = add_numbers(5, 10);
    println!("Sum: {}", result);
}
```

## Common Errors and Mistakes
- Forgetting to specify the return type of the function.
- Mismatched types between the return type and the actual value being returned.
- Not providing values for all the function parameters when calling the function.
- Using a variable before it has been initialized.
- Forgetting to include the `fn` keyword when defining a function.
- Missing or extra parentheses when calling a function.
- Not using the correct syntax for specifying the parameter types.
- Not using the correct syntax for specifying the return type.
- Not using the correct syntax for specifying the function body.

## Conclusion
Functions are an essential part of any programming language, including Rust. They allow code reusability and help in organizing the codebase. Understanding how to define and use functions correctly is crucial for writing efficient and maintainable Rust code.
