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

use structinfo::libc::{c_char, c_int, size_t};
use glib_gobject::GBoolean;
use types::{GIStructInfo, GIFunctionInfo, GIFieldInfo};

use std::mem::transmute;


#[link(name = "girepository-1.0")]
extern "C" {
    fn g_struct_info_get_n_fields(info: *GIStructInfo) -> c_int;
    fn g_struct_info_get_field(info: *GIStructInfo,
                               n: c_int) -> *GIFieldInfo;
    fn g_struct_info_get_n_methods(info: *GIStructInfo) -> c_int;
    fn g_struct_info_get_method(info: *GIStructInfo,
                                n: c_int) -> *GIFunctionInfo;
    fn g_struct_info_find_method(info: *GIStructInfo,
                                 name: *c_char) -> *GIFunctionInfo;
    fn g_struct_info_get_size(info: *GIStructInfo) -> size_t;
    fn g_struct_info_get_alignment(info: *GIStructInfo) -> size_t;
    fn g_struct_info_is_gtype_struct(info: *GIStructInfo) -> GBoolean;
    fn g_struct_info_is_foreign(info: *GIStructInfo) -> GBoolean;
}


/// Obtain the number of fields this structure has.
pub fn get_n_fields(info: *GIStructInfo) -> int {
    unsafe { g_struct_info_get_n_fields(info) as int }
}

/// Obtain the type information for field with specified index.
pub fn get_field(info: *GIStructInfo, n: int) -> *GIFieldInfo {
    unsafe { g_struct_info_get_field(info, n as c_int) }
}

/// Obtain the number of methods this structure has.
pub fn get_n_methods(info: *GIStructInfo) -> int {
    unsafe { g_struct_info_get_n_methods(info) as int }
}

/// Obtain the type information for method with specified index.
pub fn get_method(info: *GIStructInfo, n: int) -> *GIFunctionInfo {
    unsafe { g_struct_info_get_method(info, n as c_int) }
}

/// Obtain the type information for method named name.
pub fn find_method(info: *GIStructInfo, name: &str) -> *GIFunctionInfo {
    name.with_c_str(|c_name| unsafe {
        g_struct_info_find_method(info, c_name)
    })
}

/// Obtain the total size of the structure.
pub fn get_size(info: *GIStructInfo) -> size_t {
    unsafe { g_struct_info_get_size(info) }
}

/// Obtain the required alignment of the structure.
pub fn get_alignment(info: *GIStructInfo) -> size_t {
    unsafe { g_struct_info_get_alignment(info) }
}

/// Return true if this structure represents the "class structure" for some 
/// GObject or GInterface. 
pub fn is_gtype_struct(info: *GIStructInfo) -> GBoolean {
    unsafe { g_struct_info_is_gtype_struct(info) }
}

pub fn is_foreign(info: *GIStructInfo) -> GBoolean {
    unsafe { g_struct_info_is_foreign(info) }
}


/// Convert GIBaseInfo to GIStructInfo.
pub fn to_gi_struct_info<T>(object: *T) -> *GIStructInfo {
    unsafe { transmute(object) }
}
