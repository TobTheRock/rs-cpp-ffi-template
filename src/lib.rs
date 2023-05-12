mod bindings;

pub struct DeepThought {}

impl DeepThought {
    pub fn new() -> Self {
        Self {}
    }

    pub fn ask(&self, question: &str) -> String {
        println!("RS: Asking deep thought: {}", question);
        // TODO config for this
        let time_to_think_parameter: u64 = question.len().try_into().unwrap_or(0);

        println!("RS: DeepThought™ will need to think ....");
        for i in 0..time_to_think_parameter {
            let mut x: u64 = 0;
            for j in 0..1_000_000 {
                x += i * j / 42;
            }

            println!("... {:#b}", x);
        }

        println!("RS: DeepThought™ has awoken!");

        return "42".into();
    }
}
