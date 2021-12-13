use std::fmt::{self, Display, Formatter};

use cpp::cpp;
use cpp::cpp_class;

cpp!{{
    #include "vector/temp.hpp"
}}

cpp_class!(pub unsafe struct Temp as "Temp<double>");
impl Temp {
    pub fn new() -> Self {
        cpp!(unsafe [] -> Temp as "Temp<double>" {
            return Temp<double>();
        })
    }

    pub fn get_some_value(&self) -> f64 {
        cpp!(unsafe [self as "Temp<double>*"] -> f64 as "double" {
            return self->some_value;
        })
    }
}

impl Display for Temp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("T({})", self.get_some_value()))
    }
}