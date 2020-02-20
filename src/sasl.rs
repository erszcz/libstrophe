use ::libc;
extern "C" {
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
    pub type _hash_t;
    pub type _parser_t;
    pub type _tls;
    pub type _xmpp_rand_t;
    #[no_mangle]
    fn strtol(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char,
              __base: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strtok_r(__s: *mut libc::c_char, __delim: *const libc::c_char,
                __save_ptr: *mut *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /* free some blocks returned by other APIs, for example the
   buffer you get from xmpp_stanza_to_text */
    #[no_mangle]
    fn xmpp_free(ctx: *const xmpp_ctx_t, p: *mut libc::c_void);
    #[no_mangle]
    fn xmpp_jid_node(ctx: *mut xmpp_ctx_t, jid: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn xmpp_jid_domain(ctx: *mut xmpp_ctx_t, jid: *const libc::c_char)
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
    fn xmpp_base64_decode_bin(ctx: *mut xmpp_ctx_t,
                              base64: *const libc::c_char, len: size_t,
                              out: *mut *mut libc::c_uchar,
                              outlen: *mut size_t);
    /* * allocate and initialize a new hash table */
    #[no_mangle]
    fn hash_new(ctx: *mut xmpp_ctx_t, size: libc::c_int,
                free_func: hash_free_func) -> *mut hash_t;
    /* * release a hash table when no longer needed */
    #[no_mangle]
    fn hash_release(table: *mut hash_t);
    /* * add a key, value pair to a hash table.
 *  each key can appear only once; the value of any
 *  identical key will be replaced
 */
    #[no_mangle]
    fn hash_add(table: *mut hash_t, key: *const libc::c_char,
                data: *mut libc::c_void) -> libc::c_int;
    /* * look up a key in a hash table */
    #[no_mangle]
    fn hash_get(table: *mut hash_t, key: *const libc::c_char)
     -> *mut libc::c_void;
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
    fn xmpp_realloc(ctx: *const xmpp_ctx_t, p: *mut libc::c_void,
                    size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xmpp_strdup(ctx: *const xmpp_ctx_t, s: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn xmpp_error(ctx: *const xmpp_ctx_t, area: *const libc::c_char,
                  fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn MD5Init(context: *mut MD5Context);
    #[no_mangle]
    fn MD5Update(context: *mut MD5Context, buf: *const libc::c_uchar,
                 len: uint32_t);
    #[no_mangle]
    fn MD5Final(digest: *mut libc::c_uchar, context: *mut MD5Context);
    /* scram.h
 * strophe XMPP client library -- SCRAM-SHA1 helper functions
 *
 * Copyright (C) 2013 Dmitry Podgorny <pasis.ua@gmail.com>
 *
 *  This software is provided AS-IS with no warranty, either express
 *  or implied.
 *
 *  This program is dual licensed under the MIT and GPLv3 licenses.
 */
    /* * @file
 *  SCRAM-SHA1 helper functions.
 */
    /* make sure the stdint.h types are available */
    #[no_mangle]
    fn SCRAM_SHA1_ClientKey(password: *const uint8_t, len: size_t,
                            salt: *const uint8_t, salt_len: size_t,
                            i: uint32_t, key: *mut uint8_t);
    #[no_mangle]
    fn SCRAM_SHA1_ClientSignature(ClientKey: *const uint8_t,
                                  AuthMessage: *const uint8_t, len: size_t,
                                  sign: *mut uint8_t);
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
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type hash_free_func
    =
    Option<unsafe extern "C" fn(_: *const xmpp_ctx_t, _: *mut libc::c_void)
               -> ()>;
/* md5.h
** interface to MD5 hash function
**
** This code is in the Public Domain.
*/
/* * @file
 *  MD5 hash API.
 */
/* make sure the stdint.h types are available */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD5Context {
    pub buf: [uint32_t; 4],
    pub bits: [uint32_t; 2],
    pub in_0: [libc::c_uchar; 64],
}
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
/* sasl.c
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
 *  SASL authentication.
 */
/* strtok_s() has appeared in visual studio 2005.
   Use own implementation for older versions. */
/* _MSC_VER */
/* * generate authentication string for the SASL PLAIN mechanism */
#[no_mangle]
pub unsafe extern "C" fn sasl_plain(mut ctx: *mut xmpp_ctx_t,
                                    mut authid: *const libc::c_char,
                                    mut password: *const libc::c_char)
 -> *mut libc::c_char {
    let mut idlen: size_t = 0;
    let mut passlen: size_t = 0;
    let mut msglen: size_t = 0;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    /* our message is Base64(authzid,\0,authid,\0,password)
       if there is no authzid, that field is left empty */
    idlen = strlen(authid);
    passlen = strlen(password);
    msglen =
        (2 as libc::c_int as
             libc::c_ulong).wrapping_add(idlen).wrapping_add(passlen);
    msg = xmpp_alloc(ctx, msglen) as *mut libc::c_char;
    if !msg.is_null() {
        *msg.offset(0 as libc::c_int as isize) =
            '\u{0}' as i32 as libc::c_char;
        memcpy(msg.offset(1 as libc::c_int as isize) as *mut libc::c_void,
               authid as *const libc::c_void, idlen);
        *msg.offset((1 as libc::c_int as libc::c_ulong).wrapping_add(idlen) as
                        isize) = '\u{0}' as i32 as libc::c_char;
        memcpy(msg.offset(1 as libc::c_int as
                              isize).offset(idlen as
                                                isize).offset(1 as libc::c_int
                                                                  as isize) as
                   *mut libc::c_void, password as *const libc::c_void,
               passlen);
        result = xmpp_base64_encode(ctx, msg as *mut libc::c_uchar, msglen);
        xmpp_free(ctx, msg as *mut libc::c_void);
    }
    return result;
}
/* * helpers for digest auth */
/* create a new, null-terminated string from a substring */
unsafe extern "C" fn _make_string(mut ctx: *mut xmpp_ctx_t,
                                  mut s: *const libc::c_char,
                                  len: libc::c_uint) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    result =
        xmpp_alloc(ctx,
                   len.wrapping_add(1 as libc::c_int as libc::c_uint) as
                       size_t) as *mut libc::c_char;
    if !result.is_null() {
        memcpy(result as *mut libc::c_void, s as *const libc::c_void,
               len as libc::c_ulong);
        *result.offset(len as isize) = '\u{0}' as i32 as libc::c_char
    }
    return result;
}
/* create a new, null-terminated string quoting another string */
unsafe extern "C" fn _make_quoted(mut ctx: *mut xmpp_ctx_t,
                                  mut s: *const libc::c_char)
 -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = strlen(s);
    result =
        xmpp_alloc(ctx, len.wrapping_add(3 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
    if !result.is_null() {
        *result.offset(0 as libc::c_int as isize) =
            '\"' as i32 as libc::c_char;
        memcpy(result.offset(1 as libc::c_int as isize) as *mut libc::c_void,
               s as *const libc::c_void, len);
        *result.offset(len.wrapping_add(1 as libc::c_int as libc::c_ulong) as
                           isize) = '\"' as i32 as libc::c_char;
        *result.offset(len.wrapping_add(2 as libc::c_int as libc::c_ulong) as
                           isize) = '\u{0}' as i32 as libc::c_char
    }
    return result;
}
/* split key, value pairs into a hash */
unsafe extern "C" fn _parse_digest_challenge(mut ctx: *mut xmpp_ctx_t,
                                             mut msg: *const libc::c_char)
 -> *mut hash_t {
    let mut result: *mut hash_t = 0 as *mut hash_t;
    let mut text: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut t: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    text =
        xmpp_base64_decode_str(ctx, msg, strlen(msg)) as *mut libc::c_uchar;
    if text.is_null() {
        xmpp_error(ctx, b"SASL\x00" as *const u8 as *const libc::c_char,
                   b"couldn\'t Base64 decode challenge!\x00" as *const u8 as
                       *const libc::c_char);
        return 0 as *mut hash_t
    }
    result =
        hash_new(ctx, 10 as libc::c_int,
                 Some(xmpp_free as
                          unsafe extern "C" fn(_: *const xmpp_ctx_t,
                                               _: *mut libc::c_void) -> ()));
    if !result.is_null() {
        s = text;
        while *s as libc::c_int != '\u{0}' as i32 {
            /* skip any leading commas and spaces */
            while *s as libc::c_int == ',' as i32 ||
                      *s as libc::c_int == ' ' as i32 {
                s = s.offset(1)
            }
            /* accumulate a key ending at '=' */
            t = s; /* bad string */
            while *t as libc::c_int != '=' as i32 &&
                      *t as libc::c_int != '\u{0}' as i32 {
                t = t.offset(1)
            }
            if *t as libc::c_int == '\u{0}' as i32 { break ; }
            key =
                _make_string(ctx, s as *mut libc::c_char,
                             t.wrapping_offset_from(s) as libc::c_long as
                                 libc::c_uint);
            if key.is_null() { break ; }
            /* advance our start pointer past the key */
            s = t.offset(1 as libc::c_int as isize);
            t = s;
            /* if we see quotes, grab the string in between */
            if *s as libc::c_int == '\'' as i32 ||
                   *s as libc::c_int == '\"' as i32 {
                t = t.offset(1);
                while *t as libc::c_int != *s as libc::c_int &&
                          *t as libc::c_int != '\u{0}' as i32 {
                    t = t.offset(1)
                }
                value =
                    _make_string(ctx,
                                 (s as
                                      *mut libc::c_char).offset(1 as
                                                                    libc::c_int
                                                                    as isize),
                                 (t.wrapping_offset_from(s) as libc::c_long -
                                      1 as libc::c_int as libc::c_long) as
                                     libc::c_uint);
                if *t as libc::c_int == *s as libc::c_int {
                    s = t.offset(1 as libc::c_int as isize)
                } else { s = t }
                /* otherwise, accumulate a value ending in ',' or '\0' */
            } else {
                while *t as libc::c_int != ',' as i32 &&
                          *t as libc::c_int != '\u{0}' as i32 {
                    t = t.offset(1)
                }
                value =
                    _make_string(ctx, s as *mut libc::c_char,
                                 t.wrapping_offset_from(s) as libc::c_long as
                                     libc::c_uint);
                s = t
            }
            if value.is_null() {
                xmpp_free(ctx, key as *mut libc::c_void);
                break ;
            } else {
                /* TODO: check for collisions per spec */
                hash_add(result, key, value as *mut libc::c_void);
                /* hash table now owns the value, free the key */
                xmpp_free(ctx, key as *mut libc::c_void);
            }
        }
    }
    xmpp_free(ctx, text as *mut libc::c_void);
    return result;
}
/* * expand a 16 byte MD5 digest to a 32 byte hex representation */
unsafe extern "C" fn _digest_to_hex(mut digest: *const libc::c_char,
                                    mut hex: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let hexdigit: [libc::c_char; 17] =
        *::std::mem::transmute::<&[u8; 17],
                                 &[libc::c_char; 17]>(b"0123456789abcdef\x00");
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        let fresh0 = hex;
        hex = hex.offset(1);
        *fresh0 =
            hexdigit[(*digest.offset(i as isize) as libc::c_int >>
                          4 as libc::c_int & 0xf as libc::c_int) as usize];
        let fresh1 = hex;
        hex = hex.offset(1);
        *fresh1 =
            hexdigit[(*digest.offset(i as isize) as libc::c_int &
                          0xf as libc::c_int) as usize];
        i += 1
    };
}
/* * append 'key="value"' to a buffer, growing as necessary */
unsafe extern "C" fn _add_key(mut ctx: *mut xmpp_ctx_t,
                              mut table: *mut hash_t,
                              mut key: *const libc::c_char,
                              mut buf: *mut libc::c_char,
                              mut len: *mut libc::c_int,
                              mut quote: libc::c_int) -> *mut libc::c_char {
    let mut olen: libc::c_int = 0;
    let mut nlen: libc::c_int = 0;
    let mut keylen: libc::c_int = 0;
    let mut valuelen: libc::c_int = 0;
    let mut value: *const libc::c_char = 0 as *const libc::c_char;
    let mut qvalue: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    /* allocate a zero-length string if necessary */
    if buf.is_null() {
        buf =
            xmpp_alloc(ctx, 1 as libc::c_int as size_t) as *mut libc::c_char;
        *buf.offset(0 as libc::c_int as isize) =
            '\u{0}' as i32 as libc::c_char
    }
    if buf.is_null() { return 0 as *mut libc::c_char }
    /* get current string length */
    olen = strlen(buf) as libc::c_int;
    value = hash_get(table, key) as *const libc::c_char;
    if value.is_null() {
        xmpp_error(ctx, b"SASL\x00" as *const u8 as *const libc::c_char,
                   b"couldn\'t retrieve value for \'%s\'\x00" as *const u8 as
                       *const libc::c_char, key);
        value = b"\x00" as *const u8 as *const libc::c_char
    }
    if quote != 0 {
        qvalue = _make_quoted(ctx, value)
    } else { qvalue = value }
    /* added length is key + '=' + value */
    /*   (+ ',' if we're not the first entry   */
    keylen = strlen(key) as libc::c_int;
    valuelen = strlen(qvalue) as libc::c_int;
    nlen =
        (if olen != 0 { 1 as libc::c_int } else { 0 as libc::c_int }) + keylen
            + 1 as libc::c_int + valuelen + 1 as libc::c_int;
    buf =
        xmpp_realloc(ctx, buf as *mut libc::c_void, (olen + nlen) as size_t)
            as *mut libc::c_char;
    if !buf.is_null() {
        c = buf.offset(olen as isize);
        if olen != 0 {
            let fresh2 = c;
            c = c.offset(1);
            *fresh2 = ',' as i32 as libc::c_char
        }
        memcpy(c as *mut libc::c_void, key as *const libc::c_void,
               keylen as libc::c_ulong);
        c = c.offset(keylen as isize);
        let fresh3 = c;
        c = c.offset(1);
        *fresh3 = '=' as i32 as libc::c_char;
        memcpy(c as *mut libc::c_void, qvalue as *const libc::c_void,
               valuelen as libc::c_ulong);
        c = c.offset(valuelen as isize);
        let fresh4 = c;
        c = c.offset(1);
        *fresh4 = '\u{0}' as i32 as libc::c_char
    }
    if quote != 0 {
        xmpp_free(ctx, qvalue as *mut libc::c_char as *mut libc::c_void);
    }
    return buf;
}
/* * generate auth response string for the SASL DIGEST-MD5 mechanism */
#[no_mangle]
pub unsafe extern "C" fn sasl_digest_md5(mut ctx: *mut xmpp_ctx_t,
                                         mut challenge: *const libc::c_char,
                                         mut jid: *const libc::c_char,
                                         mut password: *const libc::c_char)
 -> *mut libc::c_char {
    let mut table: *mut hash_t = 0 as *mut hash_t;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut node: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut domain: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut realm: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut response: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rlen: libc::c_int = 0;
    let mut MD5: MD5Context =
        MD5Context{buf: [0; 4], bits: [0; 2], in_0: [0; 64],};
    let mut digest: [libc::c_uchar; 16] = [0; 16];
    let mut HA1: [libc::c_uchar; 16] = [0; 16];
    let mut HA2: [libc::c_uchar; 16] = [0; 16];
    let mut hex: [libc::c_char; 32] = [0; 32];
    let mut cnonce: [libc::c_char; 13] = [0; 13];
    /* our digest response is 
	Hex( KD( HEX(MD5(A1)),
	  nonce ':' nc ':' cnonce ':' qop ':' HEX(MD5(A2))
	))

       where KD(k, s) = MD5(k ':' s),
	A1 = MD5( node ':' realm ':' password ) ':' nonce ':' cnonce
	A2 = "AUTHENTICATE" ':' "xmpp/" domain

       If there is an authzid it is ':'-appended to A1 */
    /* parse the challenge */
    table = _parse_digest_challenge(ctx, challenge);
    if table.is_null() {
        xmpp_error(ctx, b"SASL\x00" as *const u8 as *const libc::c_char,
                   b"couldn\'t parse digest challenge\x00" as *const u8 as
                       *const libc::c_char);
        return 0 as *mut libc::c_char
    }
    node = xmpp_jid_node(ctx, jid);
    domain = xmpp_jid_domain(ctx, jid);
    /* generate default realm of domain if one didn't come from the
       server */
    realm =
        hash_get(table, b"realm\x00" as *const u8 as *const libc::c_char) as
            *mut libc::c_char;
    if realm.is_null() || strlen(realm) == 0 as libc::c_int as libc::c_ulong {
        hash_add(table, b"realm\x00" as *const u8 as *const libc::c_char,
                 xmpp_strdup(ctx, domain) as *mut libc::c_void);
        realm =
            hash_get(table, b"realm\x00" as *const u8 as *const libc::c_char)
                as *mut libc::c_char
    }
    /* add our response fields */
    hash_add(table, b"username\x00" as *const u8 as *const libc::c_char,
             xmpp_strdup(ctx, node) as *mut libc::c_void);
    xmpp_rand_nonce((*ctx).rand, cnonce.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 13]>() as
                        libc::c_ulong);
    hash_add(table, b"cnonce\x00" as *const u8 as *const libc::c_char,
             xmpp_strdup(ctx, cnonce.as_mut_ptr()) as *mut libc::c_void);
    hash_add(table, b"nc\x00" as *const u8 as *const libc::c_char,
             xmpp_strdup(ctx,
                         b"00000001\x00" as *const u8 as *const libc::c_char)
                 as *mut libc::c_void);
    if hash_get(table,
                b"qop\x00" as *const u8 as *const libc::c_char).is_null() {
        hash_add(table, b"qop\x00" as *const u8 as *const libc::c_char,
                 xmpp_strdup(ctx,
                             b"auth\x00" as *const u8 as *const libc::c_char)
                     as *mut libc::c_void);
    }
    value =
        xmpp_alloc(ctx,
                   (5 as libc::c_int as
                        libc::c_ulong).wrapping_add(strlen(domain)).wrapping_add(1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
            as *mut libc::c_char;
    memcpy(value as *mut libc::c_void,
           b"xmpp/\x00" as *const u8 as *const libc::c_char as
               *const libc::c_void, 5 as libc::c_int as libc::c_ulong);
    memcpy(value.offset(5 as libc::c_int as isize) as *mut libc::c_void,
           domain as *const libc::c_void, strlen(domain));
    *value.offset((5 as libc::c_int as
                       libc::c_ulong).wrapping_add(strlen(domain)) as isize) =
        '\u{0}' as i32 as libc::c_char;
    hash_add(table, b"digest-uri\x00" as *const u8 as *const libc::c_char,
             value as *mut libc::c_void);
    /* generate response */
    /* construct MD5(node : realm : password) */
    MD5Init(&mut MD5);
    MD5Update(&mut MD5, node as *mut libc::c_uchar, strlen(node) as uint32_t);
    MD5Update(&mut MD5,
              b":\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_uchar, 1 as libc::c_int as uint32_t);
    MD5Update(&mut MD5, realm as *mut libc::c_uchar,
              strlen(realm) as uint32_t);
    MD5Update(&mut MD5,
              b":\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_uchar, 1 as libc::c_int as uint32_t);
    MD5Update(&mut MD5, password as *mut libc::c_uchar,
              strlen(password) as uint32_t);
    MD5Final(digest.as_mut_ptr(), &mut MD5);
    /* digest now contains the first field of A1 */
    MD5Init(&mut MD5);
    MD5Update(&mut MD5, digest.as_mut_ptr(), 16 as libc::c_int as uint32_t);
    MD5Update(&mut MD5,
              b":\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_uchar, 1 as libc::c_int as uint32_t);
    value =
        hash_get(table, b"nonce\x00" as *const u8 as *const libc::c_char) as
            *mut libc::c_char;
    MD5Update(&mut MD5, value as *mut libc::c_uchar,
              strlen(value) as uint32_t);
    MD5Update(&mut MD5,
              b":\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_uchar, 1 as libc::c_int as uint32_t);
    value =
        hash_get(table, b"cnonce\x00" as *const u8 as *const libc::c_char) as
            *mut libc::c_char;
    MD5Update(&mut MD5, value as *mut libc::c_uchar,
              strlen(value) as uint32_t);
    MD5Final(digest.as_mut_ptr(), &mut MD5);
    /* now digest is MD5(A1) */
    memcpy(HA1.as_mut_ptr() as *mut libc::c_void,
           digest.as_mut_ptr() as *const libc::c_void,
           16 as libc::c_int as libc::c_ulong);
    /* construct MD5(A2) */
    MD5Init(&mut MD5);
    MD5Update(&mut MD5,
              b"AUTHENTICATE:\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_uchar, 13 as libc::c_int as uint32_t);
    value =
        hash_get(table, b"digest-uri\x00" as *const u8 as *const libc::c_char)
            as *mut libc::c_char;
    MD5Update(&mut MD5, value as *mut libc::c_uchar,
              strlen(value) as uint32_t);
    if strcmp(hash_get(table, b"qop\x00" as *const u8 as *const libc::c_char)
                  as *const libc::c_char,
              b"auth\x00" as *const u8 as *const libc::c_char) !=
           0 as libc::c_int {
        MD5Update(&mut MD5,
                  b":00000000000000000000000000000000\x00" as *const u8 as
                      *const libc::c_char as *mut libc::c_uchar,
                  33 as libc::c_int as uint32_t);
    }
    MD5Final(digest.as_mut_ptr(), &mut MD5);
    memcpy(HA2.as_mut_ptr() as *mut libc::c_void,
           digest.as_mut_ptr() as *const libc::c_void,
           16 as libc::c_int as libc::c_ulong);
    /* construct response */
    MD5Init(&mut MD5);
    _digest_to_hex(HA1.as_mut_ptr() as *mut libc::c_char, hex.as_mut_ptr());
    MD5Update(&mut MD5, hex.as_mut_ptr() as *mut libc::c_uchar,
              32 as libc::c_int as uint32_t);
    MD5Update(&mut MD5,
              b":\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_uchar, 1 as libc::c_int as uint32_t);
    value =
        hash_get(table, b"nonce\x00" as *const u8 as *const libc::c_char) as
            *mut libc::c_char;
    MD5Update(&mut MD5, value as *mut libc::c_uchar,
              strlen(value) as uint32_t);
    MD5Update(&mut MD5,
              b":\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_uchar, 1 as libc::c_int as uint32_t);
    value =
        hash_get(table, b"nc\x00" as *const u8 as *const libc::c_char) as
            *mut libc::c_char;
    MD5Update(&mut MD5, value as *mut libc::c_uchar,
              strlen(value) as uint32_t);
    MD5Update(&mut MD5,
              b":\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_uchar, 1 as libc::c_int as uint32_t);
    value =
        hash_get(table, b"cnonce\x00" as *const u8 as *const libc::c_char) as
            *mut libc::c_char;
    MD5Update(&mut MD5, value as *mut libc::c_uchar,
              strlen(value) as uint32_t);
    MD5Update(&mut MD5,
              b":\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_uchar, 1 as libc::c_int as uint32_t);
    value =
        hash_get(table, b"qop\x00" as *const u8 as *const libc::c_char) as
            *mut libc::c_char;
    MD5Update(&mut MD5, value as *mut libc::c_uchar,
              strlen(value) as uint32_t);
    MD5Update(&mut MD5,
              b":\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_uchar, 1 as libc::c_int as uint32_t);
    _digest_to_hex(HA2.as_mut_ptr() as *mut libc::c_char, hex.as_mut_ptr());
    MD5Update(&mut MD5, hex.as_mut_ptr() as *mut libc::c_uchar,
              32 as libc::c_int as uint32_t);
    MD5Final(digest.as_mut_ptr(), &mut MD5);
    response =
        xmpp_alloc(ctx, (32 as libc::c_int + 1 as libc::c_int) as size_t) as
            *mut libc::c_char;
    _digest_to_hex(digest.as_mut_ptr() as *mut libc::c_char,
                   hex.as_mut_ptr());
    memcpy(response as *mut libc::c_void,
           hex.as_mut_ptr() as *const libc::c_void,
           32 as libc::c_int as libc::c_ulong);
    *response.offset(32 as libc::c_int as isize) =
        '\u{0}' as i32 as libc::c_char;
    hash_add(table, b"response\x00" as *const u8 as *const libc::c_char,
             response as *mut libc::c_void);
    /* construct reply */
    result = 0 as *mut libc::c_char; /* also frees value strings */
    rlen = 0 as libc::c_int;
    result =
        _add_key(ctx, table,
                 b"username\x00" as *const u8 as *const libc::c_char, result,
                 &mut rlen, 1 as libc::c_int);
    result =
        _add_key(ctx, table, b"realm\x00" as *const u8 as *const libc::c_char,
                 result, &mut rlen, 1 as libc::c_int);
    result =
        _add_key(ctx, table, b"nonce\x00" as *const u8 as *const libc::c_char,
                 result, &mut rlen, 1 as libc::c_int);
    result =
        _add_key(ctx, table,
                 b"cnonce\x00" as *const u8 as *const libc::c_char, result,
                 &mut rlen, 1 as libc::c_int);
    result =
        _add_key(ctx, table, b"nc\x00" as *const u8 as *const libc::c_char,
                 result, &mut rlen, 0 as libc::c_int);
    result =
        _add_key(ctx, table, b"qop\x00" as *const u8 as *const libc::c_char,
                 result, &mut rlen, 0 as libc::c_int);
    result =
        _add_key(ctx, table,
                 b"digest-uri\x00" as *const u8 as *const libc::c_char,
                 result, &mut rlen, 1 as libc::c_int);
    result =
        _add_key(ctx, table,
                 b"response\x00" as *const u8 as *const libc::c_char, result,
                 &mut rlen, 0 as libc::c_int);
    result =
        _add_key(ctx, table,
                 b"charset\x00" as *const u8 as *const libc::c_char, result,
                 &mut rlen, 0 as libc::c_int);
    xmpp_free(ctx, node as *mut libc::c_void);
    xmpp_free(ctx, domain as *mut libc::c_void);
    hash_release(table);
    /* reuse response for the base64 encode of our result */
    response =
        xmpp_base64_encode(ctx, result as *mut libc::c_uchar, strlen(result));
    xmpp_free(ctx, result as *mut libc::c_void);
    return response;
}
/* * generate auth response string for the SASL SCRAM-SHA-1 mechanism */
#[no_mangle]
pub unsafe extern "C" fn sasl_scram_sha1(mut ctx: *mut xmpp_ctx_t,
                                         mut challenge: *const libc::c_char,
                                         mut first_bare: *const libc::c_char,
                                         mut jid: *const libc::c_char,
                                         mut password: *const libc::c_char)
 -> *mut libc::c_char {
    let mut key: [uint8_t; 20] = [0; 20];
    let mut sign: [uint8_t; 20] = [0; 20];
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sval: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut sval_len: size_t = 0;
    let mut ival: libc::c_long = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut saveptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut response: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut auth: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut response_b64: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sign_b64: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut response_len: size_t = 0;
    let mut auth_len: size_t = 0;
    let mut j: libc::c_int = 0;
    tmp = xmpp_strdup(ctx, challenge);
    if tmp.is_null() { return 0 as *mut libc::c_char }
    ptr =
        strtok_r(tmp, b",\x00" as *const u8 as *const libc::c_char,
                 &mut saveptr);
    while !ptr.is_null() {
        if strncmp(ptr, b"r=\x00" as *const u8 as *const libc::c_char,
                   2 as libc::c_int as libc::c_ulong) == 0 as libc::c_int {
            r = ptr
        } else if strncmp(ptr, b"s=\x00" as *const u8 as *const libc::c_char,
                          2 as libc::c_int as libc::c_ulong) ==
                      0 as libc::c_int {
            s = ptr.offset(2 as libc::c_int as isize)
        } else if strncmp(ptr, b"i=\x00" as *const u8 as *const libc::c_char,
                          2 as libc::c_int as libc::c_ulong) ==
                      0 as libc::c_int {
            i = ptr.offset(2 as libc::c_int as isize)
        }
        ptr =
            strtok_r(0 as *mut libc::c_char,
                     b",\x00" as *const u8 as *const libc::c_char,
                     &mut saveptr)
    }
    if !(r.is_null() || s.is_null() || i.is_null()) {
        xmpp_base64_decode_bin(ctx, s, strlen(s), &mut sval, &mut sval_len);
        if !sval.is_null() {
            ival = strtol(i, &mut saveptr, 10 as libc::c_int);
            auth_len =
                (10 as libc::c_int as
                     libc::c_ulong).wrapping_add(strlen(r)).wrapping_add(strlen(first_bare)).wrapping_add(strlen(challenge));
            auth = xmpp_alloc(ctx, auth_len) as *mut libc::c_char;
            if !auth.is_null() {
                response_len =
                    (39 as libc::c_int as
                         libc::c_ulong).wrapping_add(strlen(r));
                response = xmpp_alloc(ctx, response_len) as *mut libc::c_char;
                if !response.is_null() {
                    xmpp_snprintf(response, response_len,
                                  b"c=biws,%s\x00" as *const u8 as
                                      *const libc::c_char, r);
                    xmpp_snprintf(auth, auth_len,
                                  b"%s,%s,%s\x00" as *const u8 as
                                      *const libc::c_char,
                                  first_bare.offset(3 as libc::c_int as
                                                        isize), challenge,
                                  response);
                    SCRAM_SHA1_ClientKey(password as *mut uint8_t,
                                         strlen(password),
                                         sval as *mut uint8_t, sval_len,
                                         ival as uint32_t, key.as_mut_ptr());
                    SCRAM_SHA1_ClientSignature(key.as_mut_ptr(),
                                               auth as *mut uint8_t,
                                               strlen(auth),
                                               sign.as_mut_ptr());
                    j = 0 as libc::c_int;
                    while j < 20 as libc::c_int {
                        sign[j as usize] =
                            (sign[j as usize] as libc::c_int ^
                                 key[j as usize] as libc::c_int) as uint8_t;
                        j += 1
                    }
                    sign_b64 =
                        xmpp_base64_encode(ctx, sign.as_mut_ptr(),
                                           ::std::mem::size_of::<[uint8_t; 20]>()
                                               as libc::c_ulong);
                    if !sign_b64.is_null() {
                        if strlen(response).wrapping_add(strlen(sign_b64)).wrapping_add(3
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_ulong).wrapping_add(1
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_ulong)
                               > response_len {
                            xmpp_free(ctx, sign_b64 as *mut libc::c_void);
                        } else {
                            strcat(response,
                                   b",p=\x00" as *const u8 as
                                       *const libc::c_char);
                            strcat(response, sign_b64);
                            xmpp_free(ctx, sign_b64 as *mut libc::c_void);
                            response_b64 =
                                xmpp_base64_encode(ctx,
                                                   response as
                                                       *mut libc::c_uchar,
                                                   strlen(response));
                            if !response_b64.is_null() {
                                result = response_b64
                            }
                        }
                    }
                    xmpp_free(ctx, response as *mut libc::c_void);
                }
                xmpp_free(ctx, auth as *mut libc::c_void);
            }
            xmpp_free(ctx, sval as *mut libc::c_void);
        }
    }
    xmpp_free(ctx, tmp as *mut libc::c_void);
    return result;
}
