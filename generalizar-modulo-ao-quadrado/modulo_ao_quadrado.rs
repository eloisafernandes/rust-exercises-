use std::ops::{Add, Mul};

struct Ponto<T, const DIM: usize>{
    v: [T; DIM],
}

fn modulo_ao_quadrado <T, const D: usize>(p: &Ponto<T, D>) -> T
    where
    T: Mul<Output = T> + Add<Output = T> + Copy + Default,
    {
        p.v.iter()
            .map(|&x| x * x) 
            .fold(T::default(), |acc, x| acc + x)
    }

fn main(){
    let p = Ponto { v: [15, 783, 9]};
    let resultado = modulo_ao_quadrado(&p);
    println!("{}", resultado)
}