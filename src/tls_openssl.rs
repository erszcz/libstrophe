use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
extern "C" {
    pub type evp_pkey_st;
    pub type x509_st;
    pub type X509_name_st;
    pub type x509_store_ctx_st;
    pub type X509_VERIFY_PARAM_st;
    pub type ossl_init_settings_st;
    pub type ssl_st;
    pub type ssl_ctx_st;
    pub type ssl_method_st;
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
    pub type _xmpp_rand_t;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn select(__nfds: libc::c_int, __readfds: *mut fd_set,
              __writefds: *mut fd_set, __exceptfds: *mut fd_set,
              __timeout: *mut timeval) -> libc::c_int;
    #[no_mangle]
    fn OPENSSL_init_ssl(opts: uint64_t,
                        settings: *const OPENSSL_INIT_SETTINGS)
     -> libc::c_int;
    #[no_mangle]
    fn CRYPTO_free(ptr: *mut libc::c_void, file: *const libc::c_char,
                   line: libc::c_int);
    #[no_mangle]
    fn X509_VERIFY_PARAM_set1_host(param: *mut X509_VERIFY_PARAM,
                                   name: *const libc::c_char, namelen: size_t)
     -> libc::c_int;
    #[no_mangle]
    fn X509_VERIFY_PARAM_set_hostflags(param: *mut X509_VERIFY_PARAM,
                                       flags: libc::c_uint);
    #[no_mangle]
    fn X509_free(a: *mut X509);
    #[no_mangle]
    fn X509_NAME_oneline(a: *const X509_NAME, buf: *mut libc::c_char,
                         size: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn X509_get_issuer_name(a: *const X509) -> *mut X509_NAME;
    #[no_mangle]
    fn X509_get_subject_name(a: *const X509) -> *mut X509_NAME;
    #[no_mangle]
    fn SSL_CTX_set_options(ctx: *mut SSL_CTX, op: libc::c_ulong)
     -> libc::c_ulong;
    #[no_mangle]
    fn SSL_CTX_set_client_cert_cb(ctx: *mut SSL_CTX,
                                  client_cert_cb:
                                      Option<unsafe extern "C" fn(_: *mut SSL,
                                                                  _:
                                                                      *mut *mut X509,
                                                                  _:
                                                                      *mut *mut EVP_PKEY)
                                                 -> libc::c_int>);
    #[no_mangle]
    fn SSL_CTX_new(meth: *const SSL_METHOD) -> *mut SSL_CTX;
    #[no_mangle]
    fn SSL_CTX_free(_: *mut SSL_CTX);
    #[no_mangle]
    fn SSL_pending(s: *const SSL) -> libc::c_int;
    #[no_mangle]
    fn SSL_set_fd(s: *mut SSL, fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn SSL_set_verify(s: *mut SSL, mode: libc::c_int,
                      callback: SSL_verify_cb);
    #[no_mangle]
    fn SSL_get_peer_certificate(s: *const SSL) -> *mut X509;
    #[no_mangle]
    fn SSL_new(ctx: *mut SSL_CTX) -> *mut SSL;
    #[no_mangle]
    fn SSL_get0_param(ssl: *mut SSL) -> *mut X509_VERIFY_PARAM;
    #[no_mangle]
    fn SSL_free(ssl: *mut SSL);
    #[no_mangle]
    fn SSL_connect(ssl: *mut SSL) -> libc::c_int;
    #[no_mangle]
    fn SSL_read(ssl: *mut SSL, buf: *mut libc::c_void, num: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn SSL_write(ssl: *mut SSL, buf: *const libc::c_void, num: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn SSL_ctrl(ssl: *mut SSL, cmd: libc::c_int, larg: libc::c_long,
                parg: *mut libc::c_void) -> libc::c_long;
    #[no_mangle]
    fn SSL_CTX_ctrl(ctx: *mut SSL_CTX, cmd: libc::c_int, larg: libc::c_long,
                    parg: *mut libc::c_void) -> libc::c_long;
    #[no_mangle]
    fn SSL_get_error(s: *const SSL, ret_code: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn TLS_client_method() -> *const SSL_METHOD;
    #[no_mangle]
    fn SSL_shutdown(s: *mut SSL) -> libc::c_int;
    #[no_mangle]
    fn SSL_CTX_set_default_verify_paths(ctx: *mut SSL_CTX) -> libc::c_int;
    #[no_mangle]
    fn SSL_get_verify_result(ssl: *const SSL) -> libc::c_long;
    #[no_mangle]
    fn ERR_get_error() -> libc::c_ulong;
    #[no_mangle]
    fn ERR_error_string_n(e: libc::c_ulong, buf: *mut libc::c_char,
                          len: size_t);
    /* convenience functions for accessing the context */
    #[no_mangle]
    fn xmpp_alloc(ctx: *const xmpp_ctx_t, size: size_t) -> *mut libc::c_void;
    /* wrappers for xmpp_log at specific levels */
    #[no_mangle]
    fn xmpp_error(ctx: *const xmpp_ctx_t, area: *const libc::c_char,
                  fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn xmpp_debug(ctx: *const xmpp_ctx_t, area: *const libc::c_char,
                  fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn xmpp_free(ctx: *const xmpp_ctx_t, p: *mut libc::c_void);
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
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
pub type uint64_t = __uint64_t;
pub type EVP_PKEY = evp_pkey_st;
pub type X509 = x509_st;
pub type X509_NAME = X509_name_st;
pub type X509_STORE_CTX = x509_store_ctx_st;
pub type X509_VERIFY_PARAM = X509_VERIFY_PARAM_st;
pub type OPENSSL_INIT_SETTINGS = ossl_init_settings_st;
pub type SSL = ssl_st;
pub type SSL_CTX = ssl_ctx_st;
pub type SSL_METHOD = ssl_method_st;
pub type SSL_verify_cb
    =
    Option<unsafe extern "C" fn(_: libc::c_int, _: *mut X509_STORE_CTX)
               -> libc::c_int>;
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
/* tls_openssl.c
** strophe XMPP client library -- TLS abstraction openssl impl.
**
** Copyright (C) 2005-008 Collecta, Inc. 
**
**  This software is provided AS-IS with no warranty, either express
**  or implied.
**
**  This program is dual licensed under the MIT and GPLv3 licenses.
*/
/* * @file
 *  TLS implementation with OpenSSL.
 */
/* EINTR */
/*
 * Redefine OPENSSL_VERSION_NUMBER for LibreSSL.
 * LibreSSL and OpenSSL use different and incompatible version schemes. Solve
 * this issue in the way how nginx project did.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tls {
    pub ctx: *mut xmpp_ctx_t,
    pub sock: sock_t,
    pub ssl_ctx: *mut SSL_CTX,
    pub ssl: *mut SSL,
    pub lasterror: libc::c_int,
}
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
pub const TLS_TIMEOUT_USEC: C2RustUnnamed_3 = 100000;
pub const TLS_TIMEOUT_SEC: C2RustUnnamed_3 = 0;
pub const TLS_SHUTDOWN_MAX_RETRIES: C2RustUnnamed_3 = 10;
pub type C2RustUnnamed_3 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn tls_initialize() {
    OPENSSL_init_ssl(0x200000 as libc::c_long as uint64_t,
                     0 as *const OPENSSL_INIT_SETTINGS);
}
#[no_mangle]
pub unsafe extern "C" fn tls_shutdown() {
    /*
     * FIXME: Don't free global tables, program or other libraries may use
     * openssl after libstrophe finalization. Maybe better leak some fixed
     * memory rather than cause random crashes of the main program.
     */
}
#[no_mangle]
pub unsafe extern "C" fn tls_error(mut tls: *mut tls_t) -> libc::c_int {
    return (*tls).lasterror;
}
#[no_mangle]
pub unsafe extern "C" fn tls_new(mut conn: *mut xmpp_conn_t) -> *mut tls_t {
    let mut current_block: u64;
    let mut tls: *mut tls_t =
        xmpp_alloc((*conn).ctx,
                   ::std::mem::size_of::<tls_t>() as libc::c_ulong) as
            *mut tls_t;
    let mut mode: libc::c_int = 0;
    if !tls.is_null() {
        let mut ret: libc::c_int = 0;
        /* Hostname verification is supported in OpenSSL 1.0.2 and newer. */
        let mut param: *mut X509_VERIFY_PARAM = 0 as *mut X509_VERIFY_PARAM;
        memset(tls as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<tls_t>() as libc::c_ulong);
        (*tls).ctx = (*conn).ctx;
        (*tls).sock = (*conn).sock;
        (*tls).ssl_ctx = SSL_CTX_new(TLS_client_method());
        if (*tls).ssl_ctx.is_null() {
            current_block = 3699465889004314628;
        } else {
            /* Enable bug workarounds. */
            SSL_CTX_set_options((*tls).ssl_ctx,
                                (0x80000000 as libc::c_uint |
                                     0x800 as libc::c_uint |
                                     0x4 as libc::c_uint |
                                     0x10 as libc::c_uint |
                                     0x40 as libc::c_uint) as libc::c_ulong);
            /* Disable insecure SSL/TLS versions. */
            SSL_CTX_set_options((*tls).ssl_ctx,
                                0 as libc::c_int as
                                    libc::c_ulong); /* DROWN */
            SSL_CTX_set_options((*tls).ssl_ctx,
                                0x2000000 as libc::c_uint as
                                    libc::c_ulong); /* POODLE */
            SSL_CTX_set_options((*tls).ssl_ctx,
                                0x4000000 as libc::c_uint as
                                    libc::c_ulong); /* BEAST */
            SSL_CTX_set_client_cert_cb((*tls).ssl_ctx, None);
            SSL_CTX_ctrl((*tls).ssl_ctx, 33 as libc::c_int,
                         0x1 as libc::c_uint as libc::c_long,
                         0 as *mut libc::c_void);
            ret = SSL_CTX_set_default_verify_paths((*tls).ssl_ctx);
            if ret == 0 as libc::c_int && (*conn).tls_trust == 0 {
                /*
             * Returns 1 on success and 0 on failure. A missing default
             * location is still treated as a success.
             * Ignore errors when XMPP_CONN_FLAG_TRUST_TLS is set.
             */
                xmpp_error((*tls).ctx,
                           b"tls\x00" as *const u8 as *const libc::c_char,
                           b"SSL_CTX_set_default_verify_paths() failed\x00" as
                               *const u8 as *const libc::c_char);
                current_block = 18300470474662330181;
            } else {
                (*tls).ssl = SSL_new((*tls).ssl_ctx);
                if (*tls).ssl.is_null() {
                    current_block = 18300470474662330181;
                } else {
                    /* Enable SNI. */
                    SSL_ctrl((*tls).ssl, 55 as libc::c_int,
                             0 as libc::c_int as libc::c_long,
                             (*conn).domain as *mut libc::c_void);
                    /* Trust server's certificate when user sets the flag explicitly. */
                    mode =
                        if (*conn).tls_trust != 0 {
                            0 as libc::c_int
                        } else { 0x1 as libc::c_int };
                    SSL_set_verify((*tls).ssl, mode, None);
                    /* Hostname verification is supported in OpenSSL 1.0.2 and newer. */
                    param = SSL_get0_param((*tls).ssl);
                    /*
         * Allow only complete wildcards.  RFC 6125 discourages wildcard usage
         * completely, and lists internationalized domain names as a reason
         * against partial wildcards.
         * See https://tools.ietf.org/html/rfc6125#section-7.2 for more information.
         */
                    X509_VERIFY_PARAM_set_hostflags(param,
                                                    0x4 as libc::c_int as
                                                        libc::c_uint);
                    X509_VERIFY_PARAM_set1_host(param, (*conn).domain,
                                                0 as libc::c_int as size_t);
                    ret = SSL_set_fd((*tls).ssl, (*conn).sock);
                    if ret <= 0 as libc::c_int {
                        SSL_free((*tls).ssl);
                        current_block = 18300470474662330181;
                    } else { current_block = 9828876828309294594; }
                }
            }
            match current_block {
                9828876828309294594 => { }
                _ => {
                    SSL_CTX_free((*tls).ssl_ctx);
                    current_block = 3699465889004314628;
                }
            }
        }
        match current_block {
            9828876828309294594 => { }
            _ => {
                xmpp_free((*conn).ctx, tls as *mut libc::c_void);
                _tls_log_error((*conn).ctx);
                return 0 as *mut tls_t
            }
        }
    }
    return tls;
}
#[no_mangle]
pub unsafe extern "C" fn tls_free(mut tls: *mut tls_t) {
    SSL_free((*tls).ssl);
    SSL_CTX_free((*tls).ssl_ctx);
    xmpp_free((*tls).ctx, tls as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn tls_set_credentials(mut tls: *mut tls_t,
                                             mut cafilename:
                                                 *const libc::c_char)
 -> libc::c_int {
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn tls_start(mut tls: *mut tls_t) -> libc::c_int {
    let mut error: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut x509_res: libc::c_long = 0;
    loop 
         /* Since we're non-blocking, loop the connect call until it
       succeeds or fails */
         {
        ret = SSL_connect((*tls).ssl);
        error =
            if ret <= 0 as libc::c_int {
                SSL_get_error((*tls).ssl, ret)
            } else { 0 as libc::c_int };
        if !(ret == -(1 as libc::c_int) && tls_is_recoverable(error) != 0) {
            break ;
        }
        /* wait for something to happen on the sock before looping back */
        _tls_sock_wait(tls, error);
    }
    x509_res = SSL_get_verify_result((*tls).ssl);
    xmpp_debug((*tls).ctx, b"tls\x00" as *const u8 as *const libc::c_char,
               b"Certificate verification %s\x00" as *const u8 as
                   *const libc::c_char,
               if x509_res == 0 as libc::c_int as libc::c_long {
                   b"passed\x00" as *const u8 as *const libc::c_char
               } else { b"FAILED\x00" as *const u8 as *const libc::c_char });
    _tls_dump_cert_info(tls);
    _tls_set_error(tls, error);
    return if ret <= 0 as libc::c_int {
               0 as libc::c_int
           } else { 1 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn tls_stop(mut tls: *mut tls_t) -> libc::c_int {
    let mut retries: libc::c_int = 0 as libc::c_int;
    let mut error: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    /* According to OpenSSL.org, we must not call SSL_shutdown(3)
       if a previous fatal error has occurred on a connection. */
    if (*tls).lasterror == 5 as libc::c_int ||
           (*tls).lasterror == 1 as libc::c_int {
        return 1 as libc::c_int
    }
    loop  {
        retries += 1;
        ret = SSL_shutdown((*tls).ssl);
        error =
            if ret < 0 as libc::c_int {
                SSL_get_error((*tls).ssl, ret)
            } else { 0 as libc::c_int };
        if ret == 1 as libc::c_int || tls_is_recoverable(error) == 0 ||
               retries >= TLS_SHUTDOWN_MAX_RETRIES as libc::c_int {
            break ;
        }
        _tls_sock_wait(tls, error);
    }
    if error == 5 as libc::c_int && *__errno_location() == 0 as libc::c_int {
        /*
         * Handle special case when peer closes connection instead of
         * proper shutdown.
         */
        error = 0 as libc::c_int;
        ret = 1 as libc::c_int
    }
    _tls_set_error(tls, error);
    return if ret <= 0 as libc::c_int {
               0 as libc::c_int
           } else { 1 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn tls_is_recoverable(mut error: libc::c_int)
 -> libc::c_int {
    return (error == 0 as libc::c_int || error == 2 as libc::c_int ||
                error == 3 as libc::c_int || error == 7 as libc::c_int ||
                error == 8 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tls_pending(mut tls: *mut tls_t) -> libc::c_int {
    return SSL_pending((*tls).ssl);
}
#[no_mangle]
pub unsafe extern "C" fn tls_read(mut tls: *mut tls_t,
                                  buff: *mut libc::c_void, len: size_t)
 -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = SSL_read((*tls).ssl, buff, len as libc::c_int);
    _tls_set_error(tls,
                   if ret <= 0 as libc::c_int {
                       SSL_get_error((*tls).ssl, ret)
                   } else { 0 as libc::c_int });
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn tls_write(mut tls: *mut tls_t,
                                   buff: *const libc::c_void, len: size_t)
 -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = SSL_write((*tls).ssl, buff, len as libc::c_int);
    _tls_set_error(tls,
                   if ret <= 0 as libc::c_int {
                       SSL_get_error((*tls).ssl, ret)
                   } else { 0 as libc::c_int });
    return ret;
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
#[no_mangle]
pub unsafe extern "C" fn tls_clear_pending_write(mut tls: *mut tls_t)
 -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn _tls_sock_wait(mut tls: *mut tls_t,
                                    mut error: libc::c_int) {
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut rfds: fd_set = fd_set{__fds_bits: [0; 16],};
    let mut wfds: fd_set = fd_set{__fds_bits: [0; 16],};
    let mut nfds: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if error == 0 as libc::c_int { return }
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
    if error == 2 as libc::c_int {
        rfds.__fds_bits[((*tls).sock /
                             (8 as libc::c_int *
                                  ::std::mem::size_of::<__fd_mask>() as
                                      libc::c_ulong as libc::c_int)) as usize]
            |=
            ((1 as libc::c_ulong) <<
                 (*tls).sock %
                     (8 as libc::c_int *
                          ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                              as libc::c_int)) as __fd_mask
    }
    if error == 3 as libc::c_int {
        wfds.__fds_bits[((*tls).sock /
                             (8 as libc::c_int *
                                  ::std::mem::size_of::<__fd_mask>() as
                                      libc::c_ulong as libc::c_int)) as usize]
            |=
            ((1 as libc::c_ulong) <<
                 (*tls).sock %
                     (8 as libc::c_int *
                          ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                              as libc::c_int)) as __fd_mask
    }
    nfds =
        if error == 2 as libc::c_int || error == 3 as libc::c_int {
            ((*tls).sock) + 1 as libc::c_int
        } else { 0 as libc::c_int };
    loop  {
        tv.tv_sec = TLS_TIMEOUT_SEC as libc::c_int as __time_t;
        tv.tv_usec = TLS_TIMEOUT_USEC as libc::c_int as __suseconds_t;
        ret = select(nfds, &mut rfds, &mut wfds, 0 as *mut fd_set, &mut tv);
        if !(ret == -(1 as libc::c_int) &&
                 *__errno_location() == 4 as libc::c_int) {
            break ;
        }
    };
}
unsafe extern "C" fn _tls_set_error(mut tls: *mut tls_t,
                                    mut error: libc::c_int) {
    if error != 0 as libc::c_int && tls_is_recoverable(error) == 0 {
        xmpp_debug((*tls).ctx, b"tls\x00" as *const u8 as *const libc::c_char,
                   b"error=%d errno=%d\x00" as *const u8 as
                       *const libc::c_char, error, *__errno_location());
        _tls_log_error((*tls).ctx);
    }
    (*tls).lasterror = error;
}
unsafe extern "C" fn _tls_log_error(mut ctx: *mut xmpp_ctx_t) {
    let mut e: libc::c_ulong = 0;
    let mut buf: [libc::c_char; 256] = [0; 256];
    loop  {
        e = ERR_get_error();
        if e != 0 as libc::c_int as libc::c_ulong {
            ERR_error_string_n(e, buf.as_mut_ptr(),
                               ::std::mem::size_of::<[libc::c_char; 256]>() as
                                   libc::c_ulong);
            xmpp_debug(ctx, b"tls\x00" as *const u8 as *const libc::c_char,
                       b"%s\x00" as *const u8 as *const libc::c_char,
                       buf.as_mut_ptr());
        }
        if !(e != 0 as libc::c_int as libc::c_ulong) { break ; }
    };
}
unsafe extern "C" fn _tls_dump_cert_info(mut tls: *mut tls_t) {
    let mut cert: *mut X509 = 0 as *mut X509;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    cert = SSL_get_peer_certificate((*tls).ssl);
    if cert.is_null() {
        xmpp_debug((*tls).ctx, b"tls\x00" as *const u8 as *const libc::c_char,
                   b"Certificate was not presented by peer\x00" as *const u8
                       as *const libc::c_char);
    } else {
        name =
            X509_NAME_oneline(X509_get_subject_name(cert),
                              0 as *mut libc::c_char, 0 as libc::c_int);
        if !name.is_null() {
            xmpp_debug((*tls).ctx,
                       b"tls\x00" as *const u8 as *const libc::c_char,
                       b"Subject=%s\x00" as *const u8 as *const libc::c_char,
                       name);
            CRYPTO_free(name as *mut libc::c_void,
                        b"src/tls_openssl.c\x00" as *const u8 as
                            *const libc::c_char, 370 as libc::c_int);
        }
        name =
            X509_NAME_oneline(X509_get_issuer_name(cert),
                              0 as *mut libc::c_char, 0 as libc::c_int);
        if !name.is_null() {
            xmpp_debug((*tls).ctx,
                       b"tls\x00" as *const u8 as *const libc::c_char,
                       b"Issuer=%s\x00" as *const u8 as *const libc::c_char,
                       name);
            CRYPTO_free(name as *mut libc::c_void,
                        b"src/tls_openssl.c\x00" as *const u8 as
                            *const libc::c_char, 375 as libc::c_int);
        }
        X509_free(cert);
    };
}
