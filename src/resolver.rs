use ::libc;
extern "C" {
    pub type _hash_t;
    pub type _parser_t;
    pub type _tls;
    pub type _xmpp_rand_t;
    #[no_mangle]
    fn __res_query(_: *const libc::c_char, _: libc::c_int, _: libc::c_int,
                   _: *mut libc::c_uchar, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
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
    /* free some blocks returned by other APIs, for example the
   buffer you get from xmpp_stanza_to_text */
    #[no_mangle]
    fn xmpp_free(ctx: *const xmpp_ctx_t, p: *mut libc::c_void);
    #[no_mangle]
    fn xmpp_alloc(ctx: *const xmpp_ctx_t, size: size_t) -> *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
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
pub type C2RustUnnamed_3 = libc::c_uint;
pub const XMPP_DOMAIN_ALTDOMAIN: C2RustUnnamed_3 = 2;
pub const XMPP_DOMAIN_FOUND: C2RustUnnamed_3 = 1;
pub const XMPP_DOMAIN_NOT_FOUND: C2RustUnnamed_3 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct resolver_srv_rr_struc {
    pub priority: uint16_t,
    pub weight: uint16_t,
    pub port: uint16_t,
    pub target: [libc::c_char; 256],
    pub next: *mut resolver_srv_rr_struc,
}
pub type resolver_srv_rr_t = resolver_srv_rr_struc;
/* ******************************************************************************
 * Resolver raw implementation.
 *
 * This code is common for both unix and win32.
 ******************************************************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct message_header {
    pub id: uint16_t,
    pub octet2: uint8_t,
    pub octet3: uint8_t,
    pub qdcount: uint16_t,
    pub ancount: uint16_t,
    pub nscount: uint16_t,
    pub arcount: uint16_t,
}
/* !HAVE_CARES */
/* _WIN32 */
/* ******************************************************************************
 * Implementation.
 ******************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn resolver_initialize() { }
#[no_mangle]
pub unsafe extern "C" fn resolver_shutdown() { }
unsafe extern "C" fn resolver_srv_list_sort(mut srv_rr_list:
                                                *mut *mut resolver_srv_rr_t) {
    let mut rr_head: *mut resolver_srv_rr_t = 0 as *mut resolver_srv_rr_t;
    let mut rr_current: *mut resolver_srv_rr_t = 0 as *mut resolver_srv_rr_t;
    let mut rr_next: *mut resolver_srv_rr_t = 0 as *mut resolver_srv_rr_t;
    let mut rr_prev: *mut resolver_srv_rr_t = 0 as *mut resolver_srv_rr_t;
    let mut swap: libc::c_int = 0;
    rr_head = *srv_rr_list;
    if rr_head.is_null() || (*rr_head).next.is_null() {
        /* Empty or single record list */
        return
    }
    loop  {
        rr_prev = 0 as *mut resolver_srv_rr_t;
        rr_current = rr_head;
        rr_next = (*rr_head).next;
        swap = 0 as libc::c_int;
        while !rr_next.is_null() {
            /*
             * RFC2052: A client MUST attempt to contact the target host
             * with the lowest-numbered priority it can reach.
             * RFC2052: When selecting a target host among the
             * those that have the same priority, the chance of trying
             * this one first SHOULD be proportional to its weight.
             */
            if (*rr_current).priority as libc::c_int >
                   (*rr_next).priority as libc::c_int ||
                   (*rr_current).priority as libc::c_int ==
                       (*rr_next).priority as libc::c_int &&
                       ((*rr_current).weight as libc::c_int) <
                           (*rr_next).weight as libc::c_int {
                /* Swap node */
                swap = 1 as libc::c_int;
                if !rr_prev.is_null() {
                    (*rr_prev).next = rr_next
                } else {
                    /* Swap head node */
                    rr_head = rr_next
                }
                (*rr_current).next = (*rr_next).next;
                (*rr_next).next = rr_current;
                rr_prev = rr_next;
                rr_next = (*rr_current).next
            } else {
                /* Next node */
                rr_prev = rr_current;
                rr_current = rr_next;
                rr_next = (*rr_next).next
            }
        }
        if !(swap != 0 as libc::c_int) { break ; }
    }
    *srv_rr_list = rr_head;
}
#[no_mangle]
pub unsafe extern "C" fn resolver_srv_lookup_buf(mut ctx: *mut xmpp_ctx_t,
                                                 mut buf:
                                                     *const libc::c_uchar,
                                                 mut len: size_t,
                                                 mut srv_rr_list:
                                                     *mut *mut resolver_srv_rr_t)
 -> libc::c_int {
    let mut set: libc::c_int = 0;
    set = resolver_raw_srv_lookup_buf(ctx, buf, len, srv_rr_list);
    resolver_srv_list_sort(srv_rr_list);
    return set;
}
#[no_mangle]
pub unsafe extern "C" fn resolver_srv_lookup(mut ctx: *mut xmpp_ctx_t,
                                             mut service: *const libc::c_char,
                                             mut proto: *const libc::c_char,
                                             mut domain: *const libc::c_char,
                                             mut srv_rr_list:
                                                 *mut *mut resolver_srv_rr_t)
 -> libc::c_int {
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut fulldomain: [libc::c_char; 2048] = [0; 2048];
    let mut len: libc::c_int = 0;
    let mut set: libc::c_int = XMPP_DOMAIN_NOT_FOUND as libc::c_int;
    xmpp_snprintf(fulldomain.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 2048]>() as
                      libc::c_ulong,
                  b"_%s._%s.%s\x00" as *const u8 as *const libc::c_char,
                  service, proto, domain);
    *srv_rr_list = 0 as *mut resolver_srv_rr_t;
    /* HAVE_CARES */
    /* _WIN32 */
    buf =
        xmpp_alloc(ctx, 65536 as libc::c_int as size_t) as *mut libc::c_uchar;
    if buf.is_null() { return XMPP_DOMAIN_NOT_FOUND as libc::c_int }
    /* _WIN32 */
    len =
        __res_query(fulldomain.as_mut_ptr(), 1 as libc::c_int,
                    33 as libc::c_int, buf, 65536 as libc::c_int);
    /* _WIN32 */
    if len > 0 as libc::c_int {
        set = resolver_srv_lookup_buf(ctx, buf, len as size_t, srv_rr_list)
    }
    xmpp_free(ctx, buf as *mut libc::c_void);
    /* HAVE_CARES */
    return set;
}
/* resolver.h
 * strophe XMPP client library -- DNS resolver
 *
 * Copyright (C) 2015 Dmitry Podgorny <pasis.ua@gmail.com>
 *
 *  This software is provided AS-IS with no warranty, either express
 *  or implied.
 *
 *  This program is dual licensed under the MIT and GPLv3 licenses.
 */
