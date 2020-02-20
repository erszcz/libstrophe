use ::libc;
extern "C" {
    pub type _hash_t;
    pub type _parser_t;
    pub type _tls;
    pub type _xmpp_rand_t;
    pub type _hash_iterator_t;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    /* free some blocks returned by other APIs, for example the
   buffer you get from xmpp_stanza_to_text */
    #[no_mangle]
    fn xmpp_free(ctx: *const xmpp_ctx_t, p: *mut libc::c_void);
    /* * allocate and initialize a new hash table */
    /* * allocate a new reference to an existing hash table */
    /* * release a hash table when no longer needed */
    /* * add a key, value pair to a hash table.
 *  each key can appear only once; the value of any
 *  identical key will be replaced
 */
    /* * look up a key in a hash table */
    /* * delete a key from a hash table */
    #[no_mangle]
    fn hash_drop(table: *mut hash_t, key: *const libc::c_char) -> libc::c_int;
    /* * return the number of keys in a hash */
    #[no_mangle]
    fn hash_num_keys(table: *mut hash_t) -> libc::c_int;
    #[no_mangle]
    fn xmpp_alloc(ctx: *const xmpp_ctx_t, size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn hash_release(table: *mut hash_t);
    /* * allocate and initialize a new iterator */
    /* * release an iterator that is no longer needed */
    /* * return the next hash table key from the iterator.
    the returned key should not be freed */
    #[no_mangle]
    fn hash_iter_next(iter: *mut hash_iterator_t) -> *const libc::c_char;
    #[no_mangle]
    fn hash_iter_release(iter: *mut hash_iterator_t);
    #[no_mangle]
    fn hash_add(table: *mut hash_t, key: *const libc::c_char,
                data: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn xmpp_strdup(ctx: *const xmpp_ctx_t, s: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn hash_iter_new(table: *mut hash_t) -> *mut hash_iterator_t;
    /*
 * Copyright Patrick Powell 1995
 * This code is based on code written by Patrick Powell (papowell@astart.com)
 * It may be used for any purpose as long as this notice remains intact
 * on all source code distributions
 */
    /* * @file
 *  Compatibility wrappers for OSes lacking snprintf(3) and/or vsnprintf(3).
 */
    #[no_mangle]
    fn xmpp_snprintf(str: *mut libc::c_char, count: size_t,
                     fmt: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn hash_new(ctx: *mut xmpp_ctx_t, size: libc::c_int,
                free_func: hash_free_func) -> *mut hash_t;
    #[no_mangle]
    fn hash_get(table: *mut hash_t, key: *const libc::c_char)
     -> *mut libc::c_void;
    #[no_mangle]
    fn xmpp_realloc(ctx: *const xmpp_ctx_t, p: *mut libc::c_void,
                    size: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmpp_mem_t {
    pub alloc: Option<unsafe extern "C" fn(_: size_t, _: *mut libc::c_void)
                          -> *mut libc::c_void>,
    pub free: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *mut libc::c_void) -> ()>,
    pub realloc: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t,
                                             _: *mut libc::c_void)
                            -> *mut libc::c_void>,
    pub userdata: *mut libc::c_void,
}
/* run-time contexts */
/* user-replaceable memory allocator */
pub type xmpp_mem_t = _xmpp_mem_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmpp_log_t {
    pub handler: xmpp_log_handler,
    pub userdata: *mut libc::c_void,
}
pub type xmpp_log_handler
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: xmpp_log_level_t,
                                _: *const libc::c_char,
                                _: *const libc::c_char) -> ()>;
pub type xmpp_log_level_t = libc::c_uint;
pub const XMPP_LEVEL_ERROR: xmpp_log_level_t = 3;
pub const XMPP_LEVEL_WARN: xmpp_log_level_t = 2;
pub const XMPP_LEVEL_INFO: xmpp_log_level_t = 1;
pub const XMPP_LEVEL_DEBUG: xmpp_log_level_t = 0;
/* user-replaceable log object */
pub type xmpp_log_t = _xmpp_log_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmpp_ctx_t {
    pub mem: *const xmpp_mem_t,
    pub log: *const xmpp_log_t,
    pub rand: *mut xmpp_rand_t,
    pub loop_status: xmpp_loop_status_t,
    pub connlist: *mut xmpp_connlist_t,
    pub timeout: libc::c_ulong,
}
pub type xmpp_connlist_t = _xmpp_connlist_t;
/* common.h
** strophe XMPP client library -- internal common structures
**
** Copyright (C) 2005-2009 Collecta, Inc.
**
**  This software is provided AS-IS with no warranty, either express or
**  implied.
**
**  This program is dual licensed under the MIT and GPLv3 licenses.
*/
/* * @file
 *  Internally used functions and structures.
 */
/* * run-time context **/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmpp_connlist_t {
    pub conn: *mut xmpp_conn_t,
    pub next: *mut _xmpp_connlist_t,
}
/* connection */
/* opaque connection object */
pub type xmpp_conn_t = _xmpp_conn_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmpp_conn_t {
    pub ref_0: libc::c_uint,
    pub ctx: *mut xmpp_ctx_t,
    pub type_0: xmpp_conn_type_t,
    pub is_raw: libc::c_int,
    pub state: xmpp_conn_state_t,
    pub timeout_stamp: uint64_t,
    pub error: libc::c_int,
    pub stream_error: *mut xmpp_stream_error_t,
    pub sock: sock_t,
    pub ka_timeout: libc::c_int,
    pub ka_interval: libc::c_int,
    pub tls: *mut tls_t,
    pub tls_support: libc::c_int,
    pub tls_disabled: libc::c_int,
    pub tls_mandatory: libc::c_int,
    pub tls_legacy_ssl: libc::c_int,
    pub tls_trust: libc::c_int,
    pub tls_failed: libc::c_int,
    pub sasl_support: libc::c_int,
    pub auth_legacy_enabled: libc::c_int,
    pub secured: libc::c_int,
    pub bind_required: libc::c_int,
    pub session_required: libc::c_int,
    pub lang: *mut libc::c_char,
    pub domain: *mut libc::c_char,
    pub jid: *mut libc::c_char,
    pub pass: *mut libc::c_char,
    pub bound_jid: *mut libc::c_char,
    pub stream_id: *mut libc::c_char,
    pub blocking_send: libc::c_int,
    pub send_queue_max: libc::c_int,
    pub send_queue_len: libc::c_int,
    pub send_queue_head: *mut xmpp_send_queue_t,
    pub send_queue_tail: *mut xmpp_send_queue_t,
    pub reset_parser: libc::c_int,
    pub parser: *mut parser_t,
    pub connect_timeout: libc::c_uint,
    pub open_handler: xmpp_open_handler,
    pub authenticated: libc::c_int,
    pub conn_handler: xmpp_conn_handler,
    pub userdata: *mut libc::c_void,
    pub timed_handlers: *mut xmpp_handlist_t,
    pub id_handlers: *mut hash_t,
    pub handlers: *mut xmpp_handlist_t,
}
/* convenience functions for accessing the context */
/* wrappers for xmpp_log at specific levels */
/* * connection **/
/* opaque connection object */
pub type xmpp_handlist_t = _xmpp_handlist_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmpp_handlist_t {
    pub user_handler: libc::c_int,
    pub handler: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub userdata: *mut libc::c_void,
    pub enabled: libc::c_int,
    pub next: *mut xmpp_handlist_t,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub c2rust_unnamed: C2RustUnnamed_2,
    pub c2rust_unnamed_0: C2RustUnnamed_1,
    pub c2rust_unnamed_1: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub ns: *mut libc::c_char,
    pub name: *mut libc::c_char,
    pub type_0: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub id: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub period: libc::c_ulong,
    pub last_stamp: uint64_t,
}
pub type uint64_t = __uint64_t;
/* hash.h
** strophe XMPP client library -- hash table interface
** 
** Copyright (C) 2005-2009 Collecta, Inc. 
**
**  This software is provided AS-IS with no warranty, either express
**  or implied.
**
**  This program is dual licensed under the MIT and GPLv3 licenses.
*/
/* * @file
 *  Hash table API.
 */
pub type hash_t = _hash_t;
pub type xmpp_conn_handler
    =
    Option<unsafe extern "C" fn(_: *mut xmpp_conn_t, _: xmpp_conn_event_t,
                                _: libc::c_int, _: *mut xmpp_stream_error_t,
                                _: *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmpp_stream_error_t {
    pub type_0: xmpp_error_type_t,
    pub text: *mut libc::c_char,
    pub stanza: *mut xmpp_stanza_t,
}
pub type xmpp_stanza_t = _xmpp_stanza_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmpp_stanza_t {
    pub ref_0: libc::c_int,
    pub ctx: *mut xmpp_ctx_t,
    pub type_0: xmpp_stanza_type_t,
    pub prev: *mut xmpp_stanza_t,
    pub next: *mut xmpp_stanza_t,
    pub children: *mut xmpp_stanza_t,
    pub parent: *mut xmpp_stanza_t,
    pub data: *mut libc::c_char,
    pub attributes: *mut hash_t,
}
pub type xmpp_stanza_type_t = libc::c_uint;
pub const XMPP_STANZA_TAG: xmpp_stanza_type_t = 2;
pub const XMPP_STANZA_TEXT: xmpp_stanza_type_t = 1;
pub const XMPP_STANZA_UNKNOWN: xmpp_stanza_type_t = 0;
/* opaque run time context containing the above hooks */
pub type xmpp_ctx_t = _xmpp_ctx_t;
pub type xmpp_error_type_t = libc::c_uint;
pub const XMPP_SE_XML_NOT_WELL_FORMED: xmpp_error_type_t = 23;
pub const XMPP_SE_UNSUPPORTED_VERSION: xmpp_error_type_t = 22;
pub const XMPP_SE_UNSUPPORTED_STANZA_TYPE: xmpp_error_type_t = 21;
pub const XMPP_SE_UNSUPPORTED_ENCODING: xmpp_error_type_t = 20;
pub const XMPP_SE_UNDEFINED_CONDITION: xmpp_error_type_t = 19;
pub const XMPP_SE_SYSTEM_SHUTDOWN: xmpp_error_type_t = 18;
pub const XMPP_SE_SEE_OTHER_HOST: xmpp_error_type_t = 17;
pub const XMPP_SE_RESTRICTED_XML: xmpp_error_type_t = 16;
pub const XMPP_SE_RESOURCE_CONSTRAINT: xmpp_error_type_t = 15;
pub const XMPP_SE_REMOTE_CONN_FAILED: xmpp_error_type_t = 14;
pub const XMPP_SE_POLICY_VIOLATION: xmpp_error_type_t = 13;
pub const XMPP_SE_NOT_AUTHORIZED: xmpp_error_type_t = 12;
pub const XMPP_SE_INVALID_XML: xmpp_error_type_t = 11;
pub const XMPP_SE_INVALID_NS: xmpp_error_type_t = 10;
pub const XMPP_SE_INVALID_ID: xmpp_error_type_t = 9;
pub const XMPP_SE_INVALID_FROM: xmpp_error_type_t = 8;
pub const XMPP_SE_INTERNAL_SERVER_ERROR: xmpp_error_type_t = 7;
pub const XMPP_SE_IMPROPER_ADDR: xmpp_error_type_t = 6;
pub const XMPP_SE_HOST_UNKNOWN: xmpp_error_type_t = 5;
pub const XMPP_SE_HOST_GONE: xmpp_error_type_t = 4;
pub const XMPP_SE_CONN_TIMEOUT: xmpp_error_type_t = 3;
pub const XMPP_SE_CONFLICT: xmpp_error_type_t = 2;
pub const XMPP_SE_BAD_NS_PREFIX: xmpp_error_type_t = 1;
pub const XMPP_SE_BAD_FORMAT: xmpp_error_type_t = 0;
pub type xmpp_conn_event_t = libc::c_uint;
pub const XMPP_CONN_FAIL: xmpp_conn_event_t = 3;
pub const XMPP_CONN_DISCONNECT: xmpp_conn_event_t = 2;
pub const XMPP_CONN_RAW_CONNECT: xmpp_conn_event_t = 1;
pub const XMPP_CONN_CONNECT: xmpp_conn_event_t = 0;
pub type xmpp_open_handler
    =
    Option<unsafe extern "C" fn(_: *mut xmpp_conn_t) -> ()>;
/* parser.h
** strophe XMPP client library -- parser structures and functions
**
** Copyright (C) 2005-2009 Collecta, Inc. 
**
**  This software is provided AS-IS with no warranty, either express or
**  implied.
**
**  This program is dual licensed under the MIT and GPLv3 licenses.
*/
/* * @file
 *  Internally used functions and structures.
 */
pub type parser_t = _parser_t;
pub type xmpp_send_queue_t = _xmpp_send_queue_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmpp_send_queue_t {
    pub data: *mut libc::c_char,
    pub len: size_t,
    pub written: size_t,
    pub next: *mut xmpp_send_queue_t,
}
/* tls.h
** strophe XMPP client library -- TLS abstraction header
**
** Copyright (C) 2005-2009 Collecta, Inc. 
**
**  This software is provided AS-IS with no warranty, either express
**  or implied.
**
**  This program is dual licensed under the MIT and GPLv3 licenses.
*/
/* * @file
 *  TLS abstraction API.
 */
pub type tls_t = _tls;
/* sock.h
** strophe XMPP client library -- socket abstraction header
**
** Copyright (C) 2005-2009 Collecta, Inc. 
**
**  This software is provided AS-IS with no warranty, either express
**  or implied.
**
**  This program is dual licensed under the MIT and GPLv3 licenses.
*/
/* * @file
 *  Socket abstraction API.
 */
pub type sock_t = libc::c_int;
pub type xmpp_conn_state_t = libc::c_uint;
pub const XMPP_STATE_CONNECTED: xmpp_conn_state_t = 2;
pub const XMPP_STATE_CONNECTING: xmpp_conn_state_t = 1;
pub const XMPP_STATE_DISCONNECTED: xmpp_conn_state_t = 0;
pub type xmpp_conn_type_t = libc::c_uint;
pub const XMPP_COMPONENT: xmpp_conn_type_t = 2;
pub const XMPP_CLIENT: xmpp_conn_type_t = 1;
pub const XMPP_UNKNOWN: xmpp_conn_type_t = 0;
pub type xmpp_loop_status_t = libc::c_uint;
pub const XMPP_LOOP_QUIT: xmpp_loop_status_t = 2;
pub const XMPP_LOOP_RUNNING: xmpp_loop_status_t = 1;
pub const XMPP_LOOP_NOTSTARTED: xmpp_loop_status_t = 0;
/* rand.h
 * strophe XMPP client library -- pseudo-random number generator
 *
 * Copyright (C) 2014 Dmitry Podgorny <pasis.ua@gmail.com>
 *
 *  This software is provided AS-IS with no warranty, either express
 *  or implied.
 *
 *  This program is dual licensed under the MIT and GPLv3 licenses.
 */
/* * @file
 *  Pseudo-random number generator.
 */
/* size_t */
/* xmpp_ctx_t */
pub type xmpp_rand_t = _xmpp_rand_t;
/* * hash key iterator functions */
pub type hash_iterator_t = _hash_iterator_t;
pub type hash_free_func
    =
    Option<unsafe extern "C" fn(_: *const xmpp_ctx_t, _: *mut libc::c_void)
               -> ()>;
/*
void xmpp_register_stanza_handler(conn, stanza, xmlns, type, handler)
*/
/* stanzas */
/* allocate and initialize a blank stanza */
/* stanza.c
** strophe XMPP client library -- XMPP stanza object and utilities
**
** Copyright (C) 2005-2009 Collecta, Inc.
**
**  This software is provided AS-IS with no warranty, either express
**  or implied.
**
**  This program is dual licensed under the MIT and GPLv3 licenses.
*/
/* * @file
 *  Stanza creation and manipulation.
 */
/* * @defgroup Stanza Stanza creation and manipulation
 */
/* * Create a stanza object.
 *  This function allocates and initializes a blank stanza object.
 *  The stanza will have a reference count of one, so the caller does not
 *  need to clone it.
 *
 *  @param ctx a Strophe context object
 *
 *  @return a stanza object
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_new(mut ctx: *mut xmpp_ctx_t)
 -> *mut xmpp_stanza_t {
    let mut stanza: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    stanza =
        xmpp_alloc(ctx,
                   ::std::mem::size_of::<xmpp_stanza_t>() as libc::c_ulong) as
            *mut xmpp_stanza_t;
    if !stanza.is_null() {
        (*stanza).ref_0 = 1 as libc::c_int;
        (*stanza).ctx = ctx;
        (*stanza).type_0 = XMPP_STANZA_UNKNOWN;
        (*stanza).prev = 0 as *mut xmpp_stanza_t;
        (*stanza).next = 0 as *mut xmpp_stanza_t;
        (*stanza).children = 0 as *mut xmpp_stanza_t;
        (*stanza).parent = 0 as *mut xmpp_stanza_t;
        (*stanza).data = 0 as *mut libc::c_char;
        (*stanza).attributes = 0 as *mut hash_t
    }
    return stanza;
}
/* clone a stanza */
/* * Clone a stanza object.
 *  This function increments the reference count of the stanza object.
 *  
 *  @param stanza a Strophe stanza object
 *
 *  @return the stanza object with it's reference count incremented
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_clone(stanza: *mut xmpp_stanza_t)
 -> *mut xmpp_stanza_t {
    (*stanza).ref_0 += 1;
    return stanza;
}
/*
 * Copy the attributes of stanza src into stanza dst. Return -1 on error.
 */
unsafe extern "C" fn _stanza_copy_attributes(mut dst: *mut xmpp_stanza_t,
                                             src: *const xmpp_stanza_t)
 -> libc::c_int {
    let mut iter: *mut hash_iterator_t = 0 as *mut hash_iterator_t;
    let mut key: *const libc::c_char = 0 as *const libc::c_char;
    let mut val: *const libc::c_char = 0 as *const libc::c_char;
    let mut rc: libc::c_int = 0 as libc::c_int;
    iter = hash_iter_new((*src).attributes);
    if iter.is_null() { rc = -(1 as libc::c_int) }
    while rc == 0 as libc::c_int &&
              { key = hash_iter_next(iter); !key.is_null() } {
        val = hash_get((*src).attributes, key) as *const libc::c_char;
        if val.is_null() { rc = -(3 as libc::c_int) }
        if rc == 0 as libc::c_int {
            rc = xmpp_stanza_set_attribute(dst, key, val)
        }
    }
    hash_iter_release(iter);
    if rc != 0 as libc::c_int && !(*dst).attributes.is_null() {
        hash_release((*dst).attributes);
        (*dst).attributes = 0 as *mut hash_t
    }
    return rc;
}
/* copies a stanza and all children */
/* * Copy a stanza and its children.
 *  This function copies a stanza along with all its children and returns
 *  the new stanza and children with a reference count of 1.  The returned
 *  stanza will have no parent and no siblings.  This function is useful
 *  for extracting a child stanza for inclusion in another tree.
 *
 *  @param stanza a Strophe stanza object
 *
 *  @return a new Strophe stanza object
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_copy(stanza: *const xmpp_stanza_t)
 -> *mut xmpp_stanza_t {
    let mut current_block: u64;
    let mut copy: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut child: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut copychild: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut tail: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    copy = xmpp_stanza_new((*stanza).ctx);
    if !copy.is_null() {
        (*copy).type_0 = (*stanza).type_0;
        if !(*stanza).data.is_null() {
            (*copy).data = xmpp_strdup((*stanza).ctx, (*stanza).data);
            if (*copy).data.is_null() {
                current_block = 2255223553483703607;
            } else { current_block = 4906268039856690917; }
        } else { current_block = 4906268039856690917; }
        match current_block {
            2255223553483703607 => { }
            _ => {
                if !(*stanza).attributes.is_null() {
                    if _stanza_copy_attributes(copy, stanza) ==
                           -(1 as libc::c_int) {
                        current_block = 2255223553483703607;
                    } else { current_block = 1394248824506584008; }
                } else { current_block = 1394248824506584008; }
                match current_block {
                    2255223553483703607 => { }
                    _ => {
                        tail = (*copy).children;
                        child = (*stanza).children;
                        loop  {
                            if child.is_null() {
                                current_block = 5601891728916014340;
                                break ;
                            }
                            copychild = xmpp_stanza_copy(child);
                            if copychild.is_null() {
                                current_block = 2255223553483703607;
                                break ;
                            }
                            (*copychild).parent = copy;
                            if !tail.is_null() {
                                (*copychild).prev = tail;
                                (*tail).next = copychild
                            } else { (*copy).children = copychild }
                            tail = copychild;
                            child = (*child).next
                        }
                        match current_block {
                            2255223553483703607 => { }
                            _ => { return copy }
                        }
                    }
                }
            }
        }
    }
    /* release all the hitherto allocated memory */
    if !copy.is_null() { xmpp_stanza_release(copy); }
    return 0 as *mut xmpp_stanza_t;
}
/* free a stanza object and it's contents */
/* * Release a stanza object and all of its children.
 *  This function releases a stanza object and potentially all of its
 *  children, which may cause the object(s) to be freed.
 *
 *  @param stanza a Strophe stanza object
 *
 *  @return TRUE if the object was freed and FALSE otherwise
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_release(stanza: *mut xmpp_stanza_t)
 -> libc::c_int {
    let mut released: libc::c_int = 0 as libc::c_int;
    let mut child: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut tchild: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    /* release stanza */
    if (*stanza).ref_0 > 1 as libc::c_int {
        (*stanza).ref_0 -= 1
    } else {
        /* release all children */
        child = (*stanza).children;
        while !child.is_null() {
            tchild = child;
            child = (*child).next;
            xmpp_stanza_release(tchild);
        }
        if !(*stanza).attributes.is_null() {
            hash_release((*stanza).attributes);
        }
        if !(*stanza).data.is_null() {
            xmpp_free((*stanza).ctx, (*stanza).data as *mut libc::c_void);
        }
        xmpp_free((*stanza).ctx, stanza as *mut libc::c_void);
        released = 1 as libc::c_int
    }
    return released;
}
/* * Get the strophe context that the stanza is associated with.
 *
 *  @param stanza a Strophe stanza object
 *
 *  @return a Strophe context
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_get_context(stanza: *const xmpp_stanza_t)
 -> *mut xmpp_ctx_t {
    return (*stanza).ctx;
}
/* * Determine if a stanza is a text node.
 *  
 *  @param stanza a Strophe stanza object
 *
 *  @return TRUE if the stanza is a text node, FALSE otherwise
 * 
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_is_text(stanza: *mut xmpp_stanza_t)
 -> libc::c_int {
    return (!stanza.is_null() &&
                (*stanza).type_0 as libc::c_uint ==
                    XMPP_STANZA_TEXT as libc::c_int as libc::c_uint) as
               libc::c_int;
}
/* * Determine if a stanza is a tag node.
 *  
 *  @param stanza a Strophe stanza object
 *
 *  @return TRUE if the stanza is a tag node, FALSE otherwise
 * 
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_is_tag(stanza: *mut xmpp_stanza_t)
 -> libc::c_int {
    return (!stanza.is_null() &&
                (*stanza).type_0 as libc::c_uint ==
                    XMPP_STANZA_TAG as libc::c_int as libc::c_uint) as
               libc::c_int;
}
/* Escape a string with for use in a XML text node or attribute. Assumes that
 * the input string is encoded in UTF-8. On success, returns a pointer to a
 * buffer with the resulting data which must be xmpp_free()'d by the caller.
 * On failure, returns NULL.
 */
unsafe extern "C" fn _escape_xml(ctx: *mut xmpp_ctx_t,
                                 mut text: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut src: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    src = text;
    while *src as libc::c_int != '\u{0}' as i32 {
        match *src as libc::c_int {
            60 | 62 => {
                /* "&lt;" */
                /* "&gt;" */
                len =
                    (len as
                         libc::c_ulong).wrapping_add(4 as libc::c_int as
                                                         libc::c_ulong) as
                        size_t as size_t
            }
            38 => {
                /* "&amp;" */
                len =
                    (len as
                         libc::c_ulong).wrapping_add(5 as libc::c_int as
                                                         libc::c_ulong) as
                        size_t as size_t
            }
            34 => {
                len =
                    (len as
                         libc::c_ulong).wrapping_add(6 as libc::c_int as
                                                         libc::c_ulong) as
                        size_t as size_t
            }
            _ => { len = len.wrapping_add(1) }
        } /*"&quot;" */
        src = src.offset(1)
    } /* Error */
    buf =
        xmpp_alloc(ctx,
                   len.wrapping_add(1 as libc::c_int as
                                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                        as
                                                                        libc::c_ulong))
            as *mut libc::c_char;
    if buf.is_null() { return 0 as *mut libc::c_char }
    dst = buf;
    src = text;
    while *src as libc::c_int != '\u{0}' as i32 {
        match *src as libc::c_int {
            60 => {
                strcpy(dst, b"&lt;\x00" as *const u8 as *const libc::c_char);
                dst = dst.offset(4 as libc::c_int as isize)
            }
            62 => {
                strcpy(dst, b"&gt;\x00" as *const u8 as *const libc::c_char);
                dst = dst.offset(4 as libc::c_int as isize)
            }
            38 => {
                strcpy(dst, b"&amp;\x00" as *const u8 as *const libc::c_char);
                dst = dst.offset(5 as libc::c_int as isize)
            }
            34 => {
                strcpy(dst,
                       b"&quot;\x00" as *const u8 as *const libc::c_char);
                dst = dst.offset(6 as libc::c_int as isize)
            }
            _ => { *dst = *src; dst = dst.offset(1) }
        }
        src = src.offset(1)
    }
    *dst = '\u{0}' as i32 as libc::c_char;
    return buf;
}
/* small helper function */
unsafe extern "C" fn _render_update(mut written: *mut libc::c_int,
                                    length: libc::c_int,
                                    lastwrite: libc::c_int,
                                    mut left: *mut size_t,
                                    mut ptr: *mut *mut libc::c_char) {
    *written += lastwrite;
    if *written >= length {
        *left = 0 as libc::c_int as size_t;
        *ptr = 0 as *mut libc::c_char
    } else {
        *left =
            (*left as libc::c_ulong).wrapping_sub(lastwrite as libc::c_ulong)
                as size_t as size_t;
        *ptr = &mut *(*ptr).offset(lastwrite as isize) as *mut libc::c_char
    };
}
/* always returns number of bytes written or that would have been
 * written if the buffer was large enough
 * return values < 0 indicate some error occurred,
 * and return values > buflen indicate buffer was not large enough
 */
unsafe extern "C" fn _render_stanza_recursive(mut stanza: *mut xmpp_stanza_t,
                                              buf: *mut libc::c_char,
                                              buflen: size_t) -> libc::c_int {
    let mut ptr: *mut libc::c_char =
        buf; /* stanza->type == XMPP_STANZA_TAG */
    let mut left: size_t = buflen;
    let mut ret: libc::c_int = 0;
    let mut written: libc::c_int = 0;
    let mut child: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut iter: *mut hash_iterator_t = 0 as *mut hash_iterator_t;
    let mut key: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    written = 0 as libc::c_int;
    if (*stanza).type_0 as libc::c_uint ==
           XMPP_STANZA_UNKNOWN as libc::c_int as libc::c_uint {
        return -(2 as libc::c_int)
    }
    if (*stanza).type_0 as libc::c_uint ==
           XMPP_STANZA_TEXT as libc::c_int as libc::c_uint {
        if (*stanza).data.is_null() { return -(2 as libc::c_int) }
        tmp = _escape_xml((*stanza).ctx, (*stanza).data);
        if tmp.is_null() { return -(1 as libc::c_int) }
        ret =
            xmpp_snprintf(ptr, left,
                          b"%s\x00" as *const u8 as *const libc::c_char, tmp);
        xmpp_free((*stanza).ctx, tmp as *mut libc::c_void);
        if ret < 0 as libc::c_int { return -(1 as libc::c_int) }
        _render_update(&mut written, buflen as libc::c_int, ret, &mut left,
                       &mut ptr);
    } else {
        if (*stanza).data.is_null() { return -(2 as libc::c_int) }
        /* write beginning of tag and attributes */
        ret =
            xmpp_snprintf(ptr, left,
                          b"<%s\x00" as *const u8 as *const libc::c_char,
                          (*stanza).data);
        if ret < 0 as libc::c_int { return -(1 as libc::c_int) }
        _render_update(&mut written, buflen as libc::c_int, ret, &mut left,
                       &mut ptr);
        if !(*stanza).attributes.is_null() &&
               hash_num_keys((*stanza).attributes) > 0 as libc::c_int {
            iter = hash_iter_new((*stanza).attributes);
            loop  {
                key = hash_iter_next(iter);
                if key.is_null() { break ; }
                if strcmp(key,
                          b"xmlns\x00" as *const u8 as *const libc::c_char) ==
                       0 {
                    /* don't output namespace if parent stanza is the same */
                    if !(*stanza).parent.is_null() &&
                           !(*(*stanza).parent).attributes.is_null() &&
                           !hash_get((*(*stanza).parent).attributes,
                                     key).is_null() &&
                           strcmp(hash_get((*stanza).attributes, key) as
                                      *mut libc::c_char,
                                  hash_get((*(*stanza).parent).attributes,
                                           key) as *mut libc::c_char) == 0 {
                        continue ;
                    }
                    /* or if this is the stream namespace */
                    if (*stanza).parent.is_null() &&
                           strcmp(hash_get((*stanza).attributes, key) as
                                      *mut libc::c_char,
                                  b"jabber:client\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
                        continue ;
                    }
                }
                tmp =
                    _escape_xml((*stanza).ctx,
                                hash_get((*stanza).attributes, key) as
                                    *mut libc::c_char);
                if tmp.is_null() {
                    hash_iter_release(iter);
                    return -(1 as libc::c_int)
                }
                ret =
                    xmpp_snprintf(ptr, left,
                                  b" %s=\"%s\"\x00" as *const u8 as
                                      *const libc::c_char, key, tmp);
                xmpp_free((*stanza).ctx, tmp as *mut libc::c_void);
                if ret < 0 as libc::c_int {
                    hash_iter_release(iter);
                    return -(1 as libc::c_int)
                }
                _render_update(&mut written, buflen as libc::c_int, ret,
                               &mut left, &mut ptr);
            }
            hash_iter_release(iter);
        }
        if (*stanza).children.is_null() {
            /* write end if singleton tag */
            ret =
                xmpp_snprintf(ptr, left,
                              b"/>\x00" as *const u8 as *const libc::c_char);
            if ret < 0 as libc::c_int { return -(1 as libc::c_int) }
            _render_update(&mut written, buflen as libc::c_int, ret,
                           &mut left, &mut ptr);
        } else {
            /* this stanza has child stanzas */
            /* write end of start tag */
            ret =
                xmpp_snprintf(ptr, left,
                              b">\x00" as *const u8 as *const libc::c_char);
            if ret < 0 as libc::c_int { return -(1 as libc::c_int) }
            _render_update(&mut written, buflen as libc::c_int, ret,
                           &mut left, &mut ptr);
            /* iterate and recurse over child stanzas */
            child = (*stanza).children;
            while !child.is_null() {
                ret = _render_stanza_recursive(child, ptr, left);
                if ret < 0 as libc::c_int { return ret }
                _render_update(&mut written, buflen as libc::c_int, ret,
                               &mut left, &mut ptr);
                child = (*child).next
            }
            /* write end tag */
            ret =
                xmpp_snprintf(ptr, left,
                              b"</%s>\x00" as *const u8 as
                                  *const libc::c_char, (*stanza).data);
            if ret < 0 as libc::c_int { return -(1 as libc::c_int) }
            _render_update(&mut written, buflen as libc::c_int, ret,
                           &mut left, &mut ptr);
        }
    }
    return written;
}
/* marshall a stanza into text for transmission or display */
/* * Render a stanza object to text.
 *  This function renders a given stanza object, along with its
 *  children, to text.  The text is returned in an allocated,
 *  null-terminated buffer.  It starts by allocating a 1024 byte buffer
 *  and reallocates more memory if that is not large enough.
 *
 *  @param stanza a Strophe stanza object
 *  @param buf a reference to a string pointer
 *  @param buflen a reference to a size_t
 *
 *  @return 0 on success (XMPP_EOK), and a number less than 0 on failure
 *      (XMPP_EMEM, XMPP_EINVOP)
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_to_text(mut stanza: *mut xmpp_stanza_t,
                                             buf: *mut *mut libc::c_char,
                                             buflen: *mut size_t)
 -> libc::c_int {
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut length: size_t = 0;
    let mut ret: libc::c_int = 0;
    /* allocate a default sized buffer and attempt to render */
    length = 1024 as libc::c_int as size_t;
    buffer = xmpp_alloc((*stanza).ctx, length) as *mut libc::c_char;
    if buffer.is_null() {
        *buf = 0 as *mut libc::c_char;
        *buflen = 0 as libc::c_int as size_t;
        return -(1 as libc::c_int)
    }
    ret = _render_stanza_recursive(stanza, buffer, length);
    if ret < 0 as libc::c_int {
        xmpp_free((*stanza).ctx, buffer as *mut libc::c_void);
        *buf = 0 as *mut libc::c_char;
        *buflen = 0 as libc::c_int as size_t;
        return ret
    }
    if ret as size_t > length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
       {
        tmp =
            xmpp_realloc((*stanza).ctx, buffer as *mut libc::c_void,
                         (ret + 1 as libc::c_int) as size_t) as
                *mut libc::c_char;
        if tmp.is_null() {
            xmpp_free((*stanza).ctx, buffer as *mut libc::c_void);
            *buf = 0 as *mut libc::c_char;
            *buflen = 0 as libc::c_int as size_t;
            return -(1 as libc::c_int)
        }
        length = (ret + 1 as libc::c_int) as size_t;
        buffer = tmp;
        ret = _render_stanza_recursive(stanza, buffer, length);
        if ret as size_t >
               length.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            xmpp_free((*stanza).ctx, buffer as *mut libc::c_void);
            *buf = 0 as *mut libc::c_char;
            *buflen = 0 as libc::c_int as size_t;
            return -(1 as libc::c_int)
        }
    }
    *buffer.offset(length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as
                       isize) = 0 as libc::c_int as libc::c_char;
    *buf = buffer;
    *buflen = ret as size_t;
    return 0 as libc::c_int;
}
/* * Set the name of a stanza.
 *  
 *  @param stanza a Strophe stanza object
 *  @param name a string with the name of the stanza
 *
 *  @return XMPP_EOK on success, a number less than 0 on failure (XMPP_EMEM,
 *      XMPP_EINVOP)
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_set_name(mut stanza: *mut xmpp_stanza_t,
                                              name: *const libc::c_char)
 -> libc::c_int {
    if (*stanza).type_0 as libc::c_uint ==
           XMPP_STANZA_TEXT as libc::c_int as libc::c_uint {
        return -(2 as libc::c_int)
    }
    if !(*stanza).data.is_null() {
        xmpp_free((*stanza).ctx, (*stanza).data as *mut libc::c_void);
    }
    (*stanza).type_0 = XMPP_STANZA_TAG;
    (*stanza).data = xmpp_strdup((*stanza).ctx, name);
    return if (*stanza).data.is_null() {
               -(1 as libc::c_int)
           } else { 0 as libc::c_int };
}
/* * Get the stanza name.
 *  This function returns a pointer to the stanza name.  If the caller needs
 *  to store this data, it must make a copy.
 *
 *  @param stanza a Strophe stanza object
 *
 *  @return a string with the stanza name
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_get_name(stanza: *mut xmpp_stanza_t)
 -> *const libc::c_char {
    if (*stanza).type_0 as libc::c_uint ==
           XMPP_STANZA_TEXT as libc::c_int as libc::c_uint {
        return 0 as *const libc::c_char
    }
    return (*stanza).data;
}
/* * Count the attributes in a stanza object.
 *
 *  @param stanza a Strophe stanza object
 *
 *  @return the number of attributes for the stanza object
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_get_attribute_count(stanza:
                                                             *mut xmpp_stanza_t)
 -> libc::c_int {
    if (*stanza).attributes.is_null() { return 0 as libc::c_int }
    return hash_num_keys((*stanza).attributes);
}
/* * Get all attributes for a stanza object.
 *  This function populates the array with attributes from the stanza.  The
 *  attr array will be in the format:  attr[i] = attribute name, 
 *  attr[i+1] = attribute value.
 *
 *  @param stanza a Strophe stanza object
 *  @param attr the string array to populate
 *  @param attrlen the size of the array
 *
 *  @return the number of slots used in the array, which will be 2 times the 
 *      number of attributes in the stanza
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_get_attributes(stanza:
                                                        *mut xmpp_stanza_t,
                                                    mut attr:
                                                        *mut *const libc::c_char,
                                                    mut attrlen: libc::c_int)
 -> libc::c_int {
    let mut iter: *mut hash_iterator_t = 0 as *mut hash_iterator_t;
    let mut key: *const libc::c_char = 0 as *const libc::c_char;
    let mut num: libc::c_int = 0 as libc::c_int;
    if (*stanza).attributes.is_null() { return 0 as libc::c_int }
    iter = hash_iter_new((*stanza).attributes);
    loop  {
        key = hash_iter_next(iter);
        if !(!key.is_null() && attrlen != 0) { break ; }
        let fresh0 = num;
        num = num + 1;
        let ref mut fresh1 = *attr.offset(fresh0 as isize);
        *fresh1 = key;
        attrlen -= 1;
        if attrlen == 0 as libc::c_int { hash_iter_release(iter); return num }
        let fresh2 = num;
        num = num + 1;
        let ref mut fresh3 = *attr.offset(fresh2 as isize);
        *fresh3 = hash_get((*stanza).attributes, key) as *const libc::c_char;
        attrlen -= 1;
        if attrlen == 0 as libc::c_int { hash_iter_release(iter); return num }
    }
    hash_iter_release(iter);
    return num;
}
/* set_attribute adds/replaces attributes */
/* * Set an attribute for a stanza object.
 *  
 *  @param stanza a Strophe stanza object
 *  @param key a string with the attribute name
 *  @param value a string with the attribute value
 *
 *  @return XMPP_EOK (0) on success or a number less than 0 on failure
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_set_attribute(stanza: *mut xmpp_stanza_t,
                                                   key: *const libc::c_char,
                                                   value: *const libc::c_char)
 -> libc::c_int {
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rc: libc::c_int = 0;
    if (*stanza).type_0 as libc::c_uint !=
           XMPP_STANZA_TAG as libc::c_int as libc::c_uint {
        return -(2 as libc::c_int)
    }
    if (*stanza).attributes.is_null() {
        (*stanza).attributes =
            hash_new((*stanza).ctx, 8 as libc::c_int,
                     Some(xmpp_free as
                              unsafe extern "C" fn(_: *const xmpp_ctx_t,
                                                   _: *mut libc::c_void)
                                  -> ()));
        if (*stanza).attributes.is_null() { return -(1 as libc::c_int) }
    }
    val = xmpp_strdup((*stanza).ctx, value);
    if val.is_null() { return -(1 as libc::c_int) }
    rc = hash_add((*stanza).attributes, key, val as *mut libc::c_void);
    if rc < 0 as libc::c_int {
        xmpp_free((*stanza).ctx, val as *mut libc::c_void);
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
/* * Set the stanza namespace.
 *  This is a convenience function equivalent to calling:
 *  xmpp_stanza_set_attribute(stanza, "xmlns", ns);
 *
 *  @param stanza a Strophe stanza object
 *  @param ns a string with the namespace
 *
 *  @return XMPP_EOK (0) on success or a number less than 0 on failure
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_set_ns(stanza: *mut xmpp_stanza_t,
                                            ns: *const libc::c_char)
 -> libc::c_int {
    return xmpp_stanza_set_attribute(stanza,
                                     b"xmlns\x00" as *const u8 as
                                         *const libc::c_char, ns);
}
/* * Add a child stanza to a stanza object.
 *  If do_clone is TRUE, user keeps reference to the child stanza and must call
 *  xmpp_stanza_release() to release the reference. If do_clone is FALSE, user
 *  transfers ownership and must not neither call xmpp_stanza_release() for
 *  the child stanza nor use it.
 *
 *  @param stanza a Strophe stanza object
 *  @param child the child stanza object
 *  @param do_clone TRUE to increase ref count of child (default for
 *                  xmpp_stanza_add_child())
 *
 *  @return XMPP_EOK (0) on success or a number less than 0 on failure
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_add_child_ex(mut stanza:
                                                      *mut xmpp_stanza_t,
                                                  mut child:
                                                      *mut xmpp_stanza_t,
                                                  mut do_clone: libc::c_int)
 -> libc::c_int {
    let mut s: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    if do_clone != 0 {
        /* get a reference to the child */
        xmpp_stanza_clone(child);
    }
    (*child).parent = stanza;
    if (*stanza).children.is_null() {
        (*stanza).children = child
    } else {
        s = (*stanza).children;
        while !(*s).next.is_null() { s = (*s).next }
        (*s).next = child;
        (*child).prev = s
    }
    return 0 as libc::c_int;
}
/* * Add a child stanza to a stanza object.
 *  This function clones the child and appends it to the stanza object's
 *  children.
 *
 *  @param stanza a Strophe stanza object
 *  @param child the child stanza object
 *
 *  @return XMPP_EOK (0) on success or a number less than 0 on failure
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_add_child(mut stanza: *mut xmpp_stanza_t,
                                               mut child: *mut xmpp_stanza_t)
 -> libc::c_int {
    return xmpp_stanza_add_child_ex(stanza, child, 1 as libc::c_int);
}
/* * Set the text data for a text stanza.
 *  This function copies the text given and sets the stanza object's text to
 *  it.  Attempting to use this function on a stanza that has a name will
 *  fail with XMPP_EINVOP.  This function takes the text as a null-terminated
 *  string.
 *
 *  @param stanza a Strophe stanza object
 *  @param text a string with the text
 *
 *  @return XMPP_EOK (0) on success or a number less than zero on failure
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_set_text(mut stanza: *mut xmpp_stanza_t,
                                              text: *const libc::c_char)
 -> libc::c_int {
    if (*stanza).type_0 as libc::c_uint ==
           XMPP_STANZA_TAG as libc::c_int as libc::c_uint {
        return -(2 as libc::c_int)
    }
    (*stanza).type_0 = XMPP_STANZA_TEXT;
    if !(*stanza).data.is_null() {
        xmpp_free((*stanza).ctx, (*stanza).data as *mut libc::c_void);
    }
    (*stanza).data = xmpp_strdup((*stanza).ctx, text);
    return if (*stanza).data.is_null() {
               -(1 as libc::c_int)
           } else { 0 as libc::c_int };
}
/* * Set the text data for a text stanza.
 *  This function copies the text given and sets the stanza object's text to
 *  it.  Attempting to use this function on a stanza that has a name will
 *  fail with XMPP_EINVOP.  This function takes the text as buffer and a length
 *  as opposed to a null-terminated string.
 *
 *  @param stanza a Strophe stanza object
 *  @param text a buffer with the text
 *  @param size the length of the text
 *
 *  @return XMPP_EOK (0) on success and a number less than 0 on failure
 * 
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_set_text_with_size(mut stanza:
                                                            *mut xmpp_stanza_t,
                                                        text:
                                                            *const libc::c_char,
                                                        size: size_t)
 -> libc::c_int {
    if (*stanza).type_0 as libc::c_uint ==
           XMPP_STANZA_TAG as libc::c_int as libc::c_uint {
        return -(2 as libc::c_int)
    }
    (*stanza).type_0 = XMPP_STANZA_TEXT;
    if !(*stanza).data.is_null() {
        xmpp_free((*stanza).ctx, (*stanza).data as *mut libc::c_void);
    }
    (*stanza).data =
        xmpp_alloc((*stanza).ctx,
                   size.wrapping_add(1 as libc::c_int as libc::c_ulong)) as
            *mut libc::c_char;
    if (*stanza).data.is_null() { return -(1 as libc::c_int) }
    memcpy((*stanza).data as *mut libc::c_void, text as *const libc::c_void,
           size);
    *(*stanza).data.offset(size as isize) = 0 as libc::c_int as libc::c_char;
    return 0 as libc::c_int;
}
/* * Get the 'id' attribute of the stanza object.
 *  This is a convenience function equivalent to:
 *  xmpp_stanza_get_attribute(stanza, "id");
 *
 *  @param stanza a Strophe stanza object
 *
 *  @return a string with the 'id' attribute value
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_get_id(stanza: *mut xmpp_stanza_t)
 -> *const libc::c_char {
    return xmpp_stanza_get_attribute(stanza,
                                     b"id\x00" as *const u8 as
                                         *const libc::c_char);
}
/* common stanza helpers */
/* * Get the namespace attribute of the stanza object.
 *  This is a convenience function equivalent to:
 *  xmpp_stanza_get_attribute(stanza, "xmlns");
 *
 *  @param stanza a Strophe stanza object
 *
 *  @return a string with the 'xmlns' attribute value
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_get_ns(stanza: *mut xmpp_stanza_t)
 -> *const libc::c_char {
    return xmpp_stanza_get_attribute(stanza,
                                     b"xmlns\x00" as *const u8 as
                                         *const libc::c_char);
}
/* * Get the 'type' attribute of the stanza object.
 *  This is a convenience function equivalent to:
 *  xmpp_stanza_get_attribute(stanza, "type");
 *
 *  @param stanza a Strophe stanza object
 *
 *  @return a string with the 'type' attribute value
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_get_type(stanza: *mut xmpp_stanza_t)
 -> *const libc::c_char {
    return xmpp_stanza_get_attribute(stanza,
                                     b"type\x00" as *const u8 as
                                         *const libc::c_char);
}
/* * Get the 'to' attribute of the stanza object.
 *  This is a convenience function equivalent to:
 *  xmpp_stanza_get_attribute(stanza, "to");
 *
 *  @param stanza a Strophe stanza object
 *
 *  @return a string with the 'to' attribute value
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_get_to(stanza: *mut xmpp_stanza_t)
 -> *const libc::c_char {
    return xmpp_stanza_get_attribute(stanza,
                                     b"to\x00" as *const u8 as
                                         *const libc::c_char);
}
/* * Get the 'from' attribute of the stanza object.
 *  This is a convenience function equivalent to:
 *  xmpp_stanza_get_attribute(stanza, "from");
 *
 *  @param stanza a Strophe stanza object
 *
 *  @return a string with the 'from' attribute value
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_get_from(stanza: *mut xmpp_stanza_t)
 -> *const libc::c_char {
    return xmpp_stanza_get_attribute(stanza,
                                     b"from\x00" as *const u8 as
                                         *const libc::c_char);
}
/* * Get the first child of stanza with name.
 *  This function searches all the immediate children of stanza for a child
 *  stanza that matches the name.  The first matching child is returned.
 *  
 *  @param stanza a Strophe stanza object
 *  @param name a string with the name to match
 *
 *  @return the matching child stanza object or NULL if no match was found
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_get_child_by_name(stanza:
                                                           *mut xmpp_stanza_t,
                                                       name:
                                                           *const libc::c_char)
 -> *mut xmpp_stanza_t {
    let mut child: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    child = (*stanza).children;
    while !child.is_null() {
        if (*child).type_0 as libc::c_uint ==
               XMPP_STANZA_TAG as libc::c_int as libc::c_uint &&
               strcmp(name, xmpp_stanza_get_name(child)) == 0 as libc::c_int {
            break ;
        }
        child = (*child).next
    }
    return child;
}
/* * Get the first child of a stanza with a given namespace.
 *  This function searches all the immediate children of a stanza for a child
 *  stanza that matches the namespace provided.  The first matching child
 *  is returned.
 * 
 *  @param stanza a Strophe stanza object
 *  @param ns a string with the namespace to match
 *
 *  @return the matching child stanza object or NULL if no match was found
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_get_child_by_ns(stanza:
                                                         *mut xmpp_stanza_t,
                                                     ns: *const libc::c_char)
 -> *mut xmpp_stanza_t {
    let mut child: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut child_ns: *const libc::c_char = 0 as *const libc::c_char;
    child = (*stanza).children;
    while !child.is_null() {
        child_ns = xmpp_stanza_get_ns(child);
        if !child_ns.is_null() && strcmp(ns, child_ns) == 0 as libc::c_int {
            break ;
        }
        child = (*child).next
    }
    return child;
}
/* * Get the first child of stanza with name and a given namespace.
 *  This function searches all the immediate children of stanza for a child
 *  stanza that matches the name and namespace provided.
 *  The first matching child is returned.
 *
 *  @param stanza a Strophe stanza object
 *  @param name a string with the name to match
 *  @param ns a string with the namespace to match
 *
 *  @return the matching child stanza object or NULL if no match was found
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_get_child_by_name_and_ns(stanza:
                                                                  *mut xmpp_stanza_t,
                                                              name:
                                                                  *const libc::c_char,
                                                              ns:
                                                                  *const libc::c_char)
 -> *mut xmpp_stanza_t {
    let mut child: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut child_ns: *const libc::c_char = 0 as *const libc::c_char;
    child = (*stanza).children;
    while !child.is_null() {
        if (*child).type_0 as libc::c_uint ==
               XMPP_STANZA_TAG as libc::c_int as libc::c_uint &&
               strcmp(name, xmpp_stanza_get_name(child)) == 0 as libc::c_int {
            child_ns = xmpp_stanza_get_ns(child);
            if !child_ns.is_null() && strcmp(ns, child_ns) == 0 as libc::c_int
               {
                break ;
            }
        }
        child = (*child).next
    }
    return child;
}
/* * Get the list of children.
 *  This function returns the first child of the stanza object.  The rest
 *  of the children can be obtained by calling xmpp_stanza_get_next() to
 *  iterate over the siblings.
 *
 *  @param stanza a Strophe stanza object
 *
 *  @return the first child stanza or NULL if there are no children
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_get_children(stanza: *mut xmpp_stanza_t)
 -> *mut xmpp_stanza_t {
    return (*stanza).children;
}
/* * Get the next sibling of a stanza.
 *  
 *  @param stanza a Strophe stanza object
 *
 *  @return the next sibling stanza or NULL if there are no more siblings
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_get_next(stanza: *mut xmpp_stanza_t)
 -> *mut xmpp_stanza_t {
    return (*stanza).next;
}
/* concatenate all child text nodes.  this function
 * returns a string that must be freed by the caller */
