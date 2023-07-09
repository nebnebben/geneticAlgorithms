use crate::example_variable::ExampleVariable;
use crate::example_variable::NUM_GENES;
use rand::Rng;
use rand::prelude::*;
use rand::distributions::WeightedIndex;
const POP_SIZE: usize = 100;

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

    pub fn select_next_generation(&mut self) {
        self.sort_by_fitness();
        println!("Best fitness is: {} with {:?} genes", self.pop[0].fitness, self.pop[0].genes);

        let new_pairs: Vec<(ExampleVariable, ExampleVariable)> = self.pick_random_individuals_by_fitness();
        // Allow top 5 best to remain
        for i in 5..POP_SIZE {
            let parent1 = &new_pairs.get(i).unwrap().0;
            let parent2 = &new_pairs.get(i).unwrap().1;
            self.pop[i].genes = self.crossover(parent1, parent2);
            self.pop[i].genes = self.mutate_genes(self.pop[i].genes);
            self.pop[i].update_fitness();
        }

    }

    pub fn pick_random_individuals_by_fitness(&mut self) -> Vec<(ExampleVariable, ExampleVariable)> {
        let total_fitness: f64 = self.pop.iter().map(|x| x.fitness).sum();
        let relative_fitness: [f64; POP_SIZE as usize] = self.pop.iter()
            .map(|x| x.fitness/total_fitness)
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap();

        let mut rng = rand::thread_rng();
        let mut new_pop: Vec<(ExampleVariable, ExampleVariable)> = vec![];

        let mut dist = WeightedIndex::new(&relative_fitness).unwrap();
        for i in 0..POP_SIZE*2 {
            let random_index1 = dist.sample(&mut rng);
            let random_index2 = dist.sample(&mut rng);
            new_pop.push((self.pop[random_index1].clone(), self.pop[random_index2].clone()));
        }
        new_pop
    }

    pub fn sort_by_fitness(&mut self) {
        // Sort in reverse order
        self.pop.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    }

    pub fn mutate_pop(&mut self) {
        for i in 0..POP_SIZE as usize {
            self.pop[i].genes = self.mutate_genes(self.pop[i].genes);
        }
    }

    pub fn mutate_genes(&self, genes: [f64; NUM_GENES]) -> [f64; NUM_GENES] {
        let alpha: f64 = 0.1;
        let mut rng = rand::thread_rng();
        let mut new_genes : [f64; NUM_GENES] = [0.0; NUM_GENES];
        for i in 0..genes.len() {
            if rng.gen::<f64>() > self.mutation_rate {
                new_genes[i] = genes[i];
                continue
            }
            let gene = genes[i];
            let min_val: f64 = (gene - (gene*alpha)).max(0.0);
            let max_val: f64 = (gene + (gene*alpha)).min(1.0);
            let new_gene = rng.gen_range(min_val..max_val);
            new_genes[i] = new_gene;
        }
        new_genes
    }

    pub fn crossover(&self, ind1: &ExampleVariable, ind2: &ExampleVariable) ->  [f64; NUM_GENES] {
        let mut new_genes : [f64; NUM_GENES] = [0.0; NUM_GENES];
        let mut rng = rand::thread_rng();
        let weighted_chance = (ind1.fitness)/(ind1.fitness+ind2.fitness);
        for i in 0..ind1.genes.len() {
            if rng.gen::<f64>() < weighted_chance {
                new_genes[i] = ind1.genes[i];
            } else {
                new_genes[i] = ind2.genes[i];
            }
        }
        new_genes
    }

    pub fn crossover_blx(&self, ind1: &ExampleVariable, ind2: &ExampleVariable) ->  [f64; NUM_GENES] {
        let alpha: f64 = 0.5;
        let mut new_genes : [f64; NUM_GENES] = [0.0; NUM_GENES];
        let mut rng = rand::thread_rng();
        for i in 0..ind1.genes.len() {
            let gene1 = ind1.genes[i];
            let gene2 = ind2.genes[i];
            let diff = (gene1 - gene2).abs();
            let min_val = (gene1.min(gene2) - (diff*alpha)).max(0.0);
            let max_val = (gene1.max(gene2) + (diff*alpha)).min(1.0);
            let new_gene = rng.gen_range(min_val..max_val);
            new_genes[i] = new_gene;
        }
        new_genes
    }


}