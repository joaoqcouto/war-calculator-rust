use std::io;
use crate::war::Territory;
mod war;

fn individual_simulation() {
    println!("== SIMULAÇÃO PONTUAL ==");
    println!("Simule ataques para uma determinada quantidade de tropas atacantes e defensoras");
    let mut input_line = String::new();

    // getting attacking troops
    println!("Quantidade de tropas no território atacante: ");
    io::stdin()
        .read_line(&mut input_line)
        .expect("Erro ao ler a linha");
    let atk_troops: i32 = input_line.trim().parse().expect("Erro: entrada não é número inteiro");
    input_line.clear();

    // getting defending troops
    println!("\nQuantidade de tropas no território defensor: ");
    io::stdin()
        .read_line(&mut input_line)
        .expect("Erro ao ler a linha");
    let def_troops: i32 = input_line.trim().parse().expect("Erro: entrada não é número inteiro");
    input_line.clear();

    // getting attack stop threshold
    println!("\nDesistir do ataque com quantas tropas restantes: ");
    io::stdin()
        .read_line(&mut input_line)
        .expect("Erro ao ler a linha");
    let loss_threshold: i32 = input_line.trim().parse().expect("Erro: entrada não é número inteiro");
    input_line.clear();

    let mut attack_territory: Territory = Territory::new(atk_troops);
    let mut defense_territory: Territory = Territory::new(def_troops);

    // getting number of simulations
    println!("\nNúmero de rodadas de simulação: ");
    io::stdin()
        .read_line(&mut input_line)
        .expect("Erro ao ler a linha");
    let nsimulations: i32 = input_line.trim().parse().expect("Erro: entrada não é número inteiro");
    input_line.clear();

    println!("Simulando ataques...");
    let (win_rate, avg_friendlies_left_success, avg_enemies_left_fail) = attack_territory.simulate_attacks(&mut defense_territory, loss_threshold, nsimulations);

    println!("\nRESULTADOS");
    println!("Porcentagem de vitórias = {number:.2}%",number=win_rate*100.0);
    println!("Média de aliados restantes em vitórias = {number:.2}",number=avg_friendlies_left_success);
    println!("Média de inimigos restantes em derrotas = {number:.2}",number=avg_enemies_left_fail);
}

fn matrix_generator() {
    println!("== GERAÇÃO DE TABELA ==");
    println!("Gere uma tabela de probabilidades de sucesso de determinado tamanho");
    let mut input_line = String::new();

    // getting table size
    println!("Tamanho da tabela: ");
    io::stdin()
        .read_line(&mut input_line)
        .expect("Erro ao ler a linha");
    let table_size: i32 = input_line.trim().parse().expect("Erro: entrada não é número inteiro");
    input_line.clear();

    // getting number of simulations
    println!("\nNúmero de rodadas de simulação (por item da tabela): ");
    io::stdin()
        .read_line(&mut input_line)
        .expect("Erro ao ler a linha");
    let nsimulations: i32 = input_line.trim().parse().expect("Erro: entrada não é número inteiro");
    input_line.clear();

    println!("Total de rodadas de simulação = {}",nsimulations*table_size*table_size);
    println!("Gerando tabela...");
    let matrix: Vec<Vec<f32>> = Territory::gen_matrix(table_size as usize, nsimulations);

    // printing table heading
    print!("\n      ");
    for atk in 0..matrix.len() {
        print!("ATK={number:>2}    ",number=atk+1);
    }
    print!("\n");

    for def in 0..matrix.len() {
        print!("DEF={number:>2}",number=def+1);
        for val in matrix[def].iter() {
            print!("{number:>6}    ",number=val);
        }
        print!("\n");
    }
}

fn main() {
    println!("Calculador de ataques de War");
    println!("Simule ataques de War e avalie as estatísticas de sucesso\n");
    let mut input_line = String::new();

    loop {
        println!("Modos de simulação:");
        println!("  0 = exit");
        println!("  1 = Simulação pontual");
        println!("  2 = Tabela de probabilidades");
        println!("\nEscolha um modo:");

        io::stdin()
            .read_line(&mut input_line)
            .expect("Erro ao ler a linha");
        let mode: i32 = input_line.trim().parse().expect("Erro: entrada não é número inteiro");
        input_line.clear();

        match mode{
            0=>break,
            1=>individual_simulation(),
            2=>matrix_generator(),
            _=>println!("WIP")
        }
        println!("\n== == == ==\n");
    }
}
