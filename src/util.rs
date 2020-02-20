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
    pub type _tls;
    pub type _xmpp_rand_t;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_ulong;
    #[no_mangle]
    fn strspn(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_ulong;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t)
     -> libc::c_int;
    #[no_mangle]
    fn xmpp_disconnect(conn: *mut xmpp_conn_t);
    #[no_mangle]
    fn xmpp_alloc(ctx: *const xmpp_ctx_t, size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xmpp_error(ctx: *const xmpp_ctx_t, area: *const libc::c_char,
                  fmt: *const libc::c_char, _: ...);
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}
pub type __timezone_ptr_t = *mut timezone;
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
/* util.c
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
 *  Utility functions.
 */
/* * implement our own strdup that uses the ctx allocator */
/* * Duplicate a string.
 *  This function replaces the standard strdup library call with a version
 *  that uses the Strophe context object's allocator.
 *
 *  @param ctx a Strophe context object
 *  @param s a string
 *
 *  @return a new allocates string with the same data as s or NULL on error
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_strdup(ctx: *const xmpp_ctx_t,
                                     s: *const libc::c_char)
 -> *mut libc::c_char {
    let mut len: size_t = 0;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    len = strlen(s);
    copy =
        xmpp_alloc(ctx, len.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
    if copy.is_null() {
        xmpp_error(ctx, b"xmpp\x00" as *const u8 as *const libc::c_char,
                   b"failed to allocate required memory\x00" as *const u8 as
                       *const libc::c_char);
        return 0 as *mut libc::c_char
    }
    memcpy(copy as *mut libc::c_void, s as *const libc::c_void,
           len.wrapping_add(1 as libc::c_int as libc::c_ulong));
    return copy;
}
/* * strtok_r(3) implementation.
 *  This function has appeared in POSIX.1-2001, but not in C standard.
 *  For example, visual studio older than 2005 doesn't provide strtok_r()
 *  nor strtok_s().
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_strtok_r(mut s: *mut libc::c_char,
                                       mut delim: *const libc::c_char,
                                       mut saveptr: *mut *mut libc::c_char)
 -> *mut libc::c_char {
    let mut len: size_t = 0;
    s = if !s.is_null() { s } else { *saveptr };
    len = strspn(s, delim);
    s = s.offset(len as isize);
    if *s as libc::c_int == '\u{0}' as i32 { return 0 as *mut libc::c_char }
    len = strcspn(s, delim);
    *saveptr =
        if *s.offset(len as isize) as libc::c_int == '\u{0}' as i32 {
            &mut *s.offset(len as isize) as *mut libc::c_char
        } else {
            &mut *s.offset(len.wrapping_add(1 as libc::c_int as libc::c_ulong)
                               as isize) as *mut libc::c_char
        };
    *s.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    return s;
}
/* * Return an integer based time stamp.
 *  This function uses gettimeofday or timeGetTime (on Win32 platforms) to
 *  compute an integer based time stamp.  This is used internally by the
 *  event loop and timed handlers.
 *
 *  @return an integer time stamp
 */
#[no_mangle]
pub unsafe extern "C" fn time_stamp() -> uint64_t {
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    gettimeofday(&mut tv, 0 as *mut timezone);
    return (tv.tv_sec as
                uint64_t).wrapping_mul(1000 as libc::c_int as
                                           libc::c_ulong).wrapping_add((tv.tv_usec
                                                                            as
                                                                            uint64_t).wrapping_div(1000
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong));
}
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
/* * Get the time elapsed between two time stamps.
 *  This function returns the time elapsed between t1 and t2 by subtracting
 *  t1 from t2.  If t2 happened before t1, the result will be negative.  This
 *  function is used internally by the event loop and timed handlers.
 *
 *  @param t1 first time stamp
 *  @param t2 second time stamp
 *
 *  @return number of milliseconds between the stamps
 */
#[no_mangle]
pub unsafe extern "C" fn time_elapsed(mut t1: uint64_t, mut t2: uint64_t)
 -> uint64_t {
    return t2.wrapping_sub(t1);
}
/* handler management */
/* utility functions */
/* * Disconnect the stream with a memory error.
 *  This is a convenience function used internally by various parts of
 *  the Strophe library for terminating the connection because of a 
 *  memory error.
 *
 *  @param conn a Strophe connection object
 */
#[no_mangle]
pub unsafe extern "C" fn disconnect_mem_error(conn: *mut xmpp_conn_t) {
    xmpp_error((*conn).ctx, b"xmpp\x00" as *const u8 as *const libc::c_char,
               b"Memory allocation error\x00" as *const u8 as
                   *const libc::c_char);
    xmpp_disconnect(conn);
}
