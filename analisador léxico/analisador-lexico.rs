use std::io::{self, Write};

fn próximo(entrada: &str) -> Result<(usize, &str, &str), Option<usize>> {
    let mut chars = entrada.char_indices().peekable();

    while let Some((start_idx, c)) = chars.next() {
        if c.is_whitespace() {
            continue; // Ignorar espaços em branco
        }

        // A posição correta é baseada na contagem de caracteres até o índice atual.
        let pos = entrada[0..start_idx].chars().count() + 1;

        if c.is_digit(10) {
            // Detectar números
            let mut end_idx = start_idx;
            while let Some(&(idx, next_c)) = chars.peek() {
                if next_c.is_digit(10) {
                    end_idx = idx;
                    chars.next(); // Consumir o próximo caractere
                } else {
                    break;
                }
            }
            let conteúdo = &entrada[start_idx..=end_idx];
            let restante = &entrada[end_idx + 1..];
            return Ok((pos, conteúdo, restante));
        } else if "+-*/".contains(c) {
            // Detectar operadores
            let conteúdo = &entrada[start_idx..=start_idx];
            let restante = &entrada[start_idx + 1..];
            return Ok((pos, conteúdo, restante));
        } else {
            // Caractere inválido
            return Err(Some(pos));
        }
    }

    // Nenhum elemento léxico encontrado
    Err(None)
}

fn main() {
    let mut entrada = String::new();

    loop {
        print!("Digite a entrada (ou 'sair' para encerrar): ");
        std::io::stdout().flush().unwrap();

        entrada.clear();
        std::io::stdin().read_line(&mut entrada).unwrap();
        let entrada = entrada.trim();

        if entrada.eq_ignore_ascii_case("sair") {
            break;
        }

        let mut restante = entrada;

        println!("Analisando: {}", entrada);
        loop {
            match próximo(restante) {
                Ok((pos, conteúdo, novo_restante)) => {
                    println!("Token encontrado: ({}, '{}')", pos, conteúdo);
                    restante = novo_restante;
                }
                Err(Some(pos)) => {
                    println!("Erro na posição: {}", pos);
                    break;
                }
                Err(None) => {
                    println!("Fim da análise");
                    break;
                }
            }
        }
        println!();
    }
}
