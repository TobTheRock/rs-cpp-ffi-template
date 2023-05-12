use crate::DeepThought;

#[cxx::bridge(namespace = "ffi")]
mod ffi {

    // Opaque Rust type
    extern "Rust" {
        #[cxx_name = "RsDeepThought"]
        type DeepThought;
        fn ask(&self, question: &str) -> String;

        // Opaque type can only passed across the FFI with an indirection (reference, Box)
        // thus we need a dedicated create function
        fn create_deep_thought() -> Box<DeepThought>;
    }
}

fn create_deep_thought() -> Box<DeepThought> {
    Box::new(DeepThought::new())
}