/* * Get the text data for a text stanza.
 *  This function copies the text data from a stanza and returns the new
 *  allocated string.  The caller is responsible for freeing this string
 *  with xmpp_free().
 *
 *  @param stanza a Strophe stanza object
 *
 *  @return an allocated string with the text data
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_get_text(stanza: *mut xmpp_stanza_t)
 -> *mut libc::c_char {
    let mut len: size_t = 0;
    let mut clen: size_t = 0;
    let mut child: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*stanza).type_0 as libc::c_uint ==
           XMPP_STANZA_TEXT as libc::c_int as libc::c_uint {
        if !(*stanza).data.is_null() {
            return xmpp_strdup((*stanza).ctx, (*stanza).data)
        } else { return 0 as *mut libc::c_char }
    }
    len = 0 as libc::c_int as size_t;
    child = (*stanza).children;
    while !child.is_null() {
        if (*child).type_0 as libc::c_uint ==
               XMPP_STANZA_TEXT as libc::c_int as libc::c_uint {
            len =
                (len as libc::c_ulong).wrapping_add(strlen((*child).data)) as
                    size_t as size_t
        }
        child = (*child).next
    }
    if len == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_char
    }
    text =
        xmpp_alloc((*stanza).ctx,
                   len.wrapping_add(1 as libc::c_int as libc::c_ulong)) as
            *mut libc::c_char;
    if text.is_null() { return 0 as *mut libc::c_char }
    len = 0 as libc::c_int as size_t;
    child = (*stanza).children;
    while !child.is_null() {
        if (*child).type_0 as libc::c_uint ==
               XMPP_STANZA_TEXT as libc::c_int as libc::c_uint {
            clen = strlen((*child).data);
            memcpy(&mut *text.offset(len as isize) as *mut libc::c_char as
                       *mut libc::c_void,
                   (*child).data as *const libc::c_void, clen);
            len =
                (len as libc::c_ulong).wrapping_add(clen) as size_t as size_t
        }
        child = (*child).next
    }
    *text.offset(len as isize) = 0 as libc::c_int as libc::c_char;
    return text;
}
/* * Get the text data pointer for a text stanza.
 *  This function copies returns the raw pointer to the text data in the
 *  stanza.  This should only be used in very special cases where the 
 *  caller needs to translate the datatype as this will save a double
 *  allocation.  The caller should not hold onto this pointer, and is
 *  responsible for allocating a copy if it needs one.
 *
 *  @param stanza a Strophe stanza object
 *
 *  @return an string pointer to the data or NULL
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_get_text_ptr(stanza: *mut xmpp_stanza_t)
 -> *const libc::c_char {
    if (*stanza).type_0 as libc::c_uint ==
           XMPP_STANZA_TEXT as libc::c_int as libc::c_uint {
        return (*stanza).data
    }
    return 0 as *const libc::c_char;
}
/* * Set the 'id' attribute of a stanza.
 *
 *  This is a convenience function for:
 *  xmpp_stanza_set_attribute(stanza, 'id', id);
 *
 *  @param stanza a Strophe stanza object
 *  @param id a string containing the 'id' value
 *
 *  @return XMPP_EOK (0) on success or a number less than 0 on failure
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_set_id(stanza: *mut xmpp_stanza_t,
                                            id: *const libc::c_char)
 -> libc::c_int {
    return xmpp_stanza_set_attribute(stanza,
                                     b"id\x00" as *const u8 as
                                         *const libc::c_char, id);
}
/* * Set the 'type' attribute of a stanza.
 *  This is a convenience function for:
 *  xmpp_stanza_set_attribute(stanza, 'type', type);
 *
 *  @param stanza a Strophe stanza object
 *  @param type a string containing the 'type' value
 *
 *  @return XMPP_EOK (0) on success or a number less than 0 on failure
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_set_type(stanza: *mut xmpp_stanza_t,
                                              type_0: *const libc::c_char)
 -> libc::c_int {
    return xmpp_stanza_set_attribute(stanza,
                                     b"type\x00" as *const u8 as
                                         *const libc::c_char, type_0);
}
/* * Set the 'to' attribute of a stanza.
 *
 *  This is a convenience function for:
 *  xmpp_stanza_set_attribute(stanza, 'to', to);
 *
 *  @param stanza a Strophe stanza object
 *  @param to a string containing the 'to' value
 *
 *  @return XMPP_EOK (0) on success or a number less than 0 on failure
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_set_to(stanza: *mut xmpp_stanza_t,
                                            to: *const libc::c_char)
 -> libc::c_int {
    return xmpp_stanza_set_attribute(stanza,
                                     b"to\x00" as *const u8 as
                                         *const libc::c_char, to);
}
/* * Set the 'from' attribute of a stanza.
 *
 *  This is a convenience function for:
 *  xmpp_stanza_set_attribute(stanza, 'from', from);
 *
 *  @param stanza a Strophe stanza object
 *  @param from a string containing the 'from' value
 *
 *  @return XMPP_EOK (0) on success or a number less than 0 on failure
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_set_from(stanza: *mut xmpp_stanza_t,
                                              from: *const libc::c_char)
 -> libc::c_int {
    return xmpp_stanza_set_attribute(stanza,
                                     b"from\x00" as *const u8 as
                                         *const libc::c_char, from);
}
/* * Get an attribute from a stanza.
 *  This function returns a pointer to the attribute value.  If the caller
 *  wishes to save this value it must make its own copy.
 *
 *  @param stanza a Strophe stanza object
 *  @param name a string containing attribute name
 *
 *  @return a string with the attribute value or NULL on an error
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_get_attribute(stanza: *mut xmpp_stanza_t,
                                                   name: *const libc::c_char)
 -> *const libc::c_char {
    if (*stanza).type_0 as libc::c_uint !=
           XMPP_STANZA_TAG as libc::c_int as libc::c_uint {
        return 0 as *const libc::c_char
    }
    if (*stanza).attributes.is_null() { return 0 as *const libc::c_char }
    return hash_get((*stanza).attributes, name) as *const libc::c_char;
}
/* * Delete an attribute from a stanza.
 *
 *  @param stanza a Strophe stanza object
 *  @param name a string containing attribute name
 *
 *  @return XMPP_EOK (0) on success or a number less than 0 on failure
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_del_attribute(stanza: *mut xmpp_stanza_t,
                                                   name: *const libc::c_char)
 -> libc::c_int {
    if (*stanza).type_0 as libc::c_uint !=
           XMPP_STANZA_TAG as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int)
    }
    if (*stanza).attributes.is_null() { return -(1 as libc::c_int) }
    return hash_drop((*stanza).attributes, name);
}
/* allocate and initialize a stanza in reply to another */
/* * Create a stanza object in reply to another.
 *  This function makes a copy of a stanza object with the attribute “to” set
 *  its original “from”.
 *  The stanza will have a reference count of one, so the caller does not
 *  need to clone it.
 *
 *  @param stanza a Strophe stanza object
 *
 *  @return a new Strophe stanza object
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stanza_reply(stanza: *mut xmpp_stanza_t)
 -> *mut xmpp_stanza_t {
    let mut current_block: u64;
    let mut copy: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut from: *const libc::c_char = 0 as *const libc::c_char;
    let mut rc: libc::c_int = 0;
    from = xmpp_stanza_get_from(stanza);
    if !from.is_null() {
        copy = xmpp_stanza_new((*stanza).ctx);
        if !copy.is_null() {
            (*copy).type_0 = (*stanza).type_0;
            if !(*stanza).data.is_null() {
                (*copy).data = xmpp_strdup((*stanza).ctx, (*stanza).data);
                if (*copy).data.is_null() {
                    current_block = 1022329312037250113;
                } else { current_block = 11812396948646013369; }
            } else { current_block = 11812396948646013369; }
            match current_block {
                1022329312037250113 => { }
                _ => {
                    if !(*stanza).attributes.is_null() {
                        if _stanza_copy_attributes(copy, stanza) <
                               0 as libc::c_int {
                            current_block = 1022329312037250113;
                        } else { current_block = 2979737022853876585; }
                    } else { current_block = 2979737022853876585; }
                    match current_block {
                        1022329312037250113 => { }
                        _ => {
                            xmpp_stanza_del_attribute(copy,
                                                      b"to\x00" as *const u8
                                                          as
                                                          *const libc::c_char);
                            xmpp_stanza_del_attribute(copy,
                                                      b"from\x00" as *const u8
                                                          as
                                                          *const libc::c_char);
                            rc = xmpp_stanza_set_to(copy, from);
                            if !(rc != 0 as libc::c_int) { return copy }
                        }
                    }
                }
            }
        }
    }
    if !copy.is_null() { xmpp_stanza_release(copy); }
    return 0 as *mut xmpp_stanza_t;
}
unsafe extern "C" fn _stanza_new_with_attrs(mut ctx: *mut xmpp_ctx_t,
                                            name: *const libc::c_char,
                                            type_0: *const libc::c_char,
                                            id: *const libc::c_char,
                                            to: *const libc::c_char)
 -> *mut xmpp_stanza_t {
    let mut stanza: *mut xmpp_stanza_t = xmpp_stanza_new(ctx);
    let mut ret: libc::c_int = 0;
    if !stanza.is_null() {
        ret = xmpp_stanza_set_name(stanza, name);
        if ret == 0 as libc::c_int && !type_0.is_null() {
            ret = xmpp_stanza_set_type(stanza, type_0)
        }
        if ret == 0 as libc::c_int && !id.is_null() {
            ret = xmpp_stanza_set_id(stanza, id)
        }
        if ret == 0 as libc::c_int && !to.is_null() {
            ret = xmpp_stanza_set_to(stanza, to)
        }
        if ret != 0 as libc::c_int {
            xmpp_stanza_release(stanza);
            stanza = 0 as *mut xmpp_stanza_t
        }
    }
    return stanza;
}
/* stanza subclasses */
/* * Create a <message/> stanza object with given attributes.
 *  Attributes are optional and may be NULL.
 *
 *  @param ctx a Strophe context object
 *  @param type attribute 'type'
 *  @param to attribute 'to'
 *  @param id attribute 'id'
 *
 *  @return a new Strophe stanza object
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_message_new(mut ctx: *mut xmpp_ctx_t,
                                          type_0: *const libc::c_char,
                                          to: *const libc::c_char,
                                          id: *const libc::c_char)
 -> *mut xmpp_stanza_t {
    return _stanza_new_with_attrs(ctx,
                                  b"message\x00" as *const u8 as
                                      *const libc::c_char, type_0, id, to);
}
/* * Get text from <body/> child element.
 *  This function returns new allocated string. The caller is responsible
 *  for freeing this string with xmpp_free().
 *
 *  @param msg well formed <message/> stanza
 *
 *  @return allocated string or NULL on failure (no <body/> element or
 *      memory allocation error)
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_message_get_body(mut msg: *mut xmpp_stanza_t)
 -> *mut libc::c_char {
    let mut body: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    name = xmpp_stanza_get_name(msg);
    body =
        xmpp_stanza_get_child_by_name(msg,
                                      b"body\x00" as *const u8 as
                                          *const libc::c_char);
    if !name.is_null() &&
           strcmp(name, b"message\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int && !body.is_null() {
        text = xmpp_stanza_get_text(body)
    }
    return text;
}
/* * Add <body/> child element to a <message/> stanza with the given text.
 *
 *  @param msg a <message> stanza object without <body/> child element.
 *
 *  @return 0 on success (XMPP_EOK), and a number less than 0 on failure
 *      (XMPP_EMEM, XMPP_EINVOP)
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_message_set_body(mut msg: *mut xmpp_stanza_t,
                                               text: *const libc::c_char)
 -> libc::c_int {
    let mut ctx: *mut xmpp_ctx_t = (*msg).ctx;
    let mut body: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut text_stanza: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: libc::c_int = 0;
    /* check that msg is a <message/> stanza and doesn't contain <body/> */
    name = xmpp_stanza_get_name(msg);
    body =
        xmpp_stanza_get_child_by_name(msg,
                                      b"body\x00" as *const u8 as
                                          *const libc::c_char);
    if name.is_null() ||
           strcmp(name, b"message\x00" as *const u8 as *const libc::c_char) !=
               0 as libc::c_int || !body.is_null() {
        return -(2 as libc::c_int)
    }
    body = xmpp_stanza_new(ctx);
    text_stanza = xmpp_stanza_new(ctx);
    ret =
        if !body.is_null() && !text_stanza.is_null() {
            0 as libc::c_int
        } else { -(1 as libc::c_int) };
    if ret == 0 as libc::c_int {
        ret =
            xmpp_stanza_set_name(body,
                                 b"body\x00" as *const u8 as
                                     *const libc::c_char)
    }
    if ret == 0 as libc::c_int {
        ret = xmpp_stanza_set_text(text_stanza, text)
    }
    if ret == 0 as libc::c_int {
        ret = xmpp_stanza_add_child(body, text_stanza)
    }
    if ret == 0 as libc::c_int { ret = xmpp_stanza_add_child(msg, body) }
    if !text_stanza.is_null() { xmpp_stanza_release(text_stanza); }
    if !body.is_null() { xmpp_stanza_release(body); }
    return ret;
}
/* * Create an <iq/> stanza object with given attributes.
 *  Attributes are optional and may be NULL.
 *
 *  @param ctx a Strophe context object
 *  @param type attribute 'type'
 *  @param id attribute 'id'
 *
 *  @return a new Strophe stanza object
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_iq_new(mut ctx: *mut xmpp_ctx_t,
                                     type_0: *const libc::c_char,
                                     id: *const libc::c_char)
 -> *mut xmpp_stanza_t {
    return _stanza_new_with_attrs(ctx,
                                  b"iq\x00" as *const u8 as
                                      *const libc::c_char, type_0, id,
                                  0 as *const libc::c_char);
}
/* * Create a <presence/> stanza object.
 *
 *  @param ctx a Strophe context object
 *
 *  @return a new Strophe stanza object
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_presence_new(mut ctx: *mut xmpp_ctx_t)
 -> *mut xmpp_stanza_t {
    return _stanza_new_with_attrs(ctx,
                                  b"presence\x00" as *const u8 as
                                      *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char);
}
/* * Create an <stream:error/> stanza object with given type and error text.
 *  The error text is optional and may be NULL.
 *
 *  @param ctx a Strophe context object
 *  @param type enum of xmpp_error_type_t
 *  @param text content of a 'text'
 *
 *  @return a new Strophe stanza object
 *
 *  @todo Handle errors in this function
 *
 *  @ingroup Stanza
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_error_new(mut ctx: *mut xmpp_ctx_t,
                                        type_0: xmpp_error_type_t,
                                        text: *const libc::c_char)
 -> *mut xmpp_stanza_t {
    let mut error: *mut xmpp_stanza_t =
        _stanza_new_with_attrs(ctx,
                               b"stream:error\x00" as *const u8 as
                                   *const libc::c_char,
                               0 as *const libc::c_char,
                               0 as *const libc::c_char,
                               0 as *const libc::c_char);
    let mut error_type: *mut xmpp_stanza_t = xmpp_stanza_new(ctx);
    match type_0 as libc::c_uint {
        0 => {
            xmpp_stanza_set_name(error_type,
                                 b"bad-format\x00" as *const u8 as
                                     *const libc::c_char);
        }
        1 => {
            xmpp_stanza_set_name(error_type,
                                 b"bad-namespace-prefix\x00" as *const u8 as
                                     *const libc::c_char);
        }
        2 => {
            xmpp_stanza_set_name(error_type,
                                 b"conflict\x00" as *const u8 as
                                     *const libc::c_char);
        }
        3 => {
            xmpp_stanza_set_name(error_type,
                                 b"connection-timeout\x00" as *const u8 as
                                     *const libc::c_char);
        }
        4 => {
            xmpp_stanza_set_name(error_type,
                                 b"host-gone\x00" as *const u8 as
                                     *const libc::c_char);
        }
        5 => {
            xmpp_stanza_set_name(error_type,
                                 b"host-unknown\x00" as *const u8 as
                                     *const libc::c_char);
        }
        6 => {
            xmpp_stanza_set_name(error_type,
                                 b"improper-addressing\x00" as *const u8 as
                                     *const libc::c_char);
        }
        7 => {
            xmpp_stanza_set_name(error_type,
                                 b"internal-server-error\x00" as *const u8 as
                                     *const libc::c_char);
        }
        8 => {
            xmpp_stanza_set_name(error_type,
                                 b"invalid-from\x00" as *const u8 as
                                     *const libc::c_char);
        }
        9 => {
            xmpp_stanza_set_name(error_type,
                                 b"invalid-id\x00" as *const u8 as
                                     *const libc::c_char);
        }
        10 => {
            xmpp_stanza_set_name(error_type,
                                 b"invalid-namespace\x00" as *const u8 as
                                     *const libc::c_char);
        }
        11 => {
            xmpp_stanza_set_name(error_type,
                                 b"invalid-xml\x00" as *const u8 as
                                     *const libc::c_char);
        }
        12 => {
            xmpp_stanza_set_name(error_type,
                                 b"not-authorized\x00" as *const u8 as
                                     *const libc::c_char);
        }
        13 => {
            xmpp_stanza_set_name(error_type,
                                 b"policy-violation\x00" as *const u8 as
                                     *const libc::c_char);
        }
        14 => {
            xmpp_stanza_set_name(error_type,
                                 b"remote-connection-failed\x00" as *const u8
                                     as *const libc::c_char);
        }
        15 => {
            xmpp_stanza_set_name(error_type,
                                 b"resource-constraint\x00" as *const u8 as
                                     *const libc::c_char);
        }
        16 => {
            xmpp_stanza_set_name(error_type,
                                 b"restricted-xml\x00" as *const u8 as
                                     *const libc::c_char);
        }
        17 => {
            xmpp_stanza_set_name(error_type,
                                 b"see-other-host\x00" as *const u8 as
                                     *const libc::c_char);
        }
        18 => {
            xmpp_stanza_set_name(error_type,
                                 b"system-shutdown\x00" as *const u8 as
                                     *const libc::c_char);
        }
        19 => {
            xmpp_stanza_set_name(error_type,
                                 b"undefined-condition\x00" as *const u8 as
                                     *const libc::c_char);
        }
        20 => {
            xmpp_stanza_set_name(error_type,
                                 b"unsupported-encoding\x00" as *const u8 as
                                     *const libc::c_char);
        }
        21 => {
            xmpp_stanza_set_name(error_type,
                                 b"unsupported-stanza-type\x00" as *const u8
                                     as *const libc::c_char);
        }
        22 => {
            xmpp_stanza_set_name(error_type,
                                 b"unsupported-version\x00" as *const u8 as
                                     *const libc::c_char);
        }
        23 => {
            xmpp_stanza_set_name(error_type,
                                 b"xml-not-well-formed\x00" as *const u8 as
                                     *const libc::c_char);
        }
        _ => {
            xmpp_stanza_set_name(error_type,
                                 b"internal-server-error\x00" as *const u8 as
                                     *const libc::c_char);
        }
    }
    xmpp_stanza_set_ns(error_type,
                       b"urn:ietf:params:xml:ns:xmpp-streams\x00" as *const u8
                           as *const libc::c_char);
    xmpp_stanza_add_child(error, error_type);
    xmpp_stanza_release(error_type);
    if !text.is_null() {
        let mut error_text: *mut xmpp_stanza_t = xmpp_stanza_new(ctx);
        let mut content: *mut xmpp_stanza_t = xmpp_stanza_new(ctx);
        xmpp_stanza_set_name(error_text,
                             b"text\x00" as *const u8 as *const libc::c_char);
        xmpp_stanza_set_ns(error_text,
                           b"urn:ietf:params:xml:ns:xmpp-streams\x00" as
                               *const u8 as *const libc::c_char);
        xmpp_stanza_set_text(content, text);
        xmpp_stanza_add_child(error_text, content);
        xmpp_stanza_release(content);
        xmpp_stanza_add_child(error, error_text);
        xmpp_stanza_release(error_text);
    }
    return error;
}