/* * @file
 *  DNS resolver.
 */
/* * Perform lookup for RFC1035 message format.
 *  This function allocates all elements.
 *
 *  @param ctx a Strophe context object
 *  @param buf message in RFC1035 format
 *  @param len length of the message
 *  @param srv_rr_list is the result
 *
 *  @return XMPP_DOMAIN_FOUND on success or XMPP_DOMAIN_NOT_FOUND on fail
 */
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
/* * Release a list returned by resolver_srv_lookup() or
 *  resolver_srv_lookup_buf().
 *
 *  @param ctx a Strophe context object
 *  @param srv_rr_list a list allocated by lookup functions
 */
#[no_mangle]
pub unsafe extern "C" fn resolver_srv_free(mut ctx: *mut xmpp_ctx_t,
                                           mut srv_rr_list:
                                               *mut resolver_srv_rr_t) {
    let mut rr: *mut resolver_srv_rr_t = 0 as *mut resolver_srv_rr_t;
    while !srv_rr_list.is_null() {
        rr = (*srv_rr_list).next;
        xmpp_free(ctx, srv_rr_list as *mut libc::c_void);
        srv_rr_list = rr
    };
}
/* the same as ntohs(), but receives pointer to the value */
unsafe extern "C" fn xmpp_ntohs_ptr(mut ptr: *const libc::c_void)
 -> uint16_t {
    let mut p: *const uint8_t = ptr as *const uint8_t;
    return (((*p.offset(0 as libc::c_int as isize) as libc::c_int) <<
                 8 as libc::c_uint) +
                *p.offset(1 as libc::c_int as isize) as libc::c_int) as
               uint16_t;
}
unsafe extern "C" fn message_header_qr(mut header: *const message_header)
 -> uint8_t {
    return ((*header).octet2 as libc::c_int >> 7 as libc::c_int &
                1 as libc::c_int) as uint8_t;
}
unsafe extern "C" fn message_header_rcode(mut header: *const message_header)
 -> uint8_t {
    return ((*header).octet3 as libc::c_int & 0xf as libc::c_int) as uint8_t;
}
/*
 * Append a label or a dot to the target name with buffer overflow checks.
 * Returns length of the non-truncated resulting string, may be bigger than
 * name_max.
 */
