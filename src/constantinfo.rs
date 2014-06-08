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

use constantinfo::libc::c_int;
use types::{GIConstantInfo, GITypeInfo, GIArgument};

use std::mem::transmute;


#[link(name = "girepository-1.0")]
extern "C" {
    fn g_constant_info_get_type(info: *GIConstantInfo) -> *GITypeInfo;
    fn g_constant_info_free_value(info: *GIConstantInfo, value: *GIArgument);
    fn g_constant_info_get_value(info: *GIConstantInfo, value: *GIArgument) -> c_int;
}


/// Obtain the type of the constant as a GITypeInfo.
pub fn get_type(info: *GIConstantInfo) -> *GITypeInfo {
    unsafe { g_constant_info_get_type(info) }
}

/// Free the value returned from g_constant_info_get_value().
pub fn free_value(info: *GIConstantInfo, value: *GIArgument) {
    unsafe { g_constant_info_free_value(info, value) }
}

/// Obtain the value associated with the GIConstantInfo and store it in the value 
/// parameter. argument needs to be allocated before passing it in. The size of 
/// the constant value stored in argument will be returned. Free the value with 
/// g_constant_info_free_value().
pub fn get_value(info: *GIConstantInfo, value: *GIArgument) -> int {
    unsafe { g_constant_info_get_value(info, value) as int }
}


/// Convert GIBaseInfo to GIConstantInfo.
pub fn to_gi_constant_info<T>(object: *T) -> *GIConstantInfo {
    unsafe { transmute(object) }
}
