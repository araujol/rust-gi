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

use typelib::libc::{c_char, size_t};
use glib_gobject::{GError, GMappedFile, GPointer, GBoolean};
use utils::to_string;


pub enum GITypelib {}


#[link(name = "girepository-1.0")]
extern "C" {
    fn g_typelib_new_from_memory(memory: *u8, len: size_t, error: **GError) -> *GITypelib;
    fn g_typelib_new_from_const_memory (memory: *u8, len: size_t, error: **GError) -> *GITypelib;
    fn g_typelib_new_from_mapped_file(mfile: *GMappedFile, error: **GError) -> *GITypelib;
    fn g_typelib_free(typelib: *GITypelib);
    fn g_typelib_symbol(typelib: *GITypelib, symbol_name: *c_char, symbol: *GPointer) -> GBoolean;
    fn g_typelib_get_namespace(typelib: *GITypelib) -> *c_char;
}


/// Creates a new GITypelib from a memory location.
pub fn new_from_memory(memory: *u8, len: size_t, error: **GError) -> *GITypelib {
    unsafe { g_typelib_new_from_memory(memory, len, error) }
}

/// Creates a new GITypelib from a memory location.
pub fn new_from_const_memory(memory: *u8, len: size_t, error: **GError) -> *GITypelib {
    unsafe { g_typelib_new_from_const_memory(memory, len, error) }
}

/// Creates a new GITypelib from a GMappedFile.
pub fn new_from_mapped_file(mfile: *GMappedFile, error: **GError) -> *GITypelib {
    unsafe { g_typelib_new_from_mapped_file(mfile, error) }
}

/// Free a GITypelib.
pub fn free(typelib: *GITypelib) {
    unsafe { g_typelib_free(typelib) }
}

/// Loads a symbol from GITypelib.
pub fn symbol(typelib: *GITypelib, symbol_name: &str, symbol: *GPointer) -> GBoolean {
    symbol_name.with_c_str(|c_symbol_name| unsafe {
        g_typelib_symbol(typelib, c_symbol_name, symbol)
    })
}

pub fn get_namespace(typelib: *GITypelib) -> Option<String> {
    to_string(unsafe { g_typelib_get_namespace(typelib) })
}
