use ::libc;
extern "C" {
    pub type XML_ParserStruct;
    pub type _hash_t;
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
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strncat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn XML_ParserCreate_MM(encoding: *const XML_Char,
                           memsuite: *const XML_Memory_Handling_Suite,
                           namespaceSeparator: *const XML_Char) -> XML_Parser;
    #[no_mangle]
    fn XML_ParserReset(parser: XML_Parser, encoding: *const XML_Char)
     -> XML_Bool;
    #[no_mangle]
    fn XML_SetElementHandler(parser: XML_Parser,
                             start: XML_StartElementHandler,
                             end: XML_EndElementHandler);
    #[no_mangle]
    fn XML_SetCharacterDataHandler(parser: XML_Parser,
                                   handler: XML_CharacterDataHandler);
    #[no_mangle]
    fn XML_SetUserData(parser: XML_Parser, userData: *mut libc::c_void);
    #[no_mangle]
    fn XML_Parse(parser: XML_Parser, s: *const libc::c_char, len: libc::c_int,
                 isFinal: libc::c_int) -> XML_Status;
    #[no_mangle]
    fn XML_ParserFree(parser: XML_Parser);
    /* free some blocks returned by other APIs, for example the
   buffer you get from xmpp_stanza_to_text */
    #[no_mangle]
    fn xmpp_free(ctx: *const xmpp_ctx_t, p: *mut libc::c_void);
    /*
void xmpp_register_stanza_handler(conn, stanza, xmlns, type, handler)
*/
    /* stanzas */
    /* allocate and initialize a blank stanza */
    #[no_mangle]
    fn xmpp_stanza_new(ctx: *mut xmpp_ctx_t) -> *mut xmpp_stanza_t;
    /* free a stanza object and it's contents */
    #[no_mangle]
    fn xmpp_stanza_release(stanza: *mut xmpp_stanza_t) -> libc::c_int;
    #[no_mangle]
    fn xmpp_stanza_add_child(stanza: *mut xmpp_stanza_t,
                             child: *mut xmpp_stanza_t) -> libc::c_int;
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
    #[no_mangle]
    fn xmpp_stanza_set_ns(stanza: *mut xmpp_stanza_t, ns: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn xmpp_realloc(ctx: *const xmpp_ctx_t, p: *mut libc::c_void,
                    size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xmpp_alloc(ctx: *const xmpp_ctx_t, size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xmpp_strdup(ctx: *const xmpp_ctx_t, s: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn xmpp_error(ctx: *const xmpp_ctx_t, area: *const libc::c_char,
                  fmt: *const libc::c_char, _: ...);
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type XML_Char = libc::c_char;
pub type XML_Parser = *mut XML_ParserStruct;
pub type XML_Bool = libc::c_uchar;
pub type XML_Status = libc::c_uint;
pub const XML_STATUS_SUSPENDED: XML_Status = 2;
pub const XML_STATUS_OK: XML_Status = 1;
pub const XML_STATUS_ERROR: XML_Status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XML_Memory_Handling_Suite {
    pub malloc_fcn: Option<unsafe extern "C" fn(_: size_t)
                               -> *mut libc::c_void>,
    pub realloc_fcn: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                 _: size_t)
                                -> *mut libc::c_void>,
    pub free_fcn: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
}
pub type XML_StartElementHandler
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const XML_Char,
                                _: *mut *const XML_Char) -> ()>;
pub type XML_EndElementHandler
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const XML_Char)
               -> ()>;
pub type XML_CharacterDataHandler
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const XML_Char,
                                _: libc::c_int) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _parser_t {
    pub ctx: *mut xmpp_ctx_t,
    pub expat: XML_Parser,
    pub startcb: parser_start_callback,
    pub endcb: parser_end_callback,
    pub stanzacb: parser_stanza_callback,
    pub userdata: *mut libc::c_void,
    pub depth: libc::c_int,
    pub stanza: *mut xmpp_stanza_t,
    pub inner_text: *mut libc::c_char,
    pub inner_text_size: libc::c_int,
    pub inner_text_used: libc::c_int,
}
pub type parser_stanza_callback
    =
    Option<unsafe extern "C" fn(_: *mut xmpp_stanza_t, _: *mut libc::c_void)
               -> ()>;
pub type parser_end_callback
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_char, _: *mut libc::c_void)
               -> ()>;
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
pub type parser_start_callback
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                _: *mut *mut libc::c_char,
                                _: *mut libc::c_void) -> ()>;
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
/* Use the Unit Separator to delimit namespace and name in our XML */
#[no_mangle]
pub static mut namespace_sep: XML_Char = '\u{1f}' as i32 as XML_Char;
/*
 * Cached strophe ctx. It is used for memory suite.
 * Note, expat doesn't support userdata in memory suite, therefore,
 * we can support only one strophe context. If user creates more than one
 * context, this module will fallback to default library allocator for all
 * contexts other than mem_ctx.
 */
