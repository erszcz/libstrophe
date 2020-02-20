use ::libc;
extern "C" {
    pub type _hash_t;
    pub type _parser_t;
    pub type _tls;
    pub type _xmpp_rand_t;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn xmpp_free(ctx: *const xmpp_ctx_t, p: *mut libc::c_void);
    #[no_mangle]
    fn xmpp_snprintf(str: *mut libc::c_char, count: size_t,
                     fmt: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn xmpp_alloc(ctx: *const xmpp_ctx_t, size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn crypto_SHA1_Init(context: *mut SHA1_CTX);
    #[no_mangle]
    fn crypto_SHA1_Update(context: *mut SHA1_CTX, data: *const uint8_t,
                          len: size_t);
    #[no_mangle]
    fn crypto_SHA1_Final(context: *mut SHA1_CTX, digest: *mut uint8_t);
    #[no_mangle]
    fn crypto_SHA1(data: *const uint8_t, len: size_t, digest: *mut uint8_t);
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
pub type xmpp_log_t = _xmpp_log_t;
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
/* crypto.c
 * strophe XMPP client library -- public interface for digests, encodings
 *
 * Copyright (C) 2016 Dmitry Podgorny <pasis.ua@gmail.com>
 *
 *  This software is provided AS-IS with no warranty, either express
 *  or implied.
 *
 *  This program is dual licensed under the MIT and GPLv3 licenses.
 */
/* * @file
 *  Public interface for digests and encodings used in XEPs.
 */
/* * @defgroup Digests Message digests
 */
/* * @defgroup Encodings Encodings
 */
/* memset, memcpy */
/* uint8_t, size_t */
/* xmpp_snprintf */
/* xmpp_ctx_t, xmpp_free */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmpp_sha1_t {
    pub xmpp_ctx: *mut xmpp_ctx_t,
    pub ctx: SHA1_CTX,
    pub digest: [uint8_t; 20],
}
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA1_CTX {
    pub state: [uint32_t; 5],
    pub count: [uint32_t; 2],
    pub buffer: [uint8_t; 64],
}
pub type uint32_t = __uint32_t;
pub type xmpp_sha1_t = _xmpp_sha1_t;
unsafe extern "C" fn digest_to_string(mut digest: *const uint8_t,
                                      mut s: *mut libc::c_char,
                                      mut len: size_t) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    if len <
           (20 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
               libc::c_ulong {
        return 0 as *mut libc::c_char
    }
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        xmpp_snprintf(s.offset((i * 2 as libc::c_int) as isize),
                      3 as libc::c_int as size_t,
                      b"%02x\x00" as *const u8 as *const libc::c_char,
                      *digest.offset(i as isize) as libc::c_int);
        i += 1
    }
    return s;
}
unsafe extern "C" fn digest_to_string_alloc(mut ctx: *mut xmpp_ctx_t,
                                            mut digest: *const uint8_t)
 -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut slen: size_t = 0;
    slen =
        (20 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as size_t;
    s = xmpp_alloc(ctx, slen) as *mut libc::c_char;
    if !s.is_null() {
        s = digest_to_string(digest, s, slen);
        if !s.is_null() {
        } else {
            __assert_fail(b"s != NULL\x00" as *const u8 as
                              *const libc::c_char,
                          b"src/crypto.c\x00" as *const u8 as
                              *const libc::c_char,
                          59 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 60],
                                                    &[libc::c_char; 60]>(b"char *digest_to_string_alloc(xmpp_ctx_t *, const uint8_t *)\x00")).as_ptr());
        }
    }
    return s;
}
/* * Compute SHA1 message digest.
 *  Returns an allocated string which represents SHA1 message digest in
 *  hexadecimal notation. The string must be freed with xmpp_free().
 *
 *  @param ctx a Strophe context object
 *  @param data buffer for digest computation
 *  @param len size of the data buffer
 *
 *  @return an allocated string or NULL on allocation error
 *
 *  @ingroup Digests
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_sha1(mut ctx: *mut xmpp_ctx_t,
                                   mut data: *const libc::c_uchar,
                                   mut len: size_t) -> *mut libc::c_char {
    let mut digest: [uint8_t; 20] = [0; 20];
    crypto_SHA1(data as *const uint8_t, len, digest.as_mut_ptr());
    return digest_to_string_alloc(ctx, digest.as_mut_ptr());
}
/* * Compute SHA1 message digest.
 *  Stores digest in user's buffer which must be at least XMPP_SHA1_DIGEST_SIZE
 *  bytes long.
 *
 *  @param data buffer for digest computation
 *  @param len size of the data buffer
 *  @param digest output buffer of XMPP_SHA1_DIGEST_SIZE bytes
 *
 *  @ingroup Digests
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_sha1_digest(mut data: *const libc::c_uchar,
                                          mut len: size_t,
                                          mut digest: *mut libc::c_uchar) {
    crypto_SHA1(data as *const uint8_t, len, digest);
}
/* * Create new SHA1 object.
 *  SHA1 object is used to compute SHA1 digest of a buffer that is split
 *  in multiple chunks or provided in stream mode. A single buffer can be
 *  processed by short functions xmpp_sha1() and xmpp_sha1_digest().
 *  Follow the next use-case for xmpp_sha1_t object:
 *  @code
 *      xmpp_sha1_t *sha1 = xmpp_sha1_new(ctx);
 *      // Repeat update for all chunks of data
 *      xmpp_sha1_update(sha1, data, len);
 *      xmpp_sha1_final(sha1);
 *      char *digest = xmpp_sha1_to_string_alloc(sha1);
 *      xmpp_sha1_free(sha1);
 *  @endcode
 *
 *  @param ctx a Strophe context object
 *
 *  @return new SHA1 object
 *
 *  @ingroup Digests
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_sha1_new(mut ctx: *mut xmpp_ctx_t)
 -> *mut xmpp_sha1_t {
    let mut sha1: *mut xmpp_sha1_t = 0 as *mut xmpp_sha1_t;
    sha1 =
        xmpp_alloc(ctx, ::std::mem::size_of::<xmpp_sha1_t>() as libc::c_ulong)
            as *mut xmpp_sha1_t;
    if !sha1.is_null() {
        memset(sha1 as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<xmpp_sha1_t>() as libc::c_ulong);
        crypto_SHA1_Init(&mut (*sha1).ctx);
        (*sha1).xmpp_ctx = ctx
    }
    return sha1;
}
/* * Destroy SHA1 object.
 *
 *  @param sha1 a SHA1 object
 *
 *  @ingroup Digests
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_sha1_free(mut sha1: *mut xmpp_sha1_t) {
    xmpp_free((*sha1).xmpp_ctx, sha1 as *mut libc::c_void);
}
/* * Update SHA1 context with the next portion of data.
 *  Can be called repeatedly.
 *
 *  @param sha1 a SHA1 object
 *  @param data pointer to a buffer to be hashed
 *  @param len size of the data buffer
 *
 *  @ingroup Digests
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_sha1_update(mut sha1: *mut xmpp_sha1_t,
                                          mut data: *const libc::c_uchar,
                                          mut len: size_t) {
    crypto_SHA1_Update(&mut (*sha1).ctx, data, len);
}
/* * Finish SHA1 computation.
 *  Don't call xmpp_sha1_update() after this function. Retrieve resulting
 *  message digest with xmpp_sha1_to_string() or xmpp_sha1_to_digest().
 *
 *  @param sha1 a SHA1 object
 *
 *  @ingroup Digests
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_sha1_final(mut sha1: *mut xmpp_sha1_t) {
    crypto_SHA1_Final(&mut (*sha1).ctx, (*sha1).digest.as_mut_ptr());
}
/* * Return message digest rendered as a string.
 *  Stores the string to a user's buffer and returns the buffer. Call this
 *  function after xmpp_sha1_final().
 *
 *  @param sha1 a SHA1 object
 *  @param s output string
 *  @param slen size reserved for the string including '\0'
 *
 *  @return pointer s or NULL if resulting string is bigger than slen bytes
 *
 *  @ingroup Digests
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_sha1_to_string(mut sha1: *mut xmpp_sha1_t,
                                             mut s: *mut libc::c_char,
                                             mut slen: size_t)
 -> *mut libc::c_char {
    return digest_to_string((*sha1).digest.as_mut_ptr(), s, slen);
}
/* * Return message digest rendered as a string.
 *  Returns an allocated string. Free the string using the Strophe context
 *  which is passed to xmpp_sha1_new(). Call this function after
 *  xmpp_sha1_final().
 *
 *  @param sha1 a SHA1 object
 *
 *  @return an allocated string
 *
 *  @ingroup Digests
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_sha1_to_string_alloc(mut sha1: *mut xmpp_sha1_t)
 -> *mut libc::c_char {
    return digest_to_string_alloc((*sha1).xmpp_ctx,
                                  (*sha1).digest.as_mut_ptr());
}
/* * Stores message digest to a user's buffer.
 *
 *  @param sha1 a SHA1 object
 *  @param digest output buffer of XMPP_SHA1_DIGEST_SIZE bytes
 *
 *  @ingroup Digests
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_sha1_to_digest(mut sha1: *mut xmpp_sha1_t,
                                             mut digest: *mut libc::c_uchar) {
    if 20 as libc::c_int == 20 as libc::c_int {
    } else {
        __assert_fail(b"SHA1_DIGEST_SIZE == XMPP_SHA1_DIGEST_SIZE\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/crypto.c\x00" as *const u8 as *const libc::c_char,
                      213 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 57],
                                                &[libc::c_char; 57]>(b"void xmpp_sha1_to_digest(xmpp_sha1_t *, unsigned char *)\x00")).as_ptr());
    }
    memcpy(digest as *mut libc::c_void,
           (*sha1).digest.as_mut_ptr() as *const libc::c_void,
           20 as libc::c_int as libc::c_ulong);
}
/* Base64 encoding routines. Implemented according to RFC 3548. */
/* map of all byte values to the base64 values, or to
   '65' which indicates an invalid character. '=' is '64' */
