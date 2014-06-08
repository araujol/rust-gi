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
 * GIRepository — GObject Introspection repository manager
 */

extern crate libc;

use repository::libc::{c_char, c_uint};
use utils::{to_string, to_vec_string};
use glib_gobject::{GObject, GType, GSList, GQuark,
                   GList, GError, GBoolean, GOptionGroup};
use types::{GIBaseInfo, GIEnumInfo};
use typelib::GITypelib;

use std::ptr;


/* GIRepository */
enum GIRepositoryPrivate {}

pub struct GIRepository {
    /*< private >*/
    parent: GObject,
    priv_: *GIRepositoryPrivate,
}

pub enum GIRepositoryLoadFlags {
    GIRepositoryLoadFlagLazy = 1 << 0
}


#[link(name = "girepository-1.0")]
extern "C" {
    fn g_irepository_get_type() -> GType;
    fn g_irepository_get_default() -> *GIRepository;
    fn g_irepository_prepend_search_path(directory: *c_char);
    fn g_irepository_prepend_library_path(directory: *c_char);
    fn g_irepository_get_search_path() -> *GSList;
    fn g_irepository_load_typelib(repository: *GIRepository,
                                  typelib: *GITypelib,
                                  flags: GIRepositoryLoadFlags, 
                                  error: **GError) -> *c_char;
    fn g_irepository_is_registered(repository: *GIRepository,
                                   namespace_: *c_char,
                                   version: *c_char) -> GBoolean;
    fn g_irepository_find_by_name(repository: *GIRepository,
                                  namespace_: *c_char,
                                  name: *c_char) -> *GIBaseInfo;
    fn g_irepository_enumerate_versions(repository: *GIRepository,
                                        namespace_: *c_char) -> *GList;
    fn g_irepository_require(repository: *GIRepository,
                             namespace_: *c_char,
                             version: *c_char,
                             flags: GIRepositoryLoadFlags,
                             error: **GError) -> *GITypelib;
    fn g_irepository_require_private(repository: *GIRepository,
                                     typelib_dir: *c_char,
                                     namespace_: *c_char,
                                     version: *c_char,
                                     flags: GIRepositoryLoadFlags, 
                                     error: **GError) -> *GITypelib;
    fn g_irepository_get_dependencies(repository: *GIRepository,
                                      namespace_: *c_char) -> **c_char;
    fn g_irepository_get_loaded_namespaces(repository: *GIRepository) -> **c_char;
    fn g_irepository_find_by_gtype(repository: *GIRepository,
                                   gtype: GType) -> *GIBaseInfo;
    fn g_irepository_get_n_infos(repository: *GIRepository,
                                 namespace_: *c_char) -> c_uint;
    fn g_irepository_get_info(repository: *GIRepository,
                              namespace_: *c_char,
                              index: c_uint) -> *GIBaseInfo;
    fn g_irepository_find_by_error_domain(repository: *GIRepository,
                                          domain: GQuark) -> *GIEnumInfo;
    fn g_irepository_get_typelib_path(repository: *GIRepository,
                                      namespace_: *c_char) -> *c_char;
    fn g_irepository_get_shared_library(repository: *GIRepository,
                                        namespace_: *c_char) -> *c_char;
    fn g_irepository_get_c_prefix(repository: *GIRepository,
                                  namespace_: *c_char) -> *c_char;
    fn g_irepository_get_version(repository: *GIRepository,
                                 namespace_: *c_char) -> *c_char;
    fn g_irepository_get_option_group() -> *GOptionGroup;
    fn g_irepository_dump(arg: *c_char, error: **GError) -> GBoolean;
}


pub fn get_type() -> GType {
    unsafe { g_irepository_get_type() }
}

/// Returns the singleton process-global default GIRepository.
pub fn get_default() -> *GIRepository {
    unsafe { g_irepository_get_default() }
}

/// Prepends directory to the typelib search path.
pub fn prepend_search_path(directory: &str) {
    directory.with_c_str(|c_dir| unsafe {
        g_irepository_prepend_search_path(c_dir)
    })
}

/// Prepends directory to the search path that is used to search shared libraries 
/// referenced by imported namespaces.
pub fn prepend_library_path(directory: &str) {
    directory.with_c_str(|c_dir| unsafe {
        g_irepository_prepend_library_path(c_dir)
    })
}

