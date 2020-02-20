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
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /* free some blocks returned by other APIs, for example the
   buffer you get from xmpp_stanza_to_text */
    #[no_mangle]
    fn xmpp_free(ctx: *const xmpp_ctx_t, p: *mut libc::c_void);
    #[no_mangle]
    fn xmpp_timed_handler_delete(conn: *mut xmpp_conn_t,
                                 handler: xmpp_timed_handler);
    /* free a stanza object and it's contents */
    #[no_mangle]
    fn xmpp_stanza_release(stanza: *mut xmpp_stanza_t) -> libc::c_int;
    /* marshall a stanza into text for transmission or display */
    #[no_mangle]
    fn xmpp_stanza_to_text(stanza: *mut xmpp_stanza_t,
                           buf: *mut *mut libc::c_char, buflen: *mut size_t)
     -> libc::c_int;
    #[no_mangle]
    fn xmpp_error_new(ctx: *mut xmpp_ctx_t, type_0: xmpp_error_type_t,
                      text: *const libc::c_char) -> *mut xmpp_stanza_t;
    #[no_mangle]
    fn xmpp_jid_domain(ctx: *mut xmpp_ctx_t, jid: *const libc::c_char)
     -> *mut libc::c_char;
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
    #[no_mangle]
    fn sock_connect(host: *const libc::c_char, port: libc::c_ushort)
     -> sock_t;
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
    fn xmpp_vsnprintf(str: *mut libc::c_char, count: size_t,
                      fmt: *const libc::c_char, arg: ::std::ffi::VaList)
     -> libc::c_int;
    /* handler management */
    /* utility functions */
    /* auth functions */
    #[no_mangle]
    fn auth_handle_open_raw(conn: *mut xmpp_conn_t);
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
    #[no_mangle]
    fn tls_error(tls: *mut tls_t) -> libc::c_int;
    #[no_mangle]
    fn tls_start(tls: *mut tls_t) -> libc::c_int;
    #[no_mangle]
    fn tls_new(conn: *mut xmpp_conn_t) -> *mut tls_t;
    #[no_mangle]
    fn handler_add_timed(conn: *mut xmpp_conn_t, handler: xmpp_timed_handler,
                         period: libc::c_ulong, userdata: *mut libc::c_void);
    #[no_mangle]
    fn auth_handle_open_stub(conn: *mut xmpp_conn_t);
    #[no_mangle]
    fn auth_handle_open(conn: *mut xmpp_conn_t);
    #[no_mangle]
    fn auth_handle_component_open(conn: *mut xmpp_conn_t);
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
    /* checks for an error after connect, return 0 if connect successful */
    #[no_mangle]
    fn sock_set_keepalive(sock: sock_t, timeout: libc::c_int,
                          interval: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn sock_error() -> libc::c_int;
    #[no_mangle]
    fn handler_system_delete_all(conn: *mut xmpp_conn_t);
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
    #[no_mangle]
    fn hash_iter_new(table: *mut hash_t) -> *mut hash_iterator_t;
    /* * release an iterator that is no longer needed */
    /* * return the next hash table key from the iterator.
    the returned key should not be freed */
    #[no_mangle]
    fn hash_iter_next(iter: *mut hash_iterator_t) -> *const libc::c_char;
    #[no_mangle]
    fn hash_get(table: *mut hash_t, key: *const libc::c_char)
     -> *mut libc::c_void;
    #[no_mangle]
    fn hash_iter_release(iter: *mut hash_iterator_t);
    #[no_mangle]
    fn hash_release(table: *mut hash_t);
    #[no_mangle]
    fn parser_new(ctx: *mut xmpp_ctx_t, startcb: parser_start_callback,
                  endcb: parser_end_callback,
                  stanzacb: parser_stanza_callback,
                  userdata: *mut libc::c_void) -> *mut parser_t;
    #[no_mangle]
    fn parser_attr_name(ctx: *mut xmpp_ctx_t, nsname: *mut libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn handler_reset_timed(conn: *mut xmpp_conn_t, user_only: libc::c_int);
    #[no_mangle]
    fn xmpp_strdup(ctx: *const xmpp_ctx_t, s: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn tls_stop(tls: *mut tls_t) -> libc::c_int;
    #[no_mangle]
    fn tls_free(tls: *mut tls_t);
    #[no_mangle]
    fn sock_close(sock: sock_t) -> libc::c_int;
    #[no_mangle]
    fn handler_fire_stanza(conn: *mut xmpp_conn_t,
                           stanza: *mut xmpp_stanza_t);
    #[no_mangle]
    fn hash_new(ctx: *mut xmpp_ctx_t, size: libc::c_int,
                free_func: hash_free_func) -> *mut hash_t;
    #[no_mangle]
    fn xmpp_error(ctx: *const xmpp_ctx_t, area: *const libc::c_char,
                  fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn parser_free(parser: *mut parser_t);
    #[no_mangle]
    fn xmpp_alloc(ctx: *const xmpp_ctx_t, size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xmpp_debug(ctx: *const xmpp_ctx_t, area: *const libc::c_char,
                  fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn parser_reset(parser: *mut parser_t) -> libc::c_int;
    /* * Resolve SRV record.
 *
 *  @param ctx a Strophe context object
 *  @param service service of the SRV record
 *  @param proto protocol of the SRV record
 *  @param domain resolving domain
 *  @param srv_rr_list is the result
 *
 *  @return XMPP_DOMAIN_FOUND on success or XMPP_DOMAIN_NOT_FOUND on fail
 */
    #[no_mangle]
    fn resolver_srv_lookup(ctx: *mut xmpp_ctx_t, service: *const libc::c_char,
                           proto: *const libc::c_char,
                           domain: *const libc::c_char,
                           srv_rr_list: *mut *mut resolver_srv_rr_t)
     -> libc::c_int;
    /* * Release a list returned by resolver_srv_lookup() or
 *  resolver_srv_lookup_buf().
 *
 *  @param ctx a Strophe context object
 *  @param srv_rr_list a list allocated by lookup functions
 */
    #[no_mangle]
    fn resolver_srv_free(ctx: *mut xmpp_ctx_t,
                         srv_rr_list: *mut resolver_srv_rr_t);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_ulong;
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
/* convenience functions for accessing the context */
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
pub type __uint64_t = libc::c_ulong;
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
pub type tls_t = _tls;
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
pub type resolver_srv_rr_t = resolver_srv_rr_struc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct resolver_srv_rr_struc {
    pub priority: uint16_t,
    pub weight: uint16_t,
    pub port: uint16_t,
    pub target: [libc::c_char; 256],
    pub next: *mut resolver_srv_rr_struc,
}
pub type uint16_t = __uint16_t;
pub type __uint16_t = libc::c_ushort;
pub type hash_free_func
    =
    Option<unsafe extern "C" fn(_: *const xmpp_ctx_t, _: *mut libc::c_void)
               -> ()>;
/* handlers */
/* if the handle returns false it is removed */
pub type xmpp_timed_handler
    =
    Option<unsafe extern "C" fn(_: *mut xmpp_conn_t, _: *mut libc::c_void)
               -> libc::c_int>;
pub type parser_stanza_callback
    =
    Option<unsafe extern "C" fn(_: *mut xmpp_stanza_t, _: *mut libc::c_void)
               -> ()>;
pub type parser_end_callback
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_char, _: *mut libc::c_void)
               -> ()>;
pub type parser_start_callback
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                _: *mut *mut libc::c_char,
                                _: *mut libc::c_void) -> ()>;
pub type hash_iterator_t = _hash_iterator_t;
pub const XMPP_DOMAIN_NOT_FOUND: C2RustUnnamed_4 = 0;
pub const XMPP_DOMAIN_FOUND: C2RustUnnamed_4 = 1;
pub const XMPP_DOMAIN_ALTDOMAIN: C2RustUnnamed_4 = 2;
pub const XMPP_PORT_COMPONENT: C2RustUnnamed_3 = 5347;
pub const XMPP_PORT_CLIENT: C2RustUnnamed_3 = 5222;
pub const XMPP_PORT_CLIENT_LEGACY_SSL: C2RustUnnamed_3 = 5223;
pub type C2RustUnnamed_3 = libc::c_uint;
pub type C2RustUnnamed_4 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn xmpp_send_error(conn: *mut xmpp_conn_t,
                                         type_0: xmpp_error_type_t,
                                         text: *mut libc::c_char) {
    let mut error: *mut xmpp_stanza_t =
        xmpp_error_new((*conn).ctx, type_0, text);
    xmpp_send(conn, error);
    xmpp_stanza_release(error);
}
/* * Create a new Strophe connection object.
 *
 *  @param ctx a Strophe context object
 *
 *  @return a Strophe connection object or NULL on an error
 *
 *  @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_conn_new(ctx: *mut xmpp_ctx_t)
 -> *mut xmpp_conn_t {
    let mut conn: *mut xmpp_conn_t = 0 as *mut xmpp_conn_t;
    let mut tail: *mut xmpp_connlist_t = 0 as *mut xmpp_connlist_t;
    let mut item: *mut xmpp_connlist_t = 0 as *mut xmpp_connlist_t;
    if ctx.is_null() { return 0 as *mut xmpp_conn_t }
    conn =
        xmpp_alloc(ctx, ::std::mem::size_of::<xmpp_conn_t>() as libc::c_ulong)
            as *mut xmpp_conn_t;
    if !conn.is_null() {
        (*conn).ctx = ctx;
        (*conn).type_0 = XMPP_UNKNOWN;
        (*conn).state = XMPP_STATE_DISCONNECTED;
        (*conn).sock = -(1 as libc::c_int);
        (*conn).ka_timeout = 0 as libc::c_int;
        (*conn).ka_interval = 0 as libc::c_int;
        (*conn).tls = 0 as *mut tls_t;
        (*conn).timeout_stamp = 0 as libc::c_int as uint64_t;
        (*conn).error = 0 as libc::c_int;
        (*conn).stream_error = 0 as *mut xmpp_stream_error_t;
        /* default send parameters */
        (*conn).blocking_send = 0 as libc::c_int;
        (*conn).send_queue_max = 64 as libc::c_int;
        (*conn).send_queue_len = 0 as libc::c_int;
        (*conn).send_queue_head = 0 as *mut xmpp_send_queue_t;
        (*conn).send_queue_tail = 0 as *mut xmpp_send_queue_t;
        /* default timeouts */
        (*conn).connect_timeout = 5000 as libc::c_int as libc::c_uint;
        (*conn).lang =
            xmpp_strdup((*conn).ctx,
                        b"en\x00" as *const u8 as *const libc::c_char);
        if (*conn).lang.is_null() {
            xmpp_free((*conn).ctx, conn as *mut libc::c_void);
            return 0 as *mut xmpp_conn_t
        }
        (*conn).domain = 0 as *mut libc::c_char;
        (*conn).jid = 0 as *mut libc::c_char;
        (*conn).pass = 0 as *mut libc::c_char;
        (*conn).stream_id = 0 as *mut libc::c_char;
        (*conn).bound_jid = 0 as *mut libc::c_char;
        (*conn).is_raw = 0 as libc::c_int;
        (*conn).tls_support = 0 as libc::c_int;
        (*conn).tls_disabled = 0 as libc::c_int;
        (*conn).tls_mandatory = 0 as libc::c_int;
        (*conn).tls_legacy_ssl = 0 as libc::c_int;
        (*conn).tls_trust = 0 as libc::c_int;
        (*conn).tls_failed = 0 as libc::c_int;
        (*conn).sasl_support = 0 as libc::c_int;
        (*conn).auth_legacy_enabled = 0 as libc::c_int;
        (*conn).secured = 0 as libc::c_int;
        (*conn).bind_required = 0 as libc::c_int;
        (*conn).session_required = 0 as libc::c_int;
        (*conn).parser =
            parser_new((*conn).ctx,
                       Some(_handle_stream_start as
                                unsafe extern "C" fn(_: *mut libc::c_char,
                                                     _:
                                                         *mut *mut libc::c_char,
                                                     _: *mut libc::c_void)
                                    -> ()),
                       Some(_handle_stream_end as
                                unsafe extern "C" fn(_: *mut libc::c_char,
                                                     _: *mut libc::c_void)
                                    -> ()),
                       Some(_handle_stream_stanza as
                                unsafe extern "C" fn(_: *mut xmpp_stanza_t,
                                                     _: *mut libc::c_void)
                                    -> ()), conn as *mut libc::c_void);
        (*conn).reset_parser = 0 as libc::c_int;
        (*conn).authenticated = 0 as libc::c_int;
        (*conn).conn_handler = None;
        (*conn).userdata = 0 as *mut libc::c_void;
        (*conn).timed_handlers = 0 as *mut xmpp_handlist_t;
        /* we own (and will free) the hash values */
        (*conn).id_handlers = hash_new((*conn).ctx, 32 as libc::c_int, None);
        (*conn).handlers = 0 as *mut xmpp_handlist_t;
        /* give the caller a reference to connection */
        (*conn).ref_0 = 1 as libc::c_int as libc::c_uint;
        /* add connection to ctx->connlist */
        tail = (*(*conn).ctx).connlist;
        while !tail.is_null() && !(*tail).next.is_null() {
            tail = (*tail).next
        }
        item =
            xmpp_alloc((*conn).ctx,
                       ::std::mem::size_of::<xmpp_connlist_t>() as
                           libc::c_ulong) as *mut xmpp_connlist_t;
        if item.is_null() {
            xmpp_error((*conn).ctx,
                       b"xmpp\x00" as *const u8 as *const libc::c_char,
                       b"failed to allocate memory\x00" as *const u8 as
                           *const libc::c_char);
            xmpp_free((*conn).ctx, (*conn).lang as *mut libc::c_void);
            parser_free((*conn).parser);
            xmpp_free((*conn).ctx, conn as *mut libc::c_void);
            conn = 0 as *mut xmpp_conn_t
        } else {
            (*item).conn = conn;
            (*item).next = 0 as *mut _xmpp_connlist_t;
            if !tail.is_null() {
                (*tail).next = item
            } else { (*(*conn).ctx).connlist = item }
        }
    }
    return conn;
}
/* * Clone a Strophe connection object.
 *  
 *  @param conn a Strophe connection object
 *
 *  @return the same conn object passed in with its reference count
 *          incremented by 1
 *
 *  @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_conn_clone(conn: *mut xmpp_conn_t)
 -> *mut xmpp_conn_t {
    (*conn).ref_0 = (*conn).ref_0.wrapping_add(1);
    return conn;
}
/* * Set TCP keepalive parameters
 *  Turn on TCP keepalive and set timeout and interval. Zero timeout
 *  disables TCP keepalives. The parameters are applied immediately for
 *  a non disconnected object. Also, they are applied when the connection
 *  object connects successfully.
 *
 *  @param conn a Strophe connection object
 *  @param timeout TCP keepalive timeout in seconds
 *  @param interval TCP keepalive interval in seconds
 *
 *  @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_conn_set_keepalive(conn: *mut xmpp_conn_t,
                                                 mut timeout: libc::c_int,
                                                 mut interval: libc::c_int) {
    let mut ret: libc::c_int = 0 as libc::c_int;
    (*conn).ka_timeout = timeout;
    (*conn).ka_interval = interval;
    if (*conn).state as libc::c_uint !=
           XMPP_STATE_DISCONNECTED as libc::c_int as libc::c_uint {
        ret = sock_set_keepalive((*conn).sock, timeout, interval)
    }
    if ret < 0 as libc::c_int {
        xmpp_error((*conn).ctx,
                   b"xmpp\x00" as *const u8 as *const libc::c_char,
                   b"Setting TCP keepalive (%d,%d) error: %d\x00" as *const u8
                       as *const libc::c_char, timeout, interval,
                   sock_error());
    };
}
/* * Release a Strophe connection object.
 *  Decrement the reference count by one for a connection, freeing the
 *  connection object if the count reaches 0.
 *
 *  @param conn a Strophe connection object
 *
 *  @return TRUE if the connection object was freed and FALSE otherwise
 *
 *  @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_conn_release(conn: *mut xmpp_conn_t)
 -> libc::c_int {
    let mut ctx: *mut xmpp_ctx_t = 0 as *mut xmpp_ctx_t;
    let mut item: *mut xmpp_connlist_t = 0 as *mut xmpp_connlist_t;
    let mut prev: *mut xmpp_connlist_t = 0 as *mut xmpp_connlist_t;
    let mut hlitem: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    let mut thli: *mut xmpp_handlist_t = 0 as *mut xmpp_handlist_t;
    let mut iter: *mut hash_iterator_t = 0 as *mut hash_iterator_t;
    let mut key: *const libc::c_char = 0 as *const libc::c_char;
    let mut released: libc::c_int = 0 as libc::c_int;
    if (*conn).ref_0 > 1 as libc::c_int as libc::c_uint {
        (*conn).ref_0 = (*conn).ref_0.wrapping_sub(1)
    } else {
        ctx = (*conn).ctx;
        if (*conn).state as libc::c_uint ==
               XMPP_STATE_CONNECTING as libc::c_int as libc::c_uint ||
               (*conn).state as libc::c_uint ==
                   XMPP_STATE_CONNECTED as libc::c_int as libc::c_uint {
            conn_disconnect(conn);
        }
        /* remove connection from context's connlist */
        if (*(*ctx).connlist).conn == conn {
            item = (*ctx).connlist;
            (*ctx).connlist = (*item).next;
            xmpp_free(ctx, item as *mut libc::c_void);
        } else {
            prev = 0 as *mut xmpp_connlist_t;
            item = (*ctx).connlist;
            while !item.is_null() && (*item).conn != conn {
                prev = item;
                item = (*item).next
            }
            if item.is_null() {
                xmpp_error(ctx,
                           b"xmpp\x00" as *const u8 as *const libc::c_char,
                           b"Connection not in context\'s list\n\x00" as
                               *const u8 as *const libc::c_char);
            } else {
                (*prev).next = (*item).next;
                xmpp_free(ctx, item as *mut libc::c_void);
            }
        }
        _conn_reset(conn);
        /* free handler stuff
         * note that userdata is the responsibility of the client
         * and the handler pointers don't need to be freed since they
         * are pointers to functions */
        hlitem = (*conn).timed_handlers;
        while !hlitem.is_null() {
            thli = hlitem;
            hlitem = (*hlitem).next;
            xmpp_free(ctx, thli as *mut libc::c_void);
        }
        /* id handlers
         * we have to traverse the hash table freeing list elements
         * then release the hash table */
        iter = hash_iter_new((*conn).id_handlers);
        loop  {
            key = hash_iter_next(iter);
            if key.is_null() { break ; }
            hlitem =
                hash_get((*conn).id_handlers, key) as *mut xmpp_handlist_t;
            while !hlitem.is_null() {
                thli = hlitem;
                hlitem = (*hlitem).next;
                xmpp_free((*conn).ctx,
                          (*thli).u.c2rust_unnamed_0.id as *mut libc::c_void);
                xmpp_free((*conn).ctx, thli as *mut libc::c_void);
            }
        }
        hash_iter_release(iter);
        hash_release((*conn).id_handlers);
        hlitem = (*conn).handlers;
        while !hlitem.is_null() {
            thli = hlitem;
            hlitem = (*hlitem).next;
            if !(*thli).u.c2rust_unnamed_1.ns.is_null() {
                xmpp_free(ctx,
                          (*thli).u.c2rust_unnamed_1.ns as *mut libc::c_void);
            }
            if !(*thli).u.c2rust_unnamed_1.name.is_null() {
                xmpp_free(ctx,
                          (*thli).u.c2rust_unnamed_1.name as
                              *mut libc::c_void);
            }
            if !(*thli).u.c2rust_unnamed_1.type_0.is_null() {
                xmpp_free(ctx,
                          (*thli).u.c2rust_unnamed_1.type_0 as
                              *mut libc::c_void);
            }
            xmpp_free(ctx, thli as *mut libc::c_void);
        }
        parser_free((*conn).parser);
        if !(*conn).jid.is_null() {
            xmpp_free(ctx, (*conn).jid as *mut libc::c_void);
        }
        if !(*conn).pass.is_null() {
            xmpp_free(ctx, (*conn).pass as *mut libc::c_void);
        }
        if !(*conn).lang.is_null() {
            xmpp_free(ctx, (*conn).lang as *mut libc::c_void);
        }
        xmpp_free(ctx, conn as *mut libc::c_void);
        released = 1 as libc::c_int
    }
    return released;
}
/* * Get the JID which is or will be bound to the connection.
 *  
 *  @param conn a Strophe connection object
 *
 *  @return a string containing the full JID or NULL if it has not been set
 *
 *  @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_conn_get_jid(conn: *const xmpp_conn_t)
 -> *const libc::c_char {
    return (*conn).jid;
}
/* *
 * Get the JID discovered during binding time.
 *
 * This JID will contain the resource used by the current connection.
 * This is useful in the case where a resource was not specified for
 * binding.
 *
 * @param conn a Strophe connection object.
 *
 * @return a string containing the full JID or NULL if it's not been discovered
 *
 * @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_conn_get_bound_jid(conn: *const xmpp_conn_t)
 -> *const libc::c_char {
    return (*conn).bound_jid;
}
/* * Set the JID of the user that will be bound to the connection.
 *  If any JID was previously set, it will be discarded.  This should not be
 *  be used after a connection is created.  The function will make a copy of
 *  the JID string.  If the supplied JID is missing the node, SASL
 *  ANONYMOUS authentication will be used.
 *
 *  @param conn a Strophe connection object
 *  @param jid a full or bare JID
 *
 *  @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_conn_set_jid(conn: *mut xmpp_conn_t,
                                           jid: *const libc::c_char) {
    if !(*conn).jid.is_null() {
        xmpp_free((*conn).ctx, (*conn).jid as *mut libc::c_void);
    }
    (*conn).jid = xmpp_strdup((*conn).ctx, jid);
}
/* * Get the password used for authentication of a connection.
 *
 *  @param conn a Strophe connection object
 *
 *  @return a string containing the password or NULL if it has not been set
 *
 *  @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_conn_get_pass(conn: *const xmpp_conn_t)
 -> *const libc::c_char {
    return (*conn).pass;
}
/* * Set the password used to authenticate the connection.
 *  If any password was previously set, it will be discarded.  The function
 *  will make a copy of the password string.
 * 
 *  @param conn a Strophe connection object
 *  @param pass the password
 *
 *  @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_conn_set_pass(conn: *mut xmpp_conn_t,
                                            pass: *const libc::c_char) {
    if !(*conn).pass.is_null() {
        xmpp_free((*conn).ctx, (*conn).pass as *mut libc::c_void);
    }
    (*conn).pass = xmpp_strdup((*conn).ctx, pass);
}
/* * Get the strophe context that the connection is associated with.
*  @param conn a Strophe connection object
* 
*  @return a Strophe context
* 
*  @ingroup Connections
*/
#[no_mangle]
pub unsafe extern "C" fn xmpp_conn_get_context(conn: *mut xmpp_conn_t)
 -> *mut xmpp_ctx_t {
    return (*conn).ctx;
}
/* * Initiate a connection to the XMPP server.
 *  This function returns immediately after starting the connection
 *  process to the XMPP server, and notifications of connection state changes
 *  will be sent to the callback function.  The domain and port to connect to
 *  are usually determined by an SRV lookup for the xmpp-client service at
 *  the domain specified in the JID.  If SRV lookup fails, altdomain and 
 *  altport will be used instead if specified.
 *
 *  @param conn a Strophe connection object
 *  @param altdomain a string with domain to use if SRV lookup fails.  If this
 *      is NULL, the domain from the JID will be used.
 *  @param altport an integer port number to use if SRV lookup fails.  If this
 *      is 0, the default port will be assumed.
 *  @param callback a xmpp_conn_handler callback function that will receive
 *      notifications of connection status
 *  @param userdata an opaque data pointer that will be passed to the callback
 *
 *  @return XMPP_EOK (0) on success or a number less than 0 on failure
 *
 *  @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_connect_client(conn: *mut xmpp_conn_t,
                                             altdomain: *const libc::c_char,
                                             mut altport: libc::c_ushort,
                                             mut callback: xmpp_conn_handler,
                                             userdata: *mut libc::c_void)
 -> libc::c_int {
    let mut srv_rr_list: *mut resolver_srv_rr_t = 0 as *mut resolver_srv_rr_t;
    let mut rr: *mut resolver_srv_rr_t = 0 as *mut resolver_srv_rr_t;
    let mut domain: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut host: *const libc::c_char = 0 as *const libc::c_char;
    let mut port: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
    let mut found: libc::c_int = XMPP_DOMAIN_NOT_FOUND as libc::c_int;
    let mut rc: libc::c_int = 0;
    domain = xmpp_jid_domain((*conn).ctx, (*conn).jid);
    if domain.is_null() { return -(1 as libc::c_int) }
    if !altdomain.is_null() {
        xmpp_debug((*conn).ctx,
                   b"xmpp\x00" as *const u8 as *const libc::c_char,
                   b"Connecting via altdomain.\x00" as *const u8 as
                       *const libc::c_char);
        host = altdomain;
        port =
            if altport as libc::c_int != 0 {
                altport as libc::c_int
            } else { _conn_default_port(conn, XMPP_CLIENT) as libc::c_int } as
                libc::c_ushort;
        found = XMPP_DOMAIN_ALTDOMAIN as libc::c_int
        /* SSL tunneled connection on 5223 port is legacy and doesn't
     * have an SRV record. */
    } else if (*conn).tls_legacy_ssl == 0 {
        found =
            resolver_srv_lookup((*conn).ctx,
                                b"xmpp-client\x00" as *const u8 as
                                    *const libc::c_char,
                                b"tcp\x00" as *const u8 as
                                    *const libc::c_char, domain,
                                &mut srv_rr_list)
    }
    if XMPP_DOMAIN_NOT_FOUND as libc::c_int == found {
        xmpp_debug((*conn).ctx,
                   b"xmpp\x00" as *const u8 as *const libc::c_char,
                   b"SRV lookup failed, connecting via domain.\x00" as
                       *const u8 as *const libc::c_char);
        host = domain;
        port =
            if altport as libc::c_int != 0 {
                altport as libc::c_int
            } else { _conn_default_port(conn, XMPP_CLIENT) as libc::c_int } as
                libc::c_ushort;
        found = XMPP_DOMAIN_ALTDOMAIN as libc::c_int
    }
    rr = srv_rr_list;
    loop  {
        if XMPP_DOMAIN_FOUND as libc::c_int == found && !rr.is_null() {
            host = (*rr).target.as_mut_ptr();
            port = (*rr).port;
            rr = (*rr).next
        }
        rc =
            _conn_connect(conn, domain, host, port, XMPP_CLIENT, callback,
                          userdata);
        if !(rc != 0 as libc::c_int && !rr.is_null()) { break ; }
    }
    xmpp_free((*conn).ctx, domain as *mut libc::c_void);
    resolver_srv_free((*conn).ctx, srv_rr_list);
    return rc;
}
/* * Initiate a component connection to server.
 *  This function returns immediately after starting the connection
 *  process to the XMPP server, and notifications of connection state changes
 *  will be sent to the internal callback function that will set up handler
 *  for the component handshake as defined in XEP-0114.
 *  The domain and port to connect to must be provided in this case as the JID
 *  provided to the call serves as component identifier to the server and is
 *  not subject to DNS resolution.
 *
 *  @param conn a Strophe connection object
 *  @param server a string with domain to use directly as the domain can't be
 *      extracted from the component name/JID. If this is not set, the call
 *      will fail.
 *  @param port an integer port number to use to connect to server expecting
 *      an external component.  If this is 0, the port 5347 will be assumed.
 *  @param callback a xmpp_conn_handler callback function that will receive
 *      notifications of connection status
 *  @param userdata an opaque data pointer that will be passed to the callback
 *
 *  @return XMPP_EOK (0) on success or a number less than 0 on failure
 *
 *  @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_connect_component(conn: *mut xmpp_conn_t,
                                                server: *const libc::c_char,
                                                mut port: libc::c_ushort,
                                                mut callback:
                                                    xmpp_conn_handler,
                                                userdata: *mut libc::c_void)
 -> libc::c_int {
    /*  The server domain, jid and password MUST be specified. */
    if !(!server.is_null() && !(*conn).jid.is_null() &&
             !(*conn).pass.is_null()) {
        return -(2 as libc::c_int)
    }
    /* XEP-0114 does not support TLS */
    xmpp_conn_disable_tls(conn);
    if (*conn).tls_disabled == 0 {
        xmpp_error((*conn).ctx,
                   b"conn\x00" as *const u8 as *const libc::c_char,
                   b"Failed to disable TLS. XEP-0114 does not support TLS\x00"
                       as *const u8 as *const libc::c_char);
        return -(3 as libc::c_int)
    }
    port =
        if port as libc::c_int != 0 {
            port as libc::c_int
        } else { _conn_default_port(conn, XMPP_COMPONENT) as libc::c_int } as
            libc::c_ushort;
    /* JID serves as an identifier here and will be used as "to" attribute
       of the stream */
    return _conn_connect(conn, (*conn).jid, server, port, XMPP_COMPONENT,
                         callback, userdata);
}
/* * Initiate a raw connection to the XMPP server.
 *  Arguments and behaviour of the function are similar to
 *  xmpp_connect_client(), but it skips authentication process. In opposite to
 *  xmpp_connect_client() during connection process two events are generated
 *  instead of one. User's callback is called with event XMPP_CONN_RAW_CONNECT
 *  when the TCP connection with the server is established. At this point user
 *  might want to open an XMPP stream with xmpp_conn_open_stream() or establish
 *  TLS session with xmpp_conn_tls_start(). Event XMPP_CONN_CONNECT is generated
 *  when the XMPP stream is opened successfully and user may send stanzas over
 *  the connection.
 *
 *  This function doesn't use password nor node part of a jid. Therefore,
 *  the only required configuration is a domain (or full jid) passed via
 *  xmpp_conn_set_jid().
 *
 *  @see xmpp_connect_client()
 *
 *  @return XMPP_EOK (0) on success a number less than 0 on failure
 *
 *  @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_connect_raw(conn: *mut xmpp_conn_t,
                                          altdomain: *const libc::c_char,
                                          mut altport: libc::c_ushort,
                                          mut callback: xmpp_conn_handler,
                                          userdata: *mut libc::c_void)
 -> libc::c_int {
    (*conn).is_raw = 1 as libc::c_int;
    return xmpp_connect_client(conn, altdomain, altport, callback, userdata);
}
/* Called when tcp connection is established. */
#[no_mangle]
pub unsafe extern "C" fn conn_established(conn: *mut xmpp_conn_t) {
    if (*conn).tls_legacy_ssl != 0 && (*conn).is_raw == 0 {
        xmpp_debug((*conn).ctx,
                   b"xmpp\x00" as *const u8 as *const libc::c_char,
                   b"using legacy SSL connection\x00" as *const u8 as
                       *const libc::c_char);
        if conn_tls_start(conn) != 0 as libc::c_int {
            conn_disconnect(conn);
            return
        }
    }
    if (*conn).is_raw != 0 {
        handler_reset_timed(conn, 0 as libc::c_int);
        /* we skip authentication for a "raw" connection, but the event loop
           ignores user's handlers when conn->authenticated is not set. */
        (*conn).authenticated = 1 as libc::c_int;
        (*conn).conn_handler.expect("non-null function pointer")(conn,
                                                                 XMPP_CONN_RAW_CONNECT,
                                                                 0 as
                                                                     libc::c_int,
                                                                 0 as
                                                                     *mut xmpp_stream_error_t,
                                                                 (*conn).userdata);
    } else {
        /* send stream init */
        conn_open_stream(conn);
    };
}
/* * Send the default opening stream tag.
 *  The default tag is the one sent by xmpp_connect_client().
 *  User's connection handler is called with event XMPP_CONN_CONNECT when
 *  server replies with its opening tag.
 *
 *  @return XMPP_EOK (0) on success a number less than 0 on failure
 *
 *  @note The connection must be connected with xmpp_connect_raw().
 *
 *  @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_conn_open_stream_default(conn: *mut xmpp_conn_t)
 -> libc::c_int {
    if (*conn).is_raw == 0 { return -(2 as libc::c_int) }
    conn_prepare_reset(conn,
                       Some(auth_handle_open_raw as
                                unsafe extern "C" fn(_: *mut xmpp_conn_t)
                                    -> ()));
    conn_open_stream(conn);
    return 0 as libc::c_int;
}
/* * Send an opening stream tag.
 *  User's connection handler is called with event XMPP_CONN_CONNECT when
 *  server replies with its opening tag.
 *
 *  @param conn a Strophe connection object
 *  @param attributes Array of strings in format: even index points to
 *      an attribute name and odd index points to its value
 *  @param attributes_len Number of elements in the attributes array, it
 *      should be number of attributes multiplied by 2
 *
 *  @return XMPP_EOK (0) on success a number less than 0 on failure
 *
 *  @note The connection must be connected with xmpp_connect_raw().
 *
 *  @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_conn_open_stream(conn: *mut xmpp_conn_t,
                                               mut attributes:
                                                   *mut *mut libc::c_char,
                                               mut attributes_len: size_t)
 -> libc::c_int {
    let mut tag: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*conn).is_raw == 0 { return -(2 as libc::c_int) }
    tag = _conn_build_stream_tag(conn, attributes, attributes_len);
    if tag.is_null() { return -(1 as libc::c_int) }
    conn_prepare_reset(conn,
                       Some(auth_handle_open_raw as
                                unsafe extern "C" fn(_: *mut xmpp_conn_t)
                                    -> ()));
    xmpp_send_raw_string(conn,
                         b"<?xml version=\"1.0\"?>%s\x00" as *const u8 as
                             *const libc::c_char, tag);
    xmpp_free((*conn).ctx, tag as *mut libc::c_void);
    return 0 as libc::c_int;
}
/* * Start synchronous TLS handshake with the server.
 *
 *  @return XMPP_EOK (0) on success a number less than 0 on failure
 *
 *  @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_conn_tls_start(conn: *mut xmpp_conn_t)
 -> libc::c_int {
    return conn_tls_start(conn);
}
/* * Cleanly disconnect the connection.
 *  This function is only called by the stream parser when </stream:stream>
 *  is received, and it not intended to be called by code outside of Strophe.
 *
 *  @param conn a Strophe connection object
 */
