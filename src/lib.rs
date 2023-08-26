use arraystring::{typenum::U30, ArrayString};

#[cfg(feature = "enablediesel")]
#[cfg(feature = "enablediesel")]
#[macro_use]
extern crate diesel;

#[cfg(feature = "enablediesel")]
pub mod combo;
#[cfg(feature = "enablediesel")]
pub mod countryenum;
#[cfg(feature = "enableactix")]
pub mod httpmsgid;
#[cfg(feature = "enablediesel")]
pub mod langenum;
pub mod name;
pub mod password;
#[cfg(feature = "enableactix")]
pub mod responsecount;
#[cfg(feature = "enableactix")]
pub mod responsefind;
#[cfg(feature = "enableactix")]
pub mod responsemsg;
pub mod token;

pub type PhoneNumber = ArrayString<U30>;
