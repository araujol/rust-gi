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

#![crate_id="girepository#1.0-0.0.1"]
#![crate_type="lib"]

pub mod utils;
pub mod glib_gobject;
pub mod types;
pub mod typelib;
pub mod typeinfo;
pub mod baseinfo;
pub mod repository;
pub mod objectinfo;
pub mod functioninfo;
pub mod vfuncinfo;
pub mod arginfo;
pub mod callableinfo;
pub mod signalinfo;
pub mod enuminfo;
pub mod structinfo;
pub mod unioninfo;
pub mod fieldinfo;
pub mod interfaceinfo;
pub mod propertyinfo;
pub mod constantinfo;
pub mod registeredtypeinfo;
pub mod rffi;
