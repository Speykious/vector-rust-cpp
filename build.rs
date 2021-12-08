extern crate cpp_build;

fn main() {
    cpp_build::Config::new()
        .object("vector3_cpp/vector.o")
        .build("src/main.rs");
}
