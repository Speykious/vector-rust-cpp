extern crate cpp;

use std::ops::{Add, Sub, Mul, Div};
use std::ffi::CStr;
use libc::c_char;

use cpp::cpp;
use cpp::cpp_class;

cpp!{{
    #include "vector/vector.hpp"
    #include <iostream>
    #include <stdio.h>
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
    
    // This doesn't work
    fn to_string(&self) -> String {
        unsafe {
            CStr::from_ptr(
                cpp!([self as "Vector3*"] -> *const c_char as "const char*" {
                    std::string s = self->toString();
                    std::cout << s << std::endl;
                    printf("%s\n", s.c_str());
                    return s.c_str();
                })
            ).to_str().unwrap().to_string()
        }
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
    println!("a.x = {} | {}", a.get_x(), a.to_string());
    a.set_x(5.0);
    println!("a.x = {} | {}", a.get_x(), a.to_string());
}