static mut _base64_invcharmap: [libc::c_uchar; 256] =
    [65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 62 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 63 as libc::c_int as libc::c_uchar,
     52 as libc::c_int as libc::c_uchar, 53 as libc::c_int as libc::c_uchar,
     54 as libc::c_int as libc::c_uchar, 55 as libc::c_int as libc::c_uchar,
     56 as libc::c_int as libc::c_uchar, 57 as libc::c_int as libc::c_uchar,
     58 as libc::c_int as libc::c_uchar, 59 as libc::c_int as libc::c_uchar,
     60 as libc::c_int as libc::c_uchar, 61 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 64 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     1 as libc::c_int as libc::c_uchar, 2 as libc::c_int as libc::c_uchar,
     3 as libc::c_int as libc::c_uchar, 4 as libc::c_int as libc::c_uchar,
     5 as libc::c_int as libc::c_uchar, 6 as libc::c_int as libc::c_uchar,
     7 as libc::c_int as libc::c_uchar, 8 as libc::c_int as libc::c_uchar,
     9 as libc::c_int as libc::c_uchar, 10 as libc::c_int as libc::c_uchar,
     11 as libc::c_int as libc::c_uchar, 12 as libc::c_int as libc::c_uchar,
     13 as libc::c_int as libc::c_uchar, 14 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 16 as libc::c_int as libc::c_uchar,
     17 as libc::c_int as libc::c_uchar, 18 as libc::c_int as libc::c_uchar,
     19 as libc::c_int as libc::c_uchar, 20 as libc::c_int as libc::c_uchar,
     21 as libc::c_int as libc::c_uchar, 22 as libc::c_int as libc::c_uchar,
     23 as libc::c_int as libc::c_uchar, 24 as libc::c_int as libc::c_uchar,
     25 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 26 as libc::c_int as libc::c_uchar,
     27 as libc::c_int as libc::c_uchar, 28 as libc::c_int as libc::c_uchar,
     29 as libc::c_int as libc::c_uchar, 30 as libc::c_int as libc::c_uchar,
     31 as libc::c_int as libc::c_uchar, 32 as libc::c_int as libc::c_uchar,
     33 as libc::c_int as libc::c_uchar, 34 as libc::c_int as libc::c_uchar,
     35 as libc::c_int as libc::c_uchar, 36 as libc::c_int as libc::c_uchar,
     37 as libc::c_int as libc::c_uchar, 38 as libc::c_int as libc::c_uchar,
     39 as libc::c_int as libc::c_uchar, 40 as libc::c_int as libc::c_uchar,
     41 as libc::c_int as libc::c_uchar, 42 as libc::c_int as libc::c_uchar,
     43 as libc::c_int as libc::c_uchar, 44 as libc::c_int as libc::c_uchar,
     45 as libc::c_int as libc::c_uchar, 46 as libc::c_int as libc::c_uchar,
     47 as libc::c_int as libc::c_uchar, 48 as libc::c_int as libc::c_uchar,
     49 as libc::c_int as libc::c_uchar, 50 as libc::c_int as libc::c_uchar,
     51 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar,
     65 as libc::c_int as libc::c_uchar, 65 as libc::c_int as libc::c_uchar];
