//This is comment
//There is no multi line comment in rust
use std::io;
fn add(x: i32, y: i32)->i32{
    x+y
}
fn main(){
    let mut x=String::new();
    let mut y=String::new();
    println!("Enter the value of x");
    io::stdin().read_line(&mut x).expect("Failed to read the line");
    println!("Enter the value of y");
    io::stdin().read_line(&mut y).expect("Failed to read the line");
    let x: i32= x.trim().parse().expect("Can't parse into i32");
    let y: i32= y.trim().parse().expect("Can't parse into i32");
    let sum=add(x,y);
    println!("The sum of x and y is {sum}");
}