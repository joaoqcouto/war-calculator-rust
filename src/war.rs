use rand::Rng;

// dice struct: wrapper for array of rolled values
struct Dice {
    vals: [i32;3],
}
impl Dice {
    // constructor (3 dice in the array)
    fn new() -> Dice {
        let dice_array:[i32; 3] = [0;3];
        return Dice{vals: dice_array};
    }

    /*
        dice roll method:
        - n (how many dice to roll)

        > rolls dice (capped at the amount of dice), stores in order, unrolled dice = 0
    */
    fn roll(&mut self, nrolls: i32) {
        // zero all dice
        for i in 0..3 { self.vals[i] = 0; }

        // rng gen
        let mut rng = rand::thread_rng();

        // rolls at max the amount of dice you have
        let amount_to_roll: usize = if 3 > nrolls.try_into().unwrap() { nrolls.try_into().unwrap() } else { 3 };
        if amount_to_roll <= 0 { return; };

        let rolls:[i32;3] = [rng.gen_range(0..6)+1;3];
        for i in 0..amount_to_roll {
            self.vals[i] = rolls[i];
        }
        self.vals.sort();
        self.vals.reverse();
    }

    /*
        dice compare method:
        - compared (dice to compare to)
        - amount_compared (how many dice to compare, max 3)

        > compares the rolled dice 'war-style' (largest vs largest, 2nd largest vs 2nd largest, etc.)
        > does not compare if either of the dice being compared is 0, a 0 dice is 'not in play'

        returns:
        (larger, equal, smaller) -> how many of self's dice were larger, equal or smaller than the compared dice
    */
    fn compare(&self, compared: &Dice, amount_compared: usize) -> (i32, i32, i32) {
        let mut larger:i32 = 0;
        let mut equal:i32 = 0;
        let mut smaller:i32 = 0;
        for i in 0..amount_compared {
            if self.vals[i] > compared.vals[i] { larger+=1; }
            else if self.vals[i] == compared.vals[i] { equal+=1; }
            else if self.vals[i] < compared.vals[i] { smaller+=1; }
        }

        return (larger, equal, smaller);
    }
}

// territory struct: keeps how many troops are in the territory, its dice array
// dice array length determines upper limit to how many dice territory has to roll (War uses 3)
pub struct Territory {
    pub troops: i32,
    dice: Dice
}
impl Territory {
    // constructor (how many troops)
    pub fn new(ntroops: i32) -> Territory {
        return Territory{ troops: ntroops, dice: Dice::new() };
    }

    // one attack iteration function
    fn single_attack(&mut self, attacked: &mut Territory) {
        // both sides roll dice (attacker runs one less than the amount of troops)
        self.dice.roll(self.troops - 1);
        attacked.dice.roll(attacked.troops);

        // only compare as many dices as the side which rolled the least dices
        // max value to compare = 3
        let amount_compared:i32 = if self.troops-1 > attacked.troops {
            if attacked.troops > 3 {3} else {attacked.troops}
        } else {
            if self.troops-1 > 3 {3} else {self.troops-1}
        };

        let (larger, equal, smaller) = self.dice.compare(&attacked.dice, amount_compared as usize);
        self.troops -= smaller+equal;
        attacked.troops -= larger;
    }

    /*
        single attack function:
        - attacked (attacked territory)
        - threshold (attack until how many troops are left)

        > mutates structures during attack
        > returns true if attack succeeds / false if otherwise
    */
    pub fn attack(&mut self, attacked: &mut Territory, threshold: i32) -> bool {
        while self.troops > threshold && self.troops > 1 && attacked.troops > 0 {
            // run attacks
            self.single_attack(attacked);
        }

        if attacked.troops > 0 {
            return false;
        }
        
        attacked.troops = 0;
        return true;
    }

