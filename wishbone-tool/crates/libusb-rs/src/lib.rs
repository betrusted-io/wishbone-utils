//! This crate provides a safe wrapper around the native `libusb` library.

extern crate bit_set;
extern crate libc;
extern crate libusb_sys_wishbone_tool as libusb;

pub use error::{Error, Result};
pub use version::{version, LibraryVersion};

pub use context::{Context, Hotplug, LogLevel, Registration};
pub use device::Device;
pub use device_handle::DeviceHandle;
pub use device_list::{DeviceList, Devices};

pub use config_descriptor::{ConfigDescriptor, Interfaces};
pub use device_descriptor::DeviceDescriptor;
pub use endpoint_descriptor::EndpointDescriptor;
pub use fields::{
    request_type, Direction, Recipient, RequestType, Speed, SyncType, TransferType, UsageType,
    Version,
};
pub use interface_descriptor::{
    EndpointDescriptors, Interface, InterfaceDescriptor, InterfaceDescriptors,
};
pub use language::{Language, PrimaryLanguage, SubLanguage};

#[cfg(test)]
#[macro_use]
mod test_helpers;

#[macro_use]
mod error;
mod version;

mod context;
mod device;
mod device_handle;
mod device_list;

mod config_descriptor;
mod device_descriptor;
mod endpoint_descriptor;
mod fields;
mod interface_descriptor;
mod language;
