extern crate cpp_build;

fn main() {
    cpp_build::Config::new()
        .debug(true)
        // .object("vector/vector.o")
        .build("src/main.rs");
}
