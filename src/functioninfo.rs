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

use functioninfo::libc::{c_char, c_int};
use utils::to_string;
use types::{GIFunctionInfo, GIFunctionInfoFlags,
            GIVFuncInfo, GIPropertyInfo};

use std::mem::transmute;


#[link(name = "girepository-1.0")]
extern "C" {
    fn g_function_info_get_symbol(info: *GIFunctionInfo) -> *c_char;
    fn g_function_info_get_flags(info: *GIFunctionInfo) -> c_int;
    fn g_function_info_get_property(info: *GIFunctionInfo) -> *GIPropertyInfo;
    fn g_function_info_get_vfunc(info: *GIFunctionInfo) -> *GIVFuncInfo;
}


/// Obtain the symbol of the function. The symbol is the name of the exported 
/// function, suitable to be used as an argument to g_module_symbol().
pub fn get_symbol(info: *GIFunctionInfo) -> Option<String> {
    to_string(unsafe { g_function_info_get_symbol(info) })
}

/// Obtain the GIFunctionInfoFlags for the info.
pub fn get_flags(info: *GIFunctionInfo) -> Option<GIFunctionInfoFlags> {
    let flag: Option<GIFunctionInfoFlags> = 
        FromPrimitive::from_i32(unsafe { g_function_info_get_flags(info) });
    return flag
}

/// Obtain the property associated with this GIFunctionInfo. Only GIFunctionInfo 
/// with the flag GI_FUNCTION_IS_GETTER or GI_FUNCTION_IS_SETTER have a property 
/// set. For other cases, NULL will be returned.
pub fn get_property(info: *GIFunctionInfo) -> *GIPropertyInfo {
    unsafe { g_function_info_get_property(info) }
}

/// Obtain the virtual function associated with this GIFunctionInfo. Only 
/// GIFunctionInfo with the flag GI_FUNCTION_WRAPS_VFUNC has a virtual function 
/// set. For other cases, NULL will be returned.
pub fn get_vfunc(info: *GIFunctionInfo) -> *GIVFuncInfo {
    unsafe { g_function_info_get_vfunc(info) }
}


/// Convert GIBaseInfo to GIFunctionInfo.
pub fn to_gi_function_info<T>(object: *T) -> *GIFunctionInfo {
    unsafe { transmute(object) }
}
