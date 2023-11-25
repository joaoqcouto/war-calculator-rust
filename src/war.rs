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
        for i in 0..self.vals.len() { self.vals[i] = 0; }

        // rng gen
        let mut rng = rand::thread_rng();

        // rolls at max the amount of dice you have
        let amount_to_roll: usize = if self.vals.len() > nrolls.try_into().unwrap() { nrolls.try_into().unwrap() } else { self.vals.len() };
        let rolls:[i32;3] = [rng.gen_range(0..6)+1;3];
        for i in 0..amount_to_roll {
            self.vals[i] = rolls[i];
        }
        self.vals.sort();
    }

    /*
        dice compare method:
        - compared (dice to compare to)

        > compares the rolled dice 'war-style' (largest vs largest, 2nd largest vs 2nd largest, etc.)
        > does not compare if either of the dice being compared is 0, a 0 dice is 'not in play'

        returns:
        (larger, equal, smaller) -> how many of self's dice were larger, equal or smaller than the compared dice
    */
    fn compare(&self, compared: &Dice) -> (i32, i32, i32) {
        let mut larger:i32 = 0;
        let mut equal:i32 = 0;
        let mut smaller:i32 = 0;

        let amount_compared = if self.vals.len() > compared.vals.len() { self.vals.len() } else { compared.vals.len() };
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

    /*
        single attack simulation function:
        - attacked (attacked territory)
        - threshold (attack until how many troops are left)

        > mutates structures during attack
        > returns True if attack succeeds / false if otherwise
    */
    pub fn attack(&mut self, attacked: &mut Territory, threshold: i32) -> bool {
        while self.troops > threshold && self.troops > 1 && attacked.troops > 0 {
            // both sides roll dice (attacker runs one less than the amount of troops)
            self.dice.roll(self.troops - 1);
            attacked.dice.roll(attacked.troops);

            let (larger, equal, smaller) = self.dice.compare(&attacked.dice);
            self.troops -= larger;
            attacked.troops -= smaller+equal;
        }

        if attacked.troops > 0 {
            return false;
        }
        
        attacked.troops = 0;
        return true;
    }
}