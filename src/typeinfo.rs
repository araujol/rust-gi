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

use vfuncinfo::libc::{c_char, c_int};
use types::{GIBaseInfo, GITypeInfo, GITypeTag, GIInfoType, GIArrayType};
use glib_gobject::GBoolean;
use utils::to_string;


#[link(name = "girepository-1.0")]
extern "C" {
    fn g_type_tag_to_string(type_: c_int) -> *c_char;
    fn g_info_type_to_string(type_: c_int) -> *c_char;
    fn g_type_info_is_pointer(info: *GITypeInfo) -> GBoolean;
    fn g_type_info_get_tag(info: *GITypeInfo) -> c_int;
    fn g_type_info_get_param_type(info: *GITypeInfo, n: c_int) -> *GITypeInfo;
    fn g_type_info_get_interface(info: *GITypeInfo) -> *GIBaseInfo;
    fn g_type_info_get_array_length(info: *GITypeInfo) -> c_int;
    fn g_type_info_get_array_fixed_size(info: *GITypeInfo) -> c_int;
    fn g_type_info_is_zero_terminated(info: *GITypeInfo) -> GBoolean;
    fn g_type_info_get_array_type(info: *GITypeInfo) -> c_int;
}


/// Obtain a string representation of type.
pub fn type_tag_to_string(type_: GITypeTag) -> Option<String> {
    to_string(unsafe { g_type_tag_to_string(type_ as c_int) })
}

/// Obtain a string representation of type.
pub fn info_type_to_string(type_: GIInfoType) -> Option<String> {
    to_string(unsafe { g_info_type_to_string(type_ as c_int) } )
}

/// Obtain if the type is passed as a reference.
pub fn is_pointer(info: *GITypeInfo) -> GBoolean {
    unsafe { g_type_info_is_pointer(info) }
}

/// Obtain the type tag for the type.
pub fn get_tag(info: *GITypeInfo) -> Option<GITypeTag> {
    let typetag: Option<GITypeTag> =
        FromPrimitive::from_i32(unsafe { g_type_info_get_tag(info) } );
    return typetag
}

/// Obtain the parameter type n.
pub fn get_param_type(info: *GITypeInfo, n: int) -> *GITypeInfo {
    unsafe { g_type_info_get_param_type(info, n as c_int) }
}

/// For types which have GI_TYPE_TAG_INTERFACE such as GObjects and boxed values,
/// this function returns full information about the referenced type.
pub fn get_interface(info: *GITypeInfo) -> *GIBaseInfo {
    unsafe { g_type_info_get_interface(info) }
}

/// Obtain the array length of the type.
pub fn get_array_length(info: *GITypeInfo) -> int {
    unsafe { g_type_info_get_array_length(info) as int }
}

/// Obtain the fixed array size of the type.
pub fn get_array_fixed_size(info: *GITypeInfo) -> int {
    unsafe { g_type_info_get_array_fixed_size(info) as int }
}

/// Obtain if the last element of the array is NULL.
pub fn is_zero_terminated(info: *GITypeInfo) -> GBoolean {
    unsafe { g_type_info_is_zero_terminated(info) }
}

/// Obtain the array type for this type.
pub fn get_array_type(info: *GITypeInfo) -> Option<GIArrayType> {
    let arraytype: Option<GIArrayType> =
        FromPrimitive::from_i32(unsafe { g_type_info_get_array_type(info) });
    return arraytype
}
