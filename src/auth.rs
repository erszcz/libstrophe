use ::libc;
extern "C" {
    pub type _hash_t;
    pub type _parser_t;
    pub type _tls;
    pub type _xmpp_rand_t;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    /* free some blocks returned by other APIs, for example the
   buffer you get from xmpp_stanza_to_text */
    #[no_mangle]
    fn xmpp_free(ctx: *const xmpp_ctx_t, p: *mut libc::c_void);
    #[no_mangle]
    fn xmpp_conn_is_secured(conn: *mut xmpp_conn_t) -> libc::c_int;
    #[no_mangle]
    fn xmpp_disconnect(conn: *mut xmpp_conn_t);
    #[no_mangle]
    fn xmpp_send(conn: *mut xmpp_conn_t, stanza: *mut xmpp_stanza_t);
    #[no_mangle]
    fn xmpp_send_raw_string(conn: *mut xmpp_conn_t, fmt: *const libc::c_char,
                            _: ...);
    #[no_mangle]
    fn xmpp_timed_handler_delete(conn: *mut xmpp_conn_t,
                                 handler: xmpp_timed_handler);
    /*
void xmpp_register_stanza_handler(conn, stanza, xmlns, type, handler)
*/
    /* stanzas */
    /* allocate and initialize a blank stanza */
    #[no_mangle]
    fn xmpp_stanza_new(ctx: *mut xmpp_ctx_t) -> *mut xmpp_stanza_t;
    /* clone a stanza */
    #[no_mangle]
    fn xmpp_stanza_clone(stanza: *mut xmpp_stanza_t) -> *mut xmpp_stanza_t;
    /* copies a stanza and all children */
    #[no_mangle]
    fn xmpp_stanza_copy(stanza: *const xmpp_stanza_t) -> *mut xmpp_stanza_t;
    /* free a stanza object and it's contents */
    #[no_mangle]
    fn xmpp_stanza_release(stanza: *mut xmpp_stanza_t) -> libc::c_int;
    /* marshall a stanza into text for transmission or display */
    #[no_mangle]
    fn xmpp_stanza_to_text(stanza: *mut xmpp_stanza_t,
                           buf: *mut *mut libc::c_char, buflen: *mut size_t)
     -> libc::c_int;
    #[no_mangle]
    fn xmpp_stanza_get_children(stanza: *mut xmpp_stanza_t)
     -> *mut xmpp_stanza_t;
    #[no_mangle]
    fn xmpp_stanza_get_child_by_name(stanza: *mut xmpp_stanza_t,
                                     name: *const libc::c_char)
     -> *mut xmpp_stanza_t;
    #[no_mangle]
    fn xmpp_stanza_get_next(stanza: *mut xmpp_stanza_t) -> *mut xmpp_stanza_t;
    #[no_mangle]
    fn xmpp_stanza_add_child(stanza: *mut xmpp_stanza_t,
                             child: *mut xmpp_stanza_t) -> libc::c_int;
    /* concatenate all child text nodes.  this function
 * returns a string that must be freed by the caller */
    #[no_mangle]
    fn xmpp_stanza_get_text(stanza: *mut xmpp_stanza_t) -> *mut libc::c_char;
    #[no_mangle]
    fn xmpp_stanza_get_name(stanza: *mut xmpp_stanza_t)
     -> *const libc::c_char;
    /* set_attribute adds/replaces attributes */
    #[no_mangle]
    fn xmpp_stanza_set_attribute(stanza: *mut xmpp_stanza_t,
                                 key: *const libc::c_char,
                                 value: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn xmpp_stanza_set_name(stanza: *mut xmpp_stanza_t,
                            name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn xmpp_stanza_set_text(stanza: *mut xmpp_stanza_t,
                            text: *const libc::c_char) -> libc::c_int;
    /* common stanza helpers */
    #[no_mangle]
    fn xmpp_stanza_get_ns(stanza: *mut xmpp_stanza_t) -> *const libc::c_char;
    #[no_mangle]
    fn xmpp_stanza_get_type(stanza: *mut xmpp_stanza_t)
     -> *const libc::c_char;
    #[no_mangle]
    fn xmpp_stanza_set_ns(stanza: *mut xmpp_stanza_t, ns: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn xmpp_iq_new(ctx: *mut xmpp_ctx_t, type_0: *const libc::c_char,
                   id: *const libc::c_char) -> *mut xmpp_stanza_t;
    #[no_mangle]
    fn xmpp_jid_node(ctx: *mut xmpp_ctx_t, jid: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn xmpp_jid_resource(ctx: *mut xmpp_ctx_t, jid: *const libc::c_char)
     -> *mut libc::c_char;
    /* Base64 */
    #[no_mangle]
    fn xmpp_base64_encode(ctx: *mut xmpp_ctx_t, data: *const libc::c_uchar,
                          len: size_t) -> *mut libc::c_char;
    #[no_mangle]
    fn xmpp_base64_decode_str(ctx: *mut xmpp_ctx_t,
                              base64: *const libc::c_char, len: size_t)
     -> *mut libc::c_char;
    #[no_mangle]
    fn tls_new(conn: *mut xmpp_conn_t) -> *mut tls_t;
    #[no_mangle]
    fn tls_free(tls: *mut tls_t);
    /* * Generate a nonce that is printable randomized string.
 *  This function doesn't allocate memory and doesn't fail.
 *
 *  @param output A buffer where a NULL-terminated string will be placed.
 *                The string will contain len-1 printable symbols.
 *  @param len Number of bytes reserved for the output string, including
 *             end of line '\0'.
 *
 *  @ingroup Random
 */
    #[no_mangle]
    fn xmpp_rand_nonce(rand: *mut xmpp_rand_t, output: *mut libc::c_char,
                       len: size_t);
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
    fn xmpp_alloc(ctx: *const xmpp_ctx_t, size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xmpp_error(ctx: *const xmpp_ctx_t, area: *const libc::c_char,
                  fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn xmpp_warn(ctx: *const xmpp_ctx_t, area: *const libc::c_char,
                 fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn xmpp_debug(ctx: *const xmpp_ctx_t, area: *const libc::c_char,
                  fmt: *const libc::c_char, _: ...);
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
    #[no_mangle]
    fn handler_reset_timed(conn: *mut xmpp_conn_t, user_only: libc::c_int);
    #[no_mangle]
    fn conn_open_stream(conn: *mut xmpp_conn_t);
    #[no_mangle]
    fn conn_tls_start(conn: *mut xmpp_conn_t) -> libc::c_int;
    #[no_mangle]
    fn conn_prepare_reset(conn: *mut xmpp_conn_t, handler: xmpp_open_handler);
    #[no_mangle]
    fn conn_disconnect(conn: *mut xmpp_conn_t);
    #[no_mangle]
    fn handler_add_timed(conn: *mut xmpp_conn_t, handler: xmpp_timed_handler,
                         period: libc::c_ulong, userdata: *mut libc::c_void);
    #[no_mangle]
    fn handler_add_id(conn: *mut xmpp_conn_t, handler: xmpp_handler,
                      id: *const libc::c_char, userdata: *mut libc::c_void);
    #[no_mangle]
    fn handler_add(conn: *mut xmpp_conn_t, handler: xmpp_handler,
                   ns: *const libc::c_char, name: *const libc::c_char,
                   type_0: *const libc::c_char, userdata: *mut libc::c_void);
    /* utility functions */
    #[no_mangle]
    fn disconnect_mem_error(conn: *mut xmpp_conn_t);
    /* sasl.h
** strophe XMPP client library -- SASL authentication helpers
** 
** Copyright (C) 2005-2009 Collecta, Inc. 
**
**  This software is provided AS-IS with no warranty, either express
**  or implied.
**
**  This program is dual licensed under the MIT and GPLv3 licenses.
*/
    /* * @file
 * SASL authentication helpers.
 */
    /* * low-level sasl routines */
    #[no_mangle]
    fn sasl_plain(ctx: *mut xmpp_ctx_t, authid: *const libc::c_char,
                  password: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn sasl_digest_md5(ctx: *mut xmpp_ctx_t, challenge: *const libc::c_char,
                       jid: *const libc::c_char,
                       password: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn sasl_scram_sha1(ctx: *mut xmpp_ctx_t, challenge: *const libc::c_char,
                       first_bare: *const libc::c_char,
                       jid: *const libc::c_char,
                       password: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn crypto_SHA1_Init(context: *mut SHA1_CTX);
    #[no_mangle]
    fn crypto_SHA1_Update(context: *mut SHA1_CTX, data: *const uint8_t,
                          len: size_t);
    #[no_mangle]
    fn crypto_SHA1_Final(context: *mut SHA1_CTX, digest: *mut uint8_t);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
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
/* wrappers for xmpp_log at specific levels */
/* * connection **/
/* opaque connection object */
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
/* handlers */
/* if the handle returns false it is removed */
pub type xmpp_timed_handler
    =
    Option<unsafe extern "C" fn(_: *mut xmpp_conn_t, _: *mut libc::c_void)
               -> libc::c_int>;
/* if the handler returns false it is removed */
pub type xmpp_handler
    =
    Option<unsafe extern "C" fn(_: *mut xmpp_conn_t, _: *mut xmpp_stanza_t,
                                _: *mut libc::c_void) -> libc::c_int>;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA1_CTX {
    pub state: [uint32_t; 5],
    pub count: [uint32_t; 2],
    pub buffer: [uint8_t; 64],
}
/* stream:error handler */
unsafe extern "C" fn _handle_error(conn: *mut xmpp_conn_t,
                                   stanza: *mut xmpp_stanza_t,
                                   userdata: *mut libc::c_void)
 -> libc::c_int {
    let mut child: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    /* free old stream error if it's still there */
    if !(*conn).stream_error.is_null() {
        xmpp_stanza_release((*(*conn).stream_error).stanza);
        if !(*(*conn).stream_error).text.is_null() {
            xmpp_free((*conn).ctx,
                      (*(*conn).stream_error).text as *mut libc::c_void);
        }
        xmpp_free((*conn).ctx, (*conn).stream_error as *mut libc::c_void);
    }
    /* create stream error structure */
    (*conn).stream_error =
        xmpp_alloc((*conn).ctx,
                   ::std::mem::size_of::<xmpp_stream_error_t>() as
                       libc::c_ulong) as *mut xmpp_stream_error_t;
    (*(*conn).stream_error).text = 0 as *mut libc::c_char;
    (*(*conn).stream_error).type_0 = XMPP_SE_UNDEFINED_CONDITION;
    if !(*conn).stream_error.is_null() {
        child = xmpp_stanza_get_children(stanza);
        loop  {
            let mut ns: *const libc::c_char = 0 as *const libc::c_char;
            if !child.is_null() { ns = xmpp_stanza_get_ns(child) }
            if !ns.is_null() &&
                   strcmp(ns,
                          b"urn:ietf:params:xml:ns:xmpp-streams\x00" as
                              *const u8 as *const libc::c_char) ==
                       0 as libc::c_int {
                name = xmpp_stanza_get_name(child);
                if strcmp(name,
                          b"text\x00" as *const u8 as *const libc::c_char) ==
                       0 as libc::c_int {
                    if !(*(*conn).stream_error).text.is_null() {
                        xmpp_free((*conn).ctx,
                                  (*(*conn).stream_error).text as
                                      *mut libc::c_void);
                    }
                    (*(*conn).stream_error).text = xmpp_stanza_get_text(child)
                } else if strcmp(name,
                                 b"bad-format\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*(*conn).stream_error).type_0 = XMPP_SE_BAD_FORMAT
                } else if strcmp(name,
                                 b"bad-namespace-prefix\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*(*conn).stream_error).type_0 = XMPP_SE_BAD_NS_PREFIX
                } else if strcmp(name,
                                 b"conflict\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*(*conn).stream_error).type_0 = XMPP_SE_CONFLICT
                } else if strcmp(name,
                                 b"connection-timeout\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*(*conn).stream_error).type_0 = XMPP_SE_CONN_TIMEOUT
                } else if strcmp(name,
                                 b"host-gone\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*(*conn).stream_error).type_0 = XMPP_SE_HOST_GONE
                } else if strcmp(name,
                                 b"host-unknown\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*(*conn).stream_error).type_0 = XMPP_SE_HOST_UNKNOWN
                } else if strcmp(name,
                                 b"improper-addressing\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*(*conn).stream_error).type_0 = XMPP_SE_IMPROPER_ADDR
                } else if strcmp(name,
                                 b"internal-server-error\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*(*conn).stream_error).type_0 =
                        XMPP_SE_INTERNAL_SERVER_ERROR
                } else if strcmp(name,
                                 b"invalid-from\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*(*conn).stream_error).type_0 = XMPP_SE_INVALID_FROM
                } else if strcmp(name,
                                 b"invalid-id\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*(*conn).stream_error).type_0 = XMPP_SE_INVALID_ID
                } else if strcmp(name,
                                 b"invalid-namespace\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*(*conn).stream_error).type_0 = XMPP_SE_INVALID_NS
                } else if strcmp(name,
                                 b"invalid-xml\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*(*conn).stream_error).type_0 = XMPP_SE_INVALID_XML
                } else if strcmp(name,
                                 b"not-authorized\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*(*conn).stream_error).type_0 = XMPP_SE_NOT_AUTHORIZED
                } else if strcmp(name,
                                 b"policy-violation\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*(*conn).stream_error).type_0 = XMPP_SE_POLICY_VIOLATION
                } else if strcmp(name,
                                 b"remote-connection-failed\x00" as *const u8
                                     as *const libc::c_char) ==
                              0 as libc::c_int {
                    (*(*conn).stream_error).type_0 =
                        XMPP_SE_REMOTE_CONN_FAILED
                } else if strcmp(name,
                                 b"resource-constraint\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*(*conn).stream_error).type_0 =
                        XMPP_SE_RESOURCE_CONSTRAINT
                } else if strcmp(name,
                                 b"restricted-xml\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*(*conn).stream_error).type_0 = XMPP_SE_RESTRICTED_XML
                } else if strcmp(name,
                                 b"see-other-host\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*(*conn).stream_error).type_0 = XMPP_SE_SEE_OTHER_HOST
                } else if strcmp(name,
                                 b"system-shutdown\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*(*conn).stream_error).type_0 = XMPP_SE_SYSTEM_SHUTDOWN
                } else if strcmp(name,
                                 b"undefined-condition\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*(*conn).stream_error).type_0 =
                        XMPP_SE_UNDEFINED_CONDITION
                } else if strcmp(name,
                                 b"unsupported-encoding\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*(*conn).stream_error).type_0 =
                        XMPP_SE_UNSUPPORTED_ENCODING
                } else if strcmp(name,
                                 b"unsupported-stanza-type\x00" as *const u8
                                     as *const libc::c_char) ==
                              0 as libc::c_int {
                    (*(*conn).stream_error).type_0 =
                        XMPP_SE_UNSUPPORTED_STANZA_TYPE
                } else if strcmp(name,
                                 b"unsupported-version\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*(*conn).stream_error).type_0 =
                        XMPP_SE_UNSUPPORTED_VERSION
                } else if strcmp(name,
                                 b"xml-not-well-formed\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*(*conn).stream_error).type_0 =
                        XMPP_SE_XML_NOT_WELL_FORMED
                }
            }
            child = xmpp_stanza_get_next(child);
            if child.is_null() { break ; }
        }
        (*(*conn).stream_error).stanza = xmpp_stanza_clone(stanza)
    }
    return 1 as libc::c_int;
}
/* stream:features handlers */
unsafe extern "C" fn _handle_missing_features(conn: *mut xmpp_conn_t,
                                              userdata: *mut libc::c_void)
 -> libc::c_int {
    xmpp_debug((*conn).ctx, b"xmpp\x00" as *const u8 as *const libc::c_char,
               b"didn\'t get stream features\x00" as *const u8 as
                   *const libc::c_char);
    /* legacy auth will be attempted */
    _auth(conn);
    return 0 as libc::c_int;
}
unsafe extern "C" fn _handle_features(conn: *mut xmpp_conn_t,
                                      stanza: *mut xmpp_stanza_t,
                                      userdata: *mut libc::c_void)
 -> libc::c_int {
    let mut child: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut mech: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut ns: *const libc::c_char = 0 as *const libc::c_char;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    /* remove the handler that detects missing stream:features */
    xmpp_timed_handler_delete(conn,
                              Some(_handle_missing_features as
                                       unsafe extern "C" fn(_:
                                                                *mut xmpp_conn_t,
                                                            _:
                                                                *mut libc::c_void)
                                           -> libc::c_int));
    /* check for TLS */
    if (*conn).secured == 0 {
        if (*conn).tls_disabled == 0 {
            child =
                xmpp_stanza_get_child_by_name(stanza,
                                              b"starttls\x00" as *const u8 as
                                                  *const libc::c_char);
            if !child.is_null() {
                ns = xmpp_stanza_get_ns(child);
                (*conn).tls_support =
                    (!ns.is_null() &&
                         strcmp(ns,
                                b"urn:ietf:params:xml:ns:xmpp-tls\x00" as
                                    *const u8 as *const libc::c_char) ==
                             0 as libc::c_int) as libc::c_int
            }
        } else { (*conn).tls_support = 0 as libc::c_int }
    }
    /* check for SASL */
    child =
        xmpp_stanza_get_child_by_name(stanza,
                                      b"mechanisms\x00" as *const u8 as
                                          *const libc::c_char);
    ns =
        if !child.is_null() {
            xmpp_stanza_get_ns(child)
        } else { 0 as *const libc::c_char };
    if !child.is_null() && !ns.is_null() &&
           strcmp(ns,
                  b"urn:ietf:params:xml:ns:xmpp-sasl\x00" as *const u8 as
                      *const libc::c_char) == 0 as libc::c_int {
        mech = xmpp_stanza_get_children(child);
        while !mech.is_null() {
            if !xmpp_stanza_get_name(mech).is_null() &&
                   strcmp(xmpp_stanza_get_name(mech),
                          b"mechanism\x00" as *const u8 as
                              *const libc::c_char) == 0 as libc::c_int {
                text = xmpp_stanza_get_text(mech);
                if !text.is_null() {
                    if strcasecmp(text,
                                  b"PLAIN\x00" as *const u8 as
                                      *const libc::c_char) == 0 as libc::c_int
                       {
                        (*conn).sasl_support |=
                            (1 as libc::c_int) << 0 as libc::c_int
                    } else if strcasecmp(text,
                                         b"DIGEST-MD5\x00" as *const u8 as
                                             *const libc::c_char) ==
                                  0 as libc::c_int {
                        (*conn).sasl_support |=
                            (1 as libc::c_int) << 1 as libc::c_int
                    } else if strcasecmp(text,
                                         b"SCRAM-SHA-1\x00" as *const u8 as
                                             *const libc::c_char) ==
                                  0 as libc::c_int {
                        (*conn).sasl_support |=
                            (1 as libc::c_int) << 3 as libc::c_int
                    } else if strcasecmp(text,
                                         b"ANONYMOUS\x00" as *const u8 as
                                             *const libc::c_char) ==
                                  0 as libc::c_int {
                        (*conn).sasl_support |=
                            (1 as libc::c_int) << 2 as libc::c_int
                    }
                    xmpp_free((*conn).ctx, text as *mut libc::c_void);
                }
            }
            mech = xmpp_stanza_get_next(mech)
        }
    }
    /* Disable PLAIN when other secure mechanisms are supported */
    if (*conn).sasl_support &
           !((1 as libc::c_int) << 0 as libc::c_int |
                 (1 as libc::c_int) << 2 as libc::c_int) != 0 {
        (*conn).sasl_support &= !((1 as libc::c_int) << 0 as libc::c_int)
    }
    _auth(conn);
    return 0 as libc::c_int;
}
/* returns the correct auth id for a component or a client.
 * returned string must be freed by caller */
unsafe extern "C" fn _get_authid(conn: *mut xmpp_conn_t)
 -> *mut libc::c_char {
    let mut authid: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*conn).type_0 as libc::c_uint ==
           XMPP_CLIENT as libc::c_int as libc::c_uint {
        /* authid is the node portion of jid */
        if (*conn).jid.is_null() { return 0 as *mut libc::c_char }
        authid = xmpp_jid_node((*conn).ctx, (*conn).jid)
    }
    return authid;
}
unsafe extern "C" fn _handle_proceedtls_default(conn: *mut xmpp_conn_t,
                                                stanza: *mut xmpp_stanza_t,
                                                userdata: *mut libc::c_void)
 -> libc::c_int {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    name = xmpp_stanza_get_name(stanza);
    xmpp_debug((*conn).ctx, b"xmpp\x00" as *const u8 as *const libc::c_char,
               b"handle proceedtls called for %s\x00" as *const u8 as
                   *const libc::c_char, name);
    if strcmp(name, b"proceed\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        xmpp_debug((*conn).ctx,
                   b"xmpp\x00" as *const u8 as *const libc::c_char,
                   b"proceeding with TLS\x00" as *const u8 as
                       *const libc::c_char);
        if conn_tls_start(conn) == 0 as libc::c_int {
            conn_prepare_reset(conn,
                               Some(_handle_open_tls as
                                        unsafe extern "C" fn(_:
                                                                 *mut xmpp_conn_t)
                                            -> ()));
            conn_open_stream(conn);
        } else {
            /* failed tls spoils the connection, so disconnect */
            xmpp_disconnect(conn);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _handle_sasl_result(conn: *mut xmpp_conn_t,
                                         stanza: *mut xmpp_stanza_t,
                                         userdata: *mut libc::c_void)
 -> libc::c_int {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    name = xmpp_stanza_get_name(stanza);
    /* the server should send a <success> or <failure> stanza */
    if strcmp(name, b"failure\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        xmpp_debug((*conn).ctx,
                   b"xmpp\x00" as *const u8 as *const libc::c_char,
                   b"SASL %s auth failed\x00" as *const u8 as
                       *const libc::c_char, userdata as *mut libc::c_char);
        /* fall back to next auth method */
        _auth(conn);
    } else if strcmp(name, b"success\x00" as *const u8 as *const libc::c_char)
                  == 0 as libc::c_int {
        /* SASL auth successful, we need to restart the stream */
        xmpp_debug((*conn).ctx,
                   b"xmpp\x00" as *const u8 as *const libc::c_char,
                   b"SASL %s auth successful\x00" as *const u8 as
                       *const libc::c_char, userdata as *mut libc::c_char);
        /* reset parser */
        conn_prepare_reset(conn,
                           Some(_handle_open_sasl as
                                    unsafe extern "C" fn(_: *mut xmpp_conn_t)
                                        -> ()));
        /* send stream tag */
        conn_open_stream(conn);
    } else {
        /* got unexpected reply */
        xmpp_error((*conn).ctx,
                   b"xmpp\x00" as *const u8 as *const libc::c_char,
                   b"Got unexpected reply to SASL %sauthentication.\x00" as
                       *const u8 as *const libc::c_char,
                   userdata as *mut libc::c_char);
        xmpp_disconnect(conn);
    }
    return 0 as libc::c_int;
}
/* handle the challenge phase of digest auth */
unsafe extern "C" fn _handle_digestmd5_challenge(conn: *mut xmpp_conn_t,
                                                 stanza: *mut xmpp_stanza_t,
                                                 userdata: *mut libc::c_void)
 -> libc::c_int {
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut response: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut auth: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut authdata: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    name = xmpp_stanza_get_name(stanza);
    xmpp_debug((*conn).ctx, b"xmpp\x00" as *const u8 as *const libc::c_char,
               b"handle digest-md5 (challenge) called for %s\x00" as *const u8
                   as *const libc::c_char, name);
    if strcmp(name, b"challenge\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        text = xmpp_stanza_get_text(stanza);
        response =
            sasl_digest_md5((*conn).ctx, text, (*conn).jid, (*conn).pass);
        if response.is_null() {
            disconnect_mem_error(conn);
            return 0 as libc::c_int
        }
        xmpp_free((*conn).ctx, text as *mut libc::c_void);
        auth = xmpp_stanza_new((*conn).ctx);
        if auth.is_null() {
            disconnect_mem_error(conn);
            return 0 as libc::c_int
        }
        xmpp_stanza_set_name(auth,
                             b"response\x00" as *const u8 as
                                 *const libc::c_char);
        xmpp_stanza_set_ns(auth,
                           b"urn:ietf:params:xml:ns:xmpp-sasl\x00" as
                               *const u8 as *const libc::c_char);
        authdata = xmpp_stanza_new((*conn).ctx);
        if authdata.is_null() {
            disconnect_mem_error(conn);
            return 0 as libc::c_int
        }
        xmpp_stanza_set_text(authdata, response);
        xmpp_free((*conn).ctx, response as *mut libc::c_void);
        xmpp_stanza_add_child(auth, authdata);
        xmpp_stanza_release(authdata);
        handler_add(conn,
                    Some(_handle_digestmd5_rspauth as
                             unsafe extern "C" fn(_: *mut xmpp_conn_t,
                                                  _: *mut xmpp_stanza_t,
                                                  _: *mut libc::c_void)
                                 -> libc::c_int),
                    b"urn:ietf:params:xml:ns:xmpp-sasl\x00" as *const u8 as
                        *const libc::c_char, 0 as *const libc::c_char,
                    0 as *const libc::c_char, 0 as *mut libc::c_void);
        xmpp_send(conn, auth);
        xmpp_stanza_release(auth);
    } else {
        return _handle_sasl_result(conn, stanza,
                                   b"DIGEST-MD5\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_void)
    }
    /* remove ourselves */
    return 0 as libc::c_int;
}
/* handle the rspauth phase of digest auth */
unsafe extern "C" fn _handle_digestmd5_rspauth(conn: *mut xmpp_conn_t,
                                               stanza: *mut xmpp_stanza_t,
                                               userdata: *mut libc::c_void)
 -> libc::c_int {
    let mut auth: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    name = xmpp_stanza_get_name(stanza);
    xmpp_debug((*conn).ctx, b"xmpp\x00" as *const u8 as *const libc::c_char,
               b"handle digest-md5 (rspauth) called for %s\x00" as *const u8
                   as *const libc::c_char, name);
    if strcmp(name, b"challenge\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        /* assume it's an rspauth response */
        auth = xmpp_stanza_new((*conn).ctx);
        if auth.is_null() {
            disconnect_mem_error(conn);
            return 0 as libc::c_int
        }
        xmpp_stanza_set_name(auth,
                             b"response\x00" as *const u8 as
                                 *const libc::c_char);
        xmpp_stanza_set_ns(auth,
                           b"urn:ietf:params:xml:ns:xmpp-sasl\x00" as
                               *const u8 as *const libc::c_char);
        xmpp_send(conn, auth);
        xmpp_stanza_release(auth);
    } else {
        return _handle_sasl_result(conn, stanza,
                                   b"DIGEST-MD5\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_void)
    }
    return 1 as libc::c_int;
}
/* handle the challenge phase of SCRAM-SHA-1 auth */
unsafe extern "C" fn _handle_scram_sha1_challenge(conn: *mut xmpp_conn_t,
                                                  stanza: *mut xmpp_stanza_t,
                                                  userdata: *mut libc::c_void)
 -> libc::c_int {
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut response: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut auth: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut authdata: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut challenge: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut scram_init: *mut libc::c_char = userdata as *mut libc::c_char;
    name = xmpp_stanza_get_name(stanza);
    xmpp_debug((*conn).ctx, b"xmpp\x00" as *const u8 as *const libc::c_char,
               b"handle SCRAM-SHA-1 (challenge) called for %s\x00" as
                   *const u8 as *const libc::c_char, name);
    if strcmp(name, b"challenge\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        text = xmpp_stanza_get_text(stanza);
        if !text.is_null() {
            challenge =
                xmpp_base64_decode_str((*conn).ctx, text, strlen(text));
            xmpp_free((*conn).ctx, text as *mut libc::c_void);
            if !challenge.is_null() {
                response =
                    sasl_scram_sha1((*conn).ctx, challenge, scram_init,
                                    (*conn).jid, (*conn).pass);
                xmpp_free((*conn).ctx, challenge as *mut libc::c_void);
                if !response.is_null() {
                    auth = xmpp_stanza_new((*conn).ctx);
                    if !auth.is_null() {
                        xmpp_stanza_set_name(auth,
                                             b"response\x00" as *const u8 as
                                                 *const libc::c_char);
                        xmpp_stanza_set_ns(auth,
                                           b"urn:ietf:params:xml:ns:xmpp-sasl\x00"
                                               as *const u8 as
                                               *const libc::c_char);
                        authdata = xmpp_stanza_new((*conn).ctx);
                        if authdata.is_null() {
                            xmpp_stanza_release(auth);
                        } else {
                            xmpp_stanza_set_text(authdata, response);
                            xmpp_free((*conn).ctx,
                                      response as *mut libc::c_void);
                            xmpp_stanza_add_child(auth, authdata);
                            xmpp_stanza_release(authdata);
                            xmpp_send(conn, auth);
                            xmpp_stanza_release(auth);
                            return 1 as libc::c_int
                        }
                    }
                    xmpp_free((*conn).ctx, response as *mut libc::c_void);
                }
            }
        }
        xmpp_free((*conn).ctx, scram_init as *mut libc::c_void);
        disconnect_mem_error(conn);
        return 0 as libc::c_int
    } else {
        xmpp_free((*conn).ctx, scram_init as *mut libc::c_void);
        return _handle_sasl_result(conn, stanza,
                                   b"SCRAM-SHA-1\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_void)
    };
}
unsafe extern "C" fn _make_scram_sha1_init_msg(conn: *mut xmpp_conn_t)
 -> *mut libc::c_char {
    let mut ctx: *mut xmpp_ctx_t = (*conn).ctx;
    let mut message_len: size_t = 0;
    let mut node: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut message: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nonce: [libc::c_char; 32] = [0; 32];
    node = xmpp_jid_node(ctx, (*conn).jid);
    if node.is_null() { return 0 as *mut libc::c_char }
    xmpp_rand_nonce((*ctx).rand, nonce.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 32]>() as
                        libc::c_ulong);
    message_len =
        strlen(node).wrapping_add(strlen(nonce.as_mut_ptr())).wrapping_add(8
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_ulong).wrapping_add(1
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong);
    message = xmpp_alloc(ctx, message_len) as *mut libc::c_char;
    if !message.is_null() {
        xmpp_snprintf(message, message_len,
                      b"n,,n=%s,r=%s\x00" as *const u8 as *const libc::c_char,
                      node, nonce.as_mut_ptr());
    }
    xmpp_free(ctx, node as *mut libc::c_void);
    return message;
}
unsafe extern "C" fn _make_starttls(conn: *mut xmpp_conn_t)
 -> *mut xmpp_stanza_t {
    let mut starttls: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    /* build start stanza */
    starttls = xmpp_stanza_new((*conn).ctx);
    if !starttls.is_null() {
        xmpp_stanza_set_name(starttls,
                             b"starttls\x00" as *const u8 as
                                 *const libc::c_char);
        xmpp_stanza_set_ns(starttls,
                           b"urn:ietf:params:xml:ns:xmpp-tls\x00" as *const u8
                               as *const libc::c_char);
    }
    return starttls;
}
unsafe extern "C" fn _make_sasl_auth(conn: *mut xmpp_conn_t,
                                     mechanism: *const libc::c_char)
 -> *mut xmpp_stanza_t {
    let mut auth: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    /* build auth stanza */
    auth = xmpp_stanza_new((*conn).ctx);
    if !auth.is_null() {
        xmpp_stanza_set_name(auth,
                             b"auth\x00" as *const u8 as *const libc::c_char);
        xmpp_stanza_set_ns(auth,
                           b"urn:ietf:params:xml:ns:xmpp-sasl\x00" as
                               *const u8 as *const libc::c_char);
        xmpp_stanza_set_attribute(auth,
                                  b"mechanism\x00" as *const u8 as
                                      *const libc::c_char, mechanism);
    }
    return auth;
}
/* 15 seconds */
/* authenticate the connection
 * this may get called multiple times.  if any auth method fails,
 * this will get called again until one auth method succeeds or every
 * method fails
 */
unsafe extern "C" fn _auth(conn: *mut xmpp_conn_t) {
    let mut auth: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut authdata: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut authid: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut scram_init: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut anonjid: libc::c_int = 0;
    /* if there is no node in conn->jid, we assume anonymous connect */
    str = xmpp_jid_node((*conn).ctx, (*conn).jid);
    if str.is_null() {
        anonjid = 1 as libc::c_int
    } else {
        xmpp_free((*conn).ctx, str as *mut libc::c_void);
        anonjid = 0 as libc::c_int
    }
    if (*conn).tls_support != 0 {
        let mut tls: *mut tls_t = tls_new(conn);
        /* If we couldn't init tls, it isn't there, so go on */
        if tls.is_null() {
            (*conn).tls_support = 0 as libc::c_int;
            _auth(conn);
            return
        } else { tls_free(tls); }
        auth = _make_starttls(conn);
        if auth.is_null() { disconnect_mem_error(conn); return }
        handler_add(conn,
                    Some(_handle_proceedtls_default as
                             unsafe extern "C" fn(_: *mut xmpp_conn_t,
                                                  _: *mut xmpp_stanza_t,
                                                  _: *mut libc::c_void)
                                 -> libc::c_int),
                    b"urn:ietf:params:xml:ns:xmpp-tls\x00" as *const u8 as
                        *const libc::c_char, 0 as *const libc::c_char,
                    0 as *const libc::c_char, 0 as *mut libc::c_void);
        xmpp_send(conn, auth);
        xmpp_stanza_release(auth);
        /* TLS was tried, unset flag */
        (*conn).tls_support = 0 as libc::c_int;
        /* _auth() will be called later */
        return
    }
    if (*conn).tls_mandatory != 0 && xmpp_conn_is_secured(conn) == 0 {
        xmpp_error((*conn).ctx,
                   b"xmpp\x00" as *const u8 as *const libc::c_char,
                   b"TLS is not supported, but set as mandatory for this connection\x00"
                       as *const u8 as *const libc::c_char);
        conn_disconnect(conn);
        return
    }
    if anonjid != 0 &&
           (*conn).sasl_support & (1 as libc::c_int) << 2 as libc::c_int != 0
       {
        /* some crap here */
        auth =
            _make_sasl_auth(conn,
                            b"ANONYMOUS\x00" as *const u8 as
                                *const libc::c_char);
        if auth.is_null() { disconnect_mem_error(conn); return }
        handler_add(conn,
                    Some(_handle_sasl_result as
                             unsafe extern "C" fn(_: *mut xmpp_conn_t,
                                                  _: *mut xmpp_stanza_t,
                                                  _: *mut libc::c_void)
                                 -> libc::c_int),
                    b"urn:ietf:params:xml:ns:xmpp-sasl\x00" as *const u8 as
                        *const libc::c_char, 0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    b"ANONYMOUS\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_void);
        xmpp_send(conn, auth);
        xmpp_stanza_release(auth);
        /* SASL ANONYMOUS was tried, unset flag */
        (*conn).sasl_support &= !((1 as libc::c_int) << 2 as libc::c_int)
    } else if anonjid != 0 {
        xmpp_error((*conn).ctx,
                   b"auth\x00" as *const u8 as *const libc::c_char,
                   b"No node in JID, and SASL ANONYMOUS unsupported.\x00" as
                       *const u8 as *const libc::c_char);
        xmpp_disconnect(conn);
    } else if (*conn).sasl_support & (1 as libc::c_int) << 3 as libc::c_int !=
                  0 {
        auth =
            _make_sasl_auth(conn,
                            b"SCRAM-SHA-1\x00" as *const u8 as
                                *const libc::c_char);
        if auth.is_null() { disconnect_mem_error(conn); return }
        /* don't free scram_init on success */
        scram_init = _make_scram_sha1_init_msg(conn);
        if scram_init.is_null() {
            xmpp_stanza_release(auth);
            disconnect_mem_error(conn);
            return
        }
        str =
            xmpp_base64_encode((*conn).ctx, scram_init as *mut libc::c_uchar,
                               strlen(scram_init));
        if str.is_null() {
            xmpp_free((*conn).ctx, scram_init as *mut libc::c_void);
            xmpp_stanza_release(auth);
            disconnect_mem_error(conn);
            return
        }
        authdata = xmpp_stanza_new((*conn).ctx);
        if authdata.is_null() {
            xmpp_free((*conn).ctx, str as *mut libc::c_void);
            xmpp_free((*conn).ctx, scram_init as *mut libc::c_void);
            xmpp_stanza_release(auth);
            disconnect_mem_error(conn);
            return
        }
        xmpp_stanza_set_text(authdata, str);
        xmpp_free((*conn).ctx, str as *mut libc::c_void);
        xmpp_stanza_add_child(auth, authdata);
        xmpp_stanza_release(authdata);
        handler_add(conn,
                    Some(_handle_scram_sha1_challenge as
                             unsafe extern "C" fn(_: *mut xmpp_conn_t,
                                                  _: *mut xmpp_stanza_t,
                                                  _: *mut libc::c_void)
                                 -> libc::c_int),
                    b"urn:ietf:params:xml:ns:xmpp-sasl\x00" as *const u8 as
                        *const libc::c_char, 0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    scram_init as *mut libc::c_void);
        xmpp_send(conn, auth);
        xmpp_stanza_release(auth);
        /* SASL SCRAM-SHA-1 was tried, unset flag */
        (*conn).sasl_support &= !((1 as libc::c_int) << 3 as libc::c_int)
    } else if (*conn).sasl_support & (1 as libc::c_int) << 1 as libc::c_int !=
                  0 {
        auth =
            _make_sasl_auth(conn,
                            b"DIGEST-MD5\x00" as *const u8 as
                                *const libc::c_char);
        if auth.is_null() { disconnect_mem_error(conn); return }
        handler_add(conn,
                    Some(_handle_digestmd5_challenge as
                             unsafe extern "C" fn(_: *mut xmpp_conn_t,
                                                  _: *mut xmpp_stanza_t,
                                                  _: *mut libc::c_void)
                                 -> libc::c_int),
                    b"urn:ietf:params:xml:ns:xmpp-sasl\x00" as *const u8 as
                        *const libc::c_char, 0 as *const libc::c_char,
                    0 as *const libc::c_char, 0 as *mut libc::c_void);
        xmpp_send(conn, auth);
        xmpp_stanza_release(auth);
        /* SASL DIGEST-MD5 was tried, unset flag */
        (*conn).sasl_support &= !((1 as libc::c_int) << 1 as libc::c_int)
    } else if (*conn).sasl_support & (1 as libc::c_int) << 0 as libc::c_int !=
                  0 {
        auth =
            _make_sasl_auth(conn,
                            b"PLAIN\x00" as *const u8 as *const libc::c_char);
        if auth.is_null() { disconnect_mem_error(conn); return }
        authdata = xmpp_stanza_new((*conn).ctx);
        if authdata.is_null() { disconnect_mem_error(conn); return }
        authid = _get_authid(conn);
        if authid.is_null() { disconnect_mem_error(conn); return }
        str = sasl_plain((*conn).ctx, authid, (*conn).pass);
        if str.is_null() { disconnect_mem_error(conn); return }
        xmpp_stanza_set_text(authdata, str);
        xmpp_free((*conn).ctx, str as *mut libc::c_void);
        xmpp_free((*conn).ctx, authid as *mut libc::c_void);
        xmpp_stanza_add_child(auth, authdata);
        xmpp_stanza_release(authdata);
        handler_add(conn,
                    Some(_handle_sasl_result as
                             unsafe extern "C" fn(_: *mut xmpp_conn_t,
                                                  _: *mut xmpp_stanza_t,
                                                  _: *mut libc::c_void)
                                 -> libc::c_int),
                    b"urn:ietf:params:xml:ns:xmpp-sasl\x00" as *const u8 as
                        *const libc::c_char, 0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    b"PLAIN\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_void);
        xmpp_send(conn, auth);
        xmpp_stanza_release(auth);
        /* SASL PLAIN was tried */
        (*conn).sasl_support &= !((1 as libc::c_int) << 0 as libc::c_int)
    } else if (*conn).type_0 as libc::c_uint ==
                  XMPP_CLIENT as libc::c_int as libc::c_uint &&
                  (*conn).auth_legacy_enabled != 0 {
        /* legacy client authentication */
        _auth_legacy(conn);
    } else {
        xmpp_error((*conn).ctx,
                   b"auth\x00" as *const u8 as *const libc::c_char,
                   b"Cannot authenticate with known methods\x00" as *const u8
                       as *const libc::c_char);
        xmpp_disconnect(conn);
    };
}
/* auth functions */
/* * Set up handlers at stream start.
 *  This function is called internally to Strophe for handling the opening
 *  of an XMPP stream.  It's called by the parser when a stream is opened
 *  or reset, and adds the initial handlers for <stream:error/> and
 *  <stream:features/>.  This function is not intended for use outside
 *  of Strophe.
 *
 *  @param conn a Strophe connection object
 */
#[no_mangle]
pub unsafe extern "C" fn auth_handle_open(conn: *mut xmpp_conn_t) {
    /* reset all timed handlers */
    handler_reset_timed(conn, 0 as libc::c_int);
    /* setup handler for stream:error, we will keep this handler
     * for reopened streams until connection is disconnected */
    handler_add(conn,
                Some(_handle_error as
                         unsafe extern "C" fn(_: *mut xmpp_conn_t,
                                              _: *mut xmpp_stanza_t,
                                              _: *mut libc::c_void)
                             -> libc::c_int),
                b"http://etherx.jabber.org/streams\x00" as *const u8 as
                    *const libc::c_char,
                b"error\x00" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char, 0 as *mut libc::c_void);
    /* setup handlers for incoming <stream:features> */
    handler_add(conn,
                Some(_handle_features as
                         unsafe extern "C" fn(_: *mut xmpp_conn_t,
                                              _: *mut xmpp_stanza_t,
                                              _: *mut libc::c_void)
                             -> libc::c_int),
                b"http://etherx.jabber.org/streams\x00" as *const u8 as
                    *const libc::c_char,
                b"features\x00" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char, 0 as *mut libc::c_void);
    handler_add_timed(conn,
                      Some(_handle_missing_features as
                               unsafe extern "C" fn(_: *mut xmpp_conn_t,
                                                    _: *mut libc::c_void)
                                   -> libc::c_int),
                      15000 as libc::c_int as libc::c_ulong,
                      0 as *mut libc::c_void);
}
/* called when stream:stream tag received after TLS establishment */
unsafe extern "C" fn _handle_open_tls(conn: *mut xmpp_conn_t) {
    /* setup handlers for incoming <stream:features> */
    handler_add(conn,
                Some(_handle_features as
                         unsafe extern "C" fn(_: *mut xmpp_conn_t,
                                              _: *mut xmpp_stanza_t,
                                              _: *mut libc::c_void)
                             -> libc::c_int),
                b"http://etherx.jabber.org/streams\x00" as *const u8 as
                    *const libc::c_char,
                b"features\x00" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char, 0 as *mut libc::c_void);
    handler_add_timed(conn,
                      Some(_handle_missing_features as
                               unsafe extern "C" fn(_: *mut xmpp_conn_t,
                                                    _: *mut libc::c_void)
                                   -> libc::c_int),
                      15000 as libc::c_int as libc::c_ulong,
                      0 as *mut libc::c_void);
}
/* called when stream:stream tag received after SASL auth */
unsafe extern "C" fn _handle_open_sasl(conn: *mut xmpp_conn_t) {
    xmpp_debug((*conn).ctx, b"xmpp\x00" as *const u8 as *const libc::c_char,
               b"Reopened stream successfully.\x00" as *const u8 as
                   *const libc::c_char);
    /* setup stream:features handlers */
    handler_add(conn,
                Some(_handle_features_sasl as
                         unsafe extern "C" fn(_: *mut xmpp_conn_t,
                                              _: *mut xmpp_stanza_t,
                                              _: *mut libc::c_void)
                             -> libc::c_int),
                b"http://etherx.jabber.org/streams\x00" as *const u8 as
                    *const libc::c_char,
                b"features\x00" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char, 0 as *mut libc::c_void);
    handler_add_timed(conn,
                      Some(_handle_missing_features_sasl as
                               unsafe extern "C" fn(_: *mut xmpp_conn_t,
                                                    _: *mut libc::c_void)
                                   -> libc::c_int),
                      15000 as libc::c_int as libc::c_ulong,
                      0 as *mut libc::c_void);
}
unsafe extern "C" fn _handle_features_sasl(conn: *mut xmpp_conn_t,
                                           stanza: *mut xmpp_stanza_t,
                                           userdata: *mut libc::c_void)
 -> libc::c_int {
    let mut bind: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut session: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut iq: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut res: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut text: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut opt: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut ns: *const libc::c_char = 0 as *const libc::c_char;
    let mut resource: *mut libc::c_char = 0 as *mut libc::c_char;
    /* remove missing features handler */
    xmpp_timed_handler_delete(conn,
                              Some(_handle_missing_features_sasl as
                                       unsafe extern "C" fn(_:
                                                                *mut xmpp_conn_t,
                                                            _:
                                                                *mut libc::c_void)
                                           -> libc::c_int));
    /* we are expecting <bind/> and <session/> since this is a
       XMPP style connection */
    /* check whether resource binding is required */
    bind =
        xmpp_stanza_get_child_by_name(stanza,
                                      b"bind\x00" as *const u8 as
                                          *const libc::c_char);
    if !bind.is_null() {
        ns = xmpp_stanza_get_ns(bind);
        (*conn).bind_required =
            (!ns.is_null() &&
                 strcmp(ns,
                        b"urn:ietf:params:xml:ns:xmpp-bind\x00" as *const u8
                            as *const libc::c_char) == 0 as libc::c_int) as
                libc::c_int
    }
    /* check whether session establishment is required */
    session =
        xmpp_stanza_get_child_by_name(stanza,
                                      b"session\x00" as *const u8 as
                                          *const libc::c_char);
    if !session.is_null() {
        ns = xmpp_stanza_get_ns(session);
        opt =
            xmpp_stanza_get_child_by_name(session,
                                          b"optional\x00" as *const u8 as
                                              *const libc::c_char);
        if opt.is_null() {
            (*conn).session_required =
                (!ns.is_null() &&
                     strcmp(ns,
                            b"urn:ietf:params:xml:ns:xmpp-session\x00" as
                                *const u8 as *const libc::c_char) ==
                         0 as libc::c_int) as libc::c_int
        }
    }
    /* if bind is required, go ahead and start it */
    if (*conn).bind_required != 0 {
        /* bind resource */
        /* setup response handlers */
        handler_add_id(conn,
                       Some(_handle_bind as
                                unsafe extern "C" fn(_: *mut xmpp_conn_t,
                                                     _: *mut xmpp_stanza_t,
                                                     _: *mut libc::c_void)
                                    -> libc::c_int),
                       b"_xmpp_bind1\x00" as *const u8 as *const libc::c_char,
                       0 as *mut libc::c_void);
        handler_add_timed(conn,
                          Some(_handle_missing_bind as
                                   unsafe extern "C" fn(_: *mut xmpp_conn_t,
                                                        _: *mut libc::c_void)
                                       -> libc::c_int),
                          15000 as libc::c_int as libc::c_ulong,
                          0 as *mut libc::c_void);
        /* send bind request */
        iq =
            xmpp_iq_new((*conn).ctx,
                        b"set\x00" as *const u8 as *const libc::c_char,
                        b"_xmpp_bind1\x00" as *const u8 as
                            *const libc::c_char);
        if iq.is_null() {
            disconnect_mem_error(conn);
            return 0 as libc::c_int
        }
        bind = xmpp_stanza_copy(bind);
        if bind.is_null() {
            xmpp_stanza_release(iq);
            disconnect_mem_error(conn);
            return 0 as libc::c_int
        }
        /* request a specific resource if we have one */
        resource = xmpp_jid_resource((*conn).ctx, (*conn).jid);
        if !resource.is_null() &&
               strlen(resource) == 0 as libc::c_int as libc::c_ulong {
            /* jabberd2 doesn't handle an empty resource */
            xmpp_free((*conn).ctx, resource as *mut libc::c_void);
            resource = 0 as *mut libc::c_char
        }
        /* if we have a resource to request, do it. otherwise the
	   server will assign us one */
        if !resource.is_null() {
            res = xmpp_stanza_new((*conn).ctx);
            if res.is_null() {
                xmpp_stanza_release(bind);
                xmpp_stanza_release(iq);
                disconnect_mem_error(conn);
                return 0 as libc::c_int
            }
            xmpp_stanza_set_name(res,
                                 b"resource\x00" as *const u8 as
                                     *const libc::c_char);
            text = xmpp_stanza_new((*conn).ctx);
            if text.is_null() {
                xmpp_stanza_release(res);
                xmpp_stanza_release(bind);
                xmpp_stanza_release(iq);
                disconnect_mem_error(conn);
                return 0 as libc::c_int
            }
            xmpp_stanza_set_text(text, resource);
            xmpp_stanza_add_child(res, text);
            xmpp_stanza_release(text);
            xmpp_stanza_add_child(bind, res);
            xmpp_stanza_release(res);
            xmpp_free((*conn).ctx, resource as *mut libc::c_void);
        }
        xmpp_stanza_add_child(iq, bind);
        xmpp_stanza_release(bind);
        /* send bind request */
        xmpp_send(conn, iq);
        xmpp_stanza_release(iq);
    } else {
        /* can't bind, disconnect */
        xmpp_error((*conn).ctx,
                   b"xmpp\x00" as *const u8 as *const libc::c_char,
                   b"Stream features does not allow resource bind.\x00" as
                       *const u8 as *const libc::c_char);
        xmpp_disconnect(conn);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _handle_missing_features_sasl(conn: *mut xmpp_conn_t,
                                                   userdata:
                                                       *mut libc::c_void)
 -> libc::c_int {
    xmpp_error((*conn).ctx, b"xmpp\x00" as *const u8 as *const libc::c_char,
               b"Did not receive stream features after SASL authentication.\x00"
                   as *const u8 as *const libc::c_char);
    xmpp_disconnect(conn);
    return 0 as libc::c_int;
}
unsafe extern "C" fn _handle_bind(conn: *mut xmpp_conn_t,
                                  stanza: *mut xmpp_stanza_t,
                                  userdata: *mut libc::c_void)
 -> libc::c_int {
    let mut type_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut iq: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut session: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    /* delete missing bind handler */
    xmpp_timed_handler_delete(conn,
                              Some(_handle_missing_bind as
                                       unsafe extern "C" fn(_:
                                                                *mut xmpp_conn_t,
                                                            _:
                                                                *mut libc::c_void)
                                           -> libc::c_int));
    /* server has replied to bind request */
    type_0 = xmpp_stanza_get_type(stanza);
    if !type_0.is_null() &&
           strcmp(type_0, b"error\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        xmpp_error((*conn).ctx,
                   b"xmpp\x00" as *const u8 as *const libc::c_char,
                   b"Binding failed.\x00" as *const u8 as
                       *const libc::c_char);
        xmpp_disconnect(conn);
    } else if !type_0.is_null() &&
                  strcmp(type_0,
                         b"result\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
        let mut binding: *mut xmpp_stanza_t =
            xmpp_stanza_get_child_by_name(stanza,
                                          b"bind\x00" as *const u8 as
                                              *const libc::c_char);
        xmpp_debug((*conn).ctx,
                   b"xmpp\x00" as *const u8 as *const libc::c_char,
                   b"Bind successful.\x00" as *const u8 as
                       *const libc::c_char);
        if !binding.is_null() {
            let mut jid_stanza: *mut xmpp_stanza_t =
                xmpp_stanza_get_child_by_name(binding,
                                              b"jid\x00" as *const u8 as
                                                  *const libc::c_char);
            if !jid_stanza.is_null() {
                (*conn).bound_jid = xmpp_stanza_get_text(jid_stanza)
            }
        }
        /* establish a session if required */
        if (*conn).session_required != 0 {
            /* setup response handlers */
            handler_add_id(conn,
                           Some(_handle_session as
                                    unsafe extern "C" fn(_: *mut xmpp_conn_t,
                                                         _:
                                                             *mut xmpp_stanza_t,
                                                         _: *mut libc::c_void)
                                        -> libc::c_int),
                           b"_xmpp_session1\x00" as *const u8 as
                               *const libc::c_char, 0 as *mut libc::c_void);
            handler_add_timed(conn,
                              Some(_handle_missing_session as
                                       unsafe extern "C" fn(_:
                                                                *mut xmpp_conn_t,
                                                            _:
                                                                *mut libc::c_void)
                                           -> libc::c_int),
                              15000 as libc::c_int as libc::c_ulong,
                              0 as *mut libc::c_void);
            /* send session request */
            iq =
                xmpp_iq_new((*conn).ctx,
                            b"set\x00" as *const u8 as *const libc::c_char,
                            b"_xmpp_session1\x00" as *const u8 as
                                *const libc::c_char);
            if iq.is_null() {
                disconnect_mem_error(conn);
                return 0 as libc::c_int
            }
            session = xmpp_stanza_new((*conn).ctx);
            if session.is_null() {
                xmpp_stanza_release(iq);
                disconnect_mem_error(conn);
                return 0 as libc::c_int
            }
            xmpp_stanza_set_name(session,
                                 b"session\x00" as *const u8 as
                                     *const libc::c_char);
            xmpp_stanza_set_ns(session,
                               b"urn:ietf:params:xml:ns:xmpp-session\x00" as
                                   *const u8 as *const libc::c_char);
            xmpp_stanza_add_child(iq, session);
            xmpp_stanza_release(session);
            /* send session establishment request */
            xmpp_send(conn, iq);
            xmpp_stanza_release(iq);
        } else {
            (*conn).authenticated = 1 as libc::c_int;
            /* call connection handler */
            (*conn).conn_handler.expect("non-null function pointer")(conn,
                                                                     XMPP_CONN_CONNECT,
                                                                     0 as
                                                                         libc::c_int,
                                                                     0 as
                                                                         *mut xmpp_stream_error_t,
                                                                     (*conn).userdata);
        }
    } else {
        xmpp_error((*conn).ctx,
                   b"xmpp\x00" as *const u8 as *const libc::c_char,
                   b"Server sent malformed bind reply.\x00" as *const u8 as
                       *const libc::c_char);
        xmpp_disconnect(conn);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _handle_missing_bind(conn: *mut xmpp_conn_t,
                                          userdata: *mut libc::c_void)
 -> libc::c_int {
    xmpp_error((*conn).ctx, b"xmpp\x00" as *const u8 as *const libc::c_char,
               b"Server did not reply to bind request.\x00" as *const u8 as
                   *const libc::c_char);
    xmpp_disconnect(conn);
    return 0 as libc::c_int;
}
unsafe extern "C" fn _handle_session(conn: *mut xmpp_conn_t,
                                     stanza: *mut xmpp_stanza_t,
                                     userdata: *mut libc::c_void)
 -> libc::c_int {
    let mut type_0: *const libc::c_char = 0 as *const libc::c_char;
    /* delete missing session handler */
    xmpp_timed_handler_delete(conn,
                              Some(_handle_missing_session as
                                       unsafe extern "C" fn(_:
                                                                *mut xmpp_conn_t,
                                                            _:
                                                                *mut libc::c_void)
                                           -> libc::c_int));
    /* server has replied to the session request */
    type_0 = xmpp_stanza_get_type(stanza);
    if !type_0.is_null() &&
           strcmp(type_0, b"error\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        xmpp_error((*conn).ctx,
                   b"xmpp\x00" as *const u8 as *const libc::c_char,
                   b"Session establishment failed.\x00" as *const u8 as
                       *const libc::c_char);
        xmpp_disconnect(conn);
    } else if !type_0.is_null() &&
                  strcmp(type_0,
                         b"result\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
        xmpp_debug((*conn).ctx,
                   b"xmpp\x00" as *const u8 as *const libc::c_char,
                   b"Session establishment successful.\x00" as *const u8 as
                       *const libc::c_char);
        (*conn).authenticated = 1 as libc::c_int;
        /* call connection handler */
        (*conn).conn_handler.expect("non-null function pointer")(conn,
                                                                 XMPP_CONN_CONNECT,
                                                                 0 as
                                                                     libc::c_int,
                                                                 0 as
                                                                     *mut xmpp_stream_error_t,
                                                                 (*conn).userdata);
    } else {
        xmpp_error((*conn).ctx,
                   b"xmpp\x00" as *const u8 as *const libc::c_char,
                   b"Server sent malformed session reply.\x00" as *const u8 as
                       *const libc::c_char);
        xmpp_disconnect(conn);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _handle_missing_session(conn: *mut xmpp_conn_t,
                                             userdata: *mut libc::c_void)
 -> libc::c_int {
    xmpp_error((*conn).ctx, b"xmpp\x00" as *const u8 as *const libc::c_char,
               b"Server did not reply to session request.\x00" as *const u8 as
                   *const libc::c_char);
    xmpp_disconnect(conn);
    return 0 as libc::c_int;
}
unsafe extern "C" fn _handle_missing_legacy(conn: *mut xmpp_conn_t,
                                            userdata: *mut libc::c_void)
 -> libc::c_int {
    xmpp_error((*conn).ctx, b"xmpp\x00" as *const u8 as *const libc::c_char,
               b"Server did not reply to legacy authentication request.\x00"
                   as *const u8 as *const libc::c_char);
    xmpp_disconnect(conn);
    return 0 as libc::c_int;
}
unsafe extern "C" fn _handle_legacy(conn: *mut xmpp_conn_t,
                                    stanza: *mut xmpp_stanza_t,
                                    userdata: *mut libc::c_void)
 -> libc::c_int {
    let mut type_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    /* delete missing handler */
    xmpp_timed_handler_delete(conn,
                              Some(_handle_missing_legacy as
                                       unsafe extern "C" fn(_:
                                                                *mut xmpp_conn_t,
                                                            _:
                                                                *mut libc::c_void)
                                           -> libc::c_int));
    /* server responded to legacy auth request */
    type_0 = xmpp_stanza_get_type(stanza);
    name = xmpp_stanza_get_name(stanza);
    if type_0.is_null() ||
           strcmp(name, b"iq\x00" as *const u8 as *const libc::c_char) !=
               0 as libc::c_int {
        xmpp_error((*conn).ctx,
                   b"xmpp\x00" as *const u8 as *const libc::c_char,
                   b"Server sent us an unexpected response to legacy authentication request.\x00"
                       as *const u8 as *const libc::c_char);
        xmpp_disconnect(conn);
    } else if strcmp(type_0, b"error\x00" as *const u8 as *const libc::c_char)
                  == 0 as libc::c_int {
        /* legacy client auth failed, no more fallbacks */
        xmpp_error((*conn).ctx,
                   b"xmpp\x00" as *const u8 as *const libc::c_char,
                   b"Legacy client authentication failed.\x00" as *const u8 as
                       *const libc::c_char);
        xmpp_disconnect(conn);
    } else if strcmp(type_0,
                     b"result\x00" as *const u8 as *const libc::c_char) ==
                  0 as libc::c_int {
        /* auth succeeded */
        xmpp_debug((*conn).ctx,
                   b"xmpp\x00" as *const u8 as *const libc::c_char,
                   b"Legacy auth succeeded.\x00" as *const u8 as
                       *const libc::c_char);
        (*conn).authenticated = 1 as libc::c_int;
        (*conn).conn_handler.expect("non-null function pointer")(conn,
                                                                 XMPP_CONN_CONNECT,
                                                                 0 as
                                                                     libc::c_int,
                                                                 0 as
                                                                     *mut xmpp_stream_error_t,
                                                                 (*conn).userdata);
    } else {
        xmpp_error((*conn).ctx,
                   b"xmpp\x00" as *const u8 as *const libc::c_char,
                   b"Server sent us a legacy authentication response with a bad type.\x00"
                       as *const u8 as *const libc::c_char);
        xmpp_disconnect(conn);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _auth_legacy(mut conn: *mut xmpp_conn_t) {
    let mut iq: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut authdata: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut query: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut child: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    xmpp_debug((*conn).ctx, b"auth\x00" as *const u8 as *const libc::c_char,
               b"Legacy authentication request\x00" as *const u8 as
                   *const libc::c_char);
    iq =
        xmpp_iq_new((*conn).ctx,
                    b"set\x00" as *const u8 as *const libc::c_char,
                    b"_xmpp_auth1\x00" as *const u8 as *const libc::c_char);
    if !iq.is_null() {
        query = xmpp_stanza_new((*conn).ctx);
        if !query.is_null() {
            xmpp_stanza_set_name(query,
                                 b"query\x00" as *const u8 as
                                     *const libc::c_char);
            xmpp_stanza_set_ns(query,
                               b"jabber:iq:auth\x00" as *const u8 as
                                   *const libc::c_char);
            xmpp_stanza_add_child(iq, query);
            xmpp_stanza_release(query);
            child = xmpp_stanza_new((*conn).ctx);
            if !child.is_null() {
                xmpp_stanza_set_name(child,
                                     b"username\x00" as *const u8 as
                                         *const libc::c_char);
                xmpp_stanza_add_child(query, child);
                xmpp_stanza_release(child);
                authdata = xmpp_stanza_new((*conn).ctx);
                if !authdata.is_null() {
                    str = xmpp_jid_node((*conn).ctx, (*conn).jid);
                    if str.is_null() {
                        xmpp_stanza_release(authdata);
                    } else {
                        xmpp_stanza_set_text(authdata, str);
                        xmpp_free((*conn).ctx, str as *mut libc::c_void);
                        xmpp_stanza_add_child(child, authdata);
                        xmpp_stanza_release(authdata);
                        child = xmpp_stanza_new((*conn).ctx);
                        if !child.is_null() {
                            xmpp_stanza_set_name(child,
                                                 b"password\x00" as *const u8
                                                     as *const libc::c_char);
                            xmpp_stanza_add_child(query, child);
                            xmpp_stanza_release(child);
                            authdata = xmpp_stanza_new((*conn).ctx);
                            if !authdata.is_null() {
                                xmpp_stanza_set_text(authdata, (*conn).pass);
                                xmpp_stanza_add_child(child, authdata);
                                xmpp_stanza_release(authdata);
                                child = xmpp_stanza_new((*conn).ctx);
                                if !child.is_null() {
                                    xmpp_stanza_set_name(child,
                                                         b"resource\x00" as
                                                             *const u8 as
                                                             *const libc::c_char);
                                    xmpp_stanza_add_child(query, child);
                                    xmpp_stanza_release(child);
                                    authdata = xmpp_stanza_new((*conn).ctx);
                                    if !authdata.is_null() {
                                        str =
                                            xmpp_jid_resource((*conn).ctx,
                                                              (*conn).jid);
                                        if !str.is_null() {
                                            xmpp_stanza_set_text(authdata,
                                                                 str);
                                            xmpp_free((*conn).ctx,
                                                      str as
                                                          *mut libc::c_void);
                                        } else {
                                            xmpp_stanza_release(authdata);
                                            xmpp_stanza_release(iq);
                                            xmpp_error((*conn).ctx,
                                                       b"auth\x00" as
                                                           *const u8 as
                                                           *const libc::c_char,
                                                       b"Cannot authenticate without resource\x00"
                                                           as *const u8 as
                                                           *const libc::c_char);
                                            xmpp_disconnect(conn);
                                            return
                                        }
                                        xmpp_stanza_add_child(child,
                                                              authdata);
                                        xmpp_stanza_release(authdata);
                                        handler_add_id(conn,
                                                       Some(_handle_legacy as
                                                                unsafe extern "C" fn(_:
                                                                                         *mut xmpp_conn_t,
                                                                                     _:
                                                                                         *mut xmpp_stanza_t,
                                                                                     _:
                                                                                         *mut libc::c_void)
                                                                    ->
                                                                        libc::c_int),
                                                       b"_xmpp_auth1\x00" as
                                                           *const u8 as
                                                           *const libc::c_char,
                                                       0 as
                                                           *mut libc::c_void);
                                        handler_add_timed(conn,
                                                          Some(_handle_missing_legacy
                                                                   as
                                                                   unsafe extern "C" fn(_:
                                                                                            *mut xmpp_conn_t,
                                                                                        _:
                                                                                            *mut libc::c_void)
                                                                       ->
                                                                           libc::c_int),
                                                          15000 as libc::c_int
                                                              as
                                                              libc::c_ulong,
                                                          0 as
                                                              *mut libc::c_void);
                                        xmpp_send(conn, iq);
                                        xmpp_stanza_release(iq);
                                        return
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        xmpp_stanza_release(iq);
    }
    disconnect_mem_error(conn);
}
#[no_mangle]
pub unsafe extern "C" fn auth_handle_component_open(conn: *mut xmpp_conn_t) {
    let mut rc: libc::c_int = 0;
    /* reset all timed handlers */
    handler_reset_timed(conn, 0 as libc::c_int);
    handler_add(conn,
                Some(_handle_error as
                         unsafe extern "C" fn(_: *mut xmpp_conn_t,
                                              _: *mut xmpp_stanza_t,
                                              _: *mut libc::c_void)
                             -> libc::c_int),
                b"http://etherx.jabber.org/streams\x00" as *const u8 as
                    *const libc::c_char,
                b"error\x00" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char, 0 as *mut libc::c_void);
    handler_add(conn,
                Some(_handle_component_hs_response as
                         unsafe extern "C" fn(_: *mut xmpp_conn_t,
                                              _: *mut xmpp_stanza_t,
                                              _: *mut libc::c_void)
                             -> libc::c_int), 0 as *const libc::c_char,
                b"handshake\x00" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char, 0 as *mut libc::c_void);
    handler_add_timed(conn,
                      Some(_handle_missing_handshake as
                               unsafe extern "C" fn(_: *mut xmpp_conn_t,
                                                    _: *mut libc::c_void)
                                   -> libc::c_int),
                      15000 as libc::c_int as libc::c_ulong,
                      0 as *mut libc::c_void);
    rc = _handle_component_auth(conn);
    if rc != 0 as libc::c_int {
        xmpp_error((*conn).ctx,
                   b"auth\x00" as *const u8 as *const libc::c_char,
                   b"Component authentication failed.\x00" as *const u8 as
                       *const libc::c_char);
        xmpp_disconnect(conn);
    };
}
/* Will compute SHA1 and authenticate the component to the server */
unsafe extern "C" fn _handle_component_auth(conn: *mut xmpp_conn_t)
 -> libc::c_int {
    let mut md_value: [uint8_t; 20] = [0; 20];
    let mut mdctx: SHA1_CTX =
        SHA1_CTX{state: [0; 5], count: [0; 2], buffer: [0; 64],};
    let mut digest: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: size_t = 0;
    if (*conn).stream_id.is_null() {
        xmpp_error((*conn).ctx,
                   b"auth\x00" as *const u8 as *const libc::c_char,
                   b"Received no stream id from the server.\x00" as *const u8
                       as *const libc::c_char);
        return -(3 as libc::c_int)
    }
    /* Feed the session id and passphrase to the algorithm.
     * We need to compute SHA1(session_id + passphrase)
     */
    crypto_SHA1_Init(&mut mdctx);
    crypto_SHA1_Update(&mut mdctx, (*conn).stream_id as *mut uint8_t,
                       strlen((*conn).stream_id));
    crypto_SHA1_Update(&mut mdctx, (*conn).pass as *mut uint8_t,
                       strlen((*conn).pass));
    crypto_SHA1_Final(&mut mdctx, md_value.as_mut_ptr());
    digest =
        xmpp_alloc((*conn).ctx,
                   (2 as libc::c_int as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<[uint8_t; 20]>()
                                                        as
                                                        libc::c_ulong).wrapping_add(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_ulong))
            as *mut libc::c_char;
    if !digest.is_null() {
        /* convert the digest into string representation */
        i = 0 as libc::c_int as size_t;
        while i < ::std::mem::size_of::<[uint8_t; 20]>() as libc::c_ulong {
            xmpp_snprintf(digest.offset(i.wrapping_mul(2 as libc::c_int as
                                                           libc::c_ulong) as
                                            isize),
                          3 as libc::c_int as size_t,
                          b"%02x\x00" as *const u8 as *const libc::c_char,
                          md_value[i as usize] as libc::c_int);
            i = i.wrapping_add(1)
        }
        *digest.offset((2 as libc::c_int as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<[uint8_t; 20]>()
                                                            as libc::c_ulong)
                           as isize) = '\u{0}' as i32 as libc::c_char;
        xmpp_debug((*conn).ctx,
                   b"auth\x00" as *const u8 as *const libc::c_char,
                   b"Digest: %s, len: %d\x00" as *const u8 as
                       *const libc::c_char, digest, strlen(digest));
        /* Send the digest to the server */
        xmpp_send_raw_string(conn,
                             b"<handshake xmlns=\'%s\'>%s</handshake>\x00" as
                                 *const u8 as *const libc::c_char,
                             b"jabber:component:accept\x00" as *const u8 as
                                 *const libc::c_char, digest);
        xmpp_debug((*conn).ctx,
                   b"auth\x00" as *const u8 as *const libc::c_char,
                   b"Sent component handshake to the server.\x00" as *const u8
                       as *const libc::c_char);
        xmpp_free((*conn).ctx, digest as *mut libc::c_void);
    } else {
        xmpp_debug((*conn).ctx,
                   b"auth\x00" as *const u8 as *const libc::c_char,
                   b"Couldn\'t allocate memory for component handshake digest.\x00"
                       as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
/* Check if the received stanza is <handshake/> and set auth to true
 * and fire connection handler.
 */
unsafe extern "C" fn _handle_component_hs_response(conn: *mut xmpp_conn_t,
                                                   stanza: *mut xmpp_stanza_t,
                                                   userdata:
                                                       *mut libc::c_void)
 -> libc::c_int {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    xmpp_timed_handler_delete(conn,
                              Some(_handle_missing_handshake as
                                       unsafe extern "C" fn(_:
                                                                *mut xmpp_conn_t,
                                                            _:
                                                                *mut libc::c_void)
                                           -> libc::c_int));
    name = xmpp_stanza_get_name(stanza);
    if strcmp(name, b"handshake\x00" as *const u8 as *const libc::c_char) !=
           0 as libc::c_int {
        let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut msg_size: size_t = 0;
        xmpp_stanza_to_text(stanza, &mut msg, &mut msg_size);
        if !msg.is_null() {
            xmpp_debug((*conn).ctx,
                       b"auth\x00" as *const u8 as *const libc::c_char,
                       b"Handshake failed: %s\x00" as *const u8 as
                           *const libc::c_char, msg);
            xmpp_free((*conn).ctx, msg as *mut libc::c_void);
        }
        xmpp_disconnect(conn);
        return -(3 as libc::c_int)
    } else {
        (*conn).authenticated = 1 as libc::c_int;
        (*conn).conn_handler.expect("non-null function pointer")(conn,
                                                                 XMPP_CONN_CONNECT,
                                                                 0 as
                                                                     libc::c_int,
                                                                 0 as
                                                                     *mut xmpp_stream_error_t,
                                                                 (*conn).userdata);
    }
    /* We don't need this handler anymore, return 0 so it can be deleted
     * from the list of handlers.
     */
    return 0 as libc::c_int;
}
unsafe extern "C" fn _handle_missing_handshake(conn: *mut xmpp_conn_t,
                                               userdata: *mut libc::c_void)
 -> libc::c_int {
    xmpp_error((*conn).ctx, b"xmpp\x00" as *const u8 as *const libc::c_char,
               b"Server did not reply to handshake request.\x00" as *const u8
                   as *const libc::c_char);
    xmpp_disconnect(conn);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn auth_handle_open_raw(conn: *mut xmpp_conn_t) {
    handler_reset_timed(conn, 0 as libc::c_int);
    /* user handlers are not called before authentication is completed. */
    (*conn).authenticated = 1 as libc::c_int;
    (*conn).conn_handler.expect("non-null function pointer")(conn,
                                                             XMPP_CONN_CONNECT,
                                                             0 as libc::c_int,
                                                             0 as
                                                                 *mut xmpp_stream_error_t,
                                                             (*conn).userdata);
}
#[no_mangle]
pub unsafe extern "C" fn auth_handle_open_stub(conn: *mut xmpp_conn_t) {
    xmpp_warn((*conn).ctx, b"auth\x00" as *const u8 as *const libc::c_char,
              b"Stub callback is called.\x00" as *const u8 as
                  *const libc::c_char);
}
