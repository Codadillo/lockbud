#![feature(rustc_private)]
#![feature(box_patterns)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;

pub mod analysis;
pub mod callbacks;
pub mod detector;
pub mod interest;
pub mod options;
