fn main(){
    let s1=String::from("hello");
    let (s3,length)=calculate_len(s1);
    println!("{s3} {length}");
}
fn calculate_len(s: String)->(String,usize){
    let length=s.len();
    (s,length)
}