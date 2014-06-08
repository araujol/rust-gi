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

use callableinfo::libc::{c_char, c_int};
use glib_gobject::GBoolean;
use types::{GICallableInfo, GIArgInfo, GITypeInfo, GITransfer};
use utils::to_string;

use std::mem::transmute;


#[link(name = "girepository-1.0")]
extern "C" {
    fn g_callable_info_is_method(info: *GICallableInfo) -> GBoolean;
    fn g_callable_info_can_throw_gerror(info: *GICallableInfo) -> GBoolean;
    fn g_callable_info_get_return_type(info: *GICallableInfo) -> *GITypeInfo;
    fn g_callable_info_load_return_type(info: *GICallableInfo, type_: *GITypeInfo);
    fn g_callable_info_get_return_attribute(info: *GICallableInfo, name: *c_char) -> *c_char;
    fn g_callable_info_get_caller_owns(info: *GICallableInfo) -> c_int;
    fn g_callable_info_may_return_null(info: *GICallableInfo) -> GBoolean;
    fn g_callable_info_skip_return(info: *GICallableInfo) -> GBoolean;
    fn g_callable_info_get_n_args(info: *GICallableInfo) -> c_int;
    fn g_callable_info_get_arg(info: *GICallableInfo, n: c_int) -> *GIArgInfo;
    fn g_callable_info_load_arg(info: *GICallableInfo, n: c_int, arg: *GIArgInfo);
}


/// Determines if the callable info is a method.
pub fn is_method(info: *GICallableInfo) -> GBoolean {
    unsafe { g_callable_info_is_method(info) }
}

pub fn can_throw_error(info: *GICallableInfo) -> GBoolean {
    unsafe { g_callable_info_can_throw_gerror(info) }
}

/// Obtain the return type of a callable item as a GITypeInfo.
pub fn get_return_type(info: *GICallableInfo) -> *GITypeInfo {
    unsafe { g_callable_info_get_return_type(info) }
}

/// Obtain information about a return value of callable; this function is a
/// variant of g_callable_info_get_return_type() designed for stack allocation.
pub fn load_return_type(info: *GICallableInfo, type_: *GITypeInfo) {
    unsafe { g_callable_info_load_return_type(info, type_) }
}

/// Retrieve an arbitrary attribute associated with the return value.
pub fn get_return_attribute(info: *GICallableInfo, name: &str) -> Option<String> {
    to_string(name.with_c_str(|c_name| unsafe {
        g_callable_info_get_return_attribute(info, c_name)
    }))
}

/// See whether the caller owns the return value of this callable.
pub fn get_caller_owns(info: *GICallableInfo) -> Option<GITransfer> {
    let transfer: Option<GITransfer> =
        FromPrimitive::from_i32(unsafe { g_callable_info_get_caller_owns(info) });
    return transfer
}

/// See if a callable could return NULL.
pub fn may_return_null(info: *GICallableInfo) -> GBoolean {
    unsafe { g_callable_info_may_return_null(info) }
}

/// See if a callable's return value is only useful in C.
pub fn skip_return(info: *GICallableInfo) -> GBoolean {
    unsafe { g_callable_info_skip_return(info) }
}

/// Obtain the number of arguments (both IN and OUT) for this callable.
pub fn get_n_args(info: *GICallableInfo) -> int {
    unsafe { g_callable_info_get_n_args(info) as int }
}

/// Obtain information about a particular argument of this callable.
pub fn get_arg(info: *GICallableInfo, n: int) -> *GIArgInfo {
    unsafe { g_callable_info_get_arg(info, n as c_int) }
}

/// Obtain information about a particular argument of this callable; this
/// function is a variant of g_callable_info_get_arg() designed for stack
/// allocation.
pub fn load_arg(info: *GICallableInfo, n: int, arg: *GIArgInfo) {
    unsafe { g_callable_info_load_arg(info, n as c_int, arg) }
}


/// Convert GIBaseInfo to GICallableInfo.
pub fn to_gi_callable_info<T>(object: *T) -> *GICallableInfo {
    unsafe { transmute(object) }
}
