use crate::DeepThought;

#[cxx::bridge(namespace = "ffi")]
pub mod ffi {
    // Opaque C++ type
    #[namespace = "magrathea"]
    unsafe extern "C++" {
        include!("deep_thought/IBootLoader.h");
        type IBootLoader;
        fn run(&self) -> bool;
    }

    // Opaque Rust type
    extern "Rust" {
        #[cxx_name = "RsDeepThought"]
        type DeepThought;
        fn ask(&self, question: &str) -> String;

        // Opaque types can only be passed across the FFI with an indirection (reference, Box)
        // thus we need a dedicated create function
        fn create_deep_thought(
            boot_loader: &SharedPtr<IBootLoader>,
            config: Config,
        ) -> Box<DeepThought>;
    }

    // Shared type
    pub struct Config {
        clock_rate: u64,
        version: String,
    }
}

fn create_deep_thought(
    boot_loader: &cxx::SharedPtr<ffi::IBootLoader>,
    config: ffi::Config,
) -> Box<DeepThought> {
    let result = boot_loader.run();
    println!("RS: Bootloader exited with {}", result);

    Box::new(DeepThought::with_config(config))
}
