use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct AttributeInfo<T> {
    attribute: &'static str,
    value: T,
}

#[derive(Debug)]
pub enum Error {
    ZeroDivisionError,
    ConversionError,
    SingularMatrixError,
    CannotCreateVec3DError(AttributeInfo<f64>),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ZeroDivisionError => {
                write!(f, "Zero division error")
            }
            Error::ConversionError => {
                write!(f, "Cannot convert one value into another")
            }
            Error::SingularMatrixError => {
                write!(f, "Try to use the singular matrix")
            }
            Error::CannotCreateVec3DError(ref err) => {
                write!(
                    f,
                    "Cannot create Vec3D with {} equals to {}",
                    err.attribute,
                    err.value
                )
            }
        }
    }
}

impl Error {
    pub fn new_attribute_info<T>(attribute: &'static str,
                                 value: T) -> AttributeInfo<T> {
        AttributeInfo::<T> { attribute, value }
    }
}