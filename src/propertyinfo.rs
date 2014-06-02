// GObject Introspection Rust bindings.
// Copyright (C) 2014  Luis Araujo <araujoc.luisf@gmail.com>

// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.

// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA

extern crate libc;

use propertyinfo::libc::c_int;
use glib_gobject::GParamFlags;
use types::{GIBaseInfo, GIPropertyInfo, GITypeInfo, GITransfer};

use std::mem::transmute;


#[link(name = "girepository-1.0")]
extern "C" {
    fn g_property_info_get_flags(info: *GIPropertyInfo) -> c_int;
    fn g_property_info_get_type(info: *GIPropertyInfo) -> *GITypeInfo;
    fn g_property_info_get_ownership_transfer(info: *GIPropertyInfo) -> c_int;
}


/// Obtain the flags for this property info.
pub fn get_flags(info: *GIPropertyInfo) -> Option<GParamFlags> {
    let flag: Option<GParamFlags> =
        FromPrimitive::from_i32(unsafe { g_property_info_get_flags(info) });
    return flag
}

/// Obtain the type information for the property info.
pub fn get_type(info: *GIPropertyInfo) -> *GITypeInfo {
    unsafe { g_property_info_get_type(info) }
}

/// Obtain the ownership transfer for this property.
pub fn get_ownership_transfer(info: *GIPropertyInfo) -> Option<GITransfer> {
    let transfer: Option<GITransfer> =
        FromPrimitive::from_i32(unsafe { g_property_info_get_ownership_transfer(info) });
    return transfer
}


/// Convert GIBaseInfo to GIPropertyInfo.
pub fn to_gi_property_info(object: *GIBaseInfo) -> *GIPropertyInfo {
    unsafe { transmute(object) }
}