/* map of all 6-bit values to their corresponding byte
   in the base64 alphabet. Padding char is the value '64' */
static mut _base64_charmap: [libc::c_char; 65] =
    ['A' as i32 as libc::c_char, 'B' as i32 as libc::c_char,
     'C' as i32 as libc::c_char, 'D' as i32 as libc::c_char,
     'E' as i32 as libc::c_char, 'F' as i32 as libc::c_char,
     'G' as i32 as libc::c_char, 'H' as i32 as libc::c_char,
     'I' as i32 as libc::c_char, 'J' as i32 as libc::c_char,
     'K' as i32 as libc::c_char, 'L' as i32 as libc::c_char,
     'M' as i32 as libc::c_char, 'N' as i32 as libc::c_char,
     'O' as i32 as libc::c_char, 'P' as i32 as libc::c_char,
     'Q' as i32 as libc::c_char, 'R' as i32 as libc::c_char,
     'S' as i32 as libc::c_char, 'T' as i32 as libc::c_char,
     'U' as i32 as libc::c_char, 'V' as i32 as libc::c_char,
     'W' as i32 as libc::c_char, 'X' as i32 as libc::c_char,
     'Y' as i32 as libc::c_char, 'Z' as i32 as libc::c_char,
     'a' as i32 as libc::c_char, 'b' as i32 as libc::c_char,
     'c' as i32 as libc::c_char, 'd' as i32 as libc::c_char,
     'e' as i32 as libc::c_char, 'f' as i32 as libc::c_char,
     'g' as i32 as libc::c_char, 'h' as i32 as libc::c_char,
     'i' as i32 as libc::c_char, 'j' as i32 as libc::c_char,
     'k' as i32 as libc::c_char, 'l' as i32 as libc::c_char,
     'm' as i32 as libc::c_char, 'n' as i32 as libc::c_char,
     'o' as i32 as libc::c_char, 'p' as i32 as libc::c_char,
     'q' as i32 as libc::c_char, 'r' as i32 as libc::c_char,
     's' as i32 as libc::c_char, 't' as i32 as libc::c_char,
     'u' as i32 as libc::c_char, 'v' as i32 as libc::c_char,
     'w' as i32 as libc::c_char, 'x' as i32 as libc::c_char,
     'y' as i32 as libc::c_char, 'z' as i32 as libc::c_char,
     '0' as i32 as libc::c_char, '1' as i32 as libc::c_char,
     '2' as i32 as libc::c_char, '3' as i32 as libc::c_char,
     '4' as i32 as libc::c_char, '5' as i32 as libc::c_char,
     '6' as i32 as libc::c_char, '7' as i32 as libc::c_char,
     '8' as i32 as libc::c_char, '9' as i32 as libc::c_char,
     '+' as i32 as libc::c_char, '/' as i32 as libc::c_char,
     '=' as i32 as libc::c_char];
