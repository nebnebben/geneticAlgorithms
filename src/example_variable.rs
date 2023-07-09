use rand::Rng;

pub struct ExampleVariable {
    pub genes: [f64; 4],
}

impl ExampleVariable {
    pub fn new() -> ExampleVariable {
        let mut rng = rand::thread_rng();
        let array: [f64; 4] = [
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
        ];
        ExampleVariable{genes: array}
    }

    pub fn sum(&self) -> f64 {
        self.genes.iter().sum()
    }

    pub fn display(&self) {
        println!("Random Array: {:?}", self.genes);
    }

    pub fn fitness(&self) -> f64 {
        self.genes[0]
            + (self.genes[1] - 0.5).abs()
            + self.genes[2].powi(3)
            - self.genes[0]*self.genes[2]
            + self.genes[1]*self.genes[3]

    }
}
