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
use types::{GITypeInfo, GITypeTag, GIInfoType};
use glib_gobject::GBoolean;
use utils::to_string;


#[link(name = "girepository-1.0")]
extern "C" {
    fn g_type_tag_to_string(type_: c_int) -> *c_char;
    fn g_info_type_to_string(type_: c_int) -> *c_char;
    fn g_type_info_is_pointer(info: *GITypeInfo) -> GBoolean;
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
