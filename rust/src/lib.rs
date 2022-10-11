#![feature(let_else)]
#![feature(maybe_uninit_slice)]
#![feature(new_uninit)]
#![feature(nonnull_slice_from_raw_parts)]
#![feature(strict_provenance)]
#![feature(trait_alias)]

#[macro_use]
extern crate eyre;

pub mod chunk;
pub mod compiler;
pub mod obj;
pub mod value;
pub mod vm;
