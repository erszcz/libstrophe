use ::libc;
extern "C" {
    pub type _parser_t;
    pub type _tls;
    pub type _xmpp_rand_t;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    /* free some blocks returned by other APIs, for example the
   buffer you get from xmpp_stanza_to_text */
    #[no_mangle]
    fn xmpp_free(ctx: *const xmpp_ctx_t, p: *mut libc::c_void);
    /* convenience functions for accessing the context */
    #[no_mangle]
    fn xmpp_alloc(ctx: *const xmpp_ctx_t, size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xmpp_strdup(ctx: *const xmpp_ctx_t, s: *const libc::c_char)
     -> *mut libc::c_char;
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
pub type hash_t = _hash_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _hash_t {
    pub ref_0: libc::c_uint,
    pub ctx: *mut xmpp_ctx_t,
    pub free: hash_free_func,
    pub length: libc::c_int,
    pub num_keys: libc::c_int,
    pub entries: *mut *mut hashentry_t,
}
/* hash.c
** strophe XMPP client library -- hash table implementation
** 
** Copyright (C) 2005-2009 Collecta, Inc. 
**
**  This software is provided AS-IS with no warranty, either express
**  or implied.
**
**  This program is dual licensed under the MIT and GPLv3 licenses.
*/
/* * @file
 *  Hash tables.
 */
/* private types */
pub type hashentry_t = _hashentry_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _hashentry_t {
    pub next: *mut hashentry_t,
    pub key: *mut libc::c_char,
    pub value: *mut libc::c_void,
}
pub type hash_free_func
    =
    Option<unsafe extern "C" fn(_: *const xmpp_ctx_t, _: *mut libc::c_void)
               -> ()>;
/* opaque run time context containing the above hooks */
pub type xmpp_ctx_t = _xmpp_ctx_t;
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
/* TCP keepalive timeout */
/* TCP keepalive interval */
/* set when tls fails, so we don't try again */
/* if true, field is a bitfield of supported 
                         mechanisms */
/* set when stream is secured with TLS */
/* if server returns <bind/> or <session/> we must do them */
/* send queue and parameters */
/* xml parser */
/* timeouts */
/* event handlers */
/* stream open handler */
/* user handlers only get called after authentication */
/* connection events handler */
/* other handlers */
pub type xmpp_stanza_type_t = libc::c_uint;
pub const XMPP_STANZA_TAG: xmpp_stanza_type_t = 2;
pub const XMPP_STANZA_TEXT: xmpp_stanza_type_t = 1;
pub const XMPP_STANZA_UNKNOWN: xmpp_stanza_type_t = 0;
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
/* common members */
/* handlers are added disabled and enabled after the
                  * handler chain is processed to prevent stanzas from
                  * getting processed by newly added handlers */
/* timed handlers */
/* id handlers */
/* normal handlers */
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
pub type hash_iterator_t = _hash_iterator_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _hash_iterator_t {
    pub ref_0: libc::c_uint,
    pub table: *mut hash_t,
    pub entry: *mut hashentry_t,
    pub index: libc::c_int,
}
/* * allocate and initialize a new hash table */
#[no_mangle]
pub unsafe extern "C" fn hash_new(ctx: *mut xmpp_ctx_t, size: libc::c_int,
                                  mut free_func: hash_free_func)
 -> *mut hash_t {
    let mut result: *mut hash_t = 0 as *mut hash_t;
    result =
        xmpp_alloc(ctx, ::std::mem::size_of::<hash_t>() as libc::c_ulong) as
            *mut hash_t;
    if !result.is_null() {
        (*result).entries =
            xmpp_alloc(ctx,
                       (size as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut hashentry_t>()
                                                            as libc::c_ulong))
                as *mut *mut hashentry_t;
        if (*result).entries.is_null() {
            xmpp_free(ctx, result as *mut libc::c_void);
            return 0 as *mut hash_t
        }
        memset((*result).entries as *mut libc::c_void, 0 as libc::c_int,
               (size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut hashentry_t>()
                                                    as libc::c_ulong));
        (*result).length = size;
        (*result).ctx = ctx;
        (*result).free = free_func;
        (*result).num_keys = 0 as libc::c_int;
        /* give the caller a reference */
        (*result).ref_0 = 1 as libc::c_int as libc::c_uint
    }
    return result;
}
/* * obtain a new reference to an existing hash table */
#[no_mangle]
pub unsafe extern "C" fn hash_clone(table: *mut hash_t) -> *mut hash_t {
    (*table).ref_0 = (*table).ref_0.wrapping_add(1);
    return table;
}
/* * release a hash table that is no longer needed */
#[no_mangle]
pub unsafe extern "C" fn hash_release(table: *mut hash_t) {
    let mut ctx: *mut xmpp_ctx_t = (*table).ctx;
    let mut entry: *mut hashentry_t = 0 as *mut hashentry_t;
    let mut next: *mut hashentry_t = 0 as *mut hashentry_t;
    let mut i: libc::c_int = 0;
    if (*table).ref_0 > 1 as libc::c_int as libc::c_uint {
        (*table).ref_0 = (*table).ref_0.wrapping_sub(1)
    } else {
        i = 0 as libc::c_int;
        while i < (*table).length {
            entry = *(*table).entries.offset(i as isize);
            while !entry.is_null() {
                next = (*entry).next;
                xmpp_free(ctx, (*entry).key as *mut libc::c_void);
                if (*table).free.is_some() {
                    (*table).free.expect("non-null function pointer")(ctx,
                                                                      (*entry).value);
                }
                xmpp_free(ctx, entry as *mut libc::c_void);
                entry = next
            }
            i += 1
        }
        xmpp_free(ctx, (*table).entries as *mut libc::c_void);
        xmpp_free(ctx, table as *mut libc::c_void);
    };
}
/* * hash a key for our table lookup */
unsafe extern "C" fn _hash_key(mut table: *mut hash_t,
                               mut key: *const libc::c_char) -> libc::c_int {
    let mut hash: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut shift: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut c: *const libc::c_uchar = key as *const libc::c_uchar;
    while *c as libc::c_int != 0 as libc::c_int {
        /* assume 32 bit ints */
        let fresh0 = c;
        c = c.offset(1);
        hash ^= (*fresh0 as libc::c_uint) << shift;
        shift = shift.wrapping_add(8 as libc::c_int as libc::c_uint);
        if shift > 24 as libc::c_int as libc::c_uint {
            shift = 0 as libc::c_int as libc::c_uint
        }
    }
    return hash.wrapping_rem((*table).length as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _hash_entry_find(mut table: *mut hash_t,
                                          mut key: *const libc::c_char)
 -> *mut hashentry_t {
    let mut entry: *mut hashentry_t = 0 as *mut hashentry_t;
    let mut table_index: libc::c_int = _hash_key(table, key);
    /* look up the hash entry */
    entry = *(*table).entries.offset(table_index as isize);
    while !entry.is_null() {
        /* traverse the linked list looking for the key */
        if strcmp(key, (*entry).key) == 0 { break ; }
        entry = (*entry).next
    }
    return entry;
}
/* * add a key, value pair to a hash table.
 *  each key can appear only once; the value of any
 *  identical key will be replaced
 */
#[no_mangle]
pub unsafe extern "C" fn hash_add(mut table: *mut hash_t,
                                  key: *const libc::c_char,
                                  mut data: *mut libc::c_void)
 -> libc::c_int {
    let mut ctx: *mut xmpp_ctx_t = (*table).ctx;
    let mut entry: *mut hashentry_t = 0 as *mut hashentry_t;
    let mut table_index: libc::c_int = _hash_key(table, key);
    /* find and replace existing entry, if any */
    entry = _hash_entry_find(table, key);
    if entry.is_null() {
        /* allocate and fill a new entry */
        entry =
            xmpp_alloc(ctx,
                       ::std::mem::size_of::<hashentry_t>() as libc::c_ulong)
                as *mut hashentry_t;
        if entry.is_null() { return -(1 as libc::c_int) }
        (*entry).key = xmpp_strdup(ctx, key);
        if (*entry).key.is_null() {
            xmpp_free(ctx, entry as *mut libc::c_void);
            return -(1 as libc::c_int)
        }
        /* insert ourselves in the linked list */
        (*entry).next = *(*table).entries.offset(table_index as isize);
        let ref mut fresh1 = *(*table).entries.offset(table_index as isize);
        *fresh1 = entry;
        (*table).num_keys += 1
    } else if (*table).free.is_some() {
        (*table).free.expect("non-null function pointer")(ctx,
                                                          (*entry).value);
    }
    (*entry).value = data;
    return 0 as libc::c_int;
}
/* * look up a key in a hash table */
#[no_mangle]
pub unsafe extern "C" fn hash_get(mut table: *mut hash_t,
                                  mut key: *const libc::c_char)
 -> *mut libc::c_void {
    let mut entry: *mut hashentry_t = 0 as *mut hashentry_t;
    entry = _hash_entry_find(table, key);
    return if entry.is_null() {
               0 as *mut libc::c_void
           } else { (*entry).value };
}
/* * delete a key from a hash table */
#[no_mangle]
pub unsafe extern "C" fn hash_drop(mut table: *mut hash_t,
                                   mut key: *const libc::c_char)
 -> libc::c_int {
    let mut ctx: *mut xmpp_ctx_t = (*table).ctx;
    let mut entry: *mut hashentry_t = 0 as *mut hashentry_t;
    let mut prev: *mut hashentry_t = 0 as *mut hashentry_t;
    let mut table_index: libc::c_int = _hash_key(table, key);
    /* look up the hash entry */
    entry = *(*table).entries.offset(table_index as isize);
    prev = 0 as *mut hashentry_t;
    while !entry.is_null() {
        /* traverse the linked list looking for the key */
        if strcmp(key, (*entry).key) == 0 {
            /* match, remove the entry */
            xmpp_free(ctx, (*entry).key as *mut libc::c_void);
            if (*table).free.is_some() {
                (*table).free.expect("non-null function pointer")(ctx,
                                                                  (*entry).value);
            }
            if prev.is_null() {
                let ref mut fresh2 =
                    *(*table).entries.offset(table_index as isize);
                *fresh2 = (*entry).next
            } else { (*prev).next = (*entry).next }
            xmpp_free(ctx, entry as *mut libc::c_void);
            (*table).num_keys -= 1;
            return 0 as libc::c_int
        }
        prev = entry;
        entry = (*entry).next
    }
    /* no match */
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn hash_num_keys(mut table: *mut hash_t)
 -> libc::c_int {
    return (*table).num_keys;
}
/* * allocate and initialize a new iterator */
#[no_mangle]
pub unsafe extern "C" fn hash_iter_new(mut table: *mut hash_t)
 -> *mut hash_iterator_t {
    let mut ctx: *mut xmpp_ctx_t = (*table).ctx;
    let mut iter: *mut hash_iterator_t = 0 as *mut hash_iterator_t;
    iter =
        xmpp_alloc(ctx,
                   ::std::mem::size_of::<hash_iterator_t>() as libc::c_ulong)
            as *mut hash_iterator_t;
    if !iter.is_null() {
        (*iter).ref_0 = 1 as libc::c_int as libc::c_uint;
        (*iter).table = hash_clone(table);
        (*iter).entry = 0 as *mut hashentry_t;
        (*iter).index = -(1 as libc::c_int)
    }
    return iter;
}
/* * release an iterator that is no longer needed */
#[no_mangle]
pub unsafe extern "C" fn hash_iter_release(mut iter: *mut hash_iterator_t) {
    let mut ctx: *mut xmpp_ctx_t = (*(*iter).table).ctx;
    (*iter).ref_0 = (*iter).ref_0.wrapping_sub(1);
    if (*iter).ref_0 == 0 as libc::c_int as libc::c_uint {
        // ref is unsigned!!!
        hash_release((*iter).table);
        xmpp_free(ctx, iter as *mut libc::c_void);
    };
}
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
/* * allocate and initialize a new hash table */
/* * allocate a new reference to an existing hash table */
/* * release a hash table when no longer needed */
/* * add a key, value pair to a hash table.
 *  each key can appear only once; the value of any
 *  identical key will be replaced
 */
/* * look up a key in a hash table */
/* * delete a key from a hash table */
/* * return the number of keys in a hash */
/* * hash key iterator functions */
/* * allocate and initialize a new iterator */
/* * release an iterator that is no longer needed */
/* * return the next hash table key from the iterator.
    the returned key should not be freed */
/* * return the next hash table key from the iterator.
    the returned key should not be freed */
#[no_mangle]
pub unsafe extern "C" fn hash_iter_next(mut iter: *mut hash_iterator_t)
 -> *const libc::c_char {
    let mut table: *mut hash_t = (*iter).table;
    let mut entry: *mut hashentry_t = (*iter).entry;
    let mut i: libc::c_int = 0;
    /* advance until we find the next entry */
    if !entry.is_null() { entry = (*entry).next }
    if entry.is_null() {
        /* we're off the end of list, search for a new entry */
        i = (*iter).index + 1 as libc::c_int;
        while i < (*(*iter).table).length {
            entry = *(*table).entries.offset(i as isize);
            if !entry.is_null() { (*iter).index = i; break ; } else { i += 1 }
        }
    }
    if entry.is_null() {
        /* no more keys! */
        return 0 as *const libc::c_char
    }
    /* remember our current match */
    (*iter).entry = entry;
    return (*entry).key;
}
