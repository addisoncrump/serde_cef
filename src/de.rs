// Copyright 2018 Addison Crump
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate lazy_static;

use std::ops::{AddAssign, MulAssign, Neg};

use serde::de::{
    self, Deserialize, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess,
    VariantAccess, Visitor,
};

use error::{Error, Result};

lazy_static! {
    static ref CEFR: Regex = Regex::new(r"((CEF:\d+)([^=\\]+\|){0,7})(.*)").unwrap();
    static ref HEADSPLITR: Regex = Regex::new(r"(?<!\\(?:\\\\)+)\|(?:(?!$))").unwrap();
    static ref BODYPARSER: Regex = Regex::new(r"([^=\s]+)=((?:[\\]=|[^=])+)(?:(?=\s)|$)").unwrap();
}

pub struct Deserializer<'de> {
    input: &'de str,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        Deserializer { input }
    }
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: de::Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}
