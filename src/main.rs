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

    let result: bool = attack_territory.attack(&mut defense_territory, loss_threshold);

    if result {
        println!("Ataque foi um sucesso");
    } else {
        println!("Ataque foi um fracasso");
    }
    println!("Tropas restantes no ataque = {}", attack_territory.troops);
    println!("Tropas restantes na defesa = {}", defense_territory.troops);
}
