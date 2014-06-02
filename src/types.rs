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
 * Common types.
 */

extern crate libc;

use types::libc
    ::{c_short, c_ushort, c_char, c_int, 
       c_uint, c_float, c_double, c_long, 
       c_ulong, size_t, ssize_t};
use glib_gobject::{GBoolean, GPointer};
use baseinfo;


/// Represents a unresolved type in a typelib.
pub enum GIUnresolvedInfo {}

/// Stores an argument of varying type.
pub enum GIArgument {
    VBoolean(GBoolean),
    VInt8(i8),
    VUInt8(u8),
    VInt16(i16),
    VUInt16(u16),
    VInt32(i32),
    VUInt32(u32),
    VInt64(i64),
    VUInt64(u64),
    VFloat(c_float),
    VBouble(c_double),
    VShort(c_short),
    VUShort(c_ushort),
    VInt(c_int),
    VUInt(c_uint),
    VLong(c_long),
    VULong(c_ulong),
    VSSize(ssize_t),
    VSize(size_t),
    VString(*c_char),
    VPointer(GPointer)
}

#[deriving(Show, FromPrimitive)]
pub enum GIInfoType {
    GIInfoTypeInvalid,
    GIInfoTypeFunction,
    GIInfoTypeCallback,
    GIInfoTypeStruct,
    GIInfoTypeBoxed,
    GIInfoTypeEnum,
    GIInfoTypeFlags,
    GIInfoTypeObject,
    GIInfoTypeInterface,
    GIInfoTypeConstant,
    GIInfoTypeInvalid0,
    GIInfoTypeUnion,
    GIInfoTypeValue,
    GIInfoTypeSignal,
    GIInfoTypeVFunc,
    GIInfoTypeProperty,
    GIInfoTypeField,
    GIInfoTypeArg,
    GIInfoTypeType,
    GIInfoTypeUnresolved
}

#[deriving(Show, FromPrimitive)]
pub enum GITransfer {
    GITransferNothing,
    GITransferContainer,
    GITransferEverything
}

#[deriving(Show, FromPrimitive)]
pub enum GIDirection {
    GIDirectionIn,
    GIDirectionOut,
    GIDirectionInOut
}

#[deriving(Show, FromPrimitive)]
pub enum GIScopeType {
    GIScopeTypeInvalid,
    GIScopeTypeCall,
    GIScopeTypeAsync,
    GIScopeTypeNotified
}

pub enum GITypeTag {
    /* Basic types */
    GITypeTagVoid      =  0,
    GITypeTagBoolean   =  1,
    GITypeTagInt8      =  2,
    GITypeTagUInt8     =  3,
    GITypeTagInt16     =  4,
    GITypeTagUInt16    =  5,
    GITypeTagInt32     =  6,
    GITypeTagUInt32    =  7,
    GITypeTagInt64     =  8,
    GITypeTagUInt64    =  9,
    GITypeTagFloat     = 10,
    GITypeTagDouble    = 11,
    GITypeTagGType     = 12,
    GITypeTagUtf8      = 13,
    GITypeTagFilename  = 14,
    /* Non-basic types; compare with G_TYPE_TAG_IS_BASIC */
    GITypeTagArray     = 15,
    GITypeTagInterface = 16,
    GITypeTagGList     = 17,
    GITypeTagGSList    = 18,
    GITypeTagGHash     = 19,
    GITypeTagError     = 20,
    /* Another basic type */
    GITypeTagUnichar   = 21,
    /* Note - there is currently only room for 32 tags */
}

pub enum GIArrayType {
    GIArrayTypeC,
    GIArrayTypeArray,
    GIArrayTypePtrArray,
    GIArrayTypeByteArray
}

#[deriving(Show, FromPrimitive)]
pub enum GIFieldInfoFlags {
    GIFieldIsReadable = 1 << 0,
    GIFieldIsWritable = 1 << 1
}

#[deriving(Show, FromPrimitive)]
pub enum GIVFuncInfoFlags {
    GIVFuncMustChainUp       = 1 << 0,
    GIVFuncMustOverride      = 1 << 1,
    GIVFuncMustNotOverride   = 1 << 2,
    GIVFuncThrows            = 1 << 3
}

#[deriving(Show, FromPrimitive)]
pub enum GIFunctionInfoFlags {
    GIFunctionIsMethod      = 1 << 0,
    GIFunctionIsConstructor = 1 << 1,
    GIFunctionIsGetter      = 1 << 2,
    GIFunctionIsSetter      = 1 << 3,
    GIFunctionWrapsVFunc    = 1 << 4,
    GIFunctionThrows        = 1 << 5
}

pub type GIBaseInfo = baseinfo::_GIBaseInfoStub;

pub type GICallableInfo = GIBaseInfo;

pub type GIFunctionInfo = GIBaseInfo;

pub type GICallbackInfo = GIBaseInfo;

pub type GIRegisteredTypeInfo = GIBaseInfo;

pub type GIStructInfo = GIBaseInfo;

pub type GIUnionInfo = GIBaseInfo;

pub type GIEnumInfo = GIBaseInfo;

pub type GIObjectInfo = GIBaseInfo;

pub type GIInterfaceInfo = GIBaseInfo;

pub type GIConstantInfo = GIBaseInfo;

pub type GIValueInfo = GIBaseInfo;

pub type GISignalInfo = GIBaseInfo;

pub type GIVFuncInfo = GIBaseInfo;

pub type GIPropertyInfo = GIBaseInfo;

pub type GIFieldInfo = GIBaseInfo;

pub type GIArgInfo = GIBaseInfo;

pub type GITypeInfo = GIBaseInfo;

pub type GArgument = GIArgument;
