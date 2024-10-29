fn main(){
    // fatiando string
    let s = String::from("Olá");
    let a = &s[3..];
    
    println!("a = {a:?}");
}