use std::io;
fn main() {
    const MAX: usize = 100;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Error reading input");
    let n1_result = buf.trim().parse::<usize>();
    let n1 = match n1_result {
        Ok(val) if val <= MAX => val,
        _ => {
            println!("Invalid input for n1 (must be <= {MAX})");
            return;
        }
    };
    let mut arr = [0; MAX]; 
    buf.clear();
    for i in 0..n1 {
        io::stdin().read_line(&mut buf).expect("Error reading input");
        let x_result = buf.trim().parse::<i32>();
        arr[i] = match x_result {
            Ok(val) => val,
            Err(_) => {
                println!("Invalid input at position {}", i + 1);
                return;
            }
        };
        buf.clear();
    }
    for i in 0..n1 {
        print!("{} ", arr[i]);
    }
}
