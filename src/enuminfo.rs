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

use enuminfo::libc::c_int;
use types::{GIEnumInfo, GIValueInfo};

use std::mem::transmute;


#[link(name = "girepository-1.0")]
extern "C" {
    fn g_enum_info_get_n_values(info: *GIEnumInfo) -> c_int;
    fn g_enum_info_get_value(info: *GIEnumInfo, n: c_int) -> *GIValueInfo;
    fn g_enum_info_get_n_methods(info: *GIEnumInfo) -> c_int;
}


/// Obtain the number of values this enumeration contains.
pub fn get_n_values(info: *GIEnumInfo) -> int {
    unsafe { g_enum_info_get_n_values(info) as int }
}

/// Obtain a value for this enumeration.
pub fn get_value(info: *GIEnumInfo, n: int) -> *GIValueInfo {
    unsafe { g_enum_info_get_value(info, n as c_int) }
}

/// Obtain the number of methods that this enum type has.
pub fn get_n_methods(info: *GIEnumInfo) -> int {
    unsafe { g_enum_info_get_n_methods(info) as int }
}


/// Convert GIBaseInfo to GIEnumInfo.
pub fn to_gi_enum_info<T>(object: *T) -> *GIEnumInfo {
    unsafe { transmute(object) }
}
