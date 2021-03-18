#![allow(dead_code)]

pub mod base;
#[macro_use] pub mod macros;

#[cfg(test)]
#[macro_use] extern crate approx;

#[macro_use] extern crate ephem_derive;
extern crate num_traits;

#[cfg(test)]
extern crate rand;

extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

#[cfg(test)]
mod tests {
    pub const EPS: f64 = 1e-8;
    pub const ITERATIONS: i32 = 200;
}
