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

use registeredtypeinfo::libc::c_char;
use utils::to_string;
use glib_gobject::GType;
use types::GIRegisteredTypeInfo;

use std::mem::transmute;


#[link(name = "girepository-1.0")]
extern "C" {
    fn g_registered_type_info_get_type_name(info: *GIRegisteredTypeInfo) -> *c_char;
    fn g_registered_type_info_get_type_init(info: *GIRegisteredTypeInfo) -> *c_char;
    fn g_registered_type_info_get_g_type(info: *GIRegisteredTypeInfo) -> GType;
}


/// Obtain the type name of the struct within the GObject type system.
pub fn get_type_name(info: *GIRegisteredTypeInfo) -> Option<String> {
    to_string(unsafe { g_registered_type_info_get_type_name(info) })
}

/// Obtain the type init function for info. The type init function is the 
/// function which will register the GType within the GObject type system. 
pub fn get_type_init(info: *GIRegisteredTypeInfo) -> Option<String> {
    to_string(unsafe { g_registered_type_info_get_type_init(info) })
}

/// Obtain the GType for this registered type or G_TYPE_NONE which a 
/// special meaning. 
pub fn get_g_type(info: *GIRegisteredTypeInfo) -> GType {
    unsafe { g_registered_type_info_get_g_type(info) }
}


/// Convert GIBaseInfo to GIRegisteredTypeInfo.
pub fn to_gi_registeredtype_info<T>(object: *T) -> *GIRegisteredTypeInfo {
    unsafe { transmute(object) }
}
