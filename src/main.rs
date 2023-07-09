use crate::example_variable::ExampleVariable;
use crate::genetic_algorithm::GeneticAlgorithm;

mod example_variable;
mod genetic_algorithm;

fn main() {
    // let example = ExampleVariable::new();
    // example.display();
    // println!("Sum: {}", example.sum());
    // println!("Fitness: {}", example.fitness);

    let mut gen = GeneticAlgorithm::new(100, 0.05);
    for i in 0..50 {
        gen.select_next_generation();
    }
}
