use std::io;
fn main(){
    let mut buf=String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut n1:i32=buf.trim().parse().unwrap();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let mut n2:i32=buf.trim().parse().unwrap();
    println!("The sum of {} and {} is {}", n1,n2,n1+n2);
}