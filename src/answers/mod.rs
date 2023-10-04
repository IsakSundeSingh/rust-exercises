//! This is how you create modules with submodules.
//! Create a folder with the name of the module,
//! create a `mod.rs`-file, then declare the modules connected to that.
//! This file is the entry-point for the module.
//! There are other ways to do this which you can read in the documentation.
//! `use` the modules to re-export them for better ergonomics at the use-site.

// Here we make the module public but only within the crate.
pub(crate) mod exercise1;
pub(crate) mod exercise2;
pub(crate) mod exercise3;
pub(crate) mod exercise4;
pub(crate) mod exercise5;
