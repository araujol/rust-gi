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
 * Utilities functions.
 */

extern crate libc;

use utils::libc::c_char;
use std::c_str::CString;


pub fn to_string(char_str: *c_char) -> Option<String> {
    let cstr = unsafe { CString::new(char_str, false) };
    if cstr.is_not_null() {
        match cstr.as_str() {
            None => fail!("Not valid string"), 
            Some(s) => Some(s.to_string())
        }
    } else {
        None
    }
}

pub fn to_vec_string(char_ptr: **c_char) -> Vec<Option<String>> {
    let mut vstr: Vec<Option<String>> = Vec::new();
    let mut deps: **c_char = char_ptr;
    unsafe {
        while (*deps).is_not_null() {
            vstr.push(to_string(*deps));
            deps = deps.offset(1);
        }
    }
    return vstr;
}
