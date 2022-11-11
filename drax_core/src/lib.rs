//! # Drax
//!
//! Drax is a library which supports framed packet reading and writing.
//!
//! ## Transport Layer
//!
//! The transport layer consists of a modular "chain" of transformers starting with a pre-defined
//! "byte buffer".
//!
//! Layers can be inserted and removed from the chain to add or change processing during different
//! stages of a connection.
//!
//! Using a `TypeMap` to maintain a context of data, we can write additional stages which process
//! more data, eventually reading in all of the allotted data.
//!
//! ## Primitive Reading and Writing
//!
//! This crate pre-defines all primitive type reading other than `char` since the bounds for a
//! `char` should be determined by the implementation rather than the library backing it.
//!
//! ## Reading and Writing Extensions
//!
//! Alternative types such as `String` which might not necessarily fit as a primitive will exist
//! as an extension.

pub mod extension;
pub mod nbt;
pub mod prelude;
pub mod primitives;
pub mod transport;

// stub types
pub type VarInt = i32;
pub type VarLong = i64;
pub type SizedVec<T> = Vec<T>;
pub type ShortSizedVec<T> = Vec<T>;
pub type Maybe<T> = Option<T>;
