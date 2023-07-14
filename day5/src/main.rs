fn main() {
    if_expression(3);
    if_else_if(8);
}
fn if_expression(x: i32) {
    if x < 5 {
        // you can't use if x  like javascript or php it will return type mismatch error if
        // expression must be bool type
        println!("{x} is smaller than 5");
    } else {
        println!("{x} is larger or equal to 5");
    }
    if x != 0 {
        //we can use this expression to achive javascript if(x) check
        println!("{x} not equal to zero");
    }
}
fn if_else_if(x:i32){
    if x % 4 == 0 {
        println!("{x} is divisible by 4");
    } else if x % 3 == 0 {
        println!("{x} is divisible by 3");
    }else if x % 2 == 0 {
        println!("{x} is divisible by 2");
    }else {
        println!("{x} is not divisible by 4,3,2");
    }
}

