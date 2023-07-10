// Function to check if a number is palindrome or not
fn is_palindrome(num: u32) -> bool {
    let mut reversed_num = 0;
    let mut temp = num;

    while temp != 0 {
        let remainder = temp % 10;
        reversed_num = reversed_num * 10 + remainder;
        temp /= 10;
    }

    num == reversed_num
}

fn main() {
    let num = 12321;

    if is_palindrome(num) {
        println!("{} is a palindrome", num);
    } else {
        println!("{} is not a palindrome", num);
    }
}
