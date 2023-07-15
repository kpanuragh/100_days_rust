fn main() {
    let s = String::from("Hello"); //declared string 
    take_ownership(s);  //ownership transfered 
    //println!("{}",s); we can't use s here s is out of scope we can s.clone deep clone
    let x = 5; //declared x as int
    no_ownership_take(x); //passed to function but the x ownership not transfered
    println!("here the {x}"); //so we can use x here
}
fn take_ownership(my_string:String){
    println!("{}",my_string); 
} //my_string is freed here
fn no_ownership_take(x:i32){
    println!("here no ownership take {x}");
}
