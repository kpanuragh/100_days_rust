fn main() {
    let y = 6; // is a statment
               // so you can not assign let x = (let y = 6); like this bcz it not return any
               // value its simple statment
    println!("Hello, world!"); //Its not a function it's a macro
    sample_function(); //Function call
    another_function(y); //Function call with parameter
    sample_expression();
    let x = return_furntion(y); //Function with return type
    println!("Value Return from return_furntion : {x}");
}
fn sample_function() {
    //Function define with keyword fn NOTE :  main is also a function its always
    //best practice to use snakeCase for function name
    println!("Print from another function");
}
fn another_function(x: i32) {
    //function definition with parameter
    println!("The value of x is : {x}");
}
fn sample_expression() {
    let y = {
        //This is a expression it will assign value 3 to x and x+1 is return that means y
        //will store value 4
        let x = 3;
        x + 1
    };
    println!("This value of y is:  {y}");
}
fn return_furntion(x: i32) -> i32 {
    x + 5   //Never add semicolon end of the expression . it will convert the expression to a
            //stementment and it will return error
}
