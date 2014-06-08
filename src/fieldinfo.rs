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

use fieldinfo::libc::c_int;
use glib_gobject::{GPointer, GBoolean};
use types::{GITypeInfo, GIFieldInfo, GIArgument, GIFieldInfoFlags};

use std::mem::transmute;


#[link(name = "girepository-1.0")]
extern "C" {
    fn g_field_info_get_flags(info: *GIFieldInfo) -> c_int;
    fn g_field_info_get_size(info: *GIFieldInfo) -> c_int;
    fn g_field_info_get_offset(info: *GIFieldInfo) -> c_int;
    fn g_field_info_get_type(info: *GIFieldInfo) -> *GITypeInfo;
    fn g_field_info_get_field(info: *GIFieldInfo, 
                              mem: GPointer, 
                              value: *GIArgument) -> GBoolean;
    fn g_field_info_set_field(info: *GIFieldInfo,
                              mem: GPointer,
                              value: *GIArgument) -> GBoolean;
}


/// Obtain the flags for this GIFieldInfo.
pub fn get_flags(info: *GIFieldInfo) -> Option<GIFieldInfoFlags> {
    let flag: Option<GIFieldInfoFlags> =
        FromPrimitive::from_i32(unsafe { g_field_info_get_flags(info) });
    return flag
}

/// Obtain the size in bits of the field member, this is how much space 
/// you need to allocate to store the field.
pub fn get_size(info: *GIFieldInfo) -> int {
    unsafe { g_field_info_get_size(info) as int }
}

/// Obtain the offset in bits of the field member, this is relative to the 
/// beginning of the struct or union.
pub fn get_offset(info: *GIFieldInfo) -> int {
    unsafe { g_field_info_get_offset(info) as int }
}

/// Obtain the type of a field as a GITypeInfo.
pub fn get_type(info: *GIFieldInfo) -> *GITypeInfo {
    unsafe { g_field_info_get_type(info) }
}

/// Reads a field identified by a GIFieldInfo from a C structure or union.
pub fn get_field(info: *GIFieldInfo, mem: GPointer, value: *GIArgument) -> GBoolean {
    unsafe { g_field_info_get_field(info, mem, value) }
}

/// Writes a field identified by a GIFieldInfo to a C structure or union.
pub fn set_field(info: *GIFieldInfo, mem: GPointer, value: *GIArgument) -> GBoolean {
    unsafe { g_field_info_set_field(info, mem, value) }
}


/// Convert GIBaseInfo to GIFieldInfo.
pub fn to_gi_field_info<T>(object: *T) -> *GIFieldInfo {
    unsafe { transmute(object) }
}
