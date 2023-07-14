# Rust Control Flow

## if Expressions

```rust
let number = 7;

if number < 5 {
    println!("Number is less than 5");
} else if number == 5 {
    println!("Number is equal to 5");
} else {
    println!("Number is greater than 5");
}
```

## Repetition with Loops

### `loop` keyword

```rust
let mut counter = 0;

loop {
    println!("Counter: {}", counter);
    counter += 1;

    if counter == 5 {
        break;
    }
}
```

### `while` loop

```rust
let mut counter = 0;

while counter < 5 {
    println!("Counter: {}", counter);
    counter += 1;
}
```

### `for` loop

```rust
let numbers = [1, 2, 3, 4, 5];

for number in numbers.iter() {
    println!("Number: {}", number);
}
```

## Returning Values from Loops

```rust
let mut counter = 0;

let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
};

println!("Result: {}", result);
```

## Loop Labels to Disambiguate Between Multiple Loops

```rust
'outer: for x in 0..5 {
    'inner: for y in 0..5 {
        if x == 2 && y == 2 {
            break 'outer;
        }
        println!("x: {}, y: {}", x, y);
    }
}
```

## Different Types of Loops

### `loop` loop

```rust
loop {
    // Code here
}
```

### `while` loop

```rust
while condition {
    // Code here
}
```

### `for` loop

```rust
for item in iterable {
    // Code here
}
```
