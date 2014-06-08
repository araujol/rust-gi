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

use glib_gobject::libc
    ::{c_void, c_char, c_int, c_uint, c_float,
       c_long, c_ulong, c_double, size_t};


/* GObject */
pub type GType = size_t;
pub type GBoolean = c_int;
pub type GPointer = *c_void;
pub type GQuark = c_uint;

pub enum GData {}

#[deriving(Show, FromPrimitive)]
pub enum GParamFlags {
    GParamReadable           = 1 << 0,
    GParamWritable           = 1 << 1,
    GParamConstruct          = 1 << 2,
    GParamConstructOnly      = 1 << 3,
    GParamLaxValidation      = 1 << 4,
    GParamStaticName         = 1 << 5,
    GParamStaticNick         = 1 << 6,
    GParamSTaticBlurb        = 1 << 7,
    /* User defined flags go up to 30 */
    GParamDeprecated         = 1 << 31
}

#[deriving(Show, FromPrimitive)]
pub enum GSignalFlags {
    GSignalRunFirst     = 1 << 0,
    GSignalRunLast      = 1 << 1,
    GSignalRunCleanup   = 1 << 2,
    GSignalNoRecurse    = 1 << 3,
    GSignalDetailed     = 1 << 4,
    GSignalAction       = 1 << 5,
    GSignalNoHooks      = 1 << 6,
    GSignalMustCollect  = 1 << 7,
    GSignalDeprecated   = 1 << 8
}

pub struct GObject {
    g_type_instance: GTypeInstance,

    /*< private >*/
    // volatile guint ref_count;
    ref_count: c_uint,
    qdata: *GData
}

struct GTypeInstance {
    /*< private >*/
    g_class: *GTypeClass
}

struct GTypeClass {
  /*< private >*/
  g_type: GType
}

pub enum GValueData {
    GValueDataVInt(c_int),
    GValueDataVUInt(c_uint),
    GValueDataVLong(c_long),
    GValueDataVULong(c_ulong),
    GValueDataVInt64(i64),
    GValueDataVUInt64(u64),
    GValueDataVFloat(c_float),
    GValueDataVDouble(c_double),
    GValueDataVPointer(GPointer)
}

pub struct GValue {
    /*< private >*/
    g_type: GType,

    /* public for GTypeValueTable methods */
    data: [GValueData, ..2]
}

/* GLib */
pub enum GOptionGroup {}
pub enum GMappedFile {}

/* TODO: Get higher level structs for lists using generics */
pub struct GSList
{
    data: GPointer,
    next: *GSList
}

pub struct GList {
    data: GPointer,
    next: *GList,
    prev: *GList
}

pub struct GError {
    domain: GQuark,
    code: c_int,
    message: *c_char
}