static mut mem_ctx: *mut xmpp_ctx_t =
    0 as *const xmpp_ctx_t as *mut xmpp_ctx_t;
unsafe extern "C" fn parser_mem_malloc(mut size: size_t)
 -> *mut libc::c_void {
    if !mem_ctx.is_null() {
        return xmpp_alloc(mem_ctx, size)
    } else { return 0 as *mut libc::c_void };
}
unsafe extern "C" fn parser_mem_realloc(mut ptr: *mut libc::c_void,
                                        mut size: size_t)
 -> *mut libc::c_void {
    if !mem_ctx.is_null() {
        return xmpp_realloc(mem_ctx, ptr, size)
    } else { return 0 as *mut libc::c_void };
}
unsafe extern "C" fn parser_mem_free(mut ptr: *mut libc::c_void) {
    if !mem_ctx.is_null() { xmpp_free(mem_ctx, ptr); };
}
static mut parser_mem_suite: XML_Memory_Handling_Suite =
    {
        let mut init =
            XML_Memory_Handling_Suite{malloc_fcn:
                                          Some(parser_mem_malloc as
                                                   unsafe extern "C" fn(_:
                                                                            size_t)
                                                       -> *mut libc::c_void),
                                      realloc_fcn:
                                          Some(parser_mem_realloc as
                                                   unsafe extern "C" fn(_:
                                                                            *mut libc::c_void,
                                                                        _:
                                                                            size_t)
                                                       -> *mut libc::c_void),
                                      free_fcn:
                                          Some(parser_mem_free as
                                                   unsafe extern "C" fn(_:
                                                                            *mut libc::c_void)
                                                       -> ()),};
        init
    };
/* return allocated string with the name from a delimited
 * namespace/name string */
