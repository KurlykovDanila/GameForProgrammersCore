#[derive(Debug, Default)]
pub struct Statistics {
    wins: usize,
    losses: usize,
    mathes: usize,
    min_step_for_win: usize,
}

impl Statistics {
    pub fn new() -> Statistics {
        Statistics::default()
    }
    pub fn add_wins(&mut self) {
        self.wins += 1;
        self.add_matches()
    }
    pub fn get_wins(&self) -> usize {
        return self.wins;
    }
    pub fn add_losses(&mut self) {
        self.losses += 1;
        self.add_matches()
    }
    pub fn get_losses(&self) -> usize {
        return self.losses;
    }
    fn add_matches(&mut self) {
        self.mathes += 1
    }
    pub fn get_min_steps(&self) -> usize {
        return self.min_step_for_win;
    }
    pub fn add_steps(&mut self, steps: usize) {
        if self.min_step_for_win < steps {
            self.min_step_for_win = steps;
        }
    }
}