unsafe extern "C" fn message_name_append_safe(mut name: *mut libc::c_char,
                                              mut name_len: size_t,
                                              mut name_max: size_t,
                                              mut tail: *const libc::c_char,
                                              mut tail_len: size_t)
 -> size_t {
    let mut copy_len: size_t = 0;
    copy_len =
        if name_max > name_len {
            name_max.wrapping_sub(name_len)
        } else { 0 as libc::c_int as libc::c_ulong };
    copy_len = if tail_len < copy_len { tail_len } else { copy_len };
    if copy_len > 0 as libc::c_int as libc::c_ulong {
        memcpy(&mut *name.offset(name_len as isize) as *mut libc::c_char as
                   *mut libc::c_void, tail as *const libc::c_void, copy_len);
    }
    return name_len.wrapping_add(tail_len);
}
/* Returns length of the compressed name. This is NOT the same as strlen(). */
unsafe extern "C" fn message_name_get(mut buf: *const libc::c_uchar,
                                      mut buf_len: size_t,
                                      mut buf_offset: libc::c_uint,
                                      mut name: *mut libc::c_char,
                                      mut name_max: size_t) -> libc::c_uint {
    let mut name_len: size_t = 0 as libc::c_int as size_t;
    let mut i: libc::c_uint = buf_offset;
    let mut pointer: libc::c_uint = 0;
    let mut rc: libc::c_uint = 0;
    let mut label_len: libc::c_uchar = 0;
    loop  {
        if i as libc::c_ulong >= buf_len {
            return 0 as libc::c_int as libc::c_uint
        }
        let fresh0 = i;
        i = i.wrapping_add(1);
        label_len = *buf.offset(fresh0 as isize);
        if label_len as libc::c_int == 0 as libc::c_int { break ; }
        /* Label */
        if label_len as libc::c_int & 0xc0 as libc::c_int == 0 as libc::c_int
           {
            if i.wrapping_add(label_len as
                                  libc::c_uint).wrapping_sub(1 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
                   as libc::c_ulong >= buf_len {
                return 0 as libc::c_int as libc::c_uint
            }
            if !name.is_null() {
                name_len =
                    message_name_append_safe(name, name_len, name_max,
                                             &*buf.offset(i as isize) as
                                                 *const libc::c_uchar as
                                                 *mut libc::c_char,
                                             label_len as size_t);
                name_len =
                    message_name_append_safe(name, name_len, name_max,
                                             b".\x00" as *const u8 as
                                                 *const libc::c_char,
                                             1 as libc::c_int as size_t)
            }
            i = i.wrapping_add(label_len as libc::c_uint)
            /* Pointer */
        } else if label_len as libc::c_int & 0xc0 as libc::c_int ==
                      0xc0 as libc::c_int {
            if i as libc::c_ulong >= buf_len {
                return 0 as libc::c_int as libc::c_uint
            }
            let fresh1 = i;
            i = i.wrapping_add(1);
            pointer =
                ((label_len as libc::c_int & 0x3f as libc::c_int) <<
                     8 as libc::c_int |
                     *buf.offset(fresh1 as isize) as libc::c_int) as
                    libc::c_uint;
            if !name.is_null() && name_len >= name_max &&
                   name_max > 0 as libc::c_int as libc::c_ulong {
                /* The 10 and 01 combinations are reserved for future use. */
                /* We have filled the name buffer. Don't pass it recursively. */
                *name.offset(name_max.wrapping_sub(1 as libc::c_int as
                                                       libc::c_ulong) as
                                 isize) = '\u{0}' as i32 as libc::c_char;
                name = 0 as *mut libc::c_char;
                name_max = 0 as libc::c_int as size_t
            }
            rc =
                message_name_get(buf, buf_len, pointer,
                                 if !name.is_null() {
                                     &mut *name.offset(name_len as isize)
                                 } else { 0 as *mut libc::c_char },
                                 if name_max > name_len {
                                     name_max.wrapping_sub(name_len)
                                 } else {
                                     0 as libc::c_int as libc::c_ulong
                                 });
            if rc == 0 as libc::c_int as libc::c_uint {
                return 0 as libc::c_int as libc::c_uint
            }
            break ;
        } else { return 0 as libc::c_int as libc::c_uint }
    }
    if label_len as libc::c_int == 0 as libc::c_int {
        if name_len == 0 as libc::c_int as libc::c_ulong {
            name_len = 1 as libc::c_int as size_t
        }
        /*
         * At this point name_len is length of the resulting name,
         * including '\0'. This value can be exported to allocate buffer
         * of precise size.
         */
        if !name.is_null() && name_max > 0 as libc::c_int as libc::c_ulong {
            /*
             * Overwrite leading '.' with a '\0'. If the resulting name is
             * bigger than name_max it is truncated.
             */
            *name.offset((if name_len < name_max {
                              name_len
                          } else {
                              name_max
                          }).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                             as isize) = '\u{0}' as i32 as libc::c_char
        }
    }
    return i.wrapping_sub(buf_offset);
}
unsafe extern "C" fn message_name_len(mut buf: *const libc::c_uchar,
                                      mut buf_len: size_t,
                                      mut buf_offset: libc::c_uint)
 -> libc::c_uint {
    return message_name_get(buf, buf_len, buf_offset, 0 as *mut libc::c_char,
                            18446744073709551615 as libc::c_ulong);
}
/* ******************************************************************************
 * Forward declarations.
 ******************************************************************************/
/* HAVE_CARES */
unsafe extern "C" fn resolver_raw_srv_lookup_buf(mut ctx: *mut xmpp_ctx_t,
                                                 mut buf:
                                                     *const libc::c_uchar,
                                                 mut len: size_t,
                                                 mut srv_rr_list:
                                                     *mut *mut resolver_srv_rr_t)
 -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut name_len: libc::c_uint = 0;
    let mut rdlength: libc::c_uint = 0;
    let mut type_0: uint16_t = 0;
    let mut class: uint16_t = 0;
    let mut header: message_header =
        message_header{id: 0,
                       octet2: 0,
                       octet3: 0,
                       qdcount: 0,
                       ancount: 0,
                       nscount: 0,
                       arcount: 0,};
    let mut rr: *mut resolver_srv_rr_t = 0 as *mut resolver_srv_rr_t;
    *srv_rr_list = 0 as *mut resolver_srv_rr_t;
    if len < 12 as libc::c_int as libc::c_ulong {
        return XMPP_DOMAIN_NOT_FOUND as libc::c_int
    }
    header.id =
        xmpp_ntohs_ptr(&*buf.offset(0 as libc::c_int as isize) as
                           *const libc::c_uchar as *const libc::c_void);
    header.octet2 = *buf.offset(2 as libc::c_int as isize);
    header.octet3 = *buf.offset(3 as libc::c_int as isize);
    header.qdcount =
        xmpp_ntohs_ptr(&*buf.offset(4 as libc::c_int as isize) as
                           *const libc::c_uchar as *const libc::c_void);
    header.ancount =
        xmpp_ntohs_ptr(&*buf.offset(6 as libc::c_int as isize) as
                           *const libc::c_uchar as *const libc::c_void);
    header.nscount =
        xmpp_ntohs_ptr(&*buf.offset(8 as libc::c_int as isize) as
                           *const libc::c_uchar as *const libc::c_void);
    header.arcount =
        xmpp_ntohs_ptr(&*buf.offset(10 as libc::c_int as isize) as
                           *const libc::c_uchar as *const libc::c_void);
    if message_header_qr(&mut header) as libc::c_int != 1 as libc::c_int ||
           message_header_rcode(&mut header) as libc::c_int !=
               0 as libc::c_int {
        return XMPP_DOMAIN_NOT_FOUND as libc::c_int
    }
    j = 12 as libc::c_int as libc::c_uint;
    /* skip question section */
    i = 0 as libc::c_int as libc::c_uint;
    while i < header.qdcount as libc::c_uint {
        if j as libc::c_ulong >= len {
            if !(*srv_rr_list).is_null() {
                resolver_srv_free(ctx, *srv_rr_list);
            }
            *srv_rr_list = 0 as *mut resolver_srv_rr_t;
            return XMPP_DOMAIN_NOT_FOUND as libc::c_int
        }
        name_len = message_name_len(buf, len, j);
        /* error in name format */
        if name_len == 0 as libc::c_int as libc::c_uint {
            return XMPP_DOMAIN_NOT_FOUND as libc::c_int
        }
        j =
            j.wrapping_add(name_len.wrapping_add(4 as libc::c_int as
                                                     libc::c_uint));
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < header.ancount as libc::c_uint {
        if j as libc::c_ulong >= len {
            if !(*srv_rr_list).is_null() {
                resolver_srv_free(ctx, *srv_rr_list);
            }
            *srv_rr_list = 0 as *mut resolver_srv_rr_t;
            return XMPP_DOMAIN_NOT_FOUND as libc::c_int
        }
        name_len = message_name_len(buf, len, j);
        /* error in name format */
        if name_len == 0 as libc::c_int as libc::c_uint {
            return XMPP_DOMAIN_NOT_FOUND as libc::c_int
        }
        j = j.wrapping_add(name_len);
        if j.wrapping_add(16 as libc::c_int as libc::c_uint) as libc::c_ulong
               >= len {
            if !(*srv_rr_list).is_null() {
                resolver_srv_free(ctx, *srv_rr_list);
            }
            *srv_rr_list = 0 as *mut resolver_srv_rr_t;
            return XMPP_DOMAIN_NOT_FOUND as libc::c_int
        }
        type_0 =
            xmpp_ntohs_ptr(&*buf.offset(j as isize) as *const libc::c_uchar as
                               *const libc::c_void);
        class =
            xmpp_ntohs_ptr(&*buf.offset(j.wrapping_add(2 as libc::c_int as
                                                           libc::c_uint) as
                                            isize) as *const libc::c_uchar as
                               *const libc::c_void);
        rdlength =
            xmpp_ntohs_ptr(&*buf.offset(j.wrapping_add(8 as libc::c_int as
                                                           libc::c_uint) as
                                            isize) as *const libc::c_uchar as
                               *const libc::c_void) as libc::c_uint;
        j = j.wrapping_add(10 as libc::c_int as libc::c_uint);
        if type_0 as libc::c_int == 33 as libc::c_int &&
               class as libc::c_int == 1 as libc::c_int {
            rr =
                xmpp_alloc(ctx,
                           ::std::mem::size_of::<resolver_srv_rr_t>() as
                               libc::c_ulong) as *mut resolver_srv_rr_t;
            (*rr).next = *srv_rr_list;
            (*rr).priority =
                xmpp_ntohs_ptr(&*buf.offset(j as isize) as
                                   *const libc::c_uchar as
                                   *const libc::c_void);
            (*rr).weight =
                xmpp_ntohs_ptr(&*buf.offset(j.wrapping_add(2 as libc::c_int as
                                                               libc::c_uint)
                                                as isize) as
                                   *const libc::c_uchar as
                                   *const libc::c_void);
            (*rr).port =
                xmpp_ntohs_ptr(&*buf.offset(j.wrapping_add(4 as libc::c_int as
                                                               libc::c_uint)
                                                as isize) as
                                   *const libc::c_uchar as
                                   *const libc::c_void);
            name_len =
                message_name_get(buf, len,
                                 j.wrapping_add(6 as libc::c_int as
                                                    libc::c_uint),
                                 (*rr).target.as_mut_ptr(),
                                 ::std::mem::size_of::<[libc::c_char; 256]>()
                                     as libc::c_ulong);
            if name_len > 0 as libc::c_int as libc::c_uint {
                *srv_rr_list = rr
            } else { xmpp_free(ctx, rr as *mut libc::c_void); }
            /* skip broken record */
        }
        j = j.wrapping_add(rdlength);
        i = i.wrapping_add(1)
    }
    return if !(*srv_rr_list).is_null() {
               XMPP_DOMAIN_FOUND as libc::c_int
           } else { XMPP_DOMAIN_NOT_FOUND as libc::c_int };
}
/* _WIN32 */
/* HAVE_CARES */
/* !HAVE_CARES */