/// Returns the current search path GIRepository will use when loading 
/// typelib files.
pub fn get_search_path() -> *GSList {
    unsafe { g_irepository_get_search_path() }
}

pub fn load_typelib(repository: *GIRepository, typelib: *GITypelib,
                    flags: GIRepositoryLoadFlags,
                    error: **GError) -> *c_char {
    unsafe { g_irepository_load_typelib(repository, typelib, flags, error) }
}

/// Check whether a particular namespace (and optionally, a specific version 
/// thereof) is currently loaded.
pub fn is_registered(repository: *GIRepository, 
                     namespace_: &str,
                     version: &str) -> GBoolean {
    namespace_.with_c_str(|c_namespace| unsafe {
        version.with_c_str(|c_version| {
            g_irepository_is_registered(repository, c_namespace, c_version)
        })
    })
}

/// Searches for a particular entry in a namespace.
pub fn find_by_name(repository: Option<*GIRepository>, namespace: &str,
                    name: &str) -> *GIBaseInfo {
    namespace.with_c_str(|c_namespace| unsafe {
        name.with_c_str(|c_name| {
            match repository {
                None =>
                    g_irepository_find_by_name(ptr::null(), c_namespace, c_name),
                Some(repo) =>
                    g_irepository_find_by_name(repo, c_namespace, c_name)
            }
        })
    })
}

/// Obtain an unordered list of versions (either currently loaded or available)
/// for namespace_ in this repository.
pub fn enumerate_versions(repository: Option<*GIRepository>,
                          namespace: &str) -> *GList {
    namespace.with_c_str(|c_namespace| unsafe {
        match repository {
            None => g_irepository_enumerate_versions(ptr::null(), c_namespace),
            Some(repo) => g_irepository_enumerate_versions(repo, c_namespace)
        }
    })
}

/// Force the namespace namespace_ to be loaded if it isn't already.
pub fn require(repository: Option<*GIRepository>, namespace: &str,
               version: Option<&str>, flags: GIRepositoryLoadFlags,
               error: **GError) -> Option<*GITypelib> {
    let typelib =
        namespace.with_c_str(|c_namespace| unsafe {
            if version.is_none() {
                match repository {
                    None => g_irepository_require(ptr::null(), c_namespace, ptr::null(), flags, error),
                    Some(repo) => g_irepository_require(repo, c_namespace, ptr::null(), flags, error)
                }
            } else {
                version.unwrap().with_c_str(|c_version| {
                    match repository {
                        None => g_irepository_require(ptr::null(), c_namespace, c_version, flags, error),
                        Some(repo) => g_irepository_require(repo, c_namespace, c_version, flags, error)
                    }
                })
            }
        });
    if typelib.is_null() { None } else { Some(typelib) }
}

/// Force the namespace namespace_ to be loaded if it isn't already.
pub fn require_private(repository: Option<*GIRepository>, namespace: &str,
                       typelib_dir: &str, version: Option<&str>,
                       flags: GIRepositoryLoadFlags,
                       error: **GError) -> *GITypelib {
    namespace.with_c_str(|c_namespace| unsafe {
        typelib_dir.with_c_str(|c_typelib_dir| {
            if version.is_none() {
                match repository {
                    None => g_irepository_require_private(ptr::null(), c_typelib_dir, c_namespace, ptr::null(), flags, error),
                    Some(repo) => g_irepository_require_private(repo, c_typelib_dir, c_namespace, ptr::null(), flags, error)
                }
            } else {
                version.unwrap().with_c_str(|c_version| {
                    match repository {
                        None => g_irepository_require_private(ptr::null(), c_typelib_dir, c_namespace, c_version, flags, error),
                        Some(repo) => g_irepository_require_private(repo, c_typelib_dir, c_namespace, c_version, flags, error)
                    }
                })
            }
        })
    })
}

/// Return an array of all (transitive) versioned dependencies for namespace_.
/// Returned strings are of the form namespace-version.
pub fn get_dependencies(repository: Option<*GIRepository>, namespace: &str) -> Vec<Option<String>> {
    to_vec_string(namespace.with_c_str(|c_namespace| unsafe {
        match repository {
            None => g_irepository_get_dependencies(ptr::null(), c_namespace),
            Some(repo) => g_irepository_get_dependencies(repo, c_namespace)
        }
    }))
}