#[no_mangle]
pub unsafe extern "C" fn conn_disconnect_clean(conn: *mut xmpp_conn_t) {
    /* remove the timed handler */
    xmpp_timed_handler_delete(conn,
                              Some(_disconnect_cleanup as
                                       unsafe extern "C" fn(_:
                                                                *mut xmpp_conn_t,
                                                            _:
                                                                *mut libc::c_void)
                                           -> libc::c_int));
    conn_disconnect(conn);
}
/* * Disconnect from the XMPP server.
 *  This function immediately disconnects from the XMPP server, and should
 *  not be used outside of the Strophe library.
 *
 *  @param conn a Strophe connection object
 */
#[no_mangle]
pub unsafe extern "C" fn conn_disconnect(conn: *mut xmpp_conn_t) {
    xmpp_debug((*conn).ctx, b"xmpp\x00" as *const u8 as *const libc::c_char,
               b"Closing socket.\x00" as *const u8 as *const libc::c_char);
    (*conn).state = XMPP_STATE_DISCONNECTED;
    if !(*conn).tls.is_null() {
        tls_stop((*conn).tls);
        tls_free((*conn).tls);
        (*conn).tls = 0 as *mut tls_t
    }
    sock_close((*conn).sock);
    /* fire off connection handler */
    (*conn).conn_handler.expect("non-null function pointer")(conn,
                                                             XMPP_CONN_DISCONNECT,
                                                             (*conn).error,
                                                             (*conn).stream_error,
                                                             (*conn).userdata);
}
/* prepares a parser reset.  this is called from handlers. we can't
 * reset the parser immediately as it is not re-entrant. */
