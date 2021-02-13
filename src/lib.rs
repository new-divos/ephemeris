#![allow(dead_code)]

pub mod base;
#[macro_use]
pub mod macros;

#[cfg(test)]
#[macro_use]
extern crate approx;

#[macro_use]
extern crate ephem_derive;
extern crate num_traits;

#[cfg(test)]
extern crate rand;

#[cfg(test)]
mod tests {
}
