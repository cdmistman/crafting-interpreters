#![feature(allocator_api)]
#![feature(build_hasher_simple_hash_one)]
#![feature(let_chains)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_slice)]
#![feature(new_uninit)]
#![feature(nonnull_slice_from_raw_parts)]
#![feature(strict_provenance)]
#![feature(string_leak)]
#![feature(trait_alias)]

#[macro_use]
extern crate eyre;

pub mod chunk;
pub mod compiler;
pub mod mem;
pub mod obj;
pub mod value;
pub mod vm;
