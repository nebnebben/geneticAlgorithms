use crate::example_variable::ExampleVariable;
use crate::genetic_algorithm::GeneticAlgorithm;

mod example_variable;
mod genetic_algorithm;

fn main() {
    let example = ExampleVariable::new();
    example.display();
    println!("Sum: {}", example.sum());
    println!("Fitness: {}", example.fitness);

    let gen = GeneticAlgorithm::new(100, 0.1);
    gen.display_pop();
}
