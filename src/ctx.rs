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
    pub type _tls;
    pub type _xmpp_rand_t;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sock_initialize();
    #[no_mangle]
    fn sock_shutdown();
    /* * Create new xmpp_rand_t object.
 *
 *  @param ctx A Strophe context object
 *
 *  @ingroup Random
 */
    #[no_mangle]
    fn xmpp_rand_new(ctx: *mut xmpp_ctx_t) -> *mut xmpp_rand_t;
    /* * Destroy an xmpp_rand_t object.
 *
 *  @param ctx A Strophe context object
 *
 *  @ingroup Random
 */
    #[no_mangle]
    fn xmpp_rand_free(ctx: *mut xmpp_ctx_t, rand: *mut xmpp_rand_t);
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
    fn tls_initialize();
    #[no_mangle]
    fn tls_shutdown();
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
    #[no_mangle]
    fn resolver_initialize();
    #[no_mangle]
    fn resolver_shutdown();
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
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
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
pub type va_list = __builtin_va_list;
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
/* ctx.c
** strophe XMPP client library -- run-time context implementation
**
** Copyright (C) 2005-2009 Collecta, Inc.
**
**  This software is provided AS-IS with no warranty, either express
**  or implied.
**
**  This program is dual licensed under the MIT and GPLv3 licenses.
*/
/* * @file
 *  Runtime contexts, library initialization and shutdown, and versioning.
 */
/* * @defgroup Context Context objects
 *  These functions create and manipulate Strophe context objects.
 *
 *  In order to support usage in a variety of environments, the
 *  Strophe library uses a runtime context object.  This object
 *  contains the information on how to do memory allocation and
 *  logging.  This allows the user to control how memory is allocated
 *  and what do to with log messages.
 *
 *  These issues do not affect programs in the common case, but many
 *  environments require special treatment.  Abstracting these into a runtime
 *  context object makes it easy to use Strophe on embedded platforms.
 *
 *  Objects in Strophe are reference counted to ease memory management issues,
 *  but the context objects are not.
 */
/* * @defgroup Init Initialization, shutdown, and versioning
 *  These functions initialize and shutdown the library, and also allow
 *  for API version checking.  Failure to properly call these functions may
 *  result in strange (and platform dependent) behavior.
 *
 *  Specifically, the socket library on Win32 platforms must be initialized
 *  before use (although this is not the case on POSIX systems).  The TLS
 *  subsystem must also seed the random number generator.
 */
/* Workaround for visual studio without va_copy support. */
/* * Initialize the Strophe library.
 *  This function initializes subcomponents of the Strophe library and must
 *  be called for Strophe to operate correctly.
 *
 *  @ingroup Init
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_initialize() {
    sock_initialize();
    resolver_initialize();
    tls_initialize();
}
/* * Shutdown the Strophe library.
 *
 *  @ingroup Init
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_shutdown() {
    tls_shutdown();
    resolver_shutdown();
    sock_shutdown();
}
/* version */
/* * Check that Strophe supports a specific API version.
 *
 *  @param major the major version number
 *  @param minor the minor version number
 *
 *  @return TRUE if the version is supported and FALSE if unsupported
 *
 *  @ingroup Init
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_version_check(mut major: libc::c_int,
                                            mut minor: libc::c_int)
 -> libc::c_int {
    return (major == 0 as libc::c_int && minor >= 0 as libc::c_int) as
               libc::c_int;
}
/* We define the global default allocator, logger, and context here. */
/* Wrap stdlib routines malloc, free, and realloc for default memory
 * management. 
 */
