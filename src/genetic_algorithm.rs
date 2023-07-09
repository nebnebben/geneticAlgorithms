use crate::example_variable::ExampleVariable;
use rand::Rng;
const POP_SIZE: u32 = 2;

pub struct GeneticAlgorithm {
    pop: [ExampleVariable; POP_SIZE as usize],
    generations: u32,
    mutation_rate: f64
}

impl GeneticAlgorithm {
    pub fn new(generations:u32, mutation_rate:f64) -> GeneticAlgorithm {
        let mut pop: [ExampleVariable; POP_SIZE as usize] = [ExampleVariable::new(); POP_SIZE as usize];
        for i in 0..pop.len(){
            pop[i] = ExampleVariable::new();
        }
        GeneticAlgorithm{pop, generations, mutation_rate}
    }

    pub fn display_pop(&self) {
        self.pop.iter().for_each(|x| x.display());
        let mut fitnesses: Vec<f64> = self.pop.iter().map(|x| x.fitness).collect();
        fitnesses.sort_by(|a, b| a.partial_cmp(b).unwrap());
        fitnesses.iter().for_each(|x| println!("{}", x));
    }

    pub fn mutate_pop(&self) {
        self.pop.iter().for_each(|mut x| self.mutate_individual( x));
    }

    pub fn mutate_individual(&self, subject: &mut ExampleVariable) {
        let alpha: f64 = 0.1;
        let mut rng = rand::thread_rng();
        for i in 0..subject.genes.len() {
            if rng.gen::<f64>() < self.mutation_rate {
                continue
            }
            let gene = subject.genes[i];
            let min_val: f64 = (gene - (gene*alpha)).max(0.0);
            let max_val: f64 = (gene + (gene*alpha)).min(1.0);
            let new_gene = rng.gen_range(min_val..max_val);
            subject.genes[i] = new_gene;
        }
    }

}