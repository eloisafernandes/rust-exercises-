fn maior_sequencia_crescente(a: &[i32]) -> &[i32] {
    if a.is_empty() {
        return &[];
    }
    
    let mut start = 0;
    let mut end = 0;
    let mut max_start = 0;
    let mut max_end = 0;
    
    for i in 1..a.len() {
        if a[i] > a[i - 1] {
            end = i;
            if (end - start) > (max_end - max_start) {
                max_start = start;
                max_end = end;
            }
        } else {
            start = i;
            end = i;
        }
    }

    &a[max_start..=max_end]
}

fn main() {
    let a = &[10, 20, 35, 70, 13, 30, 40, 50];
    let s = maior_sequencia_crescente(a);
    
    println!("Maior sequência crescente: {:?}", s);
}