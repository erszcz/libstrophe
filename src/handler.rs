use ::libc;
extern "C" {
    pub type _hash_t;
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
    pub type _parser_t;
    pub type _tls;
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
    pub type _xmpp_rand_t;
    /* * return the number of keys in a hash */
    /* * hash key iterator functions */
    pub type _hash_iterator_t;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    /* free some blocks returned by other APIs, for example the
   buffer you get from xmpp_stanza_to_text */
    #[no_mangle]
    fn xmpp_free(ctx: *const xmpp_ctx_t, p: *mut libc::c_void);
    #[no_mangle]
    fn xmpp_stanza_get_child_by_ns(stanza: *mut xmpp_stanza_t,
                                   ns: *const libc::c_char)
     -> *mut xmpp_stanza_t;
    #[no_mangle]
    fn xmpp_stanza_get_name(stanza: *mut xmpp_stanza_t)
     -> *const libc::c_char;
    /* common stanza helpers */
    #[no_mangle]
    fn xmpp_stanza_get_ns(stanza: *mut xmpp_stanza_t) -> *const libc::c_char;
    #[no_mangle]
    fn xmpp_stanza_get_type(stanza: *mut xmpp_stanza_t)
     -> *const libc::c_char;
    #[no_mangle]
    fn xmpp_stanza_get_id(stanza: *mut xmpp_stanza_t) -> *const libc::c_char;
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
    #[no_mangle]
    fn hash_drop(table: *mut hash_t, key: *const libc::c_char) -> libc::c_int;
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
    /* convenience functions for accessing the context */
    #[no_mangle]
    fn xmpp_strdup(ctx: *const xmpp_ctx_t, s: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn hash_add(table: *mut hash_t, key: *const libc::c_char,
                data: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn xmpp_warn(ctx: *const xmpp_ctx_t, area: *const libc::c_char,
                 fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn xmpp_alloc(ctx: *const xmpp_ctx_t, size: size_t) -> *mut libc::c_void;
    /* util.h
** strophe XMPP client library -- various utility functions
**
** Copyright (C) 2005-2009 Collecta, Inc. 
**
**  This software is provided AS-IS with no warranty, either express
**  or implied.
**
**  This program is dual licensed under the MIT and GPLv3 licenses.
*/
    /* * @file
 *  Internally used utility functions.
 */
    /* TODO evaluate x and y only once */
    /* string functions */
    /* timing functions */
    #[no_mangle]
    fn time_stamp() -> uint64_t;
    #[no_mangle]
    fn hash_get(table: *mut hash_t, key: *const libc::c_char)
     -> *mut libc::c_void;
    /* * allocate and initialize a new iterator */
    #[no_mangle]
    fn hash_iter_new(table: *mut hash_t) -> *mut hash_iterator_t;
    /* * release an iterator that is no longer needed */
    #[no_mangle]
    fn hash_iter_release(iter: *mut hash_iterator_t);
    /* * return the next hash table key from the iterator.
    the returned key should not be freed */
    #[no_mangle]
    fn hash_iter_next(iter: *mut hash_iterator_t) -> *const libc::c_char;
    #[no_mangle]
    fn time_elapsed(t1: uint64_t, t2: uint64_t) -> uint64_t;
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
pub type xmpp_rand_t = _xmpp_rand_t;
/* handlers */
/* if the handle returns false it is removed */
pub type xmpp_timed_handler
    =
    Option<unsafe extern "C" fn(_: *mut xmpp_conn_t, _: *mut libc::c_void)
               -> libc::c_int>;
pub type hash_iterator_t = _hash_iterator_t;
/* if the handler returns false it is removed */
pub type xmpp_handler
    =
    Option<unsafe extern "C" fn(_: *mut xmpp_conn_t, _: *mut xmpp_stanza_t,
                                _: *mut libc::c_void) -> libc::c_int>;
/* handler.c
** strophe XMPP client library -- event handler management
**
** Copyright (C) 2005-2009 Collecta, Inc.
**
**  This software is provided AS-IS with no warranty, either express
**  or implied.
**
**  This program is dual licensed under the MIT and GPLv3 licenses.
*/
/* * @file
 *  Event handler management.
 */
/* * @defgroup Handlers Stanza and timed event handlers
 */
/* Remove item from the list pointed by head, but don't free it.
 * There can be a situation when user's handler deletes another handler which
 * is the previous in the list. handler_fire_stanza() and handler_fire_timed()
 * must handle this situation correctly. Current function helps to avoid
 * list corruption in described scenario.
 *
 * TODO Convert handler lists to double-linked lists. Current implementation
 * works for O(n).
 */
unsafe extern "C" fn _handler_item_remove(mut head: *mut *mut xmpp_handlist_t,
                                          mut item: *mut xmpp_handlist_t) {
    let mut i: *mut xmpp_handlist_t = *head;
    if i == item {
        *head = (*item).next
    } else if !i.is_null() {
        while !(*i).next.is_null() && (*i).next != item { i = (*i).next }
        if (*i).next == item { (*i).next = (*item).next }
    };
}
/* * Fire off all stanza handlers that match.
 *  This function is called internally by the event loop whenever stanzas
 *  are received from the XMPP server.
 *
 *  @param conn a Strophe connection object
 *  @param stanza a Strophe stanza object
 */
#[no_mangle]
pub unsafe extern "C" fn handler_fire_stanza(conn: *mut xmpp_conn_t,
                                             stanza: *mut xmpp_stanza_t) {
    let mut item: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    let mut next: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    let mut head: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    let mut head_old: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    let mut id: *const libc::c_char = 0 as *const libc::c_char;
    let mut ns: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut type_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: libc::c_int = 0;
    /* call id handlers */
    id = xmpp_stanza_get_id(stanza);
    if !id.is_null() {
        head = hash_get((*conn).id_handlers, id) as *mut xmpp_handlist_t;
        /* enable all added handlers */
        item = head;
        while !item.is_null() {
            (*item).enabled = 1 as libc::c_int;
            item = (*item).next
        }
        item = head;
        while !item.is_null() {
            /* don't fire user handlers until authentication succeeds and
               and skip newly added handlers */
            if (*item).user_handler != 0 && (*conn).authenticated == 0 ||
                   (*item).enabled == 0 {
                item = (*item).next
            } else {
                ret =
                    ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                       -> libc::c_int>,
                                            xmpp_handler>((*item).handler).expect("non-null function pointer")(conn,
                                                                                                               stanza,
                                                                                                               (*item).userdata);
                next = (*item).next;
                if ret == 0 {
                    /* handler is one-shot, so delete it */
                    head_old = head;
                    _handler_item_remove(&mut head, item);
                    if head != head_old {
                        /* replace old value */
                        hash_add((*conn).id_handlers, id,
                                 head as *mut libc::c_void);
                    }
                    xmpp_free((*conn).ctx,
                              (*item).u.c2rust_unnamed_0.id as
                                  *mut libc::c_void);
                    xmpp_free((*conn).ctx, item as *mut libc::c_void);
                }
                item = next
            }
        }
    }
    /* call handlers */
    ns = xmpp_stanza_get_ns(stanza);
    name = xmpp_stanza_get_name(stanza);
    type_0 = xmpp_stanza_get_type(stanza);
    /* enable all added handlers */
    item = (*conn).handlers;
    while !item.is_null() {
        (*item).enabled = 1 as libc::c_int;
        item = (*item).next
    }
    item = (*conn).handlers;
    while !item.is_null() {
        /* don't fire user handlers until authentication succeeds and
           skip newly added handlers */
        if (*item).user_handler != 0 && (*conn).authenticated == 0 ||
               (*item).enabled == 0 {
            item = (*item).next
        } else {
            next = (*item).next;
            if ((*item).u.c2rust_unnamed_1.ns.is_null() ||
                    !ns.is_null() &&
                        strcmp(ns, (*item).u.c2rust_unnamed_1.ns) ==
                            0 as libc::c_int ||
                    !xmpp_stanza_get_child_by_ns(stanza,
                                                 (*item).u.c2rust_unnamed_1.ns).is_null())
                   &&
                   ((*item).u.c2rust_unnamed_1.name.is_null() ||
                        !name.is_null() &&
                            strcmp(name, (*item).u.c2rust_unnamed_1.name) ==
                                0 as libc::c_int) &&
                   ((*item).u.c2rust_unnamed_1.type_0.is_null() ||
                        !type_0.is_null() &&
                            strcmp(type_0, (*item).u.c2rust_unnamed_1.type_0)
                                == 0 as libc::c_int) {
                ret =
                    ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                       -> libc::c_int>,
                                            xmpp_handler>((*item).handler).expect("non-null function pointer")(conn,
                                                                                                               stanza,
                                                                                                               (*item).userdata);
                /* list may be changed during execution of a handler */
                next = (*item).next;
                if ret == 0 {
                    /* handler is one-shot, so delete it */
                    _handler_item_remove(&mut (*conn).handlers, item);
                    if !(*item).u.c2rust_unnamed_1.ns.is_null() {
                        xmpp_free((*conn).ctx,
                                  (*item).u.c2rust_unnamed_1.ns as
                                      *mut libc::c_void);
                    }
                    if !(*item).u.c2rust_unnamed_1.name.is_null() {
                        xmpp_free((*conn).ctx,
                                  (*item).u.c2rust_unnamed_1.name as
                                      *mut libc::c_void);
                    }
                    if !(*item).u.c2rust_unnamed_1.type_0.is_null() {
                        xmpp_free((*conn).ctx,
                                  (*item).u.c2rust_unnamed_1.type_0 as
                                      *mut libc::c_void);
                    }
                    xmpp_free((*conn).ctx, item as *mut libc::c_void);
                }
            }
            item = next
        }
    };
}
/* * Fire off all timed handlers that are ready.
 *  This function is called internally by the event loop.
 *
 *  @param ctx a Strophe context object
 *
 *  @return the time in milliseconds until the next handler will be ready
 */
#[no_mangle]
pub unsafe extern "C" fn handler_fire_timed(ctx: *mut xmpp_ctx_t)
 -> uint64_t {
    let mut connitem: *mut xmpp_connlist_t = 0 as *mut xmpp_connlist_t;
    let mut item: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    let mut next: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    let mut conn: *mut xmpp_conn_t = 0 as *mut xmpp_conn_t;
    let mut elapsed: uint64_t = 0;
    let mut min: uint64_t = 0;
    let mut timestamp: uint64_t = 0;
    let mut ret: libc::c_int = 0;
    min = -(1 as libc::c_int) as uint64_t;
    connitem = (*ctx).connlist;
    while !connitem.is_null() {
        conn = (*connitem).conn;
        if (*conn).state as libc::c_uint !=
               XMPP_STATE_CONNECTED as libc::c_int as libc::c_uint {
            connitem = (*connitem).next
        } else {
            /* enable all handlers that were added */
            item = (*conn).timed_handlers;
            while !item.is_null() {
                (*item).enabled = 1 as libc::c_int;
                item = (*item).next
            }
            item = (*conn).timed_handlers;
            while !item.is_null() {
                /* don't fire user handlers until authentication succeeds and
               skip newly added handlers */
                if (*item).user_handler != 0 && (*conn).authenticated == 0 ||
                       (*item).enabled == 0 {
                    item = (*item).next
                } else {
                    next = (*item).next;
                    timestamp = time_stamp();
                    elapsed =
                        time_elapsed((*item).u.c2rust_unnamed.last_stamp,
                                     timestamp);
                    if elapsed >= (*item).u.c2rust_unnamed.period {
                        /* fire! */
                        (*item).u.c2rust_unnamed.last_stamp = timestamp;
                        ret =
                            ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                               ->
                                                                   libc::c_int>,
                                                    xmpp_timed_handler>((*item).handler).expect("non-null function pointer")(conn,
                                                                                                                             (*item).userdata);
                        /* list may be changed during execution of a handler */
                        next = (*item).next;
                        if ret == 0 {
                            /* delete handler if it returned false */
                            _handler_item_remove(&mut (*conn).timed_handlers,
                                                 item);
                            xmpp_free((*conn).ctx, item as *mut libc::c_void);
                        }
                    } else if min >
                                  (*item).u.c2rust_unnamed.period.wrapping_sub(elapsed)
                     {
                        min =
                            (*item).u.c2rust_unnamed.period.wrapping_sub(elapsed)
                    }
                    item = next
                }
            }
            connitem = (*connitem).next
        }
    }
    return min;
}
/* * Reset all timed handlers.
 *  This function is called internally when a connection is successful.
 *
 *  @param conn a Strophe connection object
 *  @param user_only whether to reset all handlers or only user ones
 */
#[no_mangle]
pub unsafe extern "C" fn handler_reset_timed(mut conn: *mut xmpp_conn_t,
                                             mut user_only: libc::c_int) {
    let mut handitem: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    handitem = (*conn).timed_handlers;
    while !handitem.is_null() {
        if user_only != 0 && (*handitem).user_handler != 0 || user_only == 0 {
            (*handitem).u.c2rust_unnamed.last_stamp = time_stamp()
        }
        handitem = (*handitem).next
    };
}
unsafe extern "C" fn _timed_handler_add(conn: *mut xmpp_conn_t,
                                        mut handler: xmpp_timed_handler,
                                        period: libc::c_ulong,
                                        userdata: *mut libc::c_void,
                                        user_handler: libc::c_int) {
    let mut item: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    let mut tail: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    /* check if handler is already in the list */
    item = (*conn).timed_handlers;
    while !item.is_null() {
        if (*item).handler ==
               ::std::mem::transmute::<xmpp_timed_handler,
                                       Option<unsafe extern "C" fn()
                                                  -> libc::c_int>>(handler) &&
               (*item).userdata == userdata {
            xmpp_warn((*conn).ctx,
                      b"xmpp\x00" as *const u8 as *const libc::c_char,
                      b"Timed handler already exists.\x00" as *const u8 as
                          *const libc::c_char);
            break ;
        } else { item = (*item).next }
    }
    if !item.is_null() { return }
    /* build new item */
    item =
        xmpp_alloc((*conn).ctx,
                   ::std::mem::size_of::<xmpp_handlist_t>() as libc::c_ulong)
            as *mut xmpp_handlist_t;
    if item.is_null() { return }
    (*item).user_handler = user_handler;
    (*item).handler =
        ::std::mem::transmute::<xmpp_timed_handler,
                                Option<unsafe extern "C" fn()
                                           -> libc::c_int>>(handler);
    (*item).userdata = userdata;
    (*item).enabled = 0 as libc::c_int;
    (*item).next = 0 as *mut xmpp_handlist_t;
    (*item).u.c2rust_unnamed.period = period;
    (*item).u.c2rust_unnamed.last_stamp = time_stamp();
    /* append item to list */
    if (*conn).timed_handlers.is_null() {
        (*conn).timed_handlers = item
    } else {
        tail = (*conn).timed_handlers;
        while !(*tail).next.is_null() { tail = (*tail).next }
        (*tail).next = item
    };
}
/* * Delete a timed handler.
 *
 *  @param conn a Strophe connection object
 *  @param handler function pointer to the handler
 *
 *  @ingroup Handlers
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_timed_handler_delete(conn: *mut xmpp_conn_t,
                                                   mut handler:
                                                       xmpp_timed_handler) {
    let mut item: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    let mut prev: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    if (*conn).timed_handlers.is_null() { return }
    prev = 0 as *mut xmpp_handlist_t;
    item = (*conn).timed_handlers;
    while !item.is_null() {
        if (*item).handler ==
               ::std::mem::transmute::<xmpp_timed_handler,
                                       Option<unsafe extern "C" fn()
                                                  -> libc::c_int>>(handler) {
            if !prev.is_null() {
                (*prev).next = (*item).next
            } else { (*conn).timed_handlers = (*item).next }
            xmpp_free((*conn).ctx, item as *mut libc::c_void);
            item =
                if !prev.is_null() {
                    (*prev).next
                } else { (*conn).timed_handlers }
        } else { prev = item; item = (*item).next }
    };
}
unsafe extern "C" fn _id_handler_add(conn: *mut xmpp_conn_t,
                                     mut handler: xmpp_handler,
                                     id: *const libc::c_char,
                                     userdata: *mut libc::c_void,
                                     mut user_handler: libc::c_int) {
    let mut item: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    let mut tail: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    /* check if handler is already in the list */
    item = hash_get((*conn).id_handlers, id) as *mut xmpp_handlist_t;
    while !item.is_null() {
        if (*item).handler ==
               ::std::mem::transmute::<xmpp_handler,
                                       Option<unsafe extern "C" fn()
                                                  -> libc::c_int>>(handler) &&
               (*item).userdata == userdata {
            xmpp_warn((*conn).ctx,
                      b"xmpp\x00" as *const u8 as *const libc::c_char,
                      b"Id handler already exists.\x00" as *const u8 as
                          *const libc::c_char);
            break ;
        } else { item = (*item).next }
    }
    if !item.is_null() { return }
    /* build new item */
    item =
        xmpp_alloc((*conn).ctx,
                   ::std::mem::size_of::<xmpp_handlist_t>() as libc::c_ulong)
            as *mut xmpp_handlist_t;
    if item.is_null() { return }
    (*item).user_handler = user_handler;
    (*item).handler =
        ::std::mem::transmute::<xmpp_handler,
                                Option<unsafe extern "C" fn()
                                           -> libc::c_int>>(handler);
    (*item).userdata = userdata;
    (*item).enabled = 0 as libc::c_int;
    (*item).next = 0 as *mut xmpp_handlist_t;
    (*item).u.c2rust_unnamed_0.id = xmpp_strdup((*conn).ctx, id);
    if (*item).u.c2rust_unnamed_0.id.is_null() {
        xmpp_free((*conn).ctx, item as *mut libc::c_void);
        return
    }
    /* put on list in hash table */
    tail = hash_get((*conn).id_handlers, id) as *mut xmpp_handlist_t;
    if tail.is_null() {
        hash_add((*conn).id_handlers, id, item as *mut libc::c_void);
    } else {
        while !(*tail).next.is_null() { tail = (*tail).next }
        (*tail).next = item
    };
}
/* * Delete an id based stanza handler.
 *
 *  @param conn a Strophe connection object
 *  @param handler a function pointer to a stanza handler
 *  @param id a string containing the id the handler is for
 *
 *  @ingroup Handlers
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_id_handler_delete(conn: *mut xmpp_conn_t,
                                                mut handler: xmpp_handler,
                                                id: *const libc::c_char) {
    let mut item: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    let mut prev: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    let mut next: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    prev = 0 as *mut xmpp_handlist_t;
    item = hash_get((*conn).id_handlers, id) as *mut xmpp_handlist_t;
    if item.is_null() { return }
    while !item.is_null() {
        next = (*item).next;
        if (*item).handler ==
               ::std::mem::transmute::<xmpp_handler,
                                       Option<unsafe extern "C" fn()
                                                  -> libc::c_int>>(handler) {
            if !prev.is_null() {
                (*prev).next = next
            } else {
                hash_drop((*conn).id_handlers, id);
                hash_add((*conn).id_handlers, id, next as *mut libc::c_void);
            }
            xmpp_free((*conn).ctx,
                      (*item).u.c2rust_unnamed_0.id as *mut libc::c_void);
            xmpp_free((*conn).ctx, item as *mut libc::c_void);
            item = next
        } else { prev = item; item = next }
    };
}
/* add a stanza handler */
unsafe extern "C" fn _handler_add(conn: *mut xmpp_conn_t,
                                  mut handler: xmpp_handler,
                                  ns: *const libc::c_char,
                                  name: *const libc::c_char,
                                  type_0: *const libc::c_char,
                                  userdata: *mut libc::c_void,
                                  mut user_handler: libc::c_int) {
    let mut item: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    let mut tail: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    /* check if handler already in list */
    item = (*conn).handlers;
    while !item.is_null() {
        /* same handler function can process different stanzas and
           distinguish them according to userdata. */
        if (*item).handler ==
               ::std::mem::transmute::<xmpp_handler,
                                       Option<unsafe extern "C" fn()
                                                  -> libc::c_int>>(handler) &&
               (*item).userdata == userdata {
            xmpp_warn((*conn).ctx,
                      b"xmpp\x00" as *const u8 as *const libc::c_char,
                      b"Stanza handler already exists.\x00" as *const u8 as
                          *const libc::c_char);
            break ;
        } else { item = (*item).next }
    }
    if !item.is_null() { return }
    /* build new item */
    item =
        xmpp_alloc((*conn).ctx,
                   ::std::mem::size_of::<xmpp_handlist_t>() as libc::c_ulong)
            as *mut xmpp_handlist_t;
    if item.is_null() { return }
    (*item).user_handler = user_handler;
    (*item).handler =
        ::std::mem::transmute::<xmpp_handler,
                                Option<unsafe extern "C" fn()
                                           -> libc::c_int>>(handler);
    (*item).userdata = userdata;
    (*item).enabled = 0 as libc::c_int;
    (*item).next = 0 as *mut xmpp_handlist_t;
    if !ns.is_null() {
        (*item).u.c2rust_unnamed_1.ns = xmpp_strdup((*conn).ctx, ns);
        if (*item).u.c2rust_unnamed_1.ns.is_null() {
            xmpp_free((*conn).ctx, item as *mut libc::c_void);
            return
        }
    } else { (*item).u.c2rust_unnamed_1.ns = 0 as *mut libc::c_char }
    if !name.is_null() {
        (*item).u.c2rust_unnamed_1.name = xmpp_strdup((*conn).ctx, name);
        if (*item).u.c2rust_unnamed_1.name.is_null() {
            if !(*item).u.c2rust_unnamed_1.ns.is_null() {
                xmpp_free((*conn).ctx,
                          (*item).u.c2rust_unnamed_1.ns as *mut libc::c_void);
            }
            xmpp_free((*conn).ctx, item as *mut libc::c_void);
            return
        }
    } else { (*item).u.c2rust_unnamed_1.name = 0 as *mut libc::c_char }
    if !type_0.is_null() {
        (*item).u.c2rust_unnamed_1.type_0 = xmpp_strdup((*conn).ctx, type_0);
        if (*item).u.c2rust_unnamed_1.type_0.is_null() {
            if !(*item).u.c2rust_unnamed_1.ns.is_null() {
                xmpp_free((*conn).ctx,
                          (*item).u.c2rust_unnamed_1.ns as *mut libc::c_void);
            }
            if !(*item).u.c2rust_unnamed_1.name.is_null() {
                xmpp_free((*conn).ctx,
                          (*item).u.c2rust_unnamed_1.name as
                              *mut libc::c_void);
            }
            xmpp_free((*conn).ctx, item as *mut libc::c_void);
        }
    } else { (*item).u.c2rust_unnamed_1.type_0 = 0 as *mut libc::c_char }
    /* append to list */
    if (*conn).handlers.is_null() {
        (*conn).handlers = item
    } else {
        tail = (*conn).handlers;
        while !(*tail).next.is_null() { tail = (*tail).next }
        (*tail).next = item
    };
}
/* * Delete a stanza handler.
 *
 *  @param conn a Strophe connection object
 *  @param handler a function pointer to a stanza handler
 *
 *  @ingroup Handlers
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_handler_delete(conn: *mut xmpp_conn_t,
                                             mut handler: xmpp_handler) {
    let mut prev: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    let mut item: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    if (*conn).handlers.is_null() { return }
    prev = 0 as *mut xmpp_handlist_t;
    item = (*conn).handlers;
    while !item.is_null() {
        if (*item).handler ==
               ::std::mem::transmute::<xmpp_handler,
                                       Option<unsafe extern "C" fn()
                                                  -> libc::c_int>>(handler) {
            if !prev.is_null() {
                (*prev).next = (*item).next
            } else { (*conn).handlers = (*item).next }
            if !(*item).u.c2rust_unnamed_1.ns.is_null() {
                xmpp_free((*conn).ctx,
                          (*item).u.c2rust_unnamed_1.ns as *mut libc::c_void);
            }
            if !(*item).u.c2rust_unnamed_1.name.is_null() {
                xmpp_free((*conn).ctx,
                          (*item).u.c2rust_unnamed_1.name as
                              *mut libc::c_void);
            }
            if !(*item).u.c2rust_unnamed_1.type_0.is_null() {
                xmpp_free((*conn).ctx,
                          (*item).u.c2rust_unnamed_1.type_0 as
                              *mut libc::c_void);
            }
            xmpp_free((*conn).ctx, item as *mut libc::c_void);
            item =
                if !prev.is_null() { (*prev).next } else { (*conn).handlers }
        } else { prev = item; item = (*item).next }
    };
}
/* * Add a timed handler.
 *  The handler will fire for the first time once the period has elapsed,
 *  and continue firing regularly after that.  Strophe will try its best
 *  to fire handlers as close to the period times as it can, but accuracy
 *  will vary depending on the resolution of the event loop.
 *
 *  If the handler function returns true, it will be kept, and if it
 *  returns false, it will be deleted from the list of handlers.
 *
 *  @param conn a Strophe connection object
 *  @param handler a function pointer to a timed handler
 *  @param period the time in milliseconds between firings
 *  @param userdata an opaque data pointer that will be passed to the handler
 *
 *  @ingroup Handlers
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_timed_handler_add(conn: *mut xmpp_conn_t,
                                                mut handler:
                                                    xmpp_timed_handler,
                                                period: libc::c_ulong,
                                                userdata: *mut libc::c_void) {
    _timed_handler_add(conn, handler, period, userdata, 1 as libc::c_int);
}
/* * Add a timed system handler.
 *  This function is used to add internal timed handlers and should not be
 *  used outside of the library.
 *
 *  @param conn a Strophe connection object
 *  @param handler a function pointer to a timed handler
 *  @param period the time in milliseconds between firings
 *  @param userdata an opaque data pointer that will be passed to the handler
 */
#[no_mangle]
pub unsafe extern "C" fn handler_add_timed(conn: *mut xmpp_conn_t,
                                           mut handler: xmpp_timed_handler,
                                           period: libc::c_ulong,
                                           userdata: *mut libc::c_void) {
    _timed_handler_add(conn, handler, period, userdata, 0 as libc::c_int);
}
/* * Add an id based stanza handler.

 *  This function adds a stanza handler for an &lt;iq/&gt; stanza of
 *  type 'result' or 'error' with a specific id attribute.  This can
 *  be used to handle responses to specific &lt;iq/&gt;s.
 *
 *  If the handler function returns true, it will be kept, and if it
 *  returns false, it will be deleted from the list of handlers.
 *
 *  @param conn a Strophe connection object
 *  @param handler a function pointer to a stanza handler
 *  @param id a string with the id
 *  @param userdata an opaque data pointer that will be passed to the handler
 *
 *  @ingroup Handlers
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_id_handler_add(conn: *mut xmpp_conn_t,
                                             mut handler: xmpp_handler,
                                             id: *const libc::c_char,
                                             userdata: *mut libc::c_void) {
    _id_handler_add(conn, handler, id, userdata, 1 as libc::c_int);
}
/* * Add an id based system stanza handler.
 *  This function is used to add internal id based stanza handlers and should
 *  not be used outside of the library.
 *
 *  @param conn a Strophe connection object
 *  @param handler a function pointer to a stanza handler
 *  @param id a string with the id
 *  @param userdata an opaque data pointer that will be passed to the handler
 */
#[no_mangle]
pub unsafe extern "C" fn handler_add_id(conn: *mut xmpp_conn_t,
                                        mut handler: xmpp_handler,
                                        id: *const libc::c_char,
                                        userdata: *mut libc::c_void) {
    _id_handler_add(conn, handler, id, userdata, 0 as libc::c_int);
}
/* * Add a stanza handler.
 *  This function is used to add a stanza handler to a connection.
 *  The handler will be called when the any of the filters match.  The
 *  name filter matches to the top level stanza name.  The type filter
 *  matches the 'type' attribute of the top level stanza.  The ns
 *  filter matches the namespace ('xmlns' attribute) of either the top
 *  level stanza or any of it's immediate children (this allows you do
 *  handle specific &lt;iq/&gt; stanzas based on the &lt;query/&gt;
 *  child namespace.
 *
 *  If the handler function returns true, it will be kept, and if it
 *  returns false, it will be deleted from the list of handlers.
 *
 *  @param conn a Strophe connection object
 *  @param handler a function pointer to a stanza handler
 *  @param ns a string with the namespace to match
 *  @param name a string with the stanza name to match
 *  @param type a string with the 'type' attribute to match
 *  @param userdata an opaque data pointer that will be passed to the handler
 *
 *  @ingroup Handlers
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_handler_add(conn: *mut xmpp_conn_t,
                                          mut handler: xmpp_handler,
                                          ns: *const libc::c_char,
                                          name: *const libc::c_char,
                                          type_0: *const libc::c_char,
                                          userdata: *mut libc::c_void) {
    _handler_add(conn, handler, ns, name, type_0, userdata, 1 as libc::c_int);
}
/* * Add a system stanza handler.
 *  This function is used to add internal stanza handlers and should
 *  not be used outside of the library.
 *
 *  @param conn a Strophe connection object
 *  @param handler a function pointer to a stanza handler
 *  @param ns a string with the namespace to match
 *  @param name a string with the stanza name to match
 *  @param type a string with the 'type' attribute value to match
 *  @param userdata an opaque data pointer that will be passed to the handler
 */
#[no_mangle]
pub unsafe extern "C" fn handler_add(conn: *mut xmpp_conn_t,
                                     mut handler: xmpp_handler,
                                     ns: *const libc::c_char,
                                     name: *const libc::c_char,
                                     type_0: *const libc::c_char,
                                     userdata: *mut libc::c_void) {
    _handler_add(conn, handler, ns, name, type_0, userdata, 0 as libc::c_int);
}
/* wrappers for xmpp_log at specific levels */
/* * connection **/
/* opaque connection object */
/* common members */
/* handlers are added disabled and enabled after the
                  * handler chain is processed to prevent stanzas from
                  * getting processed by newly added handlers */
/* timed handlers */
/* id handlers */
/* normal handlers */
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
/* handler management */
/* * Delete all system handlers.
 *  This function is used to reset conn object before re-connecting.
 *
 *  @param conn a Strophe connection object
 */
#[no_mangle]
pub unsafe extern "C" fn handler_system_delete_all(mut conn:
                                                       *mut xmpp_conn_t) {
    let mut item: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    let mut next: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    let mut head: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    let mut head_old: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    let mut iter: *mut hash_iterator_t = 0 as *mut hash_iterator_t;
    let mut key: *const libc::c_char = 0 as *const libc::c_char;
    let mut key2: *mut libc::c_char = 0 as *mut libc::c_char;
    /* TODO unify all kinds of handlers and avoid copy-paste below */
    item = (*conn).handlers;
    while !item.is_null() {
        if (*item).user_handler == 0 {
            next = (*item).next;
            _handler_item_remove(&mut (*conn).handlers, item);
            if !(*item).u.c2rust_unnamed_1.ns.is_null() {
                xmpp_free((*conn).ctx,
                          (*item).u.c2rust_unnamed_1.ns as *mut libc::c_void);
            }
            if !(*item).u.c2rust_unnamed_1.name.is_null() {
                xmpp_free((*conn).ctx,
                          (*item).u.c2rust_unnamed_1.name as
                              *mut libc::c_void);
            }
            if !(*item).u.c2rust_unnamed_1.type_0.is_null() {
                xmpp_free((*conn).ctx,
                          (*item).u.c2rust_unnamed_1.type_0 as
                              *mut libc::c_void);
            }
            xmpp_free((*conn).ctx, item as *mut libc::c_void);
            item = next
        } else { item = (*item).next }
    }
    item = (*conn).timed_handlers;
    while !item.is_null() {
        if (*item).user_handler == 0 {
            next = (*item).next;
            _handler_item_remove(&mut (*conn).timed_handlers, item);
            xmpp_free((*conn).ctx, item as *mut libc::c_void);
            item = next
        } else { item = (*item).next }
    }
    iter = hash_iter_new((*conn).id_handlers);
    key =
        if iter.is_null() {
            0 as *const libc::c_char
        } else { hash_iter_next(iter) };
    while !key.is_null() {
        head_old = hash_get((*conn).id_handlers, key) as *mut xmpp_handlist_t;
        head = head_old;
        item = head;
        while !item.is_null() {
            if (*item).user_handler == 0 {
                next = (*item).next;
                _handler_item_remove(&mut head, item);
                xmpp_free((*conn).ctx,
                          (*item).u.c2rust_unnamed_0.id as *mut libc::c_void);
                xmpp_free((*conn).ctx, item as *mut libc::c_void);
                item = next
            } else { item = (*item).next }
        }
        if head != head_old { key2 = xmpp_strdup((*conn).ctx, key) }
        /* Hash table implementation is not perfect, so we need to find next
           key before dropping current one. Otherwise, we will get access to
           freed memory. */
        key = hash_iter_next(iter);
        if head != head_old {
            /* hash_add() replaces value if the key exists */
            if !head.is_null() {
                hash_add((*conn).id_handlers, key2,
                         head as *mut libc::c_void);
            } else { hash_drop((*conn).id_handlers, key2); }
            xmpp_free((*conn).ctx, key2 as *mut libc::c_void);
        }
    }
    if !iter.is_null() { hash_iter_release(iter); };
}
