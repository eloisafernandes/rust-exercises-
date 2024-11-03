use std::io;

// exibir o tabuleiro
fn mostrar_tabuleiro(tabuleiro: &[[char; 3]; 3]) {
    for linha in tabuleiro {
        println!(" {} | {} | {} ", linha[0], linha[1], linha[2]);
        println!("-----------");
    }
}

// converter o número em posição na matriz (linha e coluna)
fn converter_posicao(numero: u32) -> Option<(usize, usize)> {
    if numero >= 1 && numero <= 9 {
        let numero = numero - 1;
        Some(((numero / 3) as usize, (numero % 3) as usize))
    } else {
        None
    }
}

// realizar a jogada
fn realizar_jogada(tabuleiro: &mut [[char; 3]; 3], jogador: char, posicao: u32) -> bool {
    if let Some((linha, coluna)) = converter_posicao(posicao) {
        if tabuleiro[linha][coluna] == ' ' {
            tabuleiro[linha][coluna] = jogador;
            return true;
        } else {
            println!("Posição já ocupada! Tente novamente.");
        }
    } else {
        println!("Posição inválida! Escolha um número de 1 a 9.");
    }
    false
}

// verificar a vitória
fn verificar_vitoria(tabuleiro: &[[char; 3]; 3], jogador: char) -> bool {
    for i in 0..3 {
        if (tabuleiro[i][0] == jogador && tabuleiro[i][1] == jogador && tabuleiro[i][2] == jogador) || 
           (tabuleiro[0][i] == jogador && tabuleiro[1][i] == jogador && tabuleiro[2][i] == jogador) {
            return true;
        }
    }
    (tabuleiro[0][0] == jogador && tabuleiro[1][1] == jogador && tabuleiro[2][2] == jogador) ||
    (tabuleiro[0][2] == jogador && tabuleiro[1][1] == jogador && tabuleiro[2][0] == jogador)
}

// Função principal para o fluxo do jogo
fn jogo_da_velha() {
    let mut tabuleiro = [[' '; 3]; 3];
    let mut jogador_atual = 'X';

    loop {
        mostrar_tabuleiro(&tabuleiro);
        
        println!("Jogador {}, escolha uma posição (1-9): ", jogador_atual);
        
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("Erro ao ler entrada");
        
        let posicao: u32 = match entrada.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada inválida! Por favor, insira um número de 1 a 9.");
                continue;
            }
        };

        if realizar_jogada(&mut tabuleiro, jogador_atual, posicao) {
            if verificar_vitoria(&tabuleiro, jogador_atual) {
                mostrar_tabuleiro(&tabuleiro);
                println!("Parabéns, jogador {}! Você venceu!", jogador_atual);
                break;
            } else if tabuleiro.iter().all(|linha| linha.iter().all(|&celula| celula != ' ')) {
                mostrar_tabuleiro(&tabuleiro);
                println!("Empate! O tabuleiro está cheio.");
                break;
            }

            jogador_atual = if jogador_atual == 'X' { 'O' } else { 'X' };
        }
    }
}

// iniciar o jogo
fn main() {
    jogo_da_velha();
}