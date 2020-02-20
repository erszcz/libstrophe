use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
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
    fn select(__nfds: libc::c_int, __readfds: *mut fd_set,
              __writefds: *mut fd_set, __exceptfds: *mut fd_set,
              __timeout: *mut timeval) -> libc::c_int;
    #[no_mangle]
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    /* free some blocks returned by other APIs, for example the
   buffer you get from xmpp_stanza_to_text */
    #[no_mangle]
    fn xmpp_free(ctx: *const xmpp_ctx_t, p: *mut libc::c_void);
    #[no_mangle]
    fn xmpp_send_error(conn: *mut xmpp_conn_t, type_0: xmpp_error_type_t,
                       text: *mut libc::c_char);
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
    fn handler_fire_timed(ctx: *mut xmpp_ctx_t) -> uint64_t;
    #[no_mangle]
    fn conn_disconnect(conn: *mut xmpp_conn_t);
    #[no_mangle]
    fn xmpp_debug(ctx: *const xmpp_ctx_t, area: *const libc::c_char,
                  fmt: *const libc::c_char, _: ...);
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
    fn tls_is_recoverable(error: libc::c_int) -> libc::c_int;
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
    #[no_mangle]
    fn parser_feed(parser: *mut parser_t, chunk: *mut libc::c_char,
                   len: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn tls_read(tls: *mut tls_t, buff: *mut libc::c_void, len: size_t)
     -> libc::c_int;
    #[no_mangle]
    fn tls_pending(tls: *mut tls_t) -> libc::c_int;
    #[no_mangle]
    fn conn_established(conn: *mut xmpp_conn_t);
    #[no_mangle]
    fn xmpp_error(ctx: *const xmpp_ctx_t, area: *const libc::c_char,
                  fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn xmpp_info(ctx: *const xmpp_ctx_t, area: *const libc::c_char,
                 fmt: *const libc::c_char, _: ...);
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
    fn time_elapsed(t1: uint64_t, t2: uint64_t) -> uint64_t;
    #[no_mangle]
    fn conn_parser_reset(conn: *mut xmpp_conn_t);
    #[no_mangle]
    fn tls_write(tls: *mut tls_t, buff: *const libc::c_void, len: size_t)
     -> libc::c_int;
    #[no_mangle]
    fn tls_clear_pending_write(tls: *mut tls_t) -> libc::c_int;
    #[no_mangle]
    fn sock_error() -> libc::c_int;
    #[no_mangle]
    fn sock_read(sock: sock_t, buff: *mut libc::c_void, len: size_t)
     -> libc::c_int;
    #[no_mangle]
    fn sock_write(sock: sock_t, buff: *const libc::c_void, len: size_t)
     -> libc::c_int;
    #[no_mangle]
    fn sock_is_recoverable(error: libc::c_int) -> libc::c_int;
    /* checks for an error after connect, return 0 if connect successful */
    #[no_mangle]
    fn sock_connect_error(sock: sock_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
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
/* event loop */
/* * Run the event loop once.
 *  This function will run send any data that has been queued by
 *  xmpp_send and related functions and run through the Strophe even
 *  loop a single time, and will not wait more than timeout
 *  milliseconds for events.  This is provided to support integration
 *  with event loops outside the library, and if used, should be
 *  called regularly to achieve low latency event handling.
 *
 *  @param ctx a Strophe context object
 *  @param timeout time to wait for events in milliseconds
 *
 *  @ingroup EventLoop
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_run_once(mut ctx: *mut xmpp_ctx_t,
                                       timeout: libc::c_ulong) {
    let mut connitem: *mut xmpp_connlist_t = 0 as *mut xmpp_connlist_t;
    let mut conn: *mut xmpp_conn_t = 0 as *mut xmpp_conn_t;
    let mut rfds: fd_set = fd_set{__fds_bits: [0; 16],};
    let mut wfds: fd_set = fd_set{__fds_bits: [0; 16],};
    let mut max: sock_t = 0 as libc::c_int;
    let mut ret: libc::c_int = 0;
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut sq: *mut xmpp_send_queue_t = 0 as *mut xmpp_send_queue_t;
    let mut tsq: *mut xmpp_send_queue_t = 0 as *mut xmpp_send_queue_t;
    let mut towrite: libc::c_int = 0;
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut next: uint64_t = 0;
    let mut usec: uint64_t = 0;
    let mut tls_read_bytes: libc::c_int = 0 as libc::c_int;
    if (*ctx).loop_status as libc::c_uint ==
           XMPP_LOOP_QUIT as libc::c_int as libc::c_uint {
        return
    }
    /* send queued data */
    connitem = (*ctx).connlist;
    while !connitem.is_null() {
        conn = (*connitem).conn;
        if (*conn).state as libc::c_uint !=
               XMPP_STATE_CONNECTED as libc::c_int as libc::c_uint {
            connitem = (*connitem).next
        } else {
            /* if we're running tls, there may be some remaining data waiting to
         * be sent, so push that out */
            if !(*conn).tls.is_null() {
                ret = tls_clear_pending_write((*conn).tls);
                if ret < 0 as libc::c_int &&
                       tls_is_recoverable(tls_error((*conn).tls)) == 0 {
                    /* an error occurred */
                    xmpp_debug(ctx,
                               b"xmpp\x00" as *const u8 as
                                   *const libc::c_char,
                               b"Send error occurred, disconnecting.\x00" as
                                   *const u8 as *const libc::c_char);
                    (*conn).error = 103 as libc::c_int;
                    conn_disconnect(conn);
                }
            }
            /* write all data from the send queue to the socket */
            sq = (*conn).send_queue_head; /* not all data could be sent now */
            while !sq.is_null() {
                towrite =
                    (*sq).len.wrapping_sub((*sq).written) as
                        libc::c_int; /* partial write or an error */
                if !(*conn).tls.is_null() {
                    ret =
                        tls_write((*conn).tls,
                                  &mut *(*sq).data.offset((*sq).written as
                                                              isize) as
                                      *mut libc::c_char as
                                      *const libc::c_void, towrite as size_t);
                    if ret < 0 as libc::c_int &&
                           tls_is_recoverable(tls_error((*conn).tls)) == 0 {
                        (*conn).error = tls_error((*conn).tls)
                    }
                } else {
                    ret =
                        sock_write((*conn).sock,
                                   &mut *(*sq).data.offset((*sq).written as
                                                               isize) as
                                       *mut libc::c_char as
                                       *const libc::c_void,
                                   towrite as size_t);
                    if ret < 0 as libc::c_int &&
                           sock_is_recoverable(sock_error()) == 0 {
                        (*conn).error = sock_error()
                    }
                }
                if ret > 0 as libc::c_int && ret < towrite {
                    (*sq).written =
                        ((*sq).written as
                             libc::c_ulong).wrapping_add(ret as libc::c_ulong)
                            as size_t as size_t
                }
                if ret != towrite { break ; }
                /* all data for this queue item written, delete and move on */
                xmpp_free(ctx, (*sq).data as *mut libc::c_void);
                tsq = sq;
                sq = (*sq).next;
                (*conn).send_queue_len -= 1;
                xmpp_free(ctx, tsq as *mut libc::c_void);
                /* pop the top item */
                (*conn).send_queue_head = sq;
                /* if we've sent everything update the tail */
                if sq.is_null() {
                    (*conn).send_queue_tail = 0 as *mut xmpp_send_queue_t
                }
            }
            /* tear down connection on error */
            if (*conn).error != 0 {
                /* FIXME: need to tear down send queues and random other things
             * maybe this should be abstracted */
                xmpp_debug(ctx,
                           b"xmpp\x00" as *const u8 as *const libc::c_char,
                           b"Send error occurred, disconnecting.\x00" as
                               *const u8 as *const libc::c_char);
                (*conn).error = 103 as libc::c_int;
                conn_disconnect(conn);
            }
            connitem = (*connitem).next
        }
    }
    /* reset parsers if needed */
    connitem = (*ctx).connlist;
    while !connitem.is_null() {
        if (*(*connitem).conn).reset_parser != 0 {
            conn_parser_reset((*connitem).conn);
        }
        connitem = (*connitem).next
    }
    /* fire any ready timed handlers, then make sure we don't wait past
       the time when timed handlers need to be called */
    next = handler_fire_timed(ctx);
    usec =
        (if next < timeout {
             next
         } else {
             timeout
         }).wrapping_mul(1000 as libc::c_int as libc::c_ulong);
    tv.tv_sec =
        usec.wrapping_div(1000000 as libc::c_int as libc::c_ulong) as
            libc::c_long;
    tv.tv_usec =
        usec.wrapping_rem(1000000 as libc::c_int as libc::c_ulong) as
            libc::c_long;
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh0 = &mut __d0;
    let fresh1;
    let fresh2 = &mut __d1;
    let fresh3;
    let fresh4 =
        (::std::mem::size_of::<fd_set>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<__fd_mask>() as
                                             libc::c_ulong);
    let fresh5 =
        &mut *rfds.__fds_bits.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut __fd_mask;
    asm!("cld; rep; stosq" : "={cx}" (fresh1), "={di}" (fresh3) : "{ax}"
         (0 as libc::c_int), "0"
         (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh4)), "1"
         (c2rust_asm_casts::AsmCast::cast_in(fresh2, fresh5)) : "memory" :
         "volatile");
    c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh4, fresh1);
    c2rust_asm_casts::AsmCast::cast_out(fresh2, fresh5, fresh3);
    let mut __d0_0: libc::c_int = 0;
    let mut __d1_0: libc::c_int = 0;
    let fresh6 = &mut __d0_0;
    let fresh7;
    let fresh8 = &mut __d1_0;
    let fresh9;
    let fresh10 =
        (::std::mem::size_of::<fd_set>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<__fd_mask>() as
                                             libc::c_ulong);
    let fresh11 =
        &mut *wfds.__fds_bits.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut __fd_mask;
    asm!("cld; rep; stosq" : "={cx}" (fresh7), "={di}" (fresh9) : "{ax}"
         (0 as libc::c_int), "0"
         (c2rust_asm_casts::AsmCast::cast_in(fresh6, fresh10)), "1"
         (c2rust_asm_casts::AsmCast::cast_in(fresh8, fresh11)) : "memory" :
         "volatile");
    c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh10, fresh7);
    c2rust_asm_casts::AsmCast::cast_out(fresh8, fresh11, fresh9);
    /* find events to watch */
    connitem = (*ctx).connlist;
    while !connitem.is_null() {
        conn = (*connitem).conn;
        match (*conn).state as libc::c_uint {
            1 => {
                /* connect has been called and we're waiting for it to complete */
            /* connection will give us write or error events */
                /* make sure the timeout hasn't expired */
                if time_elapsed((*conn).timeout_stamp, time_stamp()) <=
                       (*conn).connect_timeout as libc::c_ulong {
                    wfds.__fds_bits[((*conn).sock /
                                         (8 as libc::c_int *
                                              ::std::mem::size_of::<__fd_mask>()
                                                  as libc::c_ulong as
                                                  libc::c_int)) as usize] |=
                        ((1 as libc::c_ulong) <<
                             (*conn).sock %
                                 (8 as libc::c_int *
                                      ::std::mem::size_of::<__fd_mask>() as
                                          libc::c_ulong as libc::c_int)) as
                            __fd_mask
                } else {
                    (*conn).error = 110 as libc::c_int;
                    xmpp_info(ctx,
                              b"xmpp\x00" as *const u8 as *const libc::c_char,
                              b"Connection attempt timed out.\x00" as
                                  *const u8 as *const libc::c_char);
                    conn_disconnect(conn);
                }
            }
            2 => {
                rfds.__fds_bits[((*conn).sock /
                                     (8 as libc::c_int *
                                          ::std::mem::size_of::<__fd_mask>()
                                              as libc::c_ulong as
                                              libc::c_int)) as usize] |=
                    ((1 as libc::c_ulong) <<
                         (*conn).sock %
                             (8 as libc::c_int *
                                  ::std::mem::size_of::<__fd_mask>() as
                                      libc::c_ulong as libc::c_int)) as
                        __fd_mask;
                if (*conn).send_queue_len > 0 as libc::c_int {
                    wfds.__fds_bits[((*conn).sock /
                                         (8 as libc::c_int *
                                              ::std::mem::size_of::<__fd_mask>()
                                                  as libc::c_ulong as
                                                  libc::c_int)) as usize] |=
                        ((1 as libc::c_ulong) <<
                             (*conn).sock %
                                 (8 as libc::c_int *
                                      ::std::mem::size_of::<__fd_mask>() as
                                          libc::c_ulong as libc::c_int)) as
                            __fd_mask
                }
            }
            0 | _ => { }
        }
        /* Check if there is something in the SSL buffer. */
        if !(*conn).tls.is_null() {
            tls_read_bytes += tls_pending((*conn).tls)
        }
        if (*conn).state as libc::c_uint !=
               XMPP_STATE_DISCONNECTED as libc::c_int as libc::c_uint &&
               (*conn).sock > max {
            max = (*conn).sock
        }
        connitem = (*connitem).next
    }
    /* check for events */
    if max > 0 as libc::c_int {
        ret =
            select(max + 1 as libc::c_int, &mut rfds, &mut wfds,
                   0 as *mut fd_set, &mut tv)
    } else {
        if timeout > 0 as libc::c_int as libc::c_ulong {
            usleep(timeout.wrapping_mul(1000 as libc::c_int as libc::c_ulong)
                       as __useconds_t);
        }
        return
    }
    /* select errored */
    if ret < 0 as libc::c_int {
        if sock_is_recoverable(sock_error()) == 0 {
            xmpp_error(ctx, b"xmpp\x00" as *const u8 as *const libc::c_char,
                       b"event watcher internal error %d\x00" as *const u8 as
                           *const libc::c_char, sock_error());
        }
        return
    }
    /* no events happened */
    if ret == 0 as libc::c_int && tls_read_bytes == 0 as libc::c_int {
        return
    }
    /* process events */
    connitem = (*ctx).connlist;
    while !connitem.is_null() {
        conn = (*connitem).conn;
        match (*conn).state as libc::c_uint {
            1 => {
                if wfds.__fds_bits[((*conn).sock /
                                        (8 as libc::c_int *
                                             ::std::mem::size_of::<__fd_mask>()
                                                 as libc::c_ulong as
                                                 libc::c_int)) as usize] &
                       ((1 as libc::c_ulong) <<
                            (*conn).sock %
                                (8 as libc::c_int *
                                     ::std::mem::size_of::<__fd_mask>() as
                                         libc::c_ulong as libc::c_int)) as
                           __fd_mask != 0 as libc::c_int as libc::c_long {
                    /* connection complete */
                    /* check for error */
                    ret = sock_connect_error((*conn).sock);
                    if ret != 0 as libc::c_int {
                        /* connection failed */
                        xmpp_debug(ctx,
                                   b"xmpp\x00" as *const u8 as
                                       *const libc::c_char,
                                   b"connection failed, error %d\x00" as
                                       *const u8 as *const libc::c_char, ret);
                        conn_disconnect(conn);
                    } else {
                        (*conn).state = XMPP_STATE_CONNECTED;
                        xmpp_debug(ctx,
                                   b"xmpp\x00" as *const u8 as
                                       *const libc::c_char,
                                   b"connection successful\x00" as *const u8
                                       as *const libc::c_char);
                        conn_established(conn);
                    }
                }
            }
            2 => {
                if rfds.__fds_bits[((*conn).sock /
                                        (8 as libc::c_int *
                                             ::std::mem::size_of::<__fd_mask>()
                                                 as libc::c_ulong as
                                                 libc::c_int)) as usize] &
                       ((1 as libc::c_ulong) <<
                            (*conn).sock %
                                (8 as libc::c_int *
                                     ::std::mem::size_of::<__fd_mask>() as
                                         libc::c_ulong as libc::c_int)) as
                           __fd_mask != 0 as libc::c_int as libc::c_long ||
                       !(*conn).tls.is_null() && tls_pending((*conn).tls) != 0
                   {
                    if !(*conn).tls.is_null() {
                        ret =
                            tls_read((*conn).tls,
                                     buf.as_mut_ptr() as *mut libc::c_void,
                                     4096 as libc::c_int as size_t)
                    } else {
                        ret =
                            sock_read((*conn).sock,
                                      buf.as_mut_ptr() as *mut libc::c_void,
                                      4096 as libc::c_int as size_t)
                    }
                    if ret > 0 as libc::c_int {
                        ret =
                            parser_feed((*conn).parser, buf.as_mut_ptr(),
                                        ret);
                        if ret == 0 {
                            xmpp_debug(ctx,
                                       b"xmpp\x00" as *const u8 as
                                           *const libc::c_char,
                                       b"parse error [%s]\x00" as *const u8 as
                                           *const libc::c_char,
                                       buf.as_mut_ptr());
                            xmpp_send_error(conn, XMPP_SE_INVALID_XML,
                                            b"parse error\x00" as *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char);
                        }
                    } else if !(*conn).tls.is_null() {
                        if tls_is_recoverable(tls_error((*conn).tls)) == 0 {
                            xmpp_debug(ctx,
                                       b"xmpp\x00" as *const u8 as
                                           *const libc::c_char,
                                       b"Unrecoverable TLS error, %d.\x00" as
                                           *const u8 as *const libc::c_char,
                                       tls_error((*conn).tls));
                            (*conn).error = tls_error((*conn).tls);
                            conn_disconnect(conn);
                        }
                    } else {
                        /* return of 0 means socket closed by server */
                        xmpp_debug(ctx,
                                   b"xmpp\x00" as *const u8 as
                                       *const libc::c_char,
                                   b"Socket closed by remote host.\x00" as
                                       *const u8 as *const libc::c_char);
                        (*conn).error = 104 as libc::c_int;
                        conn_disconnect(conn);
                    }
                }
            }
            0 | _ => { }
        }
        connitem = (*connitem).next
    }
    /* fire any ready handlers */
    handler_fire_timed(ctx);
}
/* * Start the event loop.
 *  This function continuously calls xmpp_run_once and does not return
 *  until xmpp_stop has been called.
 *
 *  @param ctx a Strophe context object
 *
 *  @ingroup EventLoop
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_run(mut ctx: *mut xmpp_ctx_t) {
    if (*ctx).loop_status as libc::c_uint !=
           XMPP_LOOP_NOTSTARTED as libc::c_int as libc::c_uint {
        return
    }
    (*ctx).loop_status = XMPP_LOOP_RUNNING;
    while (*ctx).loop_status as libc::c_uint ==
              XMPP_LOOP_RUNNING as libc::c_int as libc::c_uint {
        xmpp_run_once(ctx, (*ctx).timeout);
    }
    /* make it possible to start event loop again */
    (*ctx).loop_status = XMPP_LOOP_NOTSTARTED;
    xmpp_debug(ctx, b"event\x00" as *const u8 as *const libc::c_char,
               b"Event loop completed.\x00" as *const u8 as
                   *const libc::c_char);
}
/* * Stop the event loop.
 *  This will stop the event loop after the current iteration and cause
 *  xmpp_run to exit.
 *
 *  @param ctx a Strophe context object
 *
 *  @ingroup EventLoop
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_stop(mut ctx: *mut xmpp_ctx_t) {
    xmpp_debug(ctx, b"event\x00" as *const u8 as *const libc::c_char,
               b"Stopping event loop.\x00" as *const u8 as
                   *const libc::c_char);
    if (*ctx).loop_status as libc::c_uint ==
           XMPP_LOOP_RUNNING as libc::c_int as libc::c_uint {
        (*ctx).loop_status = XMPP_LOOP_QUIT
    };
}
