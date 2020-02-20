use ::libc;
extern "C" {
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn socket(__domain: libc::c_int, __type: libc::c_int,
              __protocol: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn connect(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t)
     -> libc::c_int;
    #[no_mangle]
    fn getpeername(__fd: libc::c_int, __addr: *mut sockaddr,
                   __len: *mut socklen_t) -> libc::c_int;
    #[no_mangle]
    fn send(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t,
            __flags: libc::c_int) -> ssize_t;
    #[no_mangle]
    fn recv(__fd: libc::c_int, __buf: *mut libc::c_void, __n: size_t,
            __flags: libc::c_int) -> ssize_t;
    #[no_mangle]
    fn setsockopt(__fd: libc::c_int, __level: libc::c_int,
                  __optname: libc::c_int, __optval: *const libc::c_void,
                  __optlen: socklen_t) -> libc::c_int;
    #[no_mangle]
    fn getaddrinfo(__name: *const libc::c_char,
                   __service: *const libc::c_char, __req: *const addrinfo,
                   __pai: *mut *mut addrinfo) -> libc::c_int;
    #[no_mangle]
    fn freeaddrinfo(__ai: *mut addrinfo);
    #[no_mangle]
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
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
}
pub type size_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type socklen_t = __socklen_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type C2RustUnnamed = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed = 256;
pub const IPPROTO_RAW: C2RustUnnamed = 255;
pub const IPPROTO_MPLS: C2RustUnnamed = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed = 136;
pub const IPPROTO_SCTP: C2RustUnnamed = 132;
pub const IPPROTO_COMP: C2RustUnnamed = 108;
pub const IPPROTO_PIM: C2RustUnnamed = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed = 94;
pub const IPPROTO_MTP: C2RustUnnamed = 92;
pub const IPPROTO_AH: C2RustUnnamed = 51;
pub const IPPROTO_ESP: C2RustUnnamed = 50;
pub const IPPROTO_GRE: C2RustUnnamed = 47;
pub const IPPROTO_RSVP: C2RustUnnamed = 46;
pub const IPPROTO_IPV6: C2RustUnnamed = 41;
pub const IPPROTO_DCCP: C2RustUnnamed = 33;
pub const IPPROTO_TP: C2RustUnnamed = 29;
pub const IPPROTO_IDP: C2RustUnnamed = 22;
pub const IPPROTO_UDP: C2RustUnnamed = 17;
pub const IPPROTO_PUP: C2RustUnnamed = 12;
pub const IPPROTO_EGP: C2RustUnnamed = 8;
pub const IPPROTO_TCP: C2RustUnnamed = 6;
pub const IPPROTO_IPIP: C2RustUnnamed = 4;
pub const IPPROTO_IGMP: C2RustUnnamed = 2;
pub const IPPROTO_ICMP: C2RustUnnamed = 1;
pub const IPPROTO_IP: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
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
/* sock.c
** strophe XMPP client library -- socket abstraction implementation
**
** Copyright (C) 2005-2009 Collecta, Inc. 
**
**  This software is provided AS-IS with no warranty, either express
**  or implied.
**
** This program is dual licensed under the MIT and GPLv3 licenses.
*/
/* * @file
 *  Socket abstraction.
 */
#[no_mangle]
pub unsafe extern "C" fn sock_initialize() { }
#[no_mangle]
pub unsafe extern "C" fn sock_shutdown() { }
#[no_mangle]
pub unsafe extern "C" fn sock_error() -> libc::c_int {
    return *__errno_location();
}
unsafe extern "C" fn _in_progress(mut error: libc::c_int) -> libc::c_int {
    return (error == 115 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sock_connect(host: *const libc::c_char,
                                      port: libc::c_ushort) -> sock_t {
    let mut sock: sock_t = 0;
    let mut service: [libc::c_char; 6] = [0; 6];
    let mut res: *mut addrinfo = 0 as *mut addrinfo;
    let mut ainfo: *mut addrinfo = 0 as *mut addrinfo;
    let mut hints: addrinfo =
        addrinfo{ai_flags: 0,
                 ai_family: 0,
                 ai_socktype: 0,
                 ai_protocol: 0,
                 ai_addrlen: 0,
                 ai_addr: 0 as *mut sockaddr,
                 ai_canonname: 0 as *mut libc::c_char,
                 ai_next: 0 as *mut addrinfo,};
    let mut err: libc::c_int = 0;
    xmpp_snprintf(service.as_mut_ptr(), 6 as libc::c_int as size_t,
                  b"%u\x00" as *const u8 as *const libc::c_char,
                  port as libc::c_int);
    memset(&mut hints as *mut addrinfo as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<addrinfo>() as libc::c_ulong);
    hints.ai_family = 0 as libc::c_int;
    hints.ai_flags = 0x20 as libc::c_int;
    /* AI_ADDRCONFIG */
    hints.ai_protocol = IPPROTO_TCP as libc::c_int;
    hints.ai_socktype = SOCK_STREAM as libc::c_int;
    err = getaddrinfo(host, service.as_mut_ptr(), &mut hints, &mut res);
    if err != 0 as libc::c_int { return -(1 as libc::c_int) }
    ainfo = res;
    while !ainfo.is_null() {
        sock =
            socket((*ainfo).ai_family, (*ainfo).ai_socktype,
                   (*ainfo).ai_protocol);
        if !(sock < 0 as libc::c_int) {
            err = sock_set_nonblocking(sock);
            if err == 0 as libc::c_int {
                err = connect(sock, (*ainfo).ai_addr, (*ainfo).ai_addrlen);
                if err == 0 as libc::c_int || _in_progress(sock_error()) != 0
                   {
                    break ;
                }
            }
            sock_close(sock);
        }
        ainfo = (*ainfo).ai_next
    }
    freeaddrinfo(res);
    sock = if ainfo.is_null() { -(1 as libc::c_int) } else { sock };
    return sock;
}
#[no_mangle]
pub unsafe extern "C" fn sock_set_keepalive(sock: sock_t,
                                            mut timeout: libc::c_int,
                                            mut interval: libc::c_int)
 -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut optval: libc::c_int =
        if timeout != 0 && interval != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    /* This function doesn't change maximum number of keepalive probes */
    ret =
        setsockopt(sock, 1 as libc::c_int, 9 as libc::c_int,
                   &mut optval as *mut libc::c_int as *const libc::c_void,
                   ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                       socklen_t);
    if ret < 0 as libc::c_int { return ret }
    if optval != 0 {
        ret =
            setsockopt(sock, IPPROTO_TCP as libc::c_int, 4 as libc::c_int,
                       &mut timeout as *mut libc::c_int as
                           *const libc::c_void,
                       ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                           as socklen_t);
        /* TCP_KEEPINTVL */
        if ret < 0 as libc::c_int { return ret }
        ret =
            setsockopt(sock, IPPROTO_TCP as libc::c_int, 5 as libc::c_int,
                       &mut interval as *mut libc::c_int as
                           *const libc::c_void,
                       ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                           as socklen_t);
        if ret < 0 as libc::c_int { return ret }
    }
    /* TCP_KEEPIDLE */
    /* _WIN32 */
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn sock_close(sock: sock_t) -> libc::c_int {
    return close(sock);
}
unsafe extern "C" fn _sock_set_blocking_mode(mut sock: sock_t,
                                             mut blocking: libc::c_int)
 -> libc::c_int {
    let mut rc: libc::c_int = 0;
    rc = fcntl(sock, 3 as libc::c_int, 0 as *mut libc::c_void);
    if rc >= 0 as libc::c_int {
        rc =
            if blocking != 0 {
                (rc) & !(0o4000 as libc::c_int)
            } else { (rc) | 0o4000 as libc::c_int };
        rc = fcntl(sock, 4 as libc::c_int, rc)
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn sock_set_blocking(sock: sock_t) -> libc::c_int {
    return _sock_set_blocking_mode(sock, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn sock_set_nonblocking(sock: sock_t) -> libc::c_int {
    return _sock_set_blocking_mode(sock, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn sock_read(sock: sock_t, buff: *mut libc::c_void,
                                   len: size_t) -> libc::c_int {
    return recv(sock, buff, len, 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sock_write(sock: sock_t, buff: *const libc::c_void,
                                    len: size_t) -> libc::c_int {
    return send(sock, buff, len, 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sock_is_recoverable(error: libc::c_int)
 -> libc::c_int {
    return (error == 11 as libc::c_int || error == 4 as libc::c_int) as
               libc::c_int;
}
/* checks for an error after connect, return 0 if connect successful */
#[no_mangle]
pub unsafe extern "C" fn sock_connect_error(sock: sock_t) -> libc::c_int {
    let mut sa: sockaddr = sockaddr{sa_family: 0, sa_data: [0; 14],};
    let mut len: socklen_t = 0;
    let mut temp: libc::c_char = 0;
    memset(&mut sa as *mut sockaddr as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<sockaddr>() as libc::c_ulong);
    sa.sa_family = 0 as libc::c_int as sa_family_t;
    len = ::std::mem::size_of::<sockaddr>() as libc::c_ulong as socklen_t;
    /* we don't actually care about the peer name, we're just checking if
     * we're connected or not */
    if getpeername(sock, &mut sa, &mut len) == 0 as libc::c_int {
        return 0 as libc::c_int
    }
    /* it's possible that the error wasn't ENOTCONN, so if it wasn't,
     * return that */
    if sock_error() != 107 as libc::c_int { return sock_error() }
    /* load the correct error into errno through error slippage */
    recv(sock, &mut temp as *mut libc::c_char as *mut libc::c_void,
         1 as libc::c_int as size_t, 0 as libc::c_int);
    return sock_error();
}
