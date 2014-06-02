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

use arginfo::libc::c_int;
use glib_gobject::GBoolean;
use types::{GIBaseInfo, GIArgInfo, GITypeInfo, 
            GIScopeType, GIDirection, GITransfer};

use std::mem::transmute;


#[link(name = "girepository-1.0")]
extern "C" {
    fn g_arg_info_get_direction(info: *GIArgInfo) -> c_int;
    fn g_arg_info_is_return_value(info: *GIArgInfo) -> GBoolean;
    fn g_arg_info_is_optional(info: *GIArgInfo) -> GBoolean;
    fn g_arg_info_is_caller_allocates(info: *GIArgInfo) -> GBoolean;
    fn g_arg_info_may_be_null(info: *GIArgInfo) -> GBoolean;
    fn g_arg_info_is_skip(info: *GIArgInfo) -> GBoolean;
    fn g_arg_info_get_ownership_transfer(info: *GIArgInfo) -> c_int;
    fn g_arg_info_get_scope(info: *GIArgInfo) -> c_int;
    fn g_arg_info_get_closure(info: *GIArgInfo) -> c_int;
    fn g_arg_info_get_destroy(info: *GIArgInfo) -> c_int;
    fn g_arg_info_get_type(info: *GIArgInfo) -> *GITypeInfo;
    fn g_arg_info_load_type(info: *GIArgInfo, type_: *GITypeInfo);
}


/// Obtain the direction of the argument.
pub fn get_direction(info: *GIArgInfo) -> Option<GIDirection> {
    let direction: Option<GIDirection> =
        FromPrimitive::from_i32(unsafe { g_arg_info_get_direction(info) });
    return direction
}

/// Obtain if the argument is a return value. It can either be a parameter or 
/// a return value.
pub fn is_return_value(info: *GIArgInfo) -> GBoolean {
    unsafe { g_arg_info_is_return_value(info) }
}

/// Obtain if the argument is optional.
pub fn is_optional(info: *GIArgInfo) -> GBoolean {
    unsafe { g_arg_info_is_optional(info) }
}

/// Obtain if the argument is a pointer to a struct or object that will receive 
/// an output of a function. 
pub fn is_caller_allocates(info: *GIArgInfo) -> GBoolean {
    unsafe { g_arg_info_is_caller_allocates(info) }
}

/// Obtain if the argument accepts NULL.
pub fn may_be_null(info: *GIArgInfo) -> GBoolean {
    unsafe { g_arg_info_may_be_null(info) }
}

/// Obtain if an argument is only useful in C.
pub fn is_skip(info: *GIArgInfo) -> GBoolean {
    unsafe { g_arg_info_is_skip(info) }
}

/// Obtain the ownership transfer for this argument.
pub fn get_ownership_transfer(info: *GIArgInfo) -> Option<GITransfer> {
    let transfer: Option<GITransfer> =
        FromPrimitive::from_i32(unsafe { g_arg_info_get_ownership_transfer(info) });
    return transfer
}

/// Obtain the scope type for this argument.
pub fn get_scope(info: *GIArgInfo) -> Option<GIScopeType> {
    let scope: Option<GIScopeType> =
        FromPrimitive::from_i32(unsafe { g_arg_info_get_scope(info) });
    return scope
}

/// Obtain the index of the user data argument. This is only valid for arguments 
/// which are callbacks.
pub fn get_closure(info: *GIArgInfo) -> int {
    unsafe { g_arg_info_get_closure(info) as int }
}

/// Obtains the index of the GDestroyNotify argument. This is only valid for 
/// arguments which are callbacks.
pub fn get_destroy(info: *GIArgInfo) -> int {
    unsafe { g_arg_info_get_destroy(info) as int }
}

/// Obtain the type information for info.
pub fn get_type(info: *GIArgInfo) -> *GITypeInfo {
    unsafe { g_arg_info_get_type(info) }
}

/// Obtain information about the type of given argument info; this function 
/// is a variant of g_arg_info_get_type() designed for stack allocation.
pub fn load_type(info: *GIArgInfo, type_: *GITypeInfo) {
    unsafe { g_arg_info_load_type(info, type_) }
}


/// Convert GIBaseInfo to GIArgInfo.
pub fn to_gi_arg_info(object: *GIBaseInfo) -> *GIArgInfo {
    unsafe { transmute(object) }
}
