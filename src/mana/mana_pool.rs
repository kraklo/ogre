pub struct ManaPool {
    white: u64,
    blue: u64,
    black: u64,
    red: u64,
    green: u64,
    colorless: u64,
}

impl ManaPool {
    pub fn new() -> Self {
        Self {
            white: 0,
            blue: 0,
            black: 0,
            red: 0,
            green: 0,
            colorless: 0,
        }
    }

    pub fn clear(&mut self) {
        self.white = 0;
        self.blue = 0;
        self.black = 0;
        self.red = 0;
        self.green = 0;
        self.colorless = 0;
    }
}
