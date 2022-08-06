#![feature(let_else)]
#![feature(new_uninit)]
#![feature(nonnull_slice_from_raw_parts)]
#![feature(strict_provenance)]
#![feature(trait_alias)]

pub mod chunk;
pub mod compiler;
pub mod obj;
pub mod value;
pub mod vm;
