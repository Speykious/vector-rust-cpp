extern crate cpp;

use cpp::cpp;
use cpp::cpp_class;

cpp!{{
    #include "vector3_cpp/vector.hpp"
    #include <iostream>
}}

cpp_class!(pub unsafe struct Vector3 as "Vector3");
impl Vector3 {
    fn new() -> Self {
        cpp!(unsafe [] -> Vector3 as "Vector3" {
            return Vector3();
        })
    }

    fn get_x(&self) -> f64 {
        cpp!(unsafe [self as "Vector3*"] -> f64 as "double" {
            return self->x;
        })
    }

    fn set_x(&mut self, val: f64) {
        cpp!(unsafe [self as "Vector3*", val as "double"] {
            self->x = val;
        })
    }
}

fn main() {
    let mut a = Vector3::new();
    println!("a.x = {}", a.get_x());
    a.set_x(5.0);
    println!("a.x = {}", a.get_x());
}
