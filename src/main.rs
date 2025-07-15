use std::io;
fn main(){
    let mut buf=String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read input");
    let n1_result=buf.trim().parse::<i32>();
    let n1=match n1_result{
        Ok(val)=>val,
        Err(_)=>{
            println!("Failed to read first number");
            return;
        }
    };
    buf.clear();
    io::stdin().read_line(&mut buf).expect("Failed to read input");
    let n2_result=buf.trim().parse::<i32>();
    let n2=match n2_result{
        Ok(val)=>val,
        Err(_)=>{
            println!("Failed to read second number");
            return;
        }
    };
    println!("The sum of {} and {} is {}",n1,n2,n1+n2);
}