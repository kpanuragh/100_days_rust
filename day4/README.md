## Rust Functions

### 1. Function Definition

```rust
fn function_name(parameter1: type1, parameter2: type2) {
    // function body
}
```

### 2. Function Parameters

- Parameters are variables that are used to pass values to a function.
- Parameters are defined inside the parentheses after the function name.
- Multiple parameters are separated by commas.

### 3. Function Statements and Expressions

- Statements are instructions that perform some action and do not return a value.
- Expressions evaluate to a value and can be used as part of a statement or as the return value of a function.

### 4. Function with Return Values

```rust
fn function_name(parameter1: type1, parameter2: type2) -> return_type {
    // function body
    return return_value;
}
```

- The `return_type` specifies the type of value that the function will return.
- The `return` keyword is used to return a value from the function.

### Common Errors and Mistakes

1. Forgetting to specify the return type in the function signature.
2. Forgetting to use the `return` keyword to return a value from the function.
3. Mismatched types between the return type and the actual value being returned.
4. Not providing values for all the required parameters when calling a function.
5. Using the wrong number or order of arguments when calling a function.
