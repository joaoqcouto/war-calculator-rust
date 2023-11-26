use std::io;

use crate::war::Territory;
mod war;

fn main() {
    println!("Calculador de ataques de War");
    println!("Simule uma grande quantidade de ataques de War e avalie as estatísticas de sucesso\n");

    // getting attacking troops
    println!("Quantidade de tropas no território atacante: ");
    let mut input_line = String::new();
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

    let result: f32 = attack_territory.simulate_attacks(&mut defense_territory, loss_threshold, nsimulations);

    println!("Porcentagem de sucessos = {number:.2}%",number=result*100.0);
}