unsafe extern "C" fn base64_encoded_len(len: size_t) -> size_t {
    /* encoded steam is 4 bytes for every three, rounded up */
    return len.wrapping_add(2 as libc::c_int as
                                libc::c_ulong).wrapping_div(3 as libc::c_int
                                                                as
                                                                libc::c_ulong)
               << 2 as libc::c_int;
}
unsafe extern "C" fn base64_encode(mut ctx: *mut xmpp_ctx_t,
                                   buffer: *const libc::c_uchar, len: size_t)
 -> *mut libc::c_char {
    let mut clen: size_t = 0;
    let mut cbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut word: uint32_t = 0;
    let mut hextet: uint32_t = 0;
    let mut i: size_t = 0;
    clen = base64_encoded_len(len);
    cbuf =
        xmpp_alloc(ctx, clen.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
    if !cbuf.is_null() {
        c = cbuf;
        /* loop over data, turning every 3 bytes into 4 characters */
        i = 0 as libc::c_int as size_t;
        while i.wrapping_add(2 as libc::c_int as libc::c_ulong) < len {
            word =
                ((*buffer.offset(i as isize) as libc::c_int) <<
                     16 as libc::c_int |
                     (*buffer.offset(i.wrapping_add(1 as libc::c_int as
                                                        libc::c_ulong) as
                                         isize) as libc::c_int) <<
                         8 as libc::c_int |
                     *buffer.offset(i.wrapping_add(2 as libc::c_int as
                                                       libc::c_ulong) as
                                        isize) as libc::c_int) as uint32_t;
            hextet =
                (word & 0xfc0000 as libc::c_int as libc::c_uint) >>
                    18 as libc::c_int;
            let fresh0 = c;
            c = c.offset(1);
            *fresh0 = _base64_charmap[hextet as usize];
            hextet =
                (word & 0x3f000 as libc::c_int as libc::c_uint) >>
                    12 as libc::c_int;
            let fresh1 = c;
            c = c.offset(1);
            *fresh1 = _base64_charmap[hextet as usize];
            hextet =
                (word & 0xfc0 as libc::c_int as libc::c_uint) >>
                    6 as libc::c_int;
            let fresh2 = c;
            c = c.offset(1);
            *fresh2 = _base64_charmap[hextet as usize];
            hextet = word & 0x3f as libc::c_int as libc::c_uint;
            let fresh3 = c;
            c = c.offset(1);
            *fresh3 = _base64_charmap[hextet as usize];
            i =
                (i as
                     libc::c_ulong).wrapping_add(3 as libc::c_int as
                                                     libc::c_ulong) as size_t
                    as size_t
        }
        /* zero, one or two bytes left */
        match len.wrapping_sub(i) {
            1 => {
                hextet =
                    ((*buffer.offset(len.wrapping_sub(1 as libc::c_int as
                                                          libc::c_ulong) as
                                         isize) as libc::c_int &
                          0xfc as libc::c_int) >> 2 as libc::c_int) as
                        uint32_t; /* pad */
                let fresh4 = c; /* pad */
                c = c.offset(1); /* pad */
                *fresh4 = _base64_charmap[hextet as usize];
                hextet =
                    ((*buffer.offset(len.wrapping_sub(1 as libc::c_int as
                                                          libc::c_ulong) as
                                         isize) as libc::c_int &
                          0x3 as libc::c_int) << 4 as libc::c_int) as
                        uint32_t;
                let fresh5 = c;
                c = c.offset(1);
                *fresh5 = _base64_charmap[hextet as usize];
                let fresh6 = c;
                c = c.offset(1);
                *fresh6 = _base64_charmap[64 as libc::c_int as usize];
                let fresh7 = c;
                c = c.offset(1);
                *fresh7 = _base64_charmap[64 as libc::c_int as usize]
            }
            2 => {
                hextet =
                    ((*buffer.offset(len.wrapping_sub(2 as libc::c_int as
                                                          libc::c_ulong) as
                                         isize) as libc::c_int &
                          0xfc as libc::c_int) >> 2 as libc::c_int) as
                        uint32_t;
                let fresh8 = c;
                c = c.offset(1);
                *fresh8 = _base64_charmap[hextet as usize];
                hextet =
                    ((*buffer.offset(len.wrapping_sub(2 as libc::c_int as
                                                          libc::c_ulong) as
                                         isize) as libc::c_int &
                          0x3 as libc::c_int) << 4 as libc::c_int |
                         (*buffer.offset(len.wrapping_sub(1 as libc::c_int as
                                                              libc::c_ulong)
                                             as isize) as libc::c_int &
                              0xf0 as libc::c_int) >> 4 as libc::c_int) as
                        uint32_t;
                let fresh9 = c;
                c = c.offset(1);
                *fresh9 = _base64_charmap[hextet as usize];
                hextet =
                    ((*buffer.offset(len.wrapping_sub(1 as libc::c_int as
                                                          libc::c_ulong) as
                                         isize) as libc::c_int &
                          0xf as libc::c_int) << 2 as libc::c_int) as
                        uint32_t;
                let fresh10 = c;
                c = c.offset(1);
                *fresh10 = _base64_charmap[hextet as usize];
                let fresh11 = c;
                c = c.offset(1);
                *fresh11 = _base64_charmap[64 as libc::c_int as usize]
            }
            0 | _ => { }
        }
        /* add a terminal null */
        *c = '\u{0}' as i32 as libc::c_char
    }
    return cbuf;
}
unsafe extern "C" fn base64_decoded_len(buffer: *const libc::c_char,
                                        len: size_t) -> size_t {
    let mut nudge: size_t = 0 as libc::c_int as size_t;
    let mut c: libc::c_uchar = 0;
    let mut i: size_t = 0;
    if len < 4 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t
    }
    /* count the padding characters for the remainder */
    i = len;
    while i > 0 as libc::c_int as libc::c_ulong {
        c =
            _base64_invcharmap[*buffer.offset(i.wrapping_sub(1 as libc::c_int
                                                                 as
                                                                 libc::c_ulong)
                                                  as isize) as libc::c_uchar
                                   as usize];
        if (c as libc::c_int) < 64 as libc::c_int { break ; }
        if c as libc::c_int == 64 as libc::c_int {
            nudge = nudge.wrapping_add(1)
        }
        if c as libc::c_int > 64 as libc::c_int {
            return 0 as libc::c_int as size_t
        }
        i = i.wrapping_sub(1)
    }
    if nudge > 2 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t
    }
    /* decoded steam is 3 bytes for every four */
    return (3 as libc::c_int as
                libc::c_ulong).wrapping_mul(len >>
                                                2 as
                                                    libc::c_int).wrapping_sub(nudge);
}
unsafe extern "C" fn base64_decode(mut ctx: *mut xmpp_ctx_t,
                                   buffer: *const libc::c_char, len: size_t,
                                   mut out: *mut *mut libc::c_uchar,
                                   mut outlen: *mut size_t) {
    let mut current_block: u64;
    let mut dlen: size_t = 0;
    let mut dbuf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut d: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut word: uint32_t = 0;
    let mut hextet: uint32_t = 0 as libc::c_int as uint32_t;
    let mut i: size_t = 0;
    /* len must be a multiple of 4 */
    if !(len & 0x3 as libc::c_int as libc::c_ulong != 0) {
        dlen = base64_decoded_len(buffer, len);
        if !(dlen == 0 as libc::c_int as libc::c_ulong) {
            dbuf =
                xmpp_alloc(ctx,
                           dlen.wrapping_add(1 as libc::c_int as
                                                 libc::c_ulong)) as
                    *mut libc::c_uchar;
            if !dbuf.is_null() {
                d = dbuf;
                /* loop over each set of 4 characters, decoding 3 bytes */
                i = 0 as libc::c_int as size_t;
                while i.wrapping_add(3 as libc::c_int as libc::c_ulong) < len
                      {
                    hextet =
                        _base64_invcharmap[*buffer.offset(i as isize) as
                                               libc::c_uchar as usize] as
                            uint32_t;
                    if hextet & 0xc0 as libc::c_int as libc::c_uint != 0 {
                        break ;
                    }
                    word = hextet << 18 as libc::c_int;
                    hextet =
                        _base64_invcharmap[*buffer.offset(i.wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_ulong)
                                                              as isize) as
                                               libc::c_uchar as usize] as
                            uint32_t;
                    if hextet & 0xc0 as libc::c_int as libc::c_uint != 0 {
                        break ;
                    }
                    word |= hextet << 12 as libc::c_int;
                    hextet =
                        _base64_invcharmap[*buffer.offset(i.wrapping_add(2 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_ulong)
                                                              as isize) as
                                               libc::c_uchar as usize] as
                            uint32_t;
                    if hextet & 0xc0 as libc::c_int as libc::c_uint != 0 {
                        break ;
                    }
                    word |= hextet << 6 as libc::c_int;
                    hextet =
                        _base64_invcharmap[*buffer.offset(i.wrapping_add(3 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_ulong)
                                                              as isize) as
                                               libc::c_uchar as usize] as
                            uint32_t;
                    if hextet & 0xc0 as libc::c_int as libc::c_uint != 0 {
                        break ;
                    }
                    word |= hextet;
                    let fresh12 = d;
                    d = d.offset(1);
                    *fresh12 =
                        ((word & 0xff0000 as libc::c_int as libc::c_uint) >>
                             16 as libc::c_int) as libc::c_uchar;
                    let fresh13 = d;
                    d = d.offset(1);
                    *fresh13 =
                        ((word & 0xff00 as libc::c_int as libc::c_uint) >>
                             8 as libc::c_int) as libc::c_uchar;
                    let fresh14 = d;
                    d = d.offset(1);
                    *fresh14 =
                        (word & 0xff as libc::c_int as libc::c_uint) as
                            libc::c_uchar;
                    i =
                        (i as
                             libc::c_ulong).wrapping_add(4 as libc::c_int as
                                                             libc::c_ulong) as
                            size_t as size_t
                }
                if hextet > 64 as libc::c_int as libc::c_uint {
                    current_block = 3499658660735585650;
                } else {
                    /* handle the remainder */
                    match dlen.wrapping_rem(3 as libc::c_int as libc::c_ulong)
                        {
                        1 => {
                            /* redo the last quartet, checking for correctness */
                            hextet =
                                _base64_invcharmap[*buffer.offset(len.wrapping_sub(4
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_ulong)
                                                                      as
                                                                      isize)
                                                       as libc::c_uchar as
                                                       usize] as uint32_t;
                            if hextet & 0xc0 as libc::c_int as libc::c_uint !=
                                   0 {
                                current_block = 3499658660735585650;
                            } else {
                                word = hextet << 2 as libc::c_int;
                                hextet =
                                    _base64_invcharmap[*buffer.offset(len.wrapping_sub(3
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_ulong)
                                                                          as
                                                                          isize)
                                                           as libc::c_uchar as
                                                           usize] as uint32_t;
                                if hextet &
                                       0xc0 as libc::c_int as libc::c_uint !=
                                       0 {
                                    current_block = 3499658660735585650;
                                } else {
                                    word |= hextet >> 4 as libc::c_int;
                                    let fresh15 = d;
                                    d = d.offset(1);
                                    *fresh15 =
                                        (word &
                                             0xff as libc::c_int as
                                                 libc::c_uint) as
                                            libc::c_uchar;
                                    hextet =
                                        _base64_invcharmap[*buffer.offset(len.wrapping_sub(2
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_ulong)
                                                                              as
                                                                              isize)
                                                               as
                                                               libc::c_uchar
                                                               as usize] as
                                            uint32_t;
                                    if hextet !=
                                           64 as libc::c_int as libc::c_uint {
                                        current_block = 3499658660735585650;
                                    } else {
                                        hextet =
                                            _base64_invcharmap[*buffer.offset(len.wrapping_sub(1
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulong)
                                                                                  as
                                                                                  isize)
                                                                   as
                                                                   libc::c_uchar
                                                                   as usize]
                                                as uint32_t;
                                        if hextet !=
                                               64 as libc::c_int as
                                                   libc::c_uint {
                                            current_block =
                                                3499658660735585650;
                                        } else {
                                            current_block =
                                                10095721787123848864;
                                        }
                                    }
                                }
                            }
                        }
                        2 => {
                            /* redo the last quartet, checking for correctness */
                            hextet =
                                _base64_invcharmap[*buffer.offset(len.wrapping_sub(4
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_ulong)
                                                                      as
                                                                      isize)
                                                       as libc::c_uchar as
                                                       usize] as uint32_t;
                            if hextet & 0xc0 as libc::c_int as libc::c_uint !=
                                   0 {
                                current_block = 3499658660735585650;
                            } else {
                                word = hextet << 10 as libc::c_int;
                                hextet =
                                    _base64_invcharmap[*buffer.offset(len.wrapping_sub(3
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_ulong)
                                                                          as
                                                                          isize)
                                                           as libc::c_uchar as
                                                           usize] as uint32_t;
                                if hextet &
                                       0xc0 as libc::c_int as libc::c_uint !=
                                       0 {
                                    current_block = 3499658660735585650;
                                } else {
                                    word |= hextet << 4 as libc::c_int;
                                    hextet =
                                        _base64_invcharmap[*buffer.offset(len.wrapping_sub(2
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_ulong)
                                                                              as
                                                                              isize)
                                                               as
                                                               libc::c_uchar
                                                               as usize] as
                                            uint32_t;
                                    if hextet &
                                           0xc0 as libc::c_int as libc::c_uint
                                           != 0 {
                                        current_block = 3499658660735585650;
                                    } else {
                                        word |= hextet >> 2 as libc::c_int;
                                        let fresh16 = d;
                                        d = d.offset(1);
                                        *fresh16 =
                                            ((word &
                                                  0xff00 as libc::c_int as
                                                      libc::c_uint) >>
                                                 8 as libc::c_int) as
                                                libc::c_uchar;
                                        let fresh17 = d;
                                        d = d.offset(1);
                                        *fresh17 =
                                            (word &
                                                 0xff as libc::c_int as
                                                     libc::c_uint) as
                                                libc::c_uchar;
                                        hextet =
                                            _base64_invcharmap[*buffer.offset(len.wrapping_sub(1
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulong)
                                                                                  as
                                                                                  isize)
                                                                   as
                                                                   libc::c_uchar
                                                                   as usize]
                                                as uint32_t;
                                        if hextet !=
                                               64 as libc::c_int as
                                                   libc::c_uint {
                                            current_block =
                                                3499658660735585650;
                                        } else {
                                            current_block =
                                                10095721787123848864;
                                        }
                                    }
                                }
                            }
                        }
                        0 | _ => { current_block = 10095721787123848864; }
                    }
                    match current_block {
                        3499658660735585650 => { }
                        _ =>
                        /* nothing to do */
                        {
                            *d = '\u{0}' as i32 as libc::c_uchar;
                            current_block = 17747245473264231573;
                        }
                    }
                }
                match current_block {
                    17747245473264231573 => { }
                    _ => {
                        /* invalid character; abort decoding! */
                        xmpp_free(ctx, dbuf as *mut libc::c_void);
                        current_block = 18167736425092750904;
                    }
                }
            } else { current_block = 17747245473264231573; }
            match current_block {
                18167736425092750904 => { }
                _ => {
                    *out = dbuf;
                    *outlen =
                        if dbuf.is_null() {
                            0 as libc::c_int as libc::c_ulong
                        } else { dlen };
                    return
                }
            }
        }
    }
    *out = 0 as *mut libc::c_uchar;
    *outlen = 0 as libc::c_int as size_t;
}
/* * Base64 encoding routine.
 *  Returns an allocated string which must be freed with xmpp_free().
 *
 *  @param ctx a Strophe context
 *  @param data buffer to encode
 *  @param len size of the data buffer
 *
 *  @return an allocated null-terminated string or NULL on error
 *
 *  @ingroup Encodings
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_base64_encode(mut ctx: *mut xmpp_ctx_t,
                                            mut data: *const libc::c_uchar,
                                            mut len: size_t)
 -> *mut libc::c_char {
    return base64_encode(ctx, data, len);
}
/* * Base64 decoding routine.
 *  Returns an allocated string which must be freed with xmpp_free(). User
 *  calls this function when the result must be a string. When decoded buffer
 *  contains '\0' NULL is returned.
 *
 *  @param ctx a Strophe context
 *  @param base64 encoded buffer
 *  @param len size of the buffer
 *
 *  @return an allocated null-terminated string or NULL on error
 *
 *  @ingroup Encodings
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_base64_decode_str(mut ctx: *mut xmpp_ctx_t,
                                                mut base64:
                                                    *const libc::c_char,
                                                mut len: size_t)
 -> *mut libc::c_char {
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut buflen: size_t = 0;
    if len == 0 as libc::c_int as libc::c_ulong {
        /* handle empty string */
        buf =
            xmpp_alloc(ctx, 1 as libc::c_int as size_t) as *mut libc::c_uchar;
        if !buf.is_null() {
            *buf.offset(0 as libc::c_int as isize) =
                '\u{0}' as i32 as libc::c_uchar
        }
        buflen = 0 as libc::c_int as size_t
    } else { base64_decode(ctx, base64, len, &mut buf, &mut buflen); }
    if !buf.is_null() {
        if buflen != strlen(buf as *mut libc::c_char) {
            xmpp_free(ctx, buf as *mut libc::c_void);
            buf = 0 as *mut libc::c_uchar
        }
    }
    return buf as *mut libc::c_char;
}
/* allocate and initialize a stanza in reply to another */
/* stanza subclasses */
/* jid */
/* these return new strings that must be xmpp_free()'d */
/* event loop */
/* UUID */
/* SHA1 */
/* * @def XMPP_SHA1_DIGEST_SIZE
 *  Size of the SHA1 message digest.
 */
/* Base64 */
/* * Base64 decoding routine.
 *  Returns an allocated buffer which must be freed with xmpp_free().
 *
 *  @param ctx a Strophe context
 *  @param base64 encoded buffer
 *  @param len size of the encoded buffer
 *  @param out allocated buffer is stored here
 *  @param outlen size of the allocated buffer
 *
 *  @note on an error the `*out` will be NULL
 *
 *  @ingroup Encodings
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_base64_decode_bin(mut ctx: *mut xmpp_ctx_t,
                                                mut base64:
                                                    *const libc::c_char,
                                                mut len: size_t,
                                                mut out:
                                                    *mut *mut libc::c_uchar,
                                                mut outlen: *mut size_t) {
    base64_decode(ctx, base64, len, out, outlen);
}
