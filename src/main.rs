use std::io;
fn is_prime(n:i32)->bool{
    for i in 2..=((n as f64).sqrt() as i32){
        if n%i==0{
            return false;
        }
    }
    return true;
}
fn main(){
    let mut buf=String::new();
    io::stdin().read_line(&mut buf).expect("Can't read the input");
    let n1_result=buf.trim().parse::<i32>();
    let n1=match n1_result{
        Ok(val)=>val,
        Err(_)=>{
            println!("Invalid input for number");
            return;
        }
    };
    if n1==1{
        println!("Not a prime number");
        return;
    }
    let x:bool=is_prime(n1);
    if x{
        println!("The number is prime number!");
    }
    else{
        println!("The number is not a prime number!");
    }
}