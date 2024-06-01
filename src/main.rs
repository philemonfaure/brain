mod core;

fn main() {
    let mut brain = core::brain::Brain::new(8, 4, 1);
    loop
    {
        brain.compute();
        /*for neuron in &brain.neurons
        {
            println!("{}", neuron.0);
        }*/
        println!("{}", brain.neurons[1].0);
    }
}
