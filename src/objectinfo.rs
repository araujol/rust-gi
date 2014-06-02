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

use objectinfo::libc::{c_void, c_char, c_int};
use utils::to_string;
use types::{GIBaseInfo, GIObjectInfo, GIFunctionInfo};
use glib_gobject::{GValue, GBoolean};

use std::mem::transmute;


type GIObjectInfoRefFunction = fn (*c_void) -> *c_void;
type GIObjectInfoUnrefFunction = fn (*c_void);
type GIObjectInfoSetValueFunction = fn (*GValue, *c_void);
type GIObjectInfoGetValueFunction = fn (*GValue) -> *c_void;


#[link(name = "girepository-1.0")]
extern "C" {
    fn g_object_info_get_type_name(info: *GIObjectInfo) -> *c_char;
    fn g_object_info_get_type_init(info: *GIObjectInfo) -> *c_char;
    fn g_object_info_get_abstract(info: *GIObjectInfo) -> GBoolean;
    fn g_object_info_get_fundamental(info: *GIObjectInfo) -> GBoolean;
    fn g_object_info_get_n_methods(info: *GIObjectInfo) -> c_int;
    fn g_object_info_get_method(info: *GIObjectInfo, n: c_int) -> *GIFunctionInfo;
}


/// Obtain the name of the objects class/type.
pub fn get_type_name(info: *GIObjectInfo) -> Option<String> {
    to_string(unsafe { g_object_info_get_type_name(info) })
}

/// Obtain the function which when called will return the GType function for 
/// which this object type is registered.
pub fn get_type_init(info: *GIObjectInfo) -> Option<String> {
    to_string(unsafe { g_object_info_get_type_init(info) })
}

/// Obtain if the object type is an abstract type, eg if it cannot be instantiated.
pub fn get_abstract(info: *GIObjectInfo) -> GBoolean {
    unsafe { g_object_info_get_abstract(info) }
}

/// Obtain if the object type is of a fundamental type which is not G_TYPE_OBJECT.
pub fn get_fundamental(info: *GIObjectInfo) -> GBoolean {
    unsafe { g_object_info_get_fundamental(info) }
}

/// Obtain the number of methods that this object type has.
pub fn get_n_methods(info: *GIObjectInfo) -> int {
    unsafe { g_object_info_get_n_methods(info) as int }
}

/// Obtain an object type method at index n.
pub fn get_method(info: *GIObjectInfo, n: int) -> *GIFunctionInfo {
    unsafe { g_object_info_get_method(info, n as c_int) }
}


/// Convert GIBaseInfo to GIObjectInfo.
pub fn to_gi_object_info(object: *GIBaseInfo) -> *GIObjectInfo {
    unsafe { transmute(object) }
}