#[no_mangle]
pub unsafe extern "C" fn conn_prepare_reset(conn: *mut xmpp_conn_t,
                                            mut handler: xmpp_open_handler) {
    (*conn).reset_parser = 1 as libc::c_int;
    (*conn).open_handler = handler;
}
/* reset the parser */
#[no_mangle]
pub unsafe extern "C" fn conn_parser_reset(conn: *mut xmpp_conn_t) {
    (*conn).reset_parser = 0 as libc::c_int;
    parser_reset((*conn).parser);
}
/* * Initiate termination of the connection to the XMPP server.
 *  This function starts the disconnection sequence by sending
 *  </stream:stream> to the XMPP server.  This function will do nothing
 *  if the connection state is different from CONNECTING or CONNECTED.
 *
 *  @param conn a Strophe connection object
 *
 *  @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_disconnect(conn: *mut xmpp_conn_t) {
    if (*conn).state as libc::c_uint !=
           XMPP_STATE_CONNECTING as libc::c_int as libc::c_uint &&
           (*conn).state as libc::c_uint !=
               XMPP_STATE_CONNECTED as libc::c_int as libc::c_uint {
        return
    }
    /* close the stream */
    xmpp_send_raw_string(conn,
                         b"</stream:stream>\x00" as *const u8 as
                             *const libc::c_char);
    /* setup timed handler in case disconnect takes too long */
    handler_add_timed(conn,
                      Some(_disconnect_cleanup as
                               unsafe extern "C" fn(_: *mut xmpp_conn_t,
                                                    _: *mut libc::c_void)
                                   -> libc::c_int),
                      2000 as libc::c_int as libc::c_ulong,
                      0 as *mut libc::c_void);
}
/* * Send a raw string to the XMPP server.
 *  This function is a convenience function to send raw string data to the
 *  XMPP server.  It is used by Strophe to send short messages instead of
 *  building up an XML stanza with DOM methods.  This should be used with care
 *  as it does not validate the data; invalid data may result in immediate
 *  stream termination by the XMPP server.
 *
 *  @param conn a Strophe connection object
 *  @param fmt a printf-style format string followed by a variable list of
 *      arguments to format
 *
 *  @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_send_raw_string(conn: *mut xmpp_conn_t,
                                              fmt: *const libc::c_char,
                                              mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl; /* small buffer for common case */
    let mut len: size_t = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut bigbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    ap = args.clone();
    len =
        xmpp_vsnprintf(buf.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 1024]>() as
                           libc::c_ulong, fmt, ap.as_va_list()) as size_t;
    if len >= ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong {
        /* we need more space for this data, so we allocate a big
         * enough buffer and print to that */
        len = len.wrapping_add(1); /* account for trailing \0 */
        bigbuf = xmpp_alloc((*conn).ctx, len) as *mut libc::c_char;
        if bigbuf.is_null() {
            xmpp_debug((*conn).ctx,
                       b"xmpp\x00" as *const u8 as *const libc::c_char,
                       b"Could not allocate memory for send_raw_string\x00" as
                           *const u8 as *const libc::c_char);
            return
        }
        ap = args.clone();
        xmpp_vsnprintf(bigbuf, len, fmt, ap.as_va_list());
        xmpp_debug((*conn).ctx,
                   b"conn\x00" as *const u8 as *const libc::c_char,
                   b"SENT: %s\x00" as *const u8 as *const libc::c_char,
                   bigbuf);
        /* len - 1 so we don't send trailing \0 */
        xmpp_send_raw(conn, bigbuf,
                      len.wrapping_sub(1 as libc::c_int as libc::c_ulong));
        xmpp_free((*conn).ctx, bigbuf as *mut libc::c_void);
    } else {
        xmpp_debug((*conn).ctx,
                   b"conn\x00" as *const u8 as *const libc::c_char,
                   b"SENT: %s\x00" as *const u8 as *const libc::c_char,
                   buf.as_mut_ptr());
        xmpp_send_raw(conn, buf.as_mut_ptr(), len);
    };
}
/* * Send raw bytes to the XMPP server.
 *  This function is a convenience function to send raw bytes to the
 *  XMPP server.  It is used primarily by xmpp_send_raw_string().  This
 *  function should be used with care as it does not validate the bytes and
 *  invalid data may result in stream termination by the XMPP server.
 *
 *  @param conn a Strophe connection object
 *  @param data a buffer of raw bytes
 *  @param len the length of the data in the buffer
 *
 *  @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_send_raw(conn: *mut xmpp_conn_t,
                                       data: *const libc::c_char,
                                       len: size_t) {
    let mut item: *mut xmpp_send_queue_t = 0 as *mut xmpp_send_queue_t;
    if (*conn).state as libc::c_uint !=
           XMPP_STATE_CONNECTED as libc::c_int as libc::c_uint {
        return
    }
    /* create send queue item for queue */
    item =
        xmpp_alloc((*conn).ctx,
                   ::std::mem::size_of::<xmpp_send_queue_t>() as
                       libc::c_ulong) as *mut xmpp_send_queue_t;
    if item.is_null() { return }
    (*item).data = xmpp_alloc((*conn).ctx, len) as *mut libc::c_char;
    if (*item).data.is_null() {
        xmpp_free((*conn).ctx, item as *mut libc::c_void);
        return
    }
    memcpy((*item).data as *mut libc::c_void, data as *const libc::c_void,
           len);
    (*item).len = len;
    (*item).next = 0 as *mut xmpp_send_queue_t;
    (*item).written = 0 as libc::c_int as size_t;
    /* add item to the send queue */
    if (*conn).send_queue_tail.is_null() {
        /* first item, set head and tail */
        (*conn).send_queue_head = item;
        (*conn).send_queue_tail = item
    } else {
        /* add to the tail */
        (*(*conn).send_queue_tail).next = item;
        (*conn).send_queue_tail = item
    }
    (*conn).send_queue_len += 1;
}
/* * Send an XML stanza to the XMPP server.
 *  This is the main way to send data to the XMPP server.  The function will
 *  terminate without action if the connection state is not CONNECTED.
 *
 *  @param conn a Strophe connection object
 *  @param stanza a Strophe stanza object
 *
 *  @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_send(conn: *mut xmpp_conn_t,
                                   stanza: *mut xmpp_stanza_t) {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    if (*conn).state as libc::c_uint ==
           XMPP_STATE_CONNECTED as libc::c_int as libc::c_uint {
        if xmpp_stanza_to_text(stanza, &mut buf, &mut len) == 0 as libc::c_int
           {
            xmpp_send_raw(conn, buf, len);
            xmpp_debug((*conn).ctx,
                       b"conn\x00" as *const u8 as *const libc::c_char,
                       b"SENT: %s\x00" as *const u8 as *const libc::c_char,
                       buf);
            xmpp_free((*conn).ctx, buf as *mut libc::c_void);
        }
    };
}
/* * Send the opening &lt;stream:stream&gt; tag to the server.
 *  This function is used by Strophe to begin an XMPP stream.  It should
 *  not be used outside of the library.
 *
 *  @param conn a Strophe connection object
 */
