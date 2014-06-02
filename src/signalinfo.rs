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

use signalinfo::libc::c_int;
use glib_gobject::{GBoolean, GSignalFlags};
use types::{GIBaseInfo, GISignalInfo, GIVFuncInfo};

use std::mem::transmute;


#[link(name = "girepository-1.0")]
extern "C" {
    fn g_signal_info_get_flags(info: *GISignalInfo) -> c_int;
    fn g_signal_info_get_class_closure(info: *GISignalInfo) -> *GIVFuncInfo;
    fn g_signal_info_true_stops_emit(info: *GISignalInfo) -> GBoolean;
}


/// Obtain the flags for this signal info.
pub fn get_flags(info: *GISignalInfo) -> Option<GSignalFlags> {
    let flag: Option<GSignalFlags> =
        FromPrimitive::from_i32(unsafe { g_signal_info_get_flags(info) });
    return flag
}

/// Obtain the class closure for this signal if one is set.
pub fn class_closure(info: *GISignalInfo) -> *GIVFuncInfo {
    unsafe { g_signal_info_get_class_closure(info) }
}

/// Obtain if the returning true in the signal handler will stop the emission 
/// of the signal.
pub fn true_stops_emit(info: *GISignalInfo) -> GBoolean {
    unsafe { g_signal_info_true_stops_emit(info) }
}


/// Convert GIBaseInfo to GISignalInfo.
pub fn to_gi_signal_info(object: *GIBaseInfo) -> *GISignalInfo {
    unsafe { transmute(object) }
}
