
fn pular_primeiro_byte(s: &String) -> &str{
    &s[1..]

}
fn main(){
    // fatiando string
    let s = String::from("Olá");
    let a = pular_primeiro_byte(&s);
    
    println!("a = {a:?}");
}