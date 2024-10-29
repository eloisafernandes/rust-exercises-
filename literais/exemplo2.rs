fn main(){
    // let a = &mut [1, 2, 3]; não funciona
    let a = &[1, 2, 3];
    let b = a;

    println!("Olá mundo {a:?} {b:?}");
}