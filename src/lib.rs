mod bindings;

pub struct DeepThought {
    config: bindings::ffi::Config,
}

impl DeepThought {
    pub fn with_config(config: bindings::ffi::Config) -> Self {
        Self { config }
    }

    pub fn ask(&self, question: &str) -> String {
        println!(
            "RS: Asking DeepThought™ v{}: {}",
            self.config.version, question
        );
        let time_to_think: u64 =
            1_000 / self.config.clock_rate * question.len().try_into().unwrap_or(1);

        println!("RS: DeepThought™ will need to think ....");
        for i in 0..time_to_think {
            let mut x: u64 = question.chars().next().unwrap_or('a').into();
            for j in 0..1_000_000 {
                x += i * j / 42;
            }

            println!("... {:#b}", x);
        }

        println!("RS: DeepThought™ has awoken!");

        return "42".into();
    }
}

impl Default for DeepThought {
    fn default() -> Self {
        let config = bindings::ffi::Config {
            clock_rate: 50,
            version: "1.0.1".into(),
        };
        Self { config }
    }
}