#[no_mangle]
pub unsafe extern "C" fn conn_open_stream(conn: *mut xmpp_conn_t) {
    xmpp_send_raw_string(conn,
                         b"<?xml version=\"1.0\"?><stream:stream to=\"%s\" xml:lang=\"%s\" version=\"1.0\" xmlns=\"%s\" xmlns:stream=\"%s\">\x00"
                             as *const u8 as *const libc::c_char,
                         (*conn).domain, (*conn).lang,
                         if (*conn).type_0 as libc::c_uint ==
                                XMPP_CLIENT as libc::c_int as libc::c_uint {
                             b"jabber:client\x00" as *const u8 as
                                 *const libc::c_char
                         } else {
                             b"jabber:component:accept\x00" as *const u8 as
                                 *const libc::c_char
                         },
                         b"http://etherx.jabber.org/streams\x00" as *const u8
                             as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn conn_tls_start(conn: *mut xmpp_conn_t)
 -> libc::c_int {
    let mut rc: libc::c_int = 0;
    if (*conn).tls_disabled != 0 {
        (*conn).tls = 0 as *mut tls_t;
        rc = -(2 as libc::c_int)
    } else {
        (*conn).tls = tls_new(conn);
        rc =
            if (*conn).tls.is_null() {
                -(1 as libc::c_int)
            } else { 0 as libc::c_int }
    }
    if !(*conn).tls.is_null() {
        if tls_start((*conn).tls) != 0 {
            (*conn).secured = 1 as libc::c_int
        } else {
            rc = -(3 as libc::c_int);
            (*conn).error = tls_error((*conn).tls);
            tls_free((*conn).tls);
            (*conn).tls = 0 as *mut tls_t;
            (*conn).tls_failed = 1 as libc::c_int
        }
    }
    if rc != 0 as libc::c_int {
        xmpp_debug((*conn).ctx,
                   b"conn\x00" as *const u8 as *const libc::c_char,
                   b"Couldn\'t start TLS! error %d tls_error %d\x00" as
                       *const u8 as *const libc::c_char, rc, (*conn).error);
    }
    return rc;
}
/* * Return applied flags for the connection.
 *
 *  @param conn a Strophe connection object
 *
 *  @return ORed connection flags that are applied for the connection.
 *
 *  @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_conn_get_flags(conn: *const xmpp_conn_t)
 -> libc::c_long {
    let mut flags: libc::c_long = 0;
    flags =
        (((1 as libc::c_ulong) <<
              0 as
                  libc::c_int).wrapping_mul((*conn).tls_disabled as
                                                libc::c_ulong) |
             ((1 as libc::c_ulong) <<
                  1 as
                      libc::c_int).wrapping_mul((*conn).tls_mandatory as
                                                    libc::c_ulong) |
             ((1 as libc::c_ulong) <<
                  2 as
                      libc::c_int).wrapping_mul((*conn).tls_legacy_ssl as
                                                    libc::c_ulong) |
             ((1 as libc::c_ulong) <<
                  3 as
                      libc::c_int).wrapping_mul((*conn).tls_trust as
                                                    libc::c_ulong) |
             ((1 as libc::c_ulong) <<
                  4 as
                      libc::c_int).wrapping_mul((*conn).auth_legacy_enabled as
                                                    libc::c_ulong)) as
            libc::c_long;
    return flags;
}
/* * Set flags for the connection.
 *  This function applies set flags and resets unset ones. Default connection
 *  configuration is all flags unset. Flags can be applied only for a connection
 *  in disconnected state.
 *  All unsupported flags are ignored. If a flag is unset after successful set
 *  operation then the flag is not supported by current version.
 *
 *  Supported flags are:
 *
 *    - XMPP_CONN_FLAG_DISABLE_TLS
 *    - XMPP_CONN_FLAG_MANDATORY_TLS
 *    - XMPP_CONN_FLAG_LEGACY_SSL
 *    - XMPP_CONN_FLAG_TRUST_TLS
 *    - XMPP_CONN_FLAG_LEGACY_AUTH
 *
 *  @param conn a Strophe connection object
 *  @param flags ORed connection flags
 *
 *  @return XMPP_EOK (0) on success or a number less than 0 on failure
 *
 *  @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_conn_set_flags(conn: *mut xmpp_conn_t,
                                             mut flags: libc::c_long)
 -> libc::c_int {
    if (*conn).state as libc::c_uint !=
           XMPP_STATE_DISCONNECTED as libc::c_int as libc::c_uint {
        xmpp_error((*conn).ctx,
                   b"conn\x00" as *const u8 as *const libc::c_char,
                   b"Flags can be set only for disconnected connection\x00" as
                       *const u8 as *const libc::c_char);
        return -(2 as libc::c_int)
    }
    if flags as libc::c_ulong & (1 as libc::c_ulong) << 0 as libc::c_int != 0
           &&
           flags as libc::c_ulong &
               ((1 as libc::c_ulong) << 1 as libc::c_int |
                    (1 as libc::c_ulong) << 2 as libc::c_int |
                    (1 as libc::c_ulong) << 3 as libc::c_int) != 0 {
        xmpp_error((*conn).ctx,
                   b"conn\x00" as *const u8 as *const libc::c_char,
                   b"Flags 0x%04lx conflict\x00" as *const u8 as
                       *const libc::c_char, flags);
        return -(2 as libc::c_int)
    }
    (*conn).tls_disabled =
        if flags as libc::c_ulong & (1 as libc::c_ulong) << 0 as libc::c_int
               != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*conn).tls_mandatory =
        if flags as libc::c_ulong & (1 as libc::c_ulong) << 1 as libc::c_int
               != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*conn).tls_legacy_ssl =
        if flags as libc::c_ulong & (1 as libc::c_ulong) << 2 as libc::c_int
               != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*conn).tls_trust =
        if flags as libc::c_ulong & (1 as libc::c_ulong) << 3 as libc::c_int
               != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*conn).auth_legacy_enabled =
        if flags as libc::c_ulong & (1 as libc::c_ulong) << 4 as libc::c_int
               != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    return 0 as libc::c_int;
}
/* * Disable TLS for this connection, called by users of the library.
 *  Occasionally a server will be misconfigured to send the starttls
 *  feature, but will not support the handshake.
 *
 *  @param conn a Strophe connection object
 *
 *  @note this function is deprecated
 *  @see xmpp_conn_set_flags()
 *
 *  @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_conn_disable_tls(conn: *mut xmpp_conn_t) {
    let mut flags: libc::c_long = xmpp_conn_get_flags(conn);
    flags =
        (flags as libc::c_ulong | (1 as libc::c_ulong) << 0 as libc::c_int) as
            libc::c_long;
    xmpp_conn_set_flags(conn, flags);
}
/* * Return whether TLS session is established or not.
 *
 *  @return TRUE if TLS session is established and FALSE otherwise
 *
 *  @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_conn_is_secured(conn: *mut xmpp_conn_t)
 -> libc::c_int {
    return if (*conn).secured != 0 && (*conn).tls_failed == 0 &&
                  !(*conn).tls.is_null() {
               1 as libc::c_int
           } else { 0 as libc::c_int };
}
/* *
 *  @return TRUE if connection is in connecting state and FALSE otherwise
 *
 *  @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_conn_is_connecting(conn: *mut xmpp_conn_t)
 -> libc::c_int {
    return if (*conn).state as libc::c_uint ==
                  XMPP_STATE_CONNECTING as libc::c_int as libc::c_uint {
               1 as libc::c_int
           } else { 0 as libc::c_int };
}
/* *
 *  @return TRUE if connection is in connected state and FALSE otherwise
 *
 *  @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_conn_is_connected(conn: *mut xmpp_conn_t)
 -> libc::c_int {
    return if (*conn).state as libc::c_uint ==
                  XMPP_STATE_CONNECTED as libc::c_int as libc::c_uint {
               1 as libc::c_int
           } else { 0 as libc::c_int };
}
/* *
 *  @return TRUE if connection is in disconnected state and FALSE otherwise
 *
 *  @ingroup Connections
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_conn_is_disconnected(conn: *mut xmpp_conn_t)
 -> libc::c_int {
    return if (*conn).state as libc::c_uint ==
                  XMPP_STATE_DISCONNECTED as libc::c_int as libc::c_uint {
               1 as libc::c_int
           } else { 0 as libc::c_int };
}
/* 5 seconds */
/* timed handler for cleanup if normal disconnect procedure takes too long */
unsafe extern "C" fn _disconnect_cleanup(conn: *mut xmpp_conn_t,
                                         userdata: *mut libc::c_void)
 -> libc::c_int {
    xmpp_debug((*conn).ctx, b"xmpp\x00" as *const u8 as *const libc::c_char,
               b"disconnection forced by cleanup timeout\x00" as *const u8 as
                   *const libc::c_char);
    conn_disconnect(conn);
    return 0 as libc::c_int;
}
unsafe extern "C" fn _conn_build_stream_tag(conn: *mut xmpp_conn_t,
                                            mut attributes:
                                                *mut *mut libc::c_char,
                                            mut attributes_len: size_t)
 -> *mut libc::c_char {
    let mut tag: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut i: size_t = 0;
    static mut tag_head: *const libc::c_char =
        b"<stream:stream\x00" as *const u8 as *const libc::c_char;
    static mut tag_tail: *const libc::c_char =
        b">\x00" as *const u8 as *const libc::c_char;
    /* ignore the last element unless number is even */
    attributes_len &= !(1 as libc::c_int as size_t);
    len = strlen(tag_head).wrapping_add(strlen(tag_tail));
    i = 0 as libc::c_int as size_t;
    while i < attributes_len {
        len =
            (len as
                 libc::c_ulong).wrapping_add(strlen(*attributes.offset(i as
                                                                           isize)).wrapping_add(2
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_ulong))
                as size_t as size_t;
        i = i.wrapping_add(1)
    }
    tag =
        xmpp_alloc((*conn).ctx,
                   len.wrapping_add(1 as libc::c_int as libc::c_ulong)) as
            *mut libc::c_char;
    if tag.is_null() { return 0 as *mut libc::c_char }
    strcpy(tag, tag_head);
    i = 0 as libc::c_int as size_t;
    while i < attributes_len {
        if i & 1 as libc::c_int as libc::c_ulong ==
               0 as libc::c_int as libc::c_ulong {
            strcat(tag, b" \x00" as *const u8 as *const libc::c_char);
            strcat(tag, *attributes.offset(i as isize));
            strcat(tag, b"=\"\x00" as *const u8 as *const libc::c_char);
        } else {
            strcat(tag, *attributes.offset(i as isize));
            strcat(tag, b"\"\x00" as *const u8 as *const libc::c_char);
        }
        i = i.wrapping_add(1)
    }
    strcat(tag, tag_tail);
    if strlen(tag) != len {
        xmpp_error((*conn).ctx,
                   b"xmpp\x00" as *const u8 as *const libc::c_char,
                   b"Internal error in _conn_build_stream_tag().\x00" as
                       *const u8 as *const libc::c_char);
        xmpp_free((*conn).ctx, tag as *mut libc::c_void);
        tag = 0 as *mut libc::c_char
    }
    return tag;
}
unsafe extern "C" fn _conn_attributes_new(mut conn: *mut xmpp_conn_t,
                                          mut attrs: *mut *mut libc::c_char,
                                          mut attributes:
                                              *mut *mut *mut libc::c_char,
                                          mut attributes_len: *mut size_t) {
    let mut array: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut nr: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    if !attrs.is_null() {
        while !(*attrs.offset(nr as isize)).is_null() {
            nr = nr.wrapping_add(1)
        }
        array =
            xmpp_alloc((*conn).ctx,
                       (::std::mem::size_of::<*mut libc::c_char>() as
                            libc::c_ulong).wrapping_mul(nr)) as
                *mut *mut libc::c_char;
        i = 0 as libc::c_int as size_t;
        while !array.is_null() && i < nr {
            let ref mut fresh0 = *array.offset(i as isize);
            *fresh0 =
                if i & 1 as libc::c_int as libc::c_ulong ==
                       0 as libc::c_int as libc::c_ulong {
                    parser_attr_name((*conn).ctx, *attrs.offset(i as isize))
                } else {
                    xmpp_strdup((*conn).ctx, *attrs.offset(i as isize))
                };
            if (*array.offset(i as isize)).is_null() { break ; }
            i = i.wrapping_add(1)
        }
        if array.is_null() || i < nr {
            xmpp_error((*conn).ctx,
                       b"xmpp\x00" as *const u8 as *const libc::c_char,
                       b"Memory allocation error.\x00" as *const u8 as
                           *const libc::c_char);
            _conn_attributes_destroy(conn, array, i);
            array = 0 as *mut *mut libc::c_char;
            nr = 0 as libc::c_int as size_t
        }
    }
    *attributes = array;
    *attributes_len = nr;
}
unsafe extern "C" fn _conn_attributes_destroy(mut conn: *mut xmpp_conn_t,
                                              mut attributes:
                                                  *mut *mut libc::c_char,
                                              mut attributes_len: size_t) {
    let mut i: size_t = 0;
    if !attributes.is_null() {
        i = 0 as libc::c_int as size_t;
        while i < attributes_len {
            xmpp_free((*conn).ctx,
                      *attributes.offset(i as isize) as *mut libc::c_void);
            i = i.wrapping_add(1)
        }
        xmpp_free((*conn).ctx, attributes as *mut libc::c_void);
    };
}
unsafe extern "C" fn _log_open_tag(mut conn: *mut xmpp_conn_t,
                                   mut attrs: *mut *mut libc::c_char) {
    let mut attributes: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tag: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nr: size_t = 0;
    _conn_attributes_new(conn, attrs, &mut attributes, &mut nr);
    tag = _conn_build_stream_tag(conn, attributes, nr);
    if !tag.is_null() {
        xmpp_debug((*conn).ctx,
                   b"xmpp\x00" as *const u8 as *const libc::c_char,
                   b"RECV: %s\x00" as *const u8 as *const libc::c_char, tag);
        xmpp_free((*conn).ctx, tag as *mut libc::c_void);
    }
    _conn_attributes_destroy(conn, attributes, nr);
}
unsafe extern "C" fn _get_stream_attribute(mut attrs: *mut *mut libc::c_char,
                                           mut name: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    if attrs.is_null() { return 0 as *mut libc::c_char }
    i = 0 as libc::c_int;
    while !(*attrs.offset(i as isize)).is_null() {
        if strcmp(name, *attrs.offset(i as isize)) == 0 as libc::c_int {
            return *attrs.offset((i + 1 as libc::c_int) as isize)
        }
        i += 2 as libc::c_int
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn _handle_stream_start(mut name: *mut libc::c_char,
                                          mut attrs: *mut *mut libc::c_char,
                                          userdata: *mut libc::c_void) {
    let mut conn: *mut xmpp_conn_t = userdata as *mut xmpp_conn_t;
    let mut id: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut failed: libc::c_int = 0 as libc::c_int;
    if !(*conn).stream_id.is_null() {
        xmpp_free((*conn).ctx, (*conn).stream_id as *mut libc::c_void);
    }
    (*conn).stream_id = 0 as *mut libc::c_char;
    if strcmp(name, b"stream\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        _log_open_tag(conn, attrs);
        id =
            _get_stream_attribute(attrs,
                                  b"id\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char);
        if !id.is_null() { (*conn).stream_id = xmpp_strdup((*conn).ctx, id) }
        if !id.is_null() && (*conn).stream_id.is_null() {
            xmpp_error((*conn).ctx,
                       b"conn\x00" as *const u8 as *const libc::c_char,
                       b"Memory allocation failed.\x00" as *const u8 as
                           *const libc::c_char);
            failed = 1 as libc::c_int
        }
    } else {
        xmpp_error((*conn).ctx,
                   b"conn\x00" as *const u8 as *const libc::c_char,
                   b"Server did not open valid stream. name = %s.\x00" as
                       *const u8 as *const libc::c_char, name);
        failed = 1 as libc::c_int
    }
    if failed == 0 {
        /* call stream open handler */
        (*conn).open_handler.expect("non-null function pointer")(conn);
    } else { conn_disconnect(conn); };
}
unsafe extern "C" fn _handle_stream_end(mut name: *mut libc::c_char,
                                        userdata: *mut libc::c_void) {
    let mut conn: *mut xmpp_conn_t = userdata as *mut xmpp_conn_t;
    /* stream is over */
    xmpp_debug((*conn).ctx, b"xmpp\x00" as *const u8 as *const libc::c_char,
               b"RECV: </stream:stream>\x00" as *const u8 as
                   *const libc::c_char);
    conn_disconnect_clean(conn);
}
unsafe extern "C" fn _handle_stream_stanza(mut stanza: *mut xmpp_stanza_t,
                                           userdata: *mut libc::c_void) {
    let mut conn: *mut xmpp_conn_t = userdata as *mut xmpp_conn_t;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    if xmpp_stanza_to_text(stanza, &mut buf, &mut len) == 0 as libc::c_int {
        xmpp_debug((*conn).ctx,
                   b"xmpp\x00" as *const u8 as *const libc::c_char,
                   b"RECV: %s\x00" as *const u8 as *const libc::c_char, buf);
        xmpp_free((*conn).ctx, buf as *mut libc::c_void);
    }
    handler_fire_stanza(conn, stanza);
}
unsafe extern "C" fn _conn_default_port(conn: *mut xmpp_conn_t,
                                        mut type_0: xmpp_conn_type_t)
 -> libc::c_ushort {
    match type_0 as libc::c_uint {
        1 => {
            return if (*conn).tls_legacy_ssl != 0 {
                       XMPP_PORT_CLIENT_LEGACY_SSL as libc::c_int
                   } else { XMPP_PORT_CLIENT as libc::c_int } as
                       libc::c_ushort
        }
        2 => { return XMPP_PORT_COMPONENT as libc::c_int as libc::c_ushort }
        _ => { return 0 as libc::c_int as libc::c_ushort }
    };
}
unsafe extern "C" fn _conn_reset(conn: *mut xmpp_conn_t) {
    let mut ctx: *mut xmpp_ctx_t = (*conn).ctx;
    let mut sq: *mut xmpp_send_queue_t = 0 as *mut xmpp_send_queue_t;
    let mut tsq: *mut xmpp_send_queue_t = 0 as *mut xmpp_send_queue_t;
    if (*conn).state as libc::c_uint !=
           XMPP_STATE_DISCONNECTED as libc::c_int as libc::c_uint {
        xmpp_debug(ctx, b"conn\x00" as *const u8 as *const libc::c_char,
                   b"Can\'t reset connected object.\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    /* free queued */
    sq = (*conn).send_queue_head;
    while !sq.is_null() {
        tsq = sq;
        sq = (*sq).next;
        xmpp_free(ctx, (*tsq).data as *mut libc::c_void);
        xmpp_free(ctx, tsq as *mut libc::c_void);
    }
    (*conn).send_queue_head = 0 as *mut xmpp_send_queue_t;
    (*conn).send_queue_tail = 0 as *mut xmpp_send_queue_t;
    (*conn).send_queue_len = 0 as libc::c_int;
    if !(*conn).stream_error.is_null() {
        xmpp_stanza_release((*(*conn).stream_error).stanza);
        if !(*(*conn).stream_error).text.is_null() {
            xmpp_free(ctx, (*(*conn).stream_error).text as *mut libc::c_void);
        }
        xmpp_free(ctx, (*conn).stream_error as *mut libc::c_void);
        (*conn).stream_error = 0 as *mut xmpp_stream_error_t
    }
    if !(*conn).domain.is_null() {
        xmpp_free(ctx, (*conn).domain as *mut libc::c_void);
    }
    if !(*conn).bound_jid.is_null() {
        xmpp_free(ctx, (*conn).bound_jid as *mut libc::c_void);
    }
    if !(*conn).stream_id.is_null() {
        xmpp_free(ctx, (*conn).stream_id as *mut libc::c_void);
    }
    (*conn).domain = 0 as *mut libc::c_char;
    (*conn).bound_jid = 0 as *mut libc::c_char;
    (*conn).stream_id = 0 as *mut libc::c_char;
    (*conn).authenticated = 0 as libc::c_int;
    (*conn).secured = 0 as libc::c_int;
    (*conn).tls_failed = 0 as libc::c_int;
    (*conn).error = 0 as libc::c_int;
    (*conn).tls_support = 0 as libc::c_int;
    (*conn).bind_required = 0 as libc::c_int;
    (*conn).session_required = 0 as libc::c_int;
    handler_system_delete_all(conn);
}
unsafe extern "C" fn _conn_connect(conn: *mut xmpp_conn_t,
                                   domain: *const libc::c_char,
                                   host: *const libc::c_char,
                                   mut port: libc::c_ushort,
                                   mut type_0: xmpp_conn_type_t,
                                   mut callback: xmpp_conn_handler,
                                   userdata: *mut libc::c_void)
 -> libc::c_int {
    let mut open_handler: xmpp_open_handler = None;
    if (*conn).state as libc::c_uint !=
           XMPP_STATE_DISCONNECTED as libc::c_int as libc::c_uint {
        return -(2 as libc::c_int)
    }
    if type_0 as libc::c_uint != XMPP_CLIENT as libc::c_int as libc::c_uint &&
           type_0 as libc::c_uint !=
               XMPP_COMPONENT as libc::c_int as libc::c_uint {
        return -(2 as libc::c_int)
    }
    if host.is_null() || port as libc::c_int == 0 as libc::c_int {
        return -(3 as libc::c_int)
    }
    _conn_reset(conn);
    (*conn).type_0 = type_0;
    (*conn).domain = xmpp_strdup((*conn).ctx, domain);
    if (*conn).domain.is_null() { return -(1 as libc::c_int) }
    (*conn).sock = sock_connect(host, port);
    xmpp_debug((*conn).ctx, b"xmpp\x00" as *const u8 as *const libc::c_char,
               b"sock_connect() to %s:%u returned %d\x00" as *const u8 as
                   *const libc::c_char, host, port as libc::c_int,
               (*conn).sock);
    if (*conn).sock == -(1 as libc::c_int) { return -(3 as libc::c_int) }
    if (*conn).ka_timeout != 0 || (*conn).ka_interval != 0 {
        sock_set_keepalive((*conn).sock, (*conn).ka_timeout,
                           (*conn).ka_interval);
    }
    /* setup handler */
    (*conn).conn_handler = callback;
    (*conn).userdata = userdata;
    open_handler =
        if (*conn).is_raw != 0 {
            Some(auth_handle_open_stub as
                     unsafe extern "C" fn(_: *mut xmpp_conn_t) -> ())
        } else if type_0 as libc::c_uint ==
                      XMPP_CLIENT as libc::c_int as libc::c_uint {
            Some(auth_handle_open as
                     unsafe extern "C" fn(_: *mut xmpp_conn_t) -> ())
        } else {
            Some(auth_handle_component_open as
                     unsafe extern "C" fn(_: *mut xmpp_conn_t) -> ())
        };
    conn_prepare_reset(conn, open_handler);
    /* FIXME: it could happen that the connect returns immediately as
     * successful, though this is pretty unlikely.  This would be a little
     * hard to fix, since we'd have to detect and fire off the callback
     * from within the event loop */
    (*conn).state = XMPP_STATE_CONNECTING;
    (*conn).timeout_stamp = time_stamp();
    xmpp_debug((*conn).ctx, b"xmpp\x00" as *const u8 as *const libc::c_char,
               b"Attempting to connect to %s\x00" as *const u8 as
                   *const libc::c_char, host);
    return 0 as libc::c_int;
}
