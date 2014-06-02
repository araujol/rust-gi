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

/*
 * GIBaseInfo — Base struct for all GITypelib structs
 */

extern crate libc;
extern crate core;

use baseinfo::libc::{c_char, c_int, c_uint};
use utils::to_string;
use glib_gobject::{GPointer, GType};
use types::{GIBaseInfo, GIInfoType};


pub struct _GIBaseInfoStub {
    /* <private> */
    dummy1: c_int,
    dummy2: c_int,
    dummy3: GPointer,
    dummy4: GPointer,
    dummy5: GPointer,
    dummy6: c_uint,
    dummy7: c_uint,
    padding: [GPointer, ..4]
}


#[link(name = "girepository-1.0")]
extern "C" {
    fn g_base_info_gtype_get_type() -> GType;
    fn g_base_info_ref(info: *GIBaseInfo) -> *GIBaseInfo;
    fn g_base_info_unref(info: *GIBaseInfo);
    fn g_base_info_get_type(info: *GIBaseInfo) -> c_uint;
    fn g_base_info_get_name(info: *GIBaseInfo) -> *c_char;
    fn g_base_info_get_namespace(info: *GIBaseInfo) -> *c_char;
}


pub fn gtype_get_type() -> GType {
    unsafe { g_base_info_gtype_get_type() }
}

pub fn _ref(info: *GIBaseInfo) -> *GIBaseInfo {
    unsafe { g_base_info_ref(info) }
}

pub fn unref(info: *GIBaseInfo) {
    unsafe { g_base_info_unref(info) }
}

/// Obtain the info type of the GIBaseInfo.
pub fn get_type(info: *GIBaseInfo) -> GIInfoType {
    let infotype: Option<GIInfoType> = 
        FromPrimitive::from_u32(unsafe { g_base_info_get_type(info) });
    return infotype.unwrap()
}

/// Obtain the name of the info.
pub fn get_name(info: *GIBaseInfo) -> Option<String> {
    unsafe { to_string(g_base_info_get_name(info)) }
}

/// Obtain the namespace of info.
pub fn get_namespace(info: *GIBaseInfo) -> Option<String> {
    unsafe { to_string(g_base_info_get_namespace(info)) }
}
