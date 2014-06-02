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

use rffi::libc::{c_ushort, c_int, size_t};
use glib_gobject::GBoolean;
use types::{GITypeTag, GITypeInfo};


pub struct FFIType {
    size: size_t,
    alignment: c_ushort,
    type_: c_ushort,
    elements: **FFIType
}


#[link(name = "girepository-1.0")]
extern "C" {
    fn gi_type_tag_get_ffi_type(type_tag: c_int, is_pointer: GBoolean) -> *FFIType;
    fn g_type_info_get_ffi_type(info: *GITypeInfo) -> *FFIType;
}


pub fn type_tag_get_ffi_type(type_tag: GITypeTag, is_pointer: GBoolean) -> *FFIType {
    unsafe { gi_type_tag_get_ffi_type(type_tag as c_int, is_pointer) }
}

pub fn type_info_get_ffi_type(info: *GITypeInfo) -> *FFIType {
    unsafe { g_type_info_get_ffi_type(info) }
}
