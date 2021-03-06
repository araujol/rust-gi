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

use interfaceinfo::libc::{c_char, c_int};
use types::{GIBaseInfo, GIInterfaceInfo, GIPropertyInfo,
            GIFunctionInfo, GIVFuncInfo, GISignalInfo,
            GIConstantInfo, GIStructInfo};

use std::mem::transmute;


#[link(name = "girepository-1.0")]
extern "C" {
    fn g_interface_info_get_n_prerequisites(info: *GIInterfaceInfo) -> c_int;
    fn g_interface_info_get_prerequisite(info: *GIInterfaceInfo, n: c_int) -> *GIBaseInfo;
    fn g_interface_info_get_n_properties(info: *GIInterfaceInfo) -> c_int;
    fn g_interface_info_get_property(info: *GIInterfaceInfo, n: c_int) -> *GIPropertyInfo;
    fn g_interface_info_get_n_methods(info: *GIInterfaceInfo) -> c_int;
    fn g_interface_info_get_method(info: *GIInterfaceInfo, n: c_int) -> *GIFunctionInfo;
    fn g_interface_info_find_method(info: *GIInterfaceInfo, name: *c_char) -> *GIFunctionInfo;
    fn g_interface_info_get_n_signals(info: *GIInterfaceInfo) -> c_int;
    fn g_interface_info_get_signal(info: *GIInterfaceInfo, n: c_int) -> *GISignalInfo;
    fn g_interface_info_find_signal(info: *GIInterfaceInfo, name: *c_char) -> *GISignalInfo;
    fn g_interface_info_get_n_vfuncs(info: *GIInterfaceInfo) -> c_int;
    fn g_interface_info_get_vfunc(info: *GIInterfaceInfo, n: c_int) -> *GIVFuncInfo;
    fn g_interface_info_find_vfunc(info: *GIInterfaceInfo, name: *c_char) -> *GIVFuncInfo;
    fn g_interface_info_get_n_constants(info: *GIInterfaceInfo) -> c_int;
    fn g_interface_info_get_constant(info: *GIInterfaceInfo, n: c_int) -> *GIConstantInfo;
    fn g_interface_info_get_iface_struct(info: *GIInterfaceInfo) -> *GIStructInfo;
}


/// Obtain the number of prerequisites for this interface type. A prerequisites 
/// is another interface that needs to be implemented for interface, similar to 
/// a base class for GObjects.
pub fn get_n_prerequisites(info: *GIInterfaceInfo) -> int {
    unsafe { g_interface_info_get_n_prerequisites(info) as int }
}

/// Obtain an interface type prerequisites index n.
pub fn get_prerequisite(info: *GIInterfaceInfo, n: int) -> *GIBaseInfo {
    unsafe { g_interface_info_get_prerequisite(info, n as c_int) }
}

/// Obtain the number of properties that this interface type has.
pub fn get_n_properties(info: *GIInterfaceInfo) -> int {
    unsafe { g_interface_info_get_n_properties(info) as int }
}

/// Obtain an interface type property at index n.
pub fn get_property(info: *GIInterfaceInfo, n: int) -> *GIPropertyInfo {
    unsafe { g_interface_info_get_property(info, n as c_int) }
}

/// Obtain the number of methods that this interface type has.
pub fn get_n_methods(info: *GIInterfaceInfo) -> int {
    unsafe { g_interface_info_get_n_methods(info) as int }
}

/// Obtain an interface type method at index n.
pub fn get_method(info: *GIInterfaceInfo, n: int) -> *GIFunctionInfo {
    unsafe { g_interface_info_get_method(info, n as c_int) }
}

/// Obtain a method of the interface type given a name. NULL will be returned 
/// if there's no method available with that name.
pub fn find_method(info: *GIInterfaceInfo, name: &str) -> *GIFunctionInfo {
    name.with_c_str(|c_name| unsafe {
        g_interface_info_find_method(info, c_name)
    })
}

/// Obtain an interface type signal at index n.
pub fn get_signal(info: *GIInterfaceInfo, n: int) -> *GISignalInfo {
    unsafe { g_interface_info_get_signal(info, n as c_int) }
}

/// Obtain the number of signals that this interface type has.
pub fn get_n_signals(info: *GIInterfaceInfo) -> int {
    unsafe { g_interface_info_get_n_signals(info) as int }
}

pub fn find_signal(info: *GIInterfaceInfo, name: &str) -> *GISignalInfo {
    name.with_c_str(|c_name| unsafe {
        g_interface_info_find_signal(info, c_name)
    })
}

/// Obtain the number of virtual functions that this interface type has.
pub fn get_n_vfuncs(info: *GIInterfaceInfo) -> int {
    unsafe { g_interface_info_get_n_vfuncs(info) as int }
}

/// Obtain an interface type virtual function at index n.
pub fn get_vfunc(info: *GIInterfaceInfo, n: int) -> *GIVFuncInfo {
    unsafe { g_interface_info_get_vfunc(info, n as c_int) }
}

/// Locate a virtual function slot with name name.
pub fn find_vfunc(info: *GIInterfaceInfo, name: &str) -> *GIVFuncInfo {
    name.with_c_str(|c_name| unsafe {
        g_interface_info_find_vfunc(info, c_name)
    })
}

/// Obtain the number of constants that this interface type has.
pub fn get_n_constants(info: *GIInterfaceInfo) -> int {
    unsafe { g_interface_info_get_n_constants(info) as int }
}

/// Obtain an interface type constant at index n.
pub fn get_constant(info: *GIInterfaceInfo, n: int) -> *GIConstantInfo {
    unsafe { g_interface_info_get_constant(info, n as c_int) }
}

/// Returns the layout C structure associated with this GInterface.
pub fn get_iface_struct(info: *GIInterfaceInfo) -> *GIStructInfo {
    unsafe { g_interface_info_get_iface_struct(info) }
}


/// Convert GIBaseInfo to GIInterfaceInfo.
pub fn to_gi_interface_info<T>(object: *T) -> *GIInterfaceInfo {
    unsafe { transmute(object) }
}
