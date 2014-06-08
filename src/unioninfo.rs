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

use unioninfo::libc::{c_char, c_int, size_t};
use glib_gobject::GBoolean;
use types::{GITypeInfo, GIUnionInfo, GIFieldInfo,
            GIFunctionInfo, GIConstantInfo};

use std::mem::transmute;


#[link(name = "girepository-1.0")]
extern "C" {
    fn g_union_info_get_n_fields(info: *GIUnionInfo) -> c_int;
    fn g_union_info_get_field(info: *GIUnionInfo, n: c_int) -> *GIFieldInfo;
    fn g_union_info_get_n_methods(info: *GIUnionInfo) -> c_int;
    fn g_union_info_get_method(info: *GIUnionInfo, n: c_int) -> *GIFunctionInfo;
    fn g_union_info_is_discriminated(info: *GIUnionInfo) -> GBoolean;
    fn g_union_info_get_discriminator_offset(info: *GIUnionInfo) -> c_int;
    fn g_union_info_get_discriminator_type(info: *GIUnionInfo) -> *GITypeInfo;
    fn g_union_info_get_discriminator(info: *GIUnionInfo, n: c_int) -> *GIConstantInfo;
    fn g_union_info_find_method(info: *GIUnionInfo, name: *c_char) -> *GIFunctionInfo;
    fn g_union_info_get_size(info: *GIUnionInfo) -> size_t;
    fn g_union_info_get_alignment(info: *GIUnionInfo) -> size_t;
}


/// Obtain the number of fields this union has.
pub fn get_n_fields(info: *GIUnionInfo) -> int {
    unsafe { g_union_info_get_n_fields(info) as int }
}

/// Obtain the type information for field with specified index.
pub fn get_field(info: *GIUnionInfo, n: int) -> *GIFieldInfo {
    unsafe { g_union_info_get_field(info, n as c_int) }
}

/// Obtain the number of methods this union has.
pub fn get_n_methods(info: *GIUnionInfo) -> int {
    unsafe { g_union_info_get_n_methods(info) as int }
}

/// Obtain the type information for method with specified index.
pub fn get_method(info: *GIUnionInfo, n: c_int) -> *GIFunctionInfo {
    unsafe { g_union_info_get_method(info, n as c_int) }
}

/// Return true if this union contains discriminator field.
pub fn is_discriminated(info: *GIUnionInfo) -> GBoolean {
    unsafe { g_union_info_is_discriminated(info) }
}

/// Returns offset of the discriminator field in the structure.
pub fn get_discriminator_offset(info: *GIUnionInfo) -> int {
    unsafe { g_union_info_get_discriminator_offset(info) as int }
}

/// Obtain the type information of the union discriminator.
pub fn get_discriminator_type(info: *GIUnionInfo) -> *GITypeInfo {
    unsafe { g_union_info_get_discriminator_type(info) }
}

/// Obtain discriminator value assigned for n-th union field, i.e. n-th union 
/// field is the active one if discriminator contains this constant.
pub fn get_discriminator(info: *GIUnionInfo, n: int) -> *GIConstantInfo {
    unsafe { g_union_info_get_discriminator(info, n as c_int) }
}

/// Obtain the type information for method named name.
pub fn find_method(info: *GIUnionInfo, name: &str) -> *GIFunctionInfo {
    name.with_c_str(|c_name| unsafe {
        g_union_info_find_method(info, c_name)
    })
}

/// Obtain the total size of the union.
pub fn get_size(info: *GIUnionInfo) -> size_t {
    unsafe { g_union_info_get_size(info) }
}

/// Obtain the required alignment of the union.
pub fn get_alignment(info: *GIUnionInfo) -> size_t {
    unsafe { g_union_info_get_alignment(info) }
}


/// Convert GIBaseInfo to GIUnionInfo.
pub fn to_gi_union_info<T>(object: *T) -> *GIUnionInfo {
    unsafe { transmute(object) }
}