/// Return the list of currently loaded namespaces.
pub fn get_loaded_namespaces(repository: Option<*GIRepository>) -> Vec<Option<String>> {
    to_vec_string(unsafe {
        match repository {
            None => g_irepository_get_loaded_namespaces(ptr::null()),
            Some(repo) => g_irepository_get_loaded_namespaces(repo)
        }
    })
}

/// Searches all loaded namespaces for a particular GType.
pub fn find_by_gtype(repository: Option<*GIRepository>, gtype: GType) -> *GIBaseInfo {
    unsafe {
        match repository {
            None => g_irepository_find_by_gtype(ptr::null(), gtype),
            Some(repo) => g_irepository_find_by_gtype(repo, gtype)
        }
    }
}

/// This function returns the number of metadata entries in given namespace namespace_.
pub fn get_n_infos(repository: Option<*GIRepository>, namespace: &str) -> uint {
    namespace.with_c_str(|c_namespace| unsafe {
        match repository {
            None => g_irepository_get_n_infos(ptr::null(), c_namespace),
            Some(repo) => g_irepository_get_n_infos(repo, c_namespace)
        }
    }) as uint
}

/// This function returns a particular metadata entry in the given namespace namespace_. 
pub fn get_info(repository: Option<*GIRepository>, namespace: &str, index: uint) -> *GIBaseInfo {
    namespace.with_c_str(|c_namespace| unsafe {
        match repository {
            None => g_irepository_get_info(ptr::null(), c_namespace, index as c_uint),
            Some(repo) => g_irepository_get_info(repo, c_namespace, index as c_uint)
        }
    })
}

/// Searches for the enum type corresponding to the given GError domain.
pub fn find_by_error_domain(repository: Option<*GIRepository>, domain: GQuark) -> *GIEnumInfo {
    unsafe {
        match repository {
            None => g_irepository_find_by_error_domain(ptr::null(), domain),
            Some(repo) => g_irepository_find_by_error_domain(repo, domain)
        }
    }
}

/// If namespace namespace_ is loaded, return the full path to the .typelib file
/// it was loaded from.
pub fn get_typelib_path(repository: Option<*GIRepository>, namespace: &str) -> Option<String> {
    to_string(namespace.with_c_str(|c_namespace| unsafe {
        match repository {
            None => g_irepository_get_typelib_path(ptr::null(), c_namespace),
            Some(repo) => g_irepository_get_typelib_path(repo, c_namespace)
        }
    }))
}

/// This function returns the full path to the shared C library associated with
/// the given namespace namespace_.
pub fn get_shared_library(repository: Option<*GIRepository>, namespace: &str) -> Option<String> {
    to_string(namespace.with_c_str(|c_namespace| unsafe {
        match repository {
            None => g_irepository_get_shared_library(ptr::null(), c_namespace),
            Some(repo) => g_irepository_get_shared_library(repo, c_namespace)
        }
    }))
}

/// This function returns the "C prefix", or the C level namespace associated 
/// with the given introspection namespace.
pub fn get_c_prefix(repository: Option<*GIRepository>, namespace: &str) -> Option<String> {
    to_string(namespace.with_c_str(|c_namespace| unsafe {
        match repository {
            None => g_irepository_get_c_prefix(ptr::null(), c_namespace),
            Some(repo) => g_irepository_get_c_prefix(repo, c_namespace)
        }
    }))
}

/// This function returns the loaded version associated with the given
/// namespace namespace_.
pub fn get_version(repository: Option<*GIRepository>, namespace: &str) -> Option<String> {
    to_string(namespace.with_c_str(|c_namespace| unsafe {
        match repository {
            None => g_irepository_get_version(ptr::null(), c_namespace),
            Some(repo) => g_irepository_get_version(repo, c_namespace)
        }
    }))
}

/// Obtain the option group for girepository, it's used by the dumper and for
/// programs that wants to provide introspection information
pub fn get_option_group() -> *GOptionGroup {
    unsafe { g_irepository_get_option_group() }
}

/// Argument specified is a comma-separated pair of filenames; i.e. of the
/// form "input.txt,output.xml".
pub fn dump(arg: &str, error: **GError) -> GBoolean {
    arg.with_c_str(|c_arg| unsafe {
        g_irepository_dump(c_arg, error)
    })
}
