fn main(){
    // fatiando string
    let s = String::from("Olá");
    let a = &s[1..];

    println!("a = {a:?}");
}