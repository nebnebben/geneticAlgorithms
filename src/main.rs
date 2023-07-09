use crate::example_variable::ExampleVariable;

mod example_variable;

fn main() {
    let example = ExampleVariable::new();
    example.display();
    println!("Sum: {}", example.sum());
    println!("Fitness: {}", example.fitness());
}
