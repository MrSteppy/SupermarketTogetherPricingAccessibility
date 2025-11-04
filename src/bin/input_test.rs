use supermarket_together_pricing_accessibility::listener::listen;

fn main() {
  let input_receiver = listen().expect("listener");

  while let Ok(input) = input_receiver.recv() {
    println!("User pressed {:?}", input);
  }
}