unsafe extern "C" fn _xml_name(mut ctx: *mut xmpp_ctx_t,
                               mut nsname: *const libc::c_char)
 -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    c = strchr(nsname, namespace_sep as libc::c_int);
    if c.is_null() { return xmpp_strdup(ctx, nsname) }
    c = c.offset(1);
    len = strlen(c);
    result =
        xmpp_alloc(ctx, len.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
    if !result.is_null() {
        memcpy(result as *mut libc::c_void, c as *const libc::c_void, len);
        *result.offset(len as isize) = '\u{0}' as i32 as libc::c_char
    }
    return result;
}
/* return allocated string with the namespace from a delimited string */
unsafe extern "C" fn _xml_namespace(mut ctx: *mut xmpp_ctx_t,
                                    mut nsname: *const libc::c_char)
 -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: *const libc::c_char = 0 as *const libc::c_char;
    c = strchr(nsname, namespace_sep as libc::c_int);
    if !c.is_null() {
        result =
            xmpp_alloc(ctx,
                       (c.wrapping_offset_from(nsname) as libc::c_long +
                            1 as libc::c_int as libc::c_long) as size_t) as
                *mut libc::c_char;
        if !result.is_null() {
            memcpy(result as *mut libc::c_void, nsname as *const libc::c_void,
                   c.wrapping_offset_from(nsname) as libc::c_long as
                       libc::c_ulong);
            *result.offset(c.wrapping_offset_from(nsname) as libc::c_long as
                               isize) = '\u{0}' as i32 as libc::c_char
        }
    }
    return result;
}
unsafe extern "C" fn _set_attributes(mut stanza: *mut xmpp_stanza_t,
                                     mut attrs: *mut *const XML_Char) {
    let mut attr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if attrs.is_null() { return }
    i = 0 as libc::c_int;
    while !(*attrs.offset(i as isize)).is_null() {
        /* namespaced attributes aren't used in xmpp, discard namespace */
        attr = _xml_name((*stanza).ctx, *attrs.offset(i as isize));
        xmpp_stanza_set_attribute(stanza, attr,
                                  *attrs.offset((i + 1 as libc::c_int) as
                                                    isize));
        xmpp_free((*stanza).ctx, attr as *mut libc::c_void);
        i += 2 as libc::c_int
    };
}
unsafe extern "C" fn complete_inner_text(mut parser: *mut parser_t) {
    let mut stanza: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    if !(*parser).inner_text.is_null() {
        /* create and populate stanza */
        stanza = xmpp_stanza_new((*parser).ctx);
        /* FIXME: disconnect on allocation error */
        if !stanza.is_null() {
            xmpp_stanza_set_text(stanza, (*parser).inner_text);
            xmpp_stanza_add_child((*parser).stanza, stanza);
            xmpp_stanza_release(stanza);
        }
        xmpp_free((*parser).ctx, (*parser).inner_text as *mut libc::c_void);
        (*parser).inner_text = 0 as *mut libc::c_char;
        (*parser).inner_text_size = 0 as libc::c_int;
        (*parser).inner_text_used = 0 as libc::c_int
    };
}
unsafe extern "C" fn _start_element(mut userdata: *mut libc::c_void,
                                    mut nsname: *const XML_Char,
                                    mut attrs: *mut *const XML_Char) {
    let mut parser: *mut parser_t = userdata as *mut parser_t;
    let mut child: *mut xmpp_stanza_t = 0 as *mut xmpp_stanza_t;
    let mut ns: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    ns = _xml_namespace((*parser).ctx, nsname);
    name = _xml_name((*parser).ctx, nsname);
    if (*parser).depth == 0 as libc::c_int {
        /* notify the owner */
        if (*parser).startcb.is_some() {
            (*parser).startcb.expect("non-null function pointer")(name,
                                                                  attrs as
                                                                      *mut *mut libc::c_char,
                                                                  (*parser).userdata);
        }
    } else if (*parser).stanza.is_null() &&
                  (*parser).depth != 1 as libc::c_int {
        /* build stanzas at depth 1 */
        /* something terrible happened */
            /* FIXME: shutdown disconnect */
        xmpp_error((*parser).ctx,
                   b"parser\x00" as *const u8 as *const libc::c_char,
                   b"oops, where did our stanza go?\x00" as *const u8 as
                       *const libc::c_char);
    } else {
        child = xmpp_stanza_new((*parser).ctx);
        child.is_null();
        xmpp_stanza_set_name(child, name);
        _set_attributes(child, attrs);
        if !ns.is_null() { xmpp_stanza_set_ns(child, ns); }
        if !(*parser).stanza.is_null() {
            complete_inner_text(parser);
            xmpp_stanza_add_child((*parser).stanza, child);
            xmpp_stanza_release(child);
        }
        (*parser).stanza = child
    }
    if !ns.is_null() { xmpp_free((*parser).ctx, ns as *mut libc::c_void); }
    if !name.is_null() {
        xmpp_free((*parser).ctx, name as *mut libc::c_void);
    }
    (*parser).depth += 1;
}
unsafe extern "C" fn _end_element(mut userdata: *mut libc::c_void,
                                  mut name: *const XML_Char) {
    let mut parser: *mut parser_t = userdata as *mut parser_t;
    (*parser).depth -= 1;
    if (*parser).depth == 0 as libc::c_int {
        /* notify the owner */
        if (*parser).endcb.is_some() {
            (*parser).endcb.expect("non-null function pointer")(name as
                                                                    *mut libc::c_char,
                                                                (*parser).userdata);
        }
    } else {
        complete_inner_text(parser);
        if !(*(*parser).stanza).parent.is_null() {
            /* we're finishing a child stanza, so set current to the parent */
            (*parser).stanza = (*(*parser).stanza).parent
        } else {
            if (*parser).stanzacb.is_some() {
                (*parser).stanzacb.expect("non-null function pointer")((*parser).stanza,
                                                                       (*parser).userdata);
            }
            xmpp_stanza_release((*parser).stanza);
            (*parser).stanza = 0 as *mut xmpp_stanza_t
        }
    };
}
unsafe extern "C" fn _characters(mut userdata: *mut libc::c_void,
                                 mut s: *const XML_Char,
                                 mut len: libc::c_int) {
    let mut parser: *mut parser_t = userdata as *mut parser_t;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*parser).depth < 2 as libc::c_int { return }
    /* Join all parts to a single resulting string. Stanza is created in
     * _start_element() and _end_element(). */
    if (*parser).inner_text_used + len >= (*parser).inner_text_size {
        (*parser).inner_text_size =
            (*parser).inner_text_used + len + 1 as libc::c_int +
                2 as libc::c_int;
        p =
            xmpp_realloc((*parser).ctx,
                         (*parser).inner_text as *mut libc::c_void,
                         (*parser).inner_text_size as size_t) as
                *mut libc::c_char;
        if p.is_null() {
            xmpp_free((*parser).ctx,
                      (*parser).inner_text as *mut libc::c_void);
            (*parser).inner_text = 0 as *mut libc::c_char;
            (*parser).inner_text_used = 0 as libc::c_int;
            (*parser).inner_text_size = 0 as libc::c_int;
            return
        }
        (*parser).inner_text = p;
        *(*parser).inner_text.offset((*parser).inner_text_used as isize) =
            '\u{0}' as i32 as libc::c_char
    }
    (*parser).inner_text_used += len;
    strncat((*parser).inner_text, s, len as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn parser_new(mut ctx: *mut xmpp_ctx_t,
                                    mut startcb: parser_start_callback,
                                    mut endcb: parser_end_callback,
                                    mut stanzacb: parser_stanza_callback,
                                    mut userdata: *mut libc::c_void)
 -> *mut parser_t {
    let mut parser: *mut parser_t = 0 as *mut parser_t;
    parser =
        xmpp_alloc(ctx, ::std::mem::size_of::<parser_t>() as libc::c_ulong) as
            *mut parser_t;
    if !parser.is_null() {
        (*parser).ctx = ctx;
        (*parser).expat = 0 as XML_Parser;
        (*parser).startcb = startcb;
        (*parser).endcb = endcb;
        (*parser).stanzacb = stanzacb;
        (*parser).userdata = userdata;
        (*parser).depth = 0 as libc::c_int;
        (*parser).stanza = 0 as *mut xmpp_stanza_t;
        (*parser).inner_text = 0 as *mut libc::c_char;
        (*parser).inner_text_size = 0 as libc::c_int;
        (*parser).inner_text_used = 0 as libc::c_int;
        parser_reset(parser);
    }
    return parser;
}
#[no_mangle]
pub unsafe extern "C" fn parser_attr_name(mut ctx: *mut xmpp_ctx_t,
                                          mut nsname: *mut libc::c_char)
 -> *mut libc::c_char {
    return _xml_name(ctx, nsname);
}
/* free a parser */
#[no_mangle]
pub unsafe extern "C" fn parser_free(mut parser: *mut parser_t) {
    if !(*parser).expat.is_null() { XML_ParserFree((*parser).expat); }
    if !(*parser).inner_text.is_null() {
        xmpp_free((*parser).ctx, (*parser).inner_text as *mut libc::c_void);
        (*parser).inner_text = 0 as *mut libc::c_char
    }
    xmpp_free((*parser).ctx, parser as *mut libc::c_void);
}
/* shuts down and restarts XML parser.  true on success */
#[no_mangle]
pub unsafe extern "C" fn parser_reset(mut parser: *mut parser_t)
 -> libc::c_int {
    let mut ret: XML_Bool = 0;
    let mut mem: *const XML_Memory_Handling_Suite =
        0 as *const XML_Memory_Handling_Suite;
    if !(*parser).expat.is_null() {
        ret = XML_ParserReset((*parser).expat, 0 as *const XML_Char);
        if ret as libc::c_int != 1 as libc::c_int as XML_Bool as libc::c_int {
            XML_ParserFree((*parser).expat);
            (*parser).expat = 0 as XML_Parser
        }
    } else {
        if mem_ctx.is_null() { mem_ctx = (*parser).ctx }
        if (*parser).ctx == mem_ctx { mem = &parser_mem_suite }
        (*parser).expat =
            XML_ParserCreate_MM(0 as *const XML_Char, mem, &namespace_sep)
    }
    if !(*parser).stanza.is_null() {
        xmpp_stanza_release((*parser).stanza);
        (*parser).stanza = 0 as *mut xmpp_stanza_t
    }
    if !(*parser).inner_text.is_null() {
        xmpp_free((*parser).ctx, (*parser).inner_text as *mut libc::c_void);
        (*parser).inner_text = 0 as *mut libc::c_char
    }
    if (*parser).expat.is_null() { return 0 as libc::c_int }
    (*parser).depth = 0 as libc::c_int;
    XML_SetUserData((*parser).expat, parser as *mut libc::c_void);
    XML_SetElementHandler((*parser).expat,
                          Some(_start_element as
                                   unsafe extern "C" fn(_: *mut libc::c_void,
                                                        _: *const XML_Char,
                                                        _:
                                                            *mut *const XML_Char)
                                       -> ()),
                          Some(_end_element as
                                   unsafe extern "C" fn(_: *mut libc::c_void,
                                                        _: *const XML_Char)
                                       -> ()));
    XML_SetCharacterDataHandler((*parser).expat,
                                Some(_characters as
                                         unsafe extern "C" fn(_:
                                                                  *mut libc::c_void,
                                                              _:
                                                                  *const XML_Char,
                                                              _: libc::c_int)
                                             -> ()));
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn parser_feed(mut parser: *mut parser_t,
                                     mut chunk: *mut libc::c_char,
                                     mut len: libc::c_int) -> libc::c_int {
    return XML_Parse((*parser).expat, chunk, len, 0 as libc::c_int) as
               libc::c_int;
}
