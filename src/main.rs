fn main(){
    let s1=String::from("Surya");
    let length=change(&s1);
    println!("The length of {s1} is {length}");
}
fn change(s: &String)->usize{
    s.len()
}