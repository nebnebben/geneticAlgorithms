use rand::Rng;
pub(crate) const NUM_GENES: u32 = 4;

#[derive(Clone, Copy)]
pub struct ExampleVariable {
    pub genes: [f64; 4],
    pub fitness: f64
}

impl ExampleVariable {
    pub fn new() -> ExampleVariable {
        let array: [f64; 4] = Self::generate_new_genes();
        let fitness = Self::calculate_fitness(&array);
        ExampleVariable{genes: array, fitness}
    }

    pub fn sum(&self) -> f64 {
        self.genes.iter().sum()
    }

    pub fn display(&self) {
        println!("Genes: {:?}", self.genes);
    }

    fn generate_new_genes() -> [f64; 4] {
        let mut rng = rand::thread_rng();
        let array: [f64; 4] = [
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
        ];
        array
    }

    fn calculate_fitness(genes: &[f64; 4]) -> f64 {
        genes[0]
            + (genes[1] - 0.5).abs()
            + genes[2].powi(3)
            - genes[0] * genes[2]
            + genes[1] * genes[3]
    }

    pub fn update_fitness(&mut self) {
        self.fitness = Self::calculate_fitness(&self.genes);
    }
}
