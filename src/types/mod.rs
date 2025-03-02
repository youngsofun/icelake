//! Types will provide the definition of iceberg in-memory data types and
//! functions to parse from on-disk files.

mod in_memory;
pub use in_memory::*;

mod on_disk;
pub use on_disk::*;

#[cfg(feature = "arrow_schema")]
mod to_arrow;

mod to_avro;
