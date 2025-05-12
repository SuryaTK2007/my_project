use std::io;
fn main(){
    let arr=[3; 5];
    let mut index=String::new();
    io::stdin().read_line(&mut index).expect("Can't read the line");
    let index: usize = index.trim().parse().expect("Can't convert it to type u32");
    println!("The value of element at the {index} is {}",arr[index]);
}