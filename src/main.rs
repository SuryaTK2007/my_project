use std::io;
fn main(){
    let mut buf=String::new();
    println!("Enter the size of array: ");
    io::stdin().read_line(&mut buf).expect("Error in reading input");
    let n1_result=buf.trim().parse::<i32>();
    let n1=match n1_result{
        Ok(val)=>val,
        Err(_)=>{
            println!("Invalid input for n1");
            return;
        }
    };
    buf.clear();
    let mut v=Vec::new();
    println!("Enter the elemets of array: ");
    for i in 0..n1{
        io::stdin().read_line(&mut buf).expect("Error in reading input");
        let x_result=buf.trim().parse::<i32>();
        let x=match x_result{
            Ok(val)=>val,
            Err(_)=>{
                println!("Invalid input for array element");
                return;
            }
        };
        v.push(x);
        buf.clear();
    }
    for i in 0..v.len(){
        print!("{} ",v[i]);
    }
}