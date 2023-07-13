fn main() {
    println!("Hello, world!"); //Its not a function it's a macro
    sample_function(); //Function call
}
fn sample_function(){ //Function define with keyword fn NOTE :  main is also a function its always
                      //best practice to use snakeCase for function name
    println!("Print from another function");
}
