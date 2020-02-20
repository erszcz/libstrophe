use ::libc;
extern "C" {
    pub type _hash_t;
    pub type _parser_t;
    pub type _tls;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn clock() -> clock_t;
    #[no_mangle]
    fn xmpp_free(ctx: *const xmpp_ctx_t, p: *mut libc::c_void);
    #[no_mangle]
    fn xmpp_alloc(ctx: *const xmpp_ctx_t, size: size_t) -> *mut libc::c_void;
    /* timing functions */
    #[no_mangle]
    fn time_stamp() -> uint64_t;
    #[no_mangle]
    fn crypto_SHA1(data: *const uint8_t, len: size_t, digest: *mut uint8_t);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __clock_t = libc::c_long;
pub type clock_t = __clock_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmpp_rand_t {
    pub inited: libc::c_int,
    pub reseed_count: libc::c_uint,
    pub ctx: Hash_DRBG_CTX,
}
pub type Hash_DRBG_CTX = Hash_DRBG_CTX_struc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Hash_DRBG_CTX_struc {
    pub V: [uint8_t; 55],
    pub C: [uint8_t; 55],
    pub reseed_counter: uint32_t,
}
pub type uint32_t = __uint32_t;
pub type uint8_t = __uint8_t;
/* returns smallest number mupliple of y that not less than x */
/* returns smallest integer number that not less than x/y */
/* adds two arrays as numbers in big-endian representation and stores
 * result in the first one.
 */
unsafe extern "C" fn arr_add(mut arr1: *mut uint8_t, mut arr1_len: size_t,
                             mut arr2: *mut uint8_t, mut arr2_len: size_t) {
    let mut i: size_t = 0;
    let mut acc: uint32_t = 0;
    let mut carry: uint32_t = 0 as libc::c_int as uint32_t;
    if arr1_len >= arr2_len {
    } else {
        __assert_fail(b"arr1_len >= arr2_len\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/rand.c\x00" as *const u8 as *const libc::c_char,
                      72 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 51],
                                                &[libc::c_char; 51]>(b"void arr_add(uint8_t *, size_t, uint8_t *, size_t)\x00")).as_ptr());
    }
    i = 1 as libc::c_int as size_t;
    while i <= arr2_len ||
              carry != 0 as libc::c_int as libc::c_uint && i <= arr1_len {
        acc =
            (*arr1.offset(arr1_len.wrapping_sub(i) as isize) as
                 uint32_t).wrapping_add(carry);
        if i <= arr2_len {
            acc =
                (acc as
                     libc::c_uint).wrapping_add(*arr2.offset(arr2_len.wrapping_sub(i)
                                                                 as isize) as
                                                    uint32_t) as uint32_t as
                    uint32_t
        }
        carry = acc >> 8 as libc::c_int;
        *arr1.offset(arr1_len.wrapping_sub(i) as isize) =
            (acc & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        i = i.wrapping_add(1)
    };
}
/* stores 32-bit number in big-endian representation */
unsafe extern "C" fn store_be32(mut val: uint32_t, mut be: *mut uint8_t) {
    *be.offset(0 as libc::c_int as isize) =
        (val >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as
            uint8_t;
    *be.offset(1 as libc::c_int as isize) =
        (val >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as
            uint8_t;
    *be.offset(2 as libc::c_int as isize) =
        (val >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as
            uint8_t;
    *be.offset(3 as libc::c_int as isize) =
        (val & 0xff as libc::c_int as libc::c_uint) as uint8_t;
}
unsafe extern "C" fn Hash_df(mut input_string: *mut uint8_t,
                             mut input_string_len: size_t,
                             mut output_string: *mut uint8_t,
                             mut no_of_bytes_to_return: size_t) {
    let mut counter: uint8_t = 0;
    let mut temp: [uint8_t; 60] = [0; 60];
    let mut conj: [uint8_t; 197] = [0; 197];
    let mut len: size_t = 0;
    let mut i: size_t = 0;
    let mut offset: size_t = 0;
    if no_of_bytes_to_return <=
           ::std::mem::size_of::<[uint8_t; 60]>() as libc::c_ulong {
    } else {
        __assert_fail(b"no_of_bytes_to_return <= sizeof(temp)\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/rand.c\x00" as *const u8 as *const libc::c_char,
                      102 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 51],
                                                &[libc::c_char; 51]>(b"void Hash_df(uint8_t *, size_t, uint8_t *, size_t)\x00")).as_ptr());
    }
    if input_string_len.wrapping_add(5 as libc::c_int as libc::c_ulong) <=
           ::std::mem::size_of::<[uint8_t; 197]>() as libc::c_ulong {
    } else {
        __assert_fail(b"input_string_len + 5 <= sizeof(conj)\x00" as *const u8
                          as *const libc::c_char,
                      b"src/rand.c\x00" as *const u8 as *const libc::c_char,
                      103 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 51],
                                                &[libc::c_char; 51]>(b"void Hash_df(uint8_t *, size_t, uint8_t *, size_t)\x00")).as_ptr());
    }
    len =
        no_of_bytes_to_return.wrapping_add(20 as libc::c_int as
                                               libc::c_ulong).wrapping_sub(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_ulong).wrapping_div(20
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong);
    i = 1 as libc::c_int as size_t;
    while i <= len {
        offset =
            i.wrapping_sub(1 as libc::c_int as
                               libc::c_ulong).wrapping_mul(20 as libc::c_int
                                                               as
                                                               libc::c_ulong);
        counter = i as uint8_t;
        conj[0 as libc::c_int as usize] = counter;
        store_be32((no_of_bytes_to_return as
                        uint32_t).wrapping_mul(8 as libc::c_int as
                                                   libc::c_uint),
                   conj.as_mut_ptr().offset(1 as libc::c_int as isize));
        memcpy(conj.as_mut_ptr().offset(5 as libc::c_int as isize) as
                   *mut libc::c_void, input_string as *const libc::c_void,
               input_string_len);
        crypto_SHA1(conj.as_mut_ptr(),
                    input_string_len.wrapping_add(5 as libc::c_int as
                                                      libc::c_ulong),
                    temp.as_mut_ptr().offset(offset as isize));
        i = i.wrapping_add(1)
    }
    memcpy(output_string as *mut libc::c_void,
           temp.as_mut_ptr() as *const libc::c_void, no_of_bytes_to_return);
}
/* assume personalization_string is zero length string */
unsafe extern "C" fn Hash_DRBG_Instantiate(mut ctx: *mut Hash_DRBG_CTX,
                                           mut entropy_input: *mut uint8_t,
                                           mut entropy_input_len: size_t,
                                           mut nonce: *mut uint8_t,
                                           mut nonce_len: size_t) {
    let mut seed_material: [uint8_t; 136] = [0; 136];
    let mut seed0: [uint8_t; 56] = [0; 56];
    let mut seed: *mut uint8_t =
        seed0.as_mut_ptr().offset(1 as libc::c_int as isize);
    if entropy_input_len <= 128 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"entropy_input_len <= ENTROPY_MAX\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/rand.c\x00" as *const u8 as *const libc::c_char,
                      128 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 82],
                                                &[libc::c_char; 82]>(b"void Hash_DRBG_Instantiate(Hash_DRBG_CTX *, uint8_t *, size_t, uint8_t *, size_t)\x00")).as_ptr());
    }
    if nonce_len <= 8 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"nonce_len <= NONCE_MAX\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/rand.c\x00" as *const u8 as *const libc::c_char,
                      129 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 82],
                                                &[libc::c_char; 82]>(b"void Hash_DRBG_Instantiate(Hash_DRBG_CTX *, uint8_t *, size_t, uint8_t *, size_t)\x00")).as_ptr());
    }
    if !nonce.is_null() || nonce_len == 0 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"nonce != NULL || nonce_len == 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/rand.c\x00" as *const u8 as *const libc::c_char,
                      130 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 82],
                                                &[libc::c_char; 82]>(b"void Hash_DRBG_Instantiate(Hash_DRBG_CTX *, uint8_t *, size_t, uint8_t *, size_t)\x00")).as_ptr());
    }
    memcpy(seed_material.as_mut_ptr() as *mut libc::c_void,
           entropy_input as *const libc::c_void, entropy_input_len);
    if !nonce.is_null() {
        memcpy(seed_material.as_mut_ptr().offset(entropy_input_len as isize)
                   as *mut libc::c_void, nonce as *const libc::c_void,
               nonce_len);
    }
    Hash_df(seed_material.as_mut_ptr(),
            entropy_input_len.wrapping_add(nonce_len), seed,
            (440 as libc::c_int / 8 as libc::c_int) as size_t);
    seed0[0 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    memcpy((*ctx).V.as_mut_ptr() as *mut libc::c_void,
           seed as *const libc::c_void,
           (440 as libc::c_int / 8 as libc::c_int) as libc::c_ulong);
    Hash_df(seed0.as_mut_ptr(),
            ::std::mem::size_of::<[uint8_t; 56]>() as libc::c_ulong,
            (*ctx).C.as_mut_ptr(),
            (440 as libc::c_int / 8 as libc::c_int) as size_t);
    (*ctx).reseed_counter = 1 as libc::c_int as uint32_t;
}
/* assume additional_input is zero length string */
unsafe extern "C" fn Hash_DRBG_Reseed(mut ctx: *mut Hash_DRBG_CTX,
                                      mut entropy_input: *mut uint8_t,
                                      mut entropy_input_len: size_t) {
    let mut seed_material: [uint8_t; 184] = [0; 184];
    let mut seed0: [uint8_t; 56] = [0; 56];
    let mut seed: *mut uint8_t =
        seed0.as_mut_ptr().offset(1 as libc::c_int as isize);
    if entropy_input_len <= 128 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"entropy_input_len <= ENTROPY_MAX\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/rand.c\x00" as *const u8 as *const libc::c_char,
                      152 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 58],
                                                &[libc::c_char; 58]>(b"void Hash_DRBG_Reseed(Hash_DRBG_CTX *, uint8_t *, size_t)\x00")).as_ptr());
    }
    seed_material[0 as libc::c_int as usize] = 1 as libc::c_int as uint8_t;
    memcpy(seed_material.as_mut_ptr().offset(1 as libc::c_int as isize) as
               *mut libc::c_void,
           (*ctx).V.as_mut_ptr() as *const libc::c_void,
           (440 as libc::c_int / 8 as libc::c_int) as libc::c_ulong);
    memcpy(seed_material.as_mut_ptr().offset(1 as libc::c_int as
                                                 isize).offset((440 as
                                                                    libc::c_int
                                                                    /
                                                                    8 as
                                                                        libc::c_int)
                                                                   as isize)
               as *mut libc::c_void, entropy_input as *const libc::c_void,
           entropy_input_len);
    Hash_df(seed_material.as_mut_ptr(),
            entropy_input_len.wrapping_add((440 as libc::c_int /
                                                8 as libc::c_int) as
                                               libc::c_ulong).wrapping_add(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_ulong),
            seed, (440 as libc::c_int / 8 as libc::c_int) as size_t);
    seed0[0 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    memcpy((*ctx).V.as_mut_ptr() as *mut libc::c_void,
           seed as *const libc::c_void,
           (440 as libc::c_int / 8 as libc::c_int) as libc::c_ulong);
    Hash_df(seed0.as_mut_ptr(),
            ::std::mem::size_of::<[uint8_t; 56]>() as libc::c_ulong,
            (*ctx).C.as_mut_ptr(),
            (440 as libc::c_int / 8 as libc::c_int) as size_t);
    (*ctx).reseed_counter = 1 as libc::c_int as uint32_t;
}
unsafe extern "C" fn Hashgen(mut V: *mut uint8_t, mut output: *mut uint8_t,
                             mut requested_number_of_bytes: size_t) {
    let mut data: [uint8_t; 55] = [0; 55];
    let mut W: [uint8_t; 200] = [0; 200];
    let mut i1: uint8_t = 1 as libc::c_int as uint8_t;
    let mut m: size_t = 0;
    let mut i: size_t = 0;
    let mut offset: size_t = 0;
    if requested_number_of_bytes <=
           ::std::mem::size_of::<[uint8_t; 200]>() as libc::c_ulong {
    } else {
        __assert_fail(b"requested_number_of_bytes <= sizeof(W)\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/rand.c\x00" as *const u8 as *const libc::c_char,
                      175 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 43],
                                                &[libc::c_char; 43]>(b"void Hashgen(uint8_t *, uint8_t *, size_t)\x00")).as_ptr());
    }
    m =
        requested_number_of_bytes.wrapping_add(20 as libc::c_int as
                                                   libc::c_ulong).wrapping_sub(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_ulong).wrapping_div(20
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_ulong);
    memcpy(data.as_mut_ptr() as *mut libc::c_void, V as *const libc::c_void,
           (440 as libc::c_int / 8 as libc::c_int) as libc::c_ulong);
    i = 1 as libc::c_int as size_t;
    while i <= m {
        offset =
            i.wrapping_sub(1 as libc::c_int as
                               libc::c_ulong).wrapping_mul(20 as libc::c_int
                                                               as
                                                               libc::c_ulong);
        crypto_SHA1(data.as_mut_ptr(),
                    (440 as libc::c_int / 8 as libc::c_int) as size_t,
                    W.as_mut_ptr().offset(offset as isize));
        /* increase data by 1 */
        arr_add(data.as_mut_ptr(),
                ::std::mem::size_of::<[uint8_t; 55]>() as libc::c_ulong,
                &mut i1, 1 as libc::c_int as size_t);
        i = i.wrapping_add(1)
    }
    memcpy(output as *mut libc::c_void, W.as_mut_ptr() as *const libc::c_void,
           requested_number_of_bytes);
}
/* assume additional_input is zero length string */
unsafe extern "C" fn Hash_DRBG_Generate(mut ctx: *mut Hash_DRBG_CTX,
                                        mut output: *mut uint8_t,
                                        mut requested_number_of_bytes: size_t)
 -> libc::c_int {
    let mut H: [uint8_t; 20] = [0; 20];
    let mut V3: [uint8_t; 56] = [0; 56];
    let mut reseed_counter: [uint8_t; 4] = [0; 4];
    if (*ctx).reseed_counter > 0x7fffffff as libc::c_int as libc::c_uint ||
           (*ctx).reseed_counter == 0 as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int)
    }
    Hashgen((*ctx).V.as_mut_ptr(), output, requested_number_of_bytes);
    V3[0 as libc::c_int as usize] = 3 as libc::c_int as uint8_t;
    memcpy(V3.as_mut_ptr().offset(1 as libc::c_int as isize) as
               *mut libc::c_void,
           (*ctx).V.as_mut_ptr() as *const libc::c_void,
           (440 as libc::c_int / 8 as libc::c_int) as libc::c_ulong);
    crypto_SHA1(V3.as_mut_ptr(),
                ::std::mem::size_of::<[uint8_t; 56]>() as libc::c_ulong,
                H.as_mut_ptr());
    arr_add((*ctx).V.as_mut_ptr(),
            ::std::mem::size_of::<[uint8_t; 55]>() as libc::c_ulong,
            (*ctx).C.as_mut_ptr(),
            ::std::mem::size_of::<[uint8_t; 55]>() as libc::c_ulong);
    arr_add((*ctx).V.as_mut_ptr(),
            ::std::mem::size_of::<[uint8_t; 55]>() as libc::c_ulong,
            H.as_mut_ptr(),
            ::std::mem::size_of::<[uint8_t; 20]>() as libc::c_ulong);
    store_be32((*ctx).reseed_counter, reseed_counter.as_mut_ptr());
    arr_add((*ctx).V.as_mut_ptr(),
            ::std::mem::size_of::<[uint8_t; 55]>() as libc::c_ulong,
            reseed_counter.as_mut_ptr(),
            ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong);
    (*ctx).reseed_counter = (*ctx).reseed_counter.wrapping_add(1);
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmpp_rand_reseed(mut rand: *mut xmpp_rand_t) {
    let mut entropy: [uint8_t; 128] = [0; 128];
    let mut ptr: *mut uint8_t = entropy.as_mut_ptr();
    let mut last: *const uint8_t =
        entropy.as_mut_ptr().offset(::std::mem::size_of::<[uint8_t; 128]>() as
                                        libc::c_ulong as isize);
    let mut len: size_t = 0;
    /* entropy:
     *  1. time_stamp()
     *  2. clock(3)
     *  3. xmpp_rand_t address to make unique seed within one process
     *  4. counter to make unique seed within one context
     *  5. stack address
     *  6. local ports of every connection in list (getsockname)
     *  7. other non-constant info that can be retieved from socket
     *
     *  rand(3) can't be used as it isn't thread-safe.
     *  XXX 6 and 7 are not implemented yet.
     */
    let mut __arg: uint64_t = time_stamp();
    if (ptr as
            *mut libc::c_char).offset(::std::mem::size_of::<uint64_t>() as
                                          libc::c_ulong as isize) <
           last as *mut libc::c_char {
        *(ptr as *mut uint64_t) = __arg;
        ptr =
            (ptr as
                 *mut libc::c_char).offset(::std::mem::size_of::<uint64_t>()
                                               as libc::c_ulong as isize) as
                *mut libc::c_void as *mut uint8_t
    }
    let mut __arg_0: clock_t = clock();
    if (ptr as
            *mut libc::c_char).offset(::std::mem::size_of::<clock_t>() as
                                          libc::c_ulong as isize) <
           last as *mut libc::c_char {
        *(ptr as *mut clock_t) = __arg_0;
        ptr =
            (ptr as
                 *mut libc::c_char).offset(::std::mem::size_of::<clock_t>() as
                                               libc::c_ulong as isize) as
                *mut libc::c_void as *mut uint8_t
    }
    let mut __arg_1: *mut libc::c_void = rand as *mut libc::c_void;
    if (ptr as
            *mut libc::c_char).offset(::std::mem::size_of::<*mut libc::c_void>()
                                          as libc::c_ulong as isize) <
           last as *mut libc::c_char {
        let ref mut fresh0 = *(ptr as *mut *mut libc::c_void);
        *fresh0 = __arg_1;
        ptr =
            (ptr as
                 *mut libc::c_char).offset(::std::mem::size_of::<*mut libc::c_void>()
                                               as libc::c_ulong as isize) as
                *mut libc::c_void as *mut uint8_t
    }
    (*rand).reseed_count = (*rand).reseed_count.wrapping_add(1);
    let mut __arg_2: libc::c_uint = (*rand).reseed_count;
    if (ptr as
            *mut libc::c_char).offset(::std::mem::size_of::<libc::c_uint>() as
                                          libc::c_ulong as isize) <
           last as *mut libc::c_char {
        *(ptr as *mut libc::c_uint) = __arg_2;
        ptr =
            (ptr as
                 *mut libc::c_char).offset(::std::mem::size_of::<libc::c_uint>()
                                               as libc::c_ulong as isize) as
                *mut libc::c_void as *mut uint8_t
    }
    let mut __arg_3: *mut libc::c_void =
        &mut entropy as *mut [uint8_t; 128] as *mut libc::c_void;
    if (ptr as
            *mut libc::c_char).offset(::std::mem::size_of::<*mut libc::c_void>()
                                          as libc::c_ulong as isize) <
           last as *mut libc::c_char {
        let ref mut fresh1 = *(ptr as *mut *mut libc::c_void);
        *fresh1 = __arg_3;
        ptr =
            (ptr as
                 *mut libc::c_char).offset(::std::mem::size_of::<*mut libc::c_void>()
                                               as libc::c_ulong as isize) as
                *mut libc::c_void as *mut uint8_t
    }
    len =
        ptr.wrapping_offset_from(entropy.as_mut_ptr()) as libc::c_long as
            size_t;
    if (*rand).inited != 0 {
        Hash_DRBG_Reseed(&mut (*rand).ctx, entropy.as_mut_ptr(), len);
    } else {
        Hash_DRBG_Instantiate(&mut (*rand).ctx, entropy.as_mut_ptr(), len,
                              0 as *mut uint8_t, 0 as libc::c_int as size_t);
        (*rand).inited = 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmpp_rand_new(mut ctx: *mut xmpp_ctx_t)
 -> *mut xmpp_rand_t {
    let mut out: *mut xmpp_rand_t =
        xmpp_alloc(ctx, ::std::mem::size_of::<xmpp_rand_t>() as libc::c_ulong)
            as *mut xmpp_rand_t;
    if !out.is_null() {
        memset(out as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<xmpp_rand_t>() as libc::c_ulong);
    }
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn xmpp_rand_free(mut ctx: *mut xmpp_ctx_t,
                                        mut rand: *mut xmpp_rand_t) {
    xmpp_free(ctx, rand as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmpp_rand_bytes(mut rand: *mut xmpp_rand_t,
                                         mut output: *mut libc::c_uchar,
                                         mut len: size_t) {
    let mut rc: libc::c_int = 0;
    rc = Hash_DRBG_Generate(&mut (*rand).ctx, output as *mut uint8_t, len);
    if rc == -(1 as libc::c_int) {
        xmpp_rand_reseed(rand);
        rc =
            Hash_DRBG_Generate(&mut (*rand).ctx, output as *mut uint8_t, len);
        if rc == 0 as libc::c_int {
        } else {
            __assert_fail(b"rc == 0\x00" as *const u8 as *const libc::c_char,
                          b"src/rand.c\x00" as *const u8 as
                              *const libc::c_char,
                          280 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 61],
                                                    &[libc::c_char; 61]>(b"void xmpp_rand_bytes(xmpp_rand_t *, unsigned char *, size_t)\x00")).as_ptr());
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmpp_rand(mut rand: *mut xmpp_rand_t)
 -> libc::c_int {
    let mut result: libc::c_int = 0;
    xmpp_rand_bytes(rand,
                    &mut result as *mut libc::c_int as *mut libc::c_uchar,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    return result;
}
unsafe extern "C" fn rand_byte2hex(mut byte: libc::c_uchar,
                                   mut hex: *mut libc::c_char) {
    static mut hex_tbl: [libc::c_char; 16] =
        ['0' as i32 as libc::c_char, '1' as i32 as libc::c_char,
         '2' as i32 as libc::c_char, '3' as i32 as libc::c_char,
         '4' as i32 as libc::c_char, '5' as i32 as libc::c_char,
         '6' as i32 as libc::c_char, '7' as i32 as libc::c_char,
         '8' as i32 as libc::c_char, '9' as i32 as libc::c_char,
         'A' as i32 as libc::c_char, 'B' as i32 as libc::c_char,
         'C' as i32 as libc::c_char, 'D' as i32 as libc::c_char,
         'E' as i32 as libc::c_char, 'F' as i32 as libc::c_char];
    *hex.offset(0 as libc::c_int as isize) =
        hex_tbl[(byte as libc::c_int >> 4 as libc::c_int & 0xf as libc::c_int)
                    as usize];
    *hex.offset(1 as libc::c_int as isize) =
        hex_tbl[(byte as libc::c_int & 0xf as libc::c_int) as usize];
}
/* * Create new xmpp_rand_t object.
 *
 *  @param ctx A Strophe context object
 *
 *  @ingroup Random
 */
/* * Destroy an xmpp_rand_t object.
 *
 *  @param ctx A Strophe context object
 *
 *  @ingroup Random
 */
/* * Generate random integer.
 *  Analogue of rand(3).
 *
 *  @ingroup Random
 */
/* * Generate random bytes.
 *  Generates len bytes and stores them to the output buffer.
 *
 *  @ingroup Random
 */
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
pub unsafe extern "C" fn xmpp_rand_nonce(mut rand: *mut xmpp_rand_t,
                                         mut output: *mut libc::c_char,
                                         mut len: size_t) {
    let mut i: size_t = 0;
    let rand_len: size_t =
        len.wrapping_div(2 as libc::c_int as libc::c_ulong);
    /*
     * We don't want to use any allocation here, because this function
     * can't fail. Also we want to avoid VLA.
     * Current implementation uses half of the output buffer for random buffer
     * generation and then converts it to HEX representation.
     */
    if rand_len > 0 as libc::c_int as libc::c_ulong {
        xmpp_rand_bytes(rand, output as *mut libc::c_uchar, rand_len);
        i = rand_len;
        while i > 0 as libc::c_int as libc::c_ulong {
            rand_byte2hex(*output.offset(i.wrapping_sub(1 as libc::c_int as
                                                            libc::c_ulong) as
                                             isize) as libc::c_uchar,
                          &mut *output.offset(i.wrapping_sub(1 as libc::c_int
                                                                 as
                                                                 libc::c_ulong).wrapping_mul(2
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong)
                                                  as isize));
            i = i.wrapping_sub(1)
        }
    }
    if len > 0 as libc::c_int as libc::c_ulong {
        *output.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as
                           isize) = '\u{0}' as i32 as libc::c_char
    };
}