unsafe extern "C" fn _malloc(size: size_t, userdata: *mut libc::c_void)
 -> *mut libc::c_void {
    return malloc(size);
}
unsafe extern "C" fn _free(mut p: *mut libc::c_void,
                           userdata: *mut libc::c_void) {
    free(p);
}
unsafe extern "C" fn _realloc(mut p: *mut libc::c_void, size: size_t,
                              userdata: *mut libc::c_void)
 -> *mut libc::c_void {
    return realloc(p, size);
}
/* default memory function map */
static mut xmpp_default_mem: xmpp_mem_t =
    unsafe {
        {
            let mut init =
                _xmpp_mem_t{alloc:
                                Some(_malloc as
                                         unsafe extern "C" fn(_: size_t,
                                                              _:
                                                                  *mut libc::c_void)
                                             -> *mut libc::c_void),
                            free:
                                Some(_free as
                                         unsafe extern "C" fn(_:
                                                                  *mut libc::c_void,
                                                              _:
                                                                  *mut libc::c_void)
                                             -> ()),
                            realloc:
                                Some(_realloc as
                                         unsafe extern "C" fn(_:
                                                                  *mut libc::c_void,
                                                              _: size_t,
                                                              _:
                                                                  *mut libc::c_void)
                                             -> *mut libc::c_void),
                            userdata:
                                0 as *const libc::c_void as
                                    *mut libc::c_void,};
            init
        }
    };
/* log levels and names */
static mut _xmpp_log_level_name: [*const libc::c_char; 4] =
    [b"DEBUG\x00" as *const u8 as *const libc::c_char,
     b"INFO\x00" as *const u8 as *const libc::c_char,
     b"WARN\x00" as *const u8 as *const libc::c_char,
     b"ERROR\x00" as *const u8 as *const libc::c_char];
static mut _xmpp_default_logger_levels: [xmpp_log_level_t; 4] =
    [XMPP_LEVEL_DEBUG, XMPP_LEVEL_INFO, XMPP_LEVEL_WARN, XMPP_LEVEL_ERROR];
/* * Log a message.
 *  The default logger writes to stderr.
 *
 *  @param userdata the opaque data used by the default logger.  This contains
 *      the filter level in the default logger.
 *  @param level the level to log at
 *  @param area the area the log message is for
 *  @param msg the log message
 */
unsafe extern "C" fn xmpp_default_logger(userdata: *mut libc::c_void,
                                         level: xmpp_log_level_t,
                                         area: *const libc::c_char,
                                         msg: *const libc::c_char) {
    let mut filter_level: xmpp_log_level_t =
        *(userdata as *mut xmpp_log_level_t);
    if level as libc::c_uint >= filter_level as libc::c_uint {
        fprintf(stderr, b"%s %s %s\n\x00" as *const u8 as *const libc::c_char,
                area, _xmpp_log_level_name[level as usize], msg);
    };
}
// Initialized in run_static_initializers
static mut _xmpp_default_loggers: [xmpp_log_t; 4] =
    [xmpp_log_t{handler: None, userdata: 0 as *mut libc::c_void,}; 4];
/* return a default logger filtering at a given level */
/* * Get a default logger with filtering.
 *  The default logger provides a basic logging setup which writes log
 *  messages to stderr.  Only messages where level is greater than or
 *  equal to the filter level will be logged.
 *
 *  @param level the highest level the logger will log at
 *
 *  @return the log structure for the given level
 *
 *  @ingroup Context
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_get_default_logger(mut level: xmpp_log_level_t)
 -> *mut xmpp_log_t {
    /* clamp to the known range */
    if level as libc::c_uint > XMPP_LEVEL_ERROR as libc::c_int as libc::c_uint
       {
        level = XMPP_LEVEL_ERROR
    }
    return &*_xmpp_default_loggers.as_ptr().offset(level as isize) as
               *const xmpp_log_t as *mut xmpp_log_t;
}
static mut xmpp_default_log: xmpp_log_t =
    {
        let mut init =
            _xmpp_log_t{handler: None,
                        userdata:
                            0 as *const libc::c_void as *mut libc::c_void,};
        init
    };
