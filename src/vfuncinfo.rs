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

use vfuncinfo::libc::c_int;
use types::{GIVFuncInfo, GIVFuncInfoFlags, GISignalInfo, GIFunctionInfo};

use std::mem::transmute;


#[link(name = "girepository-1.0")]
extern "C" {
    fn g_vfunc_info_get_flags(info: *GIVFuncInfo) -> c_int;
    fn g_vfunc_info_get_offset(info: *GIVFuncInfo) -> c_int;
    fn g_vfunc_info_get_signal(info: *GIVFuncInfo) -> *GISignalInfo;
    fn g_vfunc_info_get_invoker(info: *GIVFuncInfo) -> *GIFunctionInfo;
}


/// Obtain the flags for this virtual function info.
pub fn get_flags(info: *GIVFuncInfo) -> Option<GIVFuncInfoFlags> {
    let flag: Option<GIVFuncInfoFlags> =
        FromPrimitive::from_i32(unsafe { g_vfunc_info_get_flags(info) });
    return flag
}

/// Obtain the offset of the function pointer in the class struct.
pub fn get_offset(info: *GIVFuncInfo) -> int {
    unsafe { g_vfunc_info_get_offset(info) as int }
}

/// Obtain the signal for the virtual function if one is set.
pub fn get_signal(info: *GIVFuncInfo) -> *GISignalInfo {
    unsafe { g_vfunc_info_get_signal(info) }
}

/// If this virtual function has an associated invoker method, this method will 
/// return it. An invoker method is a C entry point.
pub fn get_invoker(info: *GIVFuncInfo) -> *GIFunctionInfo {
    unsafe { g_vfunc_info_get_invoker(info) }
}


/// Convert GIBaseInfo to GIVFuncInfo.
pub fn to_gi_vfunc_info<T>(object: *T) -> *GIVFuncInfo {
    unsafe { transmute(object) }
}
