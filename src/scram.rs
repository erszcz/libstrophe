use ::libc;
extern "C" {
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /* public api for steve reid's public domain SHA-1 implementation */
/* this file is in the public domain */
    /* * @file
 *  SHA-1 hash API.
 */
    /* make sure the stdint.h types are available */
    #[no_mangle]
    fn crypto_SHA1(data: *const uint8_t, len: size_t, digest: *mut uint8_t);
    #[no_mangle]
    fn crypto_SHA1_Final(context: *mut SHA1_CTX, digest: *mut uint8_t);
    #[no_mangle]
    fn crypto_SHA1_Update(context: *mut SHA1_CTX, data: *const uint8_t,
                          len: size_t);
    #[no_mangle]
    fn crypto_SHA1_Init(context: *mut SHA1_CTX);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA1_CTX {
    pub state: [uint32_t; 5],
    pub count: [uint32_t; 2],
    pub buffer: [uint8_t; 64],
}
static mut ipad: uint8_t = 0x36 as libc::c_int as uint8_t;
static mut opad: uint8_t = 0x5c as libc::c_int as uint8_t;
unsafe extern "C" fn crypto_HMAC_SHA1(mut key: *const uint8_t,
                                      mut key_len: size_t,
                                      mut text: *const uint8_t,
                                      mut len: size_t,
                                      mut digest: *mut uint8_t) {
    let mut key_pad: [uint8_t; 64] = [0; 64];
    let mut key_ipad: [uint8_t; 64] = [0; 64];
    let mut key_opad: [uint8_t; 64] = [0; 64];
    let mut sha_digest: [uint8_t; 20] = [0; 20];
    let mut i: libc::c_int = 0;
    let mut ctx: SHA1_CTX =
        SHA1_CTX{state: [0; 5], count: [0; 2], buffer: [0; 64],};
    memset(key_pad.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong);
    if key_len <= 64 as libc::c_int as libc::c_ulong {
        memcpy(key_pad.as_mut_ptr() as *mut libc::c_void,
               key as *const libc::c_void, key_len);
    } else {
        /* according to RFC2104 */
        crypto_SHA1(key, key_len, key_pad.as_mut_ptr());
    }
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        key_ipad[i as usize] =
            (key_pad[i as usize] as libc::c_int ^ ipad as libc::c_int) as
                uint8_t;
        key_opad[i as usize] =
            (key_pad[i as usize] as libc::c_int ^ opad as libc::c_int) as
                uint8_t;
        i += 1
    }
    crypto_SHA1_Init(&mut ctx);
    crypto_SHA1_Update(&mut ctx, key_ipad.as_mut_ptr(),
                       64 as libc::c_int as size_t);
    crypto_SHA1_Update(&mut ctx, text, len);
    crypto_SHA1_Final(&mut ctx, sha_digest.as_mut_ptr());
    crypto_SHA1_Init(&mut ctx);
    crypto_SHA1_Update(&mut ctx, key_opad.as_mut_ptr(),
                       64 as libc::c_int as size_t);
    crypto_SHA1_Update(&mut ctx, sha_digest.as_mut_ptr(),
                       20 as libc::c_int as size_t);
    crypto_SHA1_Final(&mut ctx, digest);
}
unsafe extern "C" fn SCRAM_SHA1_Hi(mut text: *const uint8_t, mut len: size_t,
                                   mut salt: *const uint8_t,
                                   mut salt_len: size_t, mut i: uint32_t,
                                   mut digest: *mut uint8_t) {
    let mut k: libc::c_int = 0;
    let mut j: uint32_t = 0;
    let mut tmp: [uint8_t; 128] = [0; 128];
    static mut int1: [uint8_t; 4] =
        [0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
         0 as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t];
    /* assume salt + INT(1) isn't longer than sizeof(tmp) */
    if salt_len <=
           (::std::mem::size_of::<[uint8_t; 128]>() as
                libc::c_ulong).wrapping_sub(::std::mem::size_of::<[uint8_t; 4]>()
                                                as libc::c_ulong) {
    } else {
        __assert_fail(b"salt_len <= sizeof(tmp) - sizeof(int1)\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/scram.c\x00" as *const u8 as *const libc::c_char,
                      78 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 90],
                                                &[libc::c_char; 90]>(b"void SCRAM_SHA1_Hi(const uint8_t *, size_t, const uint8_t *, size_t, uint32_t, uint8_t *)\x00")).as_ptr());
    }
    memset(digest as *mut libc::c_void, 0 as libc::c_int,
           20 as libc::c_int as libc::c_ulong);
    if i == 0 as libc::c_int as libc::c_uint { return }
    memcpy(tmp.as_mut_ptr() as *mut libc::c_void, salt as *const libc::c_void,
           salt_len);
    memcpy(&mut *tmp.as_mut_ptr().offset(salt_len as isize) as *mut uint8_t as
               *mut libc::c_void, int1.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong);
    /* 'text' for Hi is a 'key' for HMAC */
    crypto_HMAC_SHA1(text, len, tmp.as_mut_ptr(),
                     salt_len.wrapping_add(::std::mem::size_of::<[uint8_t; 4]>()
                                               as libc::c_ulong), digest);
    memcpy(tmp.as_mut_ptr() as *mut libc::c_void,
           digest as *const libc::c_void, 20 as libc::c_int as libc::c_ulong);
    j = 1 as libc::c_int as uint32_t;
    while j < i {
        crypto_HMAC_SHA1(text, len, tmp.as_mut_ptr(),
                         20 as libc::c_int as size_t, tmp.as_mut_ptr());
        k = 0 as libc::c_int;
        while k < 20 as libc::c_int {
            let ref mut fresh0 = *digest.offset(k as isize);
            *fresh0 =
                (*fresh0 as libc::c_int ^ tmp[k as usize] as libc::c_int) as
                    uint8_t;
            k += 1
        }
        j = j.wrapping_add(1)
    };
}
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
pub unsafe extern "C" fn SCRAM_SHA1_ClientKey(mut password: *const uint8_t,
                                              mut len: size_t,
                                              mut salt: *const uint8_t,
                                              mut salt_len: size_t,
                                              mut i: uint32_t,
                                              mut key: *mut uint8_t) {
    let mut salted: [uint8_t; 20] = [0; 20];
    /* XXX: Normalize(password) is omitted */
    SCRAM_SHA1_Hi(password, len, salt, salt_len, i, salted.as_mut_ptr());
    crypto_HMAC_SHA1(salted.as_mut_ptr(), 20 as libc::c_int as size_t,
                     b"Client Key\x00" as *const u8 as *const libc::c_char as
                         *mut uint8_t,
                     strlen(b"Client Key\x00" as *const u8 as
                                *const libc::c_char), key);
}
#[no_mangle]
pub unsafe extern "C" fn SCRAM_SHA1_ClientSignature(mut ClientKey:
                                                        *const uint8_t,
                                                    mut AuthMessage:
                                                        *const uint8_t,
                                                    mut len: size_t,
                                                    mut sign: *mut uint8_t) {
    let mut stored: [uint8_t; 20] = [0; 20];
    crypto_SHA1(ClientKey, 20 as libc::c_int as size_t, stored.as_mut_ptr());
    crypto_HMAC_SHA1(stored.as_mut_ptr(), 20 as libc::c_int as size_t,
                     AuthMessage, len, sign);
}
#[no_mangle]
pub unsafe extern "C" fn SCRAM_SHA1_ClientProof(mut ClientKey: *const uint8_t,
                                                mut ClientSignature:
                                                    *const uint8_t,
                                                mut proof: *mut uint8_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        *proof.offset(i as isize) =
            (*ClientKey.offset(i as isize) as libc::c_int ^
                 *ClientSignature.offset(i as isize) as libc::c_int) as
                uint8_t;
        i += 1
    };
}
