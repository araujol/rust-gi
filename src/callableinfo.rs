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

use glib_gobject::GBoolean;
use types::{GIBaseInfo, GICallableInfo, GITypeInfo};

use std::mem::transmute;


#[link(name = "girepository-1.0")]
extern "C" {
    fn g_callable_info_is_method(info: *GICallableInfo) -> GBoolean;
    fn g_callable_info_can_throw_gerror(info: *GICallableInfo) -> GBoolean;
    fn g_callable_info_get_return_type(info: *GICallableInfo) -> *GITypeInfo;
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


/// Convert GIBaseInfo to GICallableInfo.
pub fn to_gi_callable_info(object: *GIBaseInfo) -> *GICallableInfo {
    unsafe { transmute(object) }
}
