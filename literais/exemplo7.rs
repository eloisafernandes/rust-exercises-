
fn pular_primeiro_byte(s: &str) -> &str{
    &s[1..]

}
fn main(){
    // mudando otipo do argumento recebido Deref converte o tipo para str (string fatiada)
    let s = String::from("Olá");
    let a = pular_primeiro_byte(&s);
    
    println!("a = {a:?}");
}