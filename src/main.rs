extern crate cpp;

mod temp;

use std::ops::{Add, Sub, Mul, Div};

use cpp::cpp;
use cpp::cpp_class;
use cxx::let_cxx_string;
use temp::Temp;

cpp!{{
    #include "vector/vector.hpp"
    #include "vector/vector.cpp"
}}

cpp_class!(pub unsafe struct Vector3 as "Vector3");

impl Vector3 {
    fn new() -> Self {
        cpp!(unsafe [] -> Vector3 as "Vector3" {
            return Vector3();
        })
    }

    fn from_xyz(x: f64, y: f64, z: f64) -> Self {
        cpp!(unsafe [x as "double", y as "double", z as "double"] -> Vector3 as "Vector3" {
            return Vector3(x, y, z);
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
    
    fn get_y(&self) -> f64 {
        cpp!(unsafe [self as "Vector3*"] -> f64 as "double" {
            return self->y;
        })
    }

    fn set_y(&mut self, val: f64) {
        cpp!(unsafe [self as "Vector3*", val as "double"] {
            self->y = val;
        })
    }

    fn get_z(&self) -> f64 {
        cpp!(unsafe [self as "Vector3*"] -> f64 as "double" {
            return self->z;
        })
    }

    fn set_z(&mut self, val: f64) {
        cpp!(unsafe [self as "Vector3*", val as "double"] {
            self->z = val;
        })
    }

    fn norm(&self) -> f64 {
        cpp!(unsafe [self as "Vector3*"] -> f64 as "double" {
            return self->norm();
        })
    }

    fn normalized(&self) -> Self {
        cpp!(unsafe [self as "Vector3*"] -> Vector3 as "Vector3" {
            return self->normalized();
        })
    }
    
    /// This is the ONE RIGHT WAY to get a Rust `String` from a C++ `std::string`
    fn to_string(&self) -> String {
        let_cxx_string!(cxxs = "");
        cpp!(unsafe [self as "Vector3*", cxxs as "std::string*"] {
            *cxxs = self->toString();
        });
        cxxs.to_string()
    }
}

impl Add for Vector3 {
    type Output = Self;
    fn add(self, other: Vector3) -> Self::Output {
        cpp!(unsafe [self as "Vector3", other as "Vector3"] -> Vector3 as "Vector3" {
            return self + other;
        })
    }
}

impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, other: Vector3) -> Self::Output {
        cpp!(unsafe [self as "Vector3", other as "Vector3"] -> Vector3 as "Vector3" {
            return self - other;
        })
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;
    fn mul(self, factor: f64) -> Self::Output {
        cpp!(unsafe [self as "Vector3", factor as "double"] -> Vector3 as "Vector3" {
            return self * factor;
        })
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;
    fn div(self, factor: f64) -> Self::Output {
        cpp!(unsafe [self as "Vector3", factor as "double"] -> Vector3 as "Vector3" {
            return self / factor;
        })
    }
}

fn main() {
    let mut a = Vector3::new();
    a.set_x(5.);
    println!("a.x = {}", a.get_x());
    a.set_y(1.);
    println!("a.y = {}", a.get_y());
    a.set_z(-1.);
    println!("a.z = {}", a.get_z());
    println!("a = {}", a.to_string());

    let b = Vector3::from_xyz(3., 4., 5.);
    println!("b = {} - |{}|", b.normalized().to_string(), b.norm());
    println!();

    println!("a + b = {}", (a + b).to_string());
    println!("a - b = {}", (a - b).to_string());
    println!("a * 4 = {}", (a * 4.).to_string());
    println!("b / 4 = {}", (a / 4.).to_string());
    
    println!("Temp: {}", Temp::new());
}
