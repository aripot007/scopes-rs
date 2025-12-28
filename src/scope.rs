use std::str::FromStr;

#[cfg(feature = "hierarchy")]
use crate::hierarchy::Hierarchized;


#[cfg(not(feature = "hierarchy"))]
pub trait Scope: FromStr + PartialEq {}

#[cfg(feature = "hierarchy")]
pub trait Scope: FromStr + PartialEq + Hierarchized {}
