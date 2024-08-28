#![doc = include_str!("../README.md")]

mod any;
mod generated;

pub mod pb {
    pub use crate::generated::*;
}

pub use any::WellKnownTypes;

/// A serialized file descriptor set containing the entirety of the XDS API.
///
/// See [`prost_types`][fd] and the GRPC documentation for information on how to use
/// a descriptor set.
///
/// [fd]: https://docs.rs/prost-types/0.13.1/prost_types/struct.FileDescriptorSet.html
#[cfg(feature = "descriptor")]
pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("xds-descriptors.bin");
