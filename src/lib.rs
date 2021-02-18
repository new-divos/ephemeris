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

extern crate serde;
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
mod tests {
}
