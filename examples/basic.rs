#![feature(extern_types)]
#![feature(main)]
use ::libc;
extern crate strophe;
extern "C" {
    pub type _xmpp_ctx_t;
    pub type _xmpp_conn_t;
    pub type _xmpp_stanza_t;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    /* strophe.h
** strophe XMPP client library C API
**
** Copyright (C) 2005-2009 Collecta, Inc.
**
**  This software is provided AS-IS with no warranty, either express or
**  implied.
**
**  This software is dual licensed under the MIT and GPLv3 licenses.
*/
    /* * @file
 *  Strophe public C API definitions.
 */
    /* size_t */
    /* namespace defines */
/* * @def XMPP_NS_CLIENT
 *  Namespace definition for 'jabber:client'.
 */
    /* * @def XMPP_NS_COMPONENT
 *  Namespace definition for 'jabber:component:accept'.
 */
    /* * @def XMPP_NS_STREAMS
 *  Namespace definition for 'http://etherx.jabber.org/streams'.
 */
    /* * @def XMPP_NS_STREAMS_IETF
 *  Namespace definition for 'urn:ietf:params:xml:ns:xmpp-streams'.
 */
    /* * @def XMPP_NS_TLS
 *  Namespace definition for 'url:ietf:params:xml:ns:xmpp-tls'.
 */
    /* * @def XMPP_NS_SASL
 *  Namespace definition for 'urn:ietf:params:xml:ns:xmpp-sasl'.
 */
    /* * @def XMPP_NS_BIND
 *  Namespace definition for 'urn:ietf:params:xml:ns:xmpp-bind'.
 */
    /* * @def XMPP_NS_SESSION
 *  Namespace definition for 'urn:ietf:params:xml:ns:xmpp-session'.
 */
    /* * @def XMPP_NS_AUTH
 *  Namespace definition for 'jabber:iq:auth'.
 */
    /* * @def XMPP_NS_DISCO_INFO
 *  Namespace definition for 'http://jabber.org/protocol/disco#info'.
 */
    /* * @def XMPP_NS_DISCO_ITEMS
 *  Namespace definition for 'http://jabber.org/protocol/disco#items'.
 */
    /* * @def XMPP_NS_ROSTER
 *  Namespace definition for 'jabber:iq:roster'.
 */
    /* error defines */
/* * @def XMPP_EOK
 *  Success error code.
 */
    /* * @def XMPP_EMEM
 *  Memory related failure error code.
 *  
 *  This is returned on allocation errors and signals that the host may
 *  be out of memory.
 */
    /* * @def XMPP_EINVOP
 *  Invalid operation error code.
 *
 *  This error code is returned when the operation was invalid and signals
 *  that the Strophe API is being used incorrectly.
 */
    /* * @def XMPP_EINT
 *  Internal failure error code.
 */
    /* initialization and shutdown */
    #[no_mangle]
    fn xmpp_initialize();
    #[no_mangle]
    fn xmpp_shutdown();
    #[no_mangle]
    fn xmpp_ctx_new(mem: *const xmpp_mem_t, log: *const xmpp_log_t)
     -> *mut xmpp_ctx_t;
    #[no_mangle]
    fn xmpp_ctx_free(ctx: *mut xmpp_ctx_t);
    /* return a default logger filtering at a given level */
    #[no_mangle]
    fn xmpp_get_default_logger(level: xmpp_log_level_t) -> *mut xmpp_log_t;
    #[no_mangle]
    fn xmpp_conn_new(ctx: *mut xmpp_ctx_t) -> *mut xmpp_conn_t;
    #[no_mangle]
    fn xmpp_conn_release(conn: *mut xmpp_conn_t) -> libc::c_int;
    #[no_mangle]
    fn xmpp_conn_set_flags(conn: *mut xmpp_conn_t, flags: libc::c_long)
     -> libc::c_int;
    #[no_mangle]
    fn xmpp_conn_set_jid(conn: *mut xmpp_conn_t, jid: *const libc::c_char);
    #[no_mangle]
    fn xmpp_conn_set_pass(conn: *mut xmpp_conn_t, pass: *const libc::c_char);
    #[no_mangle]
    fn xmpp_conn_is_secured(conn: *mut xmpp_conn_t) -> libc::c_int;
    #[no_mangle]
    fn xmpp_conn_set_keepalive(conn: *mut xmpp_conn_t, timeout: libc::c_int,
                               interval: libc::c_int);
    #[no_mangle]
    fn xmpp_connect_client(conn: *mut xmpp_conn_t,
                           altdomain: *const libc::c_char,
                           altport: libc::c_ushort,
                           callback: xmpp_conn_handler,
                           userdata: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn xmpp_disconnect(conn: *mut xmpp_conn_t);
    #[no_mangle]
    fn xmpp_run(ctx: *mut xmpp_ctx_t);
    #[no_mangle]
    fn xmpp_stop(ctx: *mut xmpp_ctx_t);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
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
/* opaque run time context containing the above hooks */
pub type xmpp_ctx_t = _xmpp_ctx_t;
/* connection */
/* opaque connection object */
pub type xmpp_conn_t = _xmpp_conn_t;
pub type xmpp_stanza_t = _xmpp_stanza_t;
pub type xmpp_conn_event_t = libc::c_uint;
pub const XMPP_CONN_FAIL: xmpp_conn_event_t = 3;
pub const XMPP_CONN_DISCONNECT: xmpp_conn_event_t = 2;
pub const XMPP_CONN_RAW_CONNECT: xmpp_conn_event_t = 1;
pub const XMPP_CONN_CONNECT: xmpp_conn_event_t = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmpp_stream_error_t {
    pub type_0: xmpp_error_type_t,
    pub text: *mut libc::c_char,
    pub stanza: *mut xmpp_stanza_t,
}
pub type xmpp_conn_handler
    =
    Option<unsafe extern "C" fn(_: *mut xmpp_conn_t, _: xmpp_conn_event_t,
                                _: libc::c_int, _: *mut xmpp_stream_error_t,
                                _: *mut libc::c_void) -> ()>;
/* define a handler for connection events */
#[no_mangle]
pub unsafe extern "C" fn conn_handler(conn: *mut xmpp_conn_t,
                                      status: xmpp_conn_event_t,
                                      error: libc::c_int,
                                      stream_error: *mut xmpp_stream_error_t,
                                      userdata: *mut libc::c_void) {
    let mut ctx: *mut xmpp_ctx_t = userdata as *mut xmpp_ctx_t;
    let mut secured: libc::c_int = 0;
    if status as libc::c_uint ==
           XMPP_CONN_CONNECT as libc::c_int as libc::c_uint {
        fprintf(stderr,
                b"DEBUG: connected\n\x00" as *const u8 as
                    *const libc::c_char);
        secured = xmpp_conn_is_secured(conn);
        fprintf(stderr,
                b"DEBUG: connection is %s.\n\x00" as *const u8 as
                    *const libc::c_char,
                if secured != 0 {
                    b"secured\x00" as *const u8 as *const libc::c_char
                } else {
                    b"NOT secured\x00" as *const u8 as *const libc::c_char
                });
        xmpp_disconnect(conn);
    } else {
        fprintf(stderr,
                b"DEBUG: disconnected\n\x00" as *const u8 as
                    *const libc::c_char);
        xmpp_stop(ctx);
    };
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut ctx: *mut xmpp_ctx_t = 0 as *mut xmpp_ctx_t;
    let mut conn: *mut xmpp_conn_t = 0 as *mut xmpp_conn_t;
    let mut log: *mut xmpp_log_t = 0 as *mut xmpp_log_t;
    let mut jid: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pass: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut host: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flags: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut tcp_keepalive: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    /* take a jid and password on the command line */
    i = 1 as libc::c_int;
    while i < argc {
        if strcmp(*argv.offset(i as isize),
                  b"--disable-tls\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
            flags =
                (flags as libc::c_ulong |
                     (1 as libc::c_ulong) << 0 as libc::c_int) as libc::c_long
        } else if strcmp(*argv.offset(i as isize),
                         b"--mandatory-tls\x00" as *const u8 as
                             *const libc::c_char) == 0 as libc::c_int {
            flags =
                (flags as libc::c_ulong |
                     (1 as libc::c_ulong) << 1 as libc::c_int) as libc::c_long
        } else if strcmp(*argv.offset(i as isize),
                         b"--legacy-ssl\x00" as *const u8 as
                             *const libc::c_char) == 0 as libc::c_int {
            flags =
                (flags as libc::c_ulong |
                     (1 as libc::c_ulong) << 2 as libc::c_int) as libc::c_long
        } else {
            if !(strcmp(*argv.offset(i as isize),
                        b"--tcp-keepalive\x00" as *const u8 as
                            *const libc::c_char) == 0 as libc::c_int) {
                break ;
            }
            tcp_keepalive = 1 as libc::c_int
        }
        i += 1
    }
    if argc - i < 2 as libc::c_int || argc - i > 3 as libc::c_int {
        fprintf(stderr,
                b"Usage: basic [options] <jid> <pass> [<host>]\n\nOptions:\n  --disable-tls        Disable TLS.\n  --mandatory-tls      Deny plaintext connection.\n  --legacy-ssl         Use old style SSL.\n  --tcp-keepalive      Configure TCP keepalive.\n\nNote: --disable-tls conflicts with --mandatory-tls or --legacy-ssl\n\x00"
                    as *const u8 as *const libc::c_char);
        return 1 as libc::c_int
    }
    jid = *argv.offset(i as isize);
    pass = *argv.offset((i + 1 as libc::c_int) as isize);
    if (i + 2 as libc::c_int) < argc {
        host = *argv.offset((i + 2 as libc::c_int) as isize)
    }
    /*
     * Note, this example doesn't handle errors. Applications should check
     * return values of non-void functions.
     */
    /* init library */
    xmpp_initialize();
    /* create a context */
    log =
        xmpp_get_default_logger(XMPP_LEVEL_DEBUG); /* pass NULL instead to silence output */
    ctx = xmpp_ctx_new(0 as *const xmpp_mem_t, log);
    /* create a connection */
    conn = xmpp_conn_new(ctx);
    /* configure connection properties (optional) */
    xmpp_conn_set_flags(conn, flags);
    /* configure TCP keepalive (optional) */
    if tcp_keepalive != 0 {
        xmpp_conn_set_keepalive(conn, 60 as libc::c_int, 1 as libc::c_int);
    }
    /* setup authentication information */
    xmpp_conn_set_jid(conn, jid);
    xmpp_conn_set_pass(conn, pass);
    /* initiate connection */
    xmpp_connect_client(conn, host, 0 as libc::c_int as libc::c_ushort,
                        Some(conn_handler as
                                 unsafe extern "C" fn(_: *mut xmpp_conn_t,
                                                      _: xmpp_conn_event_t,
                                                      _: libc::c_int,
                                                      _:
                                                          *mut xmpp_stream_error_t,
                                                      _: *mut libc::c_void)
                                     -> ()), ctx as *mut libc::c_void);
    /* enter the event loop -
       our connect handler will trigger an exit */
    xmpp_run(ctx);
    /* release our connection and context */
    xmpp_conn_release(conn);
    xmpp_ctx_free(ctx);
    /* final shutdown of the library */
    xmpp_shutdown();
    return 0 as libc::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
