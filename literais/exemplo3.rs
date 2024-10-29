fn main(){
    let a = [10, 20, 30, 40];

    let b = &a[1..];
    let c = &a[..];
    let d = &a[..1];
    let e = &a[..=1];

    println!("Olá mundo a={a:?} b={b:?} c={c:?} d={d:?} e={e:?}");
}