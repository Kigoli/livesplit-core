use TimingMethod;
use super::{Alignment, Color, Gradient};
use time::formatter::{Accuracy, DigitsFormat};
use std::result::Result as StdResult;

#[derive(From, Serialize, Deserialize)]
pub enum Value {
    Bool(bool),
    UInt(u64),
    Int(i64),
    String(String),
    OptionalString(Option<String>),
    Float(f64),
    Accuracy(Accuracy),
    DigitsFormat(DigitsFormat),
    OptionalTimingMethod(Option<TimingMethod>),
    Color(Color),
    OptionalColor(Option<Color>),
    Gradient(Gradient),
    Alignment(Alignment),
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        WrongType
    }
}

pub type Result<T> = StdResult<T, Error>;

impl Value {
    pub fn into_bool(self) -> Result<bool> {
        match self {
            Value::Bool(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    pub fn into_uint(self) -> Result<u64> {
        match self {
            Value::UInt(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    pub fn into_int(self) -> Result<i64> {
        match self {
            Value::Int(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    pub fn into_string(self) -> Result<String> {
        match self {
            Value::String(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    pub fn into_optional_string(self) -> Result<Option<String>> {
        match self {
            Value::OptionalString(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    pub fn into_float(self) -> Result<f64> {
        match self {
            Value::Float(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    pub fn into_accuracy(self) -> Result<Accuracy> {
        match self {
            Value::Accuracy(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    pub fn into_digits_format(self) -> Result<DigitsFormat> {
        match self {
            Value::DigitsFormat(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    pub fn into_optional_timing_method(self) -> Result<Option<TimingMethod>> {
        match self {
            Value::OptionalTimingMethod(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    pub fn into_color(self) -> Result<Color> {
        match self {
            Value::Color(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    pub fn into_optional_color(self) -> Result<Option<Color>> {
        match self {
            Value::OptionalColor(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    pub fn into_gradient(self) -> Result<Gradient> {
        match self {
            Value::Color(v) => Ok(Gradient::Plain(v)),
            Value::Gradient(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    pub fn into_alignment(self) -> Result<Alignment> {
        match self {
            Value::Alignment(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }
}

impl Into<bool> for Value {
    fn into(self) -> bool {
        self.into_bool().unwrap()
    }
}

impl Into<u64> for Value {
    fn into(self) -> u64 {
        self.into_uint().unwrap()
    }
}

impl Into<i64> for Value {
    fn into(self) -> i64 {
        self.into_int().unwrap()
    }
}

impl Into<String> for Value {
    fn into(self) -> String {
        self.into_string().unwrap()
    }
}

impl Into<Option<String>> for Value {
    fn into(self) -> Option<String> {
        self.into_optional_string().unwrap()
    }
}

impl Into<f64> for Value {
    fn into(self) -> f64 {
        self.into_float().unwrap()
    }
}

impl Into<Accuracy> for Value {
    fn into(self) -> Accuracy {
        self.into_accuracy().unwrap()
    }
}

impl Into<DigitsFormat> for Value {
    fn into(self) -> DigitsFormat {
        self.into_digits_format().unwrap()
    }
}

impl Into<Option<TimingMethod>> for Value {
    fn into(self) -> Option<TimingMethod> {
        self.into_optional_timing_method().unwrap()
    }
}

impl Into<Color> for Value {
    fn into(self) -> Color {
        self.into_color().unwrap()
    }
}

impl Into<Option<Color>> for Value {
    fn into(self) -> Option<Color> {
        self.into_optional_color().unwrap()
    }
}

impl Into<Gradient> for Value {
    fn into(self) -> Gradient {
        self.into_gradient().unwrap()
    }
}

impl Into<Alignment> for Value {
    fn into(self) -> Alignment {
        self.into_alignment().unwrap()
    }
}