    /*
        attack simulator function:
        - attacked (attacked territory)
        - threshold (attack until how many troops are left)
        - simulations (number of simulations made)

        > mutates structures during each round, but recovers original values
        > returns percentage of successful attacks
    */
    pub fn simulate_attacks(&mut self, attacked: &mut Territory, threshold: i32, simulations: i32) -> (f32, f32, f32) {
        let mut wins: i32 = 0;
        let mut friendlies_left_success: i32 = 0;
        let mut enemies_left_fail: i32 = 0;

        for _ in 0..simulations {
            let atk_troops: i32 = self.troops;
            let def_troops: i32 = attacked.troops;

            // simulating attack
            let result: bool = self.attack(attacked, threshold);

            if result {
                wins += 1;
                friendlies_left_success += self.troops;
            } else {
                enemies_left_fail += attacked.troops;
            }

            // resetting troops
            self.troops = atk_troops;
            attacked.troops = def_troops;
        }
        return (
            (wins as f32)/(simulations as f32),
            (friendlies_left_success as f32)/(wins as f32),
            (enemies_left_fail as f32)/((simulations-wins) as f32)
        );
    }

    /*
        matrix generator function:
        - attacked (attacked territory)
        - size (size of generated matrix)
        - simulations (number of simulations made)

        > simulates for every combination of attacks and defenses (with no threshold)
        > returns matrix (row = increasing attackers; column = increasing defenders)
    */
    pub fn gen_matrix(size:usize, simulations: i32) -> Vec<Vec<f32>> {
        // end matrix
        let mut matrix:Vec<Vec<f32>> = Vec::with_capacity(size);
        let mut attack_territory: Territory = Territory::new(0);
        let mut defense_territory: Territory = Territory::new(0);

        // loop of defenders (1 -> size defenders simulated)
        for i in 0..size {
            // starting rows with all zeroes
            let mut row:Vec<f32> = vec![0.0;size];
            defense_territory.troops = (i as i32)+1;

            // loop of attackers (1 -> size attackers simulated)
            for j in 1..size {
                attack_territory.troops = (j as i32)+1;
                let (win_rate, _, _) = attack_territory.simulate_attacks(&mut defense_territory, 1, simulations);
                row[j] = win_rate;
            }
            matrix.push(row);
        }

        return matrix;
    }

    /*
        fast matrix generator function:
        - attacked (attacked territory)
        - size (size of generated matrix)
        - simulations (number of simulations made)

        > simulates for every combination of attacks and defenses
        > uses matrix for memoing values for smaller simulations, speeding up process for large simulation values
        > returns matrix (row = increasing attackers; column = increasing defenders)
    */
    pub fn gen_matrix_fast(size:usize, simulations: i32) -> Vec<Vec<f32>> {
        // end matrix
        let mut matrix:Vec<Vec<f32>> = Vec::with_capacity(size);
        let mut attack_territory: Territory = Territory::new(0);
        let mut defense_territory: Territory = Territory::new(0);

        // filling up matrix with zeroes
        for _ in 0..size {
            let row:Vec<f32> = vec![0.0;size];
            matrix.push(row);
        }

        // loop of defenders (1 -> size defenders simulated)
        for i in 0..size {
            // loop of attackers (1 -> size attackers simulated)
            for j in 1..size {

                // fast simulation loop (uses matrix as cache)
                let mut wins: f32 = 0.0;
                for _ in 0..simulations {
                    defense_territory.troops = (i as i32)+1;
                    attack_territory.troops = (j as i32)+1;
        
                    // simulating attack, getting memo values
                    attack_territory.single_attack(&mut defense_territory);
                    if defense_territory.troops == 0 { wins += 1.0; }
                    else if attack_territory.troops == 0 { wins += 0.0; }
                    else { wins += matrix[(defense_territory.troops-1) as usize][(attack_territory.troops-1) as usize]; }
                }

                matrix[i][j] = wins/(simulations as f32);
            }
        }

        return matrix;
    }
}