/* convenience functions for accessing the context */
/* * Allocate memory in a Strophe context.
 *  All Strophe functions will use this to allocate memory.
 *
 *  @param ctx a Strophe context object
 *  @param size the number of bytes to allocate
 *
 *  @return a pointer to the allocated memory or NULL on an error
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_alloc(ctx: *const xmpp_ctx_t, size: size_t)
 -> *mut libc::c_void {
    return (*(*ctx).mem).alloc.expect("non-null function pointer")(size,
                                                                   (*(*ctx).mem).userdata);
}
/* free some blocks returned by other APIs, for example the
   buffer you get from xmpp_stanza_to_text */
/* * Free memory in a Strophe context.
 *  All Strophe functions will use this to free allocated memory.
 *
 *  @param ctx a Strophe context object
 *  @param p a pointer referencing memory to be freed
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_free(ctx: *const xmpp_ctx_t,
                                   mut p: *mut libc::c_void) {
    (*(*ctx).mem).free.expect("non-null function pointer")(p,
                                                           (*(*ctx).mem).userdata);
}
/* * Reallocate memory in a Strophe context.
 *  All Strophe functions will use this to reallocate memory.
 *
 *  @param ctx a Strophe context object
 *  @param p a pointer to previously allocated memory
 *  @param size the new size in bytes to allocate
 *
 *  @return a pointer to the reallocated memory or NULL on an error
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_realloc(ctx: *const xmpp_ctx_t,
                                      mut p: *mut libc::c_void, size: size_t)
 -> *mut libc::c_void {
    return (*(*ctx).mem).realloc.expect("non-null function pointer")(p, size,
                                                                     (*(*ctx).mem).userdata);
}
/* * Write a log message to the logger.
 *  Write a log message to the logger for the context for the specified
 *  level and area.  This function takes a printf-style format string and a
 *  variable argument list (in va_list) format.  This function is not meant
 *  to be called directly, but is used via xmpp_error, xmpp_warn, xmpp_info,
 *  and xmpp_debug.
 *
 *  @param ctx a Strophe context object
 *  @param level the level at which to log
 *  @param area the area to log for
 *  @param fmt a printf-style format string for the message
 *  @param ap variable argument list supplied for the format string
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_log(ctx: *const xmpp_ctx_t,
                                  level: xmpp_log_level_t,
                                  area: *const libc::c_char,
                                  fmt: *const libc::c_char,
                                  mut ap: ::std::ffi::VaList) {
    let mut oldret: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut smbuf: [libc::c_char; 1024] = [0; 1024];
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut copy: ::std::ffi::VaListImpl;
    copy = ap.clone();
    ret =
        xmpp_vsnprintf(smbuf.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 1024]>() as
                           libc::c_ulong, fmt, ap.as_va_list());
    if ret >=
           ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as
               libc::c_int {
        buf =
            xmpp_alloc(ctx, (ret + 1 as libc::c_int) as size_t) as
                *mut libc::c_char;
        if buf.is_null() {
            buf = 0 as *mut libc::c_char;
            xmpp_error(ctx, b"log\x00" as *const u8 as *const libc::c_char,
                       b"Failed allocating memory for log message.\x00" as
                           *const u8 as *const libc::c_char);
            return
        }
        oldret = ret;
        ret =
            xmpp_vsnprintf(buf, (ret + 1 as libc::c_int) as size_t, fmt,
                           copy.as_va_list());
        if ret > oldret {
            xmpp_error(ctx, b"log\x00" as *const u8 as *const libc::c_char,
                       b"Unexpected error\x00" as *const u8 as
                           *const libc::c_char);
            xmpp_free(ctx, buf as *mut libc::c_void);
            return
        }
    } else { buf = smbuf.as_mut_ptr() }
    if (*(*ctx).log).handler.is_some() {
        (*(*ctx).log).handler.expect("non-null function pointer")((*(*ctx).log).userdata,
                                                                  level, area,
                                                                  buf);
    }
    if buf != smbuf.as_mut_ptr() {
        xmpp_free(ctx, buf as *mut libc::c_void);
    };
}
/* * Write to the log at the ERROR level.
 *  This is a convenience function for writing to the log at the
 *  ERROR level.  It takes a printf-style format string followed by a
 *  variable list of arguments for formatting.
 *
 *  @param ctx a Strophe context object
 *  @param area the area to log for
 *  @param fmt a printf-style format string followed by a variable list of
 *      arguments to format
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_error(ctx: *const xmpp_ctx_t,
                                    area: *const libc::c_char,
                                    fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    xmpp_log(ctx, XMPP_LEVEL_ERROR, area, fmt, ap.as_va_list());
}
/* * Write to the log at the WARN level.
 *  This is a convenience function for writing to the log at the WARN level.
 *  It takes a printf-style format string followed by a variable list of
 *  arguments for formatting.
 *
 *  @param ctx a Strophe context object
 *  @param area the area to log for
 *  @param fmt a printf-style format string followed by a variable list of
 *      arguments to format
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_warn(ctx: *const xmpp_ctx_t,
                                   area: *const libc::c_char,
                                   fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    xmpp_log(ctx, XMPP_LEVEL_WARN, area, fmt, ap.as_va_list());
}
/* * Write to the log at the INFO level.
 *  This is a convenience function for writing to the log at the INFO level.
 *  It takes a printf-style format string followed by a variable list of
 *  arguments for formatting.
 *
 *  @param ctx a Strophe context object
 *  @param area the area to log for
 *  @param fmt a printf-style format string followed by a variable list of
 *      arguments to format
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_info(ctx: *const xmpp_ctx_t,
                                   area: *const libc::c_char,
                                   fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    xmpp_log(ctx, XMPP_LEVEL_INFO, area, fmt, ap.as_va_list());
}
/* * Write to the log at the DEBUG level.
 *  This is a convenience function for writing to the log at the DEBUG level.
 *  It takes a printf-style format string followed by a variable list of
 *  arguments for formatting.
 *
 *  @param ctx a Strophe context object
 *  @param area the area to log for
 *  @param fmt a printf-style format string followed by a variable list of
 *      arguments to format
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_debug(ctx: *const xmpp_ctx_t,
                                    area: *const libc::c_char,
                                    fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    xmpp_log(ctx, XMPP_LEVEL_DEBUG, area, fmt, ap.as_va_list());
}
/* * Create and initialize a Strophe context object.
 *  If mem is NULL, a default allocation setup will be used which
 *  wraps malloc(), free(), and realloc() from the standard library.
 *  If log is NULL, a default logger will be used which does no
 *  logging.  Basic filtered logging to stderr can be done with the
 *  xmpp_get_default_logger() convenience function.
 *
 *  @param mem a pointer to an xmpp_mem_t structure or NULL
 *  @param log a pointer to an xmpp_log_t structure or NULL
 *
 *  @return the allocated Strophe context object or NULL on an error
 *
 *  @ingroup Context
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_ctx_new(mem: *const xmpp_mem_t,
                                      log: *const xmpp_log_t)
 -> *mut xmpp_ctx_t {
    let mut ctx: *mut xmpp_ctx_t = 0 as *mut xmpp_ctx_t;
    if mem.is_null() {
        ctx =
            xmpp_default_mem.alloc.expect("non-null function pointer")(::std::mem::size_of::<xmpp_ctx_t>()
                                                                           as
                                                                           libc::c_ulong,
                                                                       0 as
                                                                           *mut libc::c_void)
                as *mut xmpp_ctx_t
    } else {
        ctx =
            (*mem).alloc.expect("non-null function pointer")(::std::mem::size_of::<xmpp_ctx_t>()
                                                                 as
                                                                 libc::c_ulong,
                                                             (*mem).userdata)
                as *mut xmpp_ctx_t
    }
    if !ctx.is_null() {
        if !mem.is_null() {
            (*ctx).mem = mem
        } else { (*ctx).mem = &mut xmpp_default_mem }
        if log.is_null() {
            (*ctx).log = &mut xmpp_default_log
        } else { (*ctx).log = log }
        (*ctx).connlist = 0 as *mut xmpp_connlist_t;
        (*ctx).loop_status = XMPP_LOOP_NOTSTARTED;
        (*ctx).rand = xmpp_rand_new(ctx);
        (*ctx).timeout = 1000 as libc::c_int as libc::c_ulong;
        if (*ctx).rand.is_null() {
            xmpp_free(ctx, ctx as *mut libc::c_void);
            ctx = 0 as *mut xmpp_ctx_t
        }
    }
    return ctx;
}
/* * Free a Strophe context object that is no longer in use.
 *
 *  @param ctx a Strophe context object
 *
 *  @ingroup Context
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_ctx_free(ctx: *mut xmpp_ctx_t) {
    /* mem and log are owned by their suppliers */
    xmpp_rand_free(ctx, (*ctx).rand);
    xmpp_free(ctx, ctx as *mut libc::c_void);
    /* pull the hole in after us */
}
/* * Set the timeout to use when calling xmpp_run().
 *
 *  @param ctx a Strophe context object
 *  @param timeout the time to wait for events in milliseconds
 *
 *  @ingroup Context
 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_ctx_set_timeout(ctx: *mut xmpp_ctx_t,
                                              timeout: libc::c_ulong) {
    (*ctx).timeout = timeout;
}
unsafe extern "C" fn run_static_initializers() {
    _xmpp_default_loggers =
        [{
             let mut init =
                 _xmpp_log_t{handler:
                                 Some(xmpp_default_logger as
                                          unsafe extern "C" fn(_:
                                                                   *mut libc::c_void,
                                                               _:
                                                                   xmpp_log_level_t,
                                                               _:
                                                                   *const libc::c_char,
                                                               _:
                                                                   *const libc::c_char)
                                              -> ()),
                             userdata:
                                 &*_xmpp_default_logger_levels.as_ptr().offset(XMPP_LEVEL_DEBUG
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                     as *const xmpp_log_level_t as
                                     *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 _xmpp_log_t{handler:
                                 Some(xmpp_default_logger as
                                          unsafe extern "C" fn(_:
                                                                   *mut libc::c_void,
                                                               _:
                                                                   xmpp_log_level_t,
                                                               _:
                                                                   *const libc::c_char,
                                                               _:
                                                                   *const libc::c_char)
                                              -> ()),
                             userdata:
                                 &*_xmpp_default_logger_levels.as_ptr().offset(XMPP_LEVEL_INFO
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                     as *const xmpp_log_level_t as
                                     *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 _xmpp_log_t{handler:
                                 Some(xmpp_default_logger as
                                          unsafe extern "C" fn(_:
                                                                   *mut libc::c_void,
                                                               _:
                                                                   xmpp_log_level_t,
                                                               _:
                                                                   *const libc::c_char,
                                                               _:
                                                                   *const libc::c_char)
                                              -> ()),
                             userdata:
                                 &*_xmpp_default_logger_levels.as_ptr().offset(XMPP_LEVEL_WARN
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                     as *const xmpp_log_level_t as
                                     *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 _xmpp_log_t{handler:
                                 Some(xmpp_default_logger as
                                          unsafe extern "C" fn(_:
                                                                   *mut libc::c_void,
                                                               _:
                                                                   xmpp_log_level_t,
                                                               _:
                                                                   *const libc::c_char,
                                                               _:
                                                                   *const libc::c_char)
                                              -> ()),
                             userdata:
                                 &*_xmpp_default_logger_levels.as_ptr().offset(XMPP_LEVEL_ERROR
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                     as *const xmpp_log_level_t as
                                     *mut libc::c_void,};
             init
         }]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
