use ::libc;
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD5Context {
    pub buf: [uint32_t; 4],
    pub bits: [uint32_t; 2],
    pub in_0: [libc::c_uchar; 64],
}
/*
 * Start MD5 accumulation.  Set bit count to 0 and buffer to mysterious
 * initialization constants.
 */
#[no_mangle]
pub unsafe extern "C" fn MD5Init(mut ctx: *mut MD5Context) {
    (*ctx).buf[0 as libc::c_int as usize] =
        0x67452301 as libc::c_int as uint32_t;
    (*ctx).buf[1 as libc::c_int as usize] = 0xefcdab89 as libc::c_uint;
    (*ctx).buf[2 as libc::c_int as usize] = 0x98badcfe as libc::c_uint;
    (*ctx).buf[3 as libc::c_int as usize] =
        0x10325476 as libc::c_int as uint32_t;
    (*ctx).bits[0 as libc::c_int as usize] = 0 as libc::c_int as uint32_t;
    (*ctx).bits[1 as libc::c_int as usize] = 0 as libc::c_int as uint32_t;
    memset((*ctx).in_0.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           64 as libc::c_int as libc::c_ulong);
}
/*
 * Update context to reflect the concatenation of another buffer full
 * of bytes.
 */
#[no_mangle]
pub unsafe extern "C" fn MD5Update(mut ctx: *mut MD5Context,
                                   mut buf: *const libc::c_uchar,
                                   mut len: uint32_t) {
    let mut t: uint32_t = 0;
    /* Update bitcount */
    t = (*ctx).bits[0 as libc::c_int as usize]; /* Carry from low to high */
    (*ctx).bits[0 as libc::c_int as usize] =
        t.wrapping_add(len << 3 as libc::c_int) &
            0xffffffff as libc::c_uint; /* Bytes already in shsInfo->data */
    if (*ctx).bits[0 as libc::c_int as usize] < t {
        (*ctx).bits[1 as libc::c_int as usize] =
            (*ctx).bits[1 as libc::c_int as usize].wrapping_add(1)
    }
    (*ctx).bits[1 as libc::c_int as usize] =
        ((*ctx).bits[1 as libc::c_int as usize] as
             libc::c_uint).wrapping_add(len >> 29 as libc::c_int) as uint32_t
            as uint32_t;
    t = t >> 3 as libc::c_int & 0x3f as libc::c_int as libc::c_uint;
    /* Handle any leading odd-sized chunks */
    if t != 0 {
        let mut p: *mut libc::c_uchar =
            (*ctx).in_0.as_mut_ptr().offset(t as isize);
        t = (64 as libc::c_int as libc::c_uint).wrapping_sub(t);
        if len < t {
            memcpy(p as *mut libc::c_void, buf as *const libc::c_void,
                   len as libc::c_ulong);
            return
        }
        memcpy(p as *mut libc::c_void, buf as *const libc::c_void,
               t as libc::c_ulong);
        MD5Transform((*ctx).buf.as_mut_ptr(),
                     (*ctx).in_0.as_mut_ptr() as *const libc::c_uchar);
        buf = buf.offset(t as isize);
        len = (len as libc::c_uint).wrapping_sub(t) as uint32_t as uint32_t
    }
    /* Process data in 64-byte chunks */
    while len >= 64 as libc::c_int as libc::c_uint {
        memcpy((*ctx).in_0.as_mut_ptr() as *mut libc::c_void,
               buf as *const libc::c_void,
               64 as libc::c_int as libc::c_ulong);
        MD5Transform((*ctx).buf.as_mut_ptr(),
                     (*ctx).in_0.as_mut_ptr() as *const libc::c_uchar);
        buf = buf.offset(64 as libc::c_int as isize);
        len =
            (len as
                 libc::c_uint).wrapping_sub(64 as libc::c_int as libc::c_uint)
                as uint32_t as uint32_t
    }
    /* Handle any remaining bytes of data. */
    memcpy((*ctx).in_0.as_mut_ptr() as *mut libc::c_void,
           buf as *const libc::c_void, len as libc::c_ulong);
}
/* md5.h
** interface to MD5 hash function
**
** This code is in the Public Domain.
*/
/* * @file
 *  MD5 hash API.
 */
/* make sure the stdint.h types are available */
/*
 * Final wrapup - pad to 64-byte boundary with the bit pattern 
 * 1 0* (64-bit count of bits processed, MSB-first)
 */
#[no_mangle]
pub unsafe extern "C" fn MD5Final(mut digest: *mut libc::c_uchar,
                                  mut ctx: *mut MD5Context) {
    let mut count: libc::c_uint = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    /* Compute number of bytes mod 64 */
    count =
        (*ctx).bits[0 as libc::c_int as usize] >> 3 as libc::c_int &
            0x3f as libc::c_int as libc::c_uint;
    /* Set the first char of padding to 0x80.  This is safe since there is
       always at least one byte free */
    p = (*ctx).in_0.as_mut_ptr().offset(count as isize);
    let fresh0 = p;
    p = p.offset(1);
    *fresh0 = 0x80 as libc::c_int as libc::c_uchar;
    /* Bytes of padding needed to make 64 bytes */
    count =
        ((64 as libc::c_int - 1 as libc::c_int) as
             libc::c_uint).wrapping_sub(count);
    /* Pad out to 56 mod 64 */
    if count < 8 as libc::c_int as libc::c_uint {
        /* Two lots of padding:  Pad the first block to 64 bytes */
        memset(p as *mut libc::c_void, 0 as libc::c_int,
               count as libc::c_ulong);
        MD5Transform((*ctx).buf.as_mut_ptr(),
                     (*ctx).in_0.as_mut_ptr() as *const libc::c_uchar);
        /* Now fill the next block with 56 bytes */
        memset((*ctx).in_0.as_mut_ptr() as *mut libc::c_void,
               0 as libc::c_int, 56 as libc::c_int as libc::c_ulong);
    } else {
        /* Pad block to 56 bytes */
        memset(p as *mut libc::c_void, 0 as libc::c_int,
               count.wrapping_sub(8 as libc::c_int as libc::c_uint) as
                   libc::c_ulong);
    }
    /* Append length in bits and transform */
    *(*ctx).in_0.as_mut_ptr().offset(56 as libc::c_int as
                                         isize).offset(0 as libc::c_int as
                                                           isize) =
        ((*ctx).bits[0 as libc::c_int as usize] &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *(*ctx).in_0.as_mut_ptr().offset(56 as libc::c_int as
                                         isize).offset(1 as libc::c_int as
                                                           isize) =
        ((*ctx).bits[0 as libc::c_int as usize] >> 8 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *(*ctx).in_0.as_mut_ptr().offset(56 as libc::c_int as
                                         isize).offset(2 as libc::c_int as
                                                           isize) =
        ((*ctx).bits[0 as libc::c_int as usize] >> 16 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *(*ctx).in_0.as_mut_ptr().offset(56 as libc::c_int as
                                         isize).offset(3 as libc::c_int as
                                                           isize) =
        ((*ctx).bits[0 as libc::c_int as usize] >> 24 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *(*ctx).in_0.as_mut_ptr().offset(60 as libc::c_int as
                                         isize).offset(0 as libc::c_int as
                                                           isize) =
        ((*ctx).bits[1 as libc::c_int as usize] &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *(*ctx).in_0.as_mut_ptr().offset(60 as libc::c_int as
                                         isize).offset(1 as libc::c_int as
                                                           isize) =
        ((*ctx).bits[1 as libc::c_int as usize] >> 8 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *(*ctx).in_0.as_mut_ptr().offset(60 as libc::c_int as
                                         isize).offset(2 as libc::c_int as
                                                           isize) =
        ((*ctx).bits[1 as libc::c_int as usize] >> 16 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *(*ctx).in_0.as_mut_ptr().offset(60 as libc::c_int as
                                         isize).offset(3 as libc::c_int as
                                                           isize) =
        ((*ctx).bits[1 as libc::c_int as usize] >> 24 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    MD5Transform((*ctx).buf.as_mut_ptr(),
                 (*ctx).in_0.as_mut_ptr() as *const libc::c_uchar);
    *digest.offset(0 as libc::c_int as isize) =
        ((*ctx).buf[0 as libc::c_int as usize] &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *digest.offset(1 as libc::c_int as isize) =
        ((*ctx).buf[0 as libc::c_int as usize] >> 8 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *digest.offset(2 as libc::c_int as isize) =
        ((*ctx).buf[0 as libc::c_int as usize] >> 16 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *digest.offset(3 as libc::c_int as isize) =
        ((*ctx).buf[0 as libc::c_int as usize] >> 24 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *digest.offset(4 as libc::c_int as
                       isize).offset(0 as libc::c_int as isize) =
        ((*ctx).buf[1 as libc::c_int as usize] &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *digest.offset(4 as libc::c_int as
                       isize).offset(1 as libc::c_int as isize) =
        ((*ctx).buf[1 as libc::c_int as usize] >> 8 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *digest.offset(4 as libc::c_int as
                       isize).offset(2 as libc::c_int as isize) =
        ((*ctx).buf[1 as libc::c_int as usize] >> 16 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *digest.offset(4 as libc::c_int as
                       isize).offset(3 as libc::c_int as isize) =
        ((*ctx).buf[1 as libc::c_int as usize] >> 24 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *digest.offset(8 as libc::c_int as
                       isize).offset(0 as libc::c_int as isize) =
        ((*ctx).buf[2 as libc::c_int as usize] &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *digest.offset(8 as libc::c_int as
                       isize).offset(1 as libc::c_int as isize) =
        ((*ctx).buf[2 as libc::c_int as usize] >> 8 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *digest.offset(8 as libc::c_int as
                       isize).offset(2 as libc::c_int as isize) =
        ((*ctx).buf[2 as libc::c_int as usize] >> 16 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *digest.offset(8 as libc::c_int as
                       isize).offset(3 as libc::c_int as isize) =
        ((*ctx).buf[2 as libc::c_int as usize] >> 24 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *digest.offset(12 as libc::c_int as
                       isize).offset(0 as libc::c_int as isize) =
        ((*ctx).buf[3 as libc::c_int as usize] &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *digest.offset(12 as libc::c_int as
                       isize).offset(1 as libc::c_int as isize) =
        ((*ctx).buf[3 as libc::c_int as usize] >> 8 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *digest.offset(12 as libc::c_int as
                       isize).offset(2 as libc::c_int as isize) =
        ((*ctx).buf[3 as libc::c_int as usize] >> 16 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *digest.offset(12 as libc::c_int as
                       isize).offset(3 as libc::c_int as isize) =
        ((*ctx).buf[3 as libc::c_int as usize] >> 24 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    memset(ctx as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<MD5Context>() as libc::c_ulong);
    /* In case it's sensitive */
}
/* md5.c
** MD5 hash function implemention, adapted for local use
**
** This code is in the Public Domain
*/
/*
 * This code implements the MD5 message-digest algorithm.
 * The algorithm is due to Ron Rivest.  This code was
 * written by Colin Plumb in 1993, no copyright is claimed.
 * This code is in the public domain; do with it what you wish.
 *
 * Equivalent code is available from RSA Data Security, Inc.
 * This code has been tested against that, and is equivalent,
 * except that you don't need to include two pages of legalese
 * with every copy.
 *
 * To compute the message digest of a chunk of bytes, declare an
 * MD5Context structure, pass it to MD5Init, call MD5Update as
 * needed on buffers full of bytes, and then call MD5Final, which
 * will fill a supplied 16-byte array with the digest.
 */
/* * @file
 *  MD5 hash.
 */
/* memcpy(), memset() */
/* little-endian word access macros */
/* The four core functions - F1 is optimized somewhat */
/* #define F1(x, y, z) (x & y | ~x & z) */
/* This is the central step in the MD5 algorithm. */
/* debugging version: */
/*
#define MD5STEP(f, w, x, y, z, data, s) \
        printf("MD5STEP:  w: %x x: %x y: %x z: %x data: %x s: %x\n", \
                w, x, y, z, data, s); \
        printf("f(x,y,z) = %x\n", f(x,y,z)+data); \
        ( w += f(x, y, z) + data,  printf(" - w: %x ", w), \
        w = w<<s | w>>(32-s),  printf(" - w: %x\n", w), w += x )
*/
/*
 * The core of the MD5 algorithm, this alters an existing MD5 hash to
 * reflect the addition of 16 longwords of new data.  MD5Update blocks
 * the data and converts bytes into longwords for this routine.
 */
unsafe extern "C" fn MD5Transform(mut buf: *mut uint32_t,
                                  mut inext: *const libc::c_uchar) {
    let mut a: uint32_t = 0;
    let mut b: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut d: uint32_t = 0;
    let mut i: uint32_t = 0;
    let mut in_0: [uint32_t; 16] = [0; 16];
    i = 0 as libc::c_int as uint32_t;
    while i < 16 as libc::c_int as libc::c_uint {
        in_0[i as usize] =
            *inext.offset((4 as libc::c_int as libc::c_uint).wrapping_mul(i)
                              as isize).offset(0 as libc::c_int as isize) as
                uint32_t |
                (*inext.offset((4 as libc::c_int as
                                    libc::c_uint).wrapping_mul(i) as
                                   isize).offset(1 as libc::c_int as isize) as
                     uint32_t) << 8 as libc::c_int |
                (*inext.offset((4 as libc::c_int as
                                    libc::c_uint).wrapping_mul(i) as
                                   isize).offset(2 as libc::c_int as isize) as
                     uint32_t) << 16 as libc::c_int |
                (*inext.offset((4 as libc::c_int as
                                    libc::c_uint).wrapping_mul(i) as
                                   isize).offset(3 as libc::c_int as isize) as
                     uint32_t) << 24 as libc::c_int;
        i = i.wrapping_add(1)
    }
    a = *buf.offset(0 as libc::c_int as isize);
    b = *buf.offset(1 as libc::c_int as isize);
    c = *buf.offset(2 as libc::c_int as isize);
    d = *buf.offset(3 as libc::c_int as isize);
    a =
        (a as
             libc::c_uint).wrapping_add((d ^
                                             b &
                                                 (c ^
                                                      d)).wrapping_add(in_0[0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0xd76aa478
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((c ^
                                             a &
                                                 (b ^
                                                      c)).wrapping_add(in_0[1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0xe8c7b756
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((b ^
                                             d &
                                                 (a ^
                                                      b)).wrapping_add(in_0[2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0x242070db
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((a ^
                                             c &
                                                 (d ^
                                                      a)).wrapping_add(in_0[3
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0xc1bdceee
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((d ^
                                             b &
                                                 (c ^
                                                      d)).wrapping_add(in_0[4
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0xf57c0faf
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((c ^
                                             a &
                                                 (b ^
                                                      c)).wrapping_add(in_0[5
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0x4787c62a
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((b ^
                                             d &
                                                 (a ^
                                                      b)).wrapping_add(in_0[6
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0xa8304613
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((a ^
                                             c &
                                                 (d ^
                                                      a)).wrapping_add(in_0[7
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0xfd469501
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((d ^
                                             b &
                                                 (c ^
                                                      d)).wrapping_add(in_0[8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0x698098d8
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((c ^
                                             a &
                                                 (b ^
                                                      c)).wrapping_add(in_0[9
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0x8b44f7af
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((b ^
                                             d &
                                                 (a ^
                                                      b)).wrapping_add(in_0[10
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0xffff5bb1
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((a ^
                                             c &
                                                 (d ^
                                                      a)).wrapping_add(in_0[11
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0x895cd7be
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((d ^
                                             b &
                                                 (c ^
                                                      d)).wrapping_add(in_0[12
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0x6b901122
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((c ^
                                             a &
                                                 (b ^
                                                      c)).wrapping_add(in_0[13
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0xfd987193
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((b ^
                                             d &
                                                 (a ^
                                                      b)).wrapping_add(in_0[14
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0xa679438e
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((a ^
                                             c &
                                                 (d ^
                                                      a)).wrapping_add(in_0[15
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0x49b40821
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             d &
                                                 (b ^
                                                      c)).wrapping_add(in_0[1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0xf61e2562
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             c &
                                                 (a ^
                                                      b)).wrapping_add(in_0[6
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0xc040b340
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             b &
                                                 (d ^
                                                      a)).wrapping_add(in_0[11
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0x265e5a51
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             a &
                                                 (c ^
                                                      d)).wrapping_add(in_0[0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0xe9b6c7aa
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             d &
                                                 (b ^
                                                      c)).wrapping_add(in_0[5
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0xd62f105d
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             c &
                                                 (a ^
                                                      b)).wrapping_add(in_0[10
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0x2441453
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             b &
                                                 (d ^
                                                      a)).wrapping_add(in_0[15
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0xd8a1e681
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             a &
                                                 (c ^
                                                      d)).wrapping_add(in_0[4
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0xe7d3fbc8
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             d &
                                                 (b ^
                                                      c)).wrapping_add(in_0[9
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0x21e1cde6
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             c &
                                                 (a ^
                                                      b)).wrapping_add(in_0[14
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0xc33707d6
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             b &
                                                 (d ^
                                                      a)).wrapping_add(in_0[3
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0xf4d50d87
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             a &
                                                 (c ^
                                                      d)).wrapping_add(in_0[8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0x455a14ed
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             d &
                                                 (b ^
                                                      c)).wrapping_add(in_0[13
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0xa9e3e905
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             c &
                                                 (a ^
                                                      b)).wrapping_add(in_0[2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0xfcefa3f8
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             b &
                                                 (d ^
                                                      a)).wrapping_add(in_0[7
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0x676f02d9
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             a &
                                                 (c ^
                                                      d)).wrapping_add(in_0[12
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]).wrapping_add(0x8d2a4c8a
                                                                                                         as
                                                                                                         libc::c_uint))
            as uint32_t as uint32_t;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((b ^ c ^
                                             d).wrapping_add(in_0[5 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]).wrapping_add(0xfffa3942
                                                                                               as
                                                                                               libc::c_uint))
            as uint32_t as uint32_t;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((a ^ b ^
                                             c).wrapping_add(in_0[8 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]).wrapping_add(0x8771f681
                                                                                               as
                                                                                               libc::c_uint))
            as uint32_t as uint32_t;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((d ^ a ^
                                             b).wrapping_add(in_0[11 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]).wrapping_add(0x6d9d6122
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_uint))
            as uint32_t as uint32_t;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((c ^ d ^
                                             a).wrapping_add(in_0[14 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]).wrapping_add(0xfde5380c
                                                                                               as
                                                                                               libc::c_uint))
            as uint32_t as uint32_t;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((b ^ c ^
                                             d).wrapping_add(in_0[1 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]).wrapping_add(0xa4beea44
                                                                                               as
                                                                                               libc::c_uint))
            as uint32_t as uint32_t;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((a ^ b ^
                                             c).wrapping_add(in_0[4 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]).wrapping_add(0x4bdecfa9
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_uint))
            as uint32_t as uint32_t;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((d ^ a ^
                                             b).wrapping_add(in_0[7 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]).wrapping_add(0xf6bb4b60
                                                                                               as
                                                                                               libc::c_uint))
            as uint32_t as uint32_t;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((c ^ d ^
                                             a).wrapping_add(in_0[10 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]).wrapping_add(0xbebfbc70
                                                                                               as
                                                                                               libc::c_uint))
            as uint32_t as uint32_t;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((b ^ c ^
                                             d).wrapping_add(in_0[13 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]).wrapping_add(0x289b7ec6
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_uint))
            as uint32_t as uint32_t;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((a ^ b ^
                                             c).wrapping_add(in_0[0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]).wrapping_add(0xeaa127fa
                                                                                               as
                                                                                               libc::c_uint))
            as uint32_t as uint32_t;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((d ^ a ^
                                             b).wrapping_add(in_0[3 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]).wrapping_add(0xd4ef3085
                                                                                               as
                                                                                               libc::c_uint))
            as uint32_t as uint32_t;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((c ^ d ^
                                             a).wrapping_add(in_0[6 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]).wrapping_add(0x4881d05
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_uint))
            as uint32_t as uint32_t;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((b ^ c ^
                                             d).wrapping_add(in_0[9 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]).wrapping_add(0xd9d4d039
                                                                                               as
                                                                                               libc::c_uint))
            as uint32_t as uint32_t;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((a ^ b ^
                                             c).wrapping_add(in_0[12 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]).wrapping_add(0xe6db99e5
                                                                                               as
                                                                                               libc::c_uint))
            as uint32_t as uint32_t;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((d ^ a ^
                                             b).wrapping_add(in_0[15 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]).wrapping_add(0x1fa27cf8
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_uint))
            as uint32_t as uint32_t;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((c ^ d ^
                                             a).wrapping_add(in_0[2 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]).wrapping_add(0xc4ac5665
                                                                                               as
                                                                                               libc::c_uint))
            as uint32_t as uint32_t;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             (b |
                                                  !d)).wrapping_add(in_0[0 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]).wrapping_add(0xf4292244
                                                                                                      as
                                                                                                      libc::c_uint))
            as uint32_t as uint32_t;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             (a |
                                                  !c)).wrapping_add(in_0[7 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]).wrapping_add(0x432aff97
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_uint))
            as uint32_t as uint32_t;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             (d |
                                                  !b)).wrapping_add(in_0[14 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]).wrapping_add(0xab9423a7
                                                                                                      as
                                                                                                      libc::c_uint))
            as uint32_t as uint32_t;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             (c |
                                                  !a)).wrapping_add(in_0[5 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]).wrapping_add(0xfc93a039
                                                                                                      as
                                                                                                      libc::c_uint))
            as uint32_t as uint32_t;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             (b |
                                                  !d)).wrapping_add(in_0[12 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]).wrapping_add(0x655b59c3
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_uint))
            as uint32_t as uint32_t;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             (a |
                                                  !c)).wrapping_add(in_0[3 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]).wrapping_add(0x8f0ccc92
                                                                                                      as
                                                                                                      libc::c_uint))
            as uint32_t as uint32_t;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             (d |
                                                  !b)).wrapping_add(in_0[10 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]).wrapping_add(0xffeff47d
                                                                                                      as
                                                                                                      libc::c_uint))
            as uint32_t as uint32_t;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             (c |
                                                  !a)).wrapping_add(in_0[1 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]).wrapping_add(0x85845dd1
                                                                                                      as
                                                                                                      libc::c_uint))
            as uint32_t as uint32_t;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             (b |
                                                  !d)).wrapping_add(in_0[8 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]).wrapping_add(0x6fa87e4f
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_uint))
            as uint32_t as uint32_t;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             (a |
                                                  !c)).wrapping_add(in_0[15 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]).wrapping_add(0xfe2ce6e0
                                                                                                      as
                                                                                                      libc::c_uint))
            as uint32_t as uint32_t;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             (d |
                                                  !b)).wrapping_add(in_0[6 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]).wrapping_add(0xa3014314
                                                                                                      as
                                                                                                      libc::c_uint))
            as uint32_t as uint32_t;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             (c |
                                                  !a)).wrapping_add(in_0[13 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]).wrapping_add(0x4e0811a1
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_uint))
            as uint32_t as uint32_t;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             (b |
                                                  !d)).wrapping_add(in_0[4 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]).wrapping_add(0xf7537e82
                                                                                                      as
                                                                                                      libc::c_uint))
            as uint32_t as uint32_t;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             (a |
                                                  !c)).wrapping_add(in_0[11 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]).wrapping_add(0xbd3af235
                                                                                                      as
                                                                                                      libc::c_uint))
            as uint32_t as uint32_t;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             (d |
                                                  !b)).wrapping_add(in_0[2 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]).wrapping_add(0x2ad7d2bb
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_uint))
            as uint32_t as uint32_t;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             (c |
                                                  !a)).wrapping_add(in_0[9 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]).wrapping_add(0xeb86d391
                                                                                                      as
                                                                                                      libc::c_uint))
            as uint32_t as uint32_t;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    let ref mut fresh1 = *buf.offset(0 as libc::c_int as isize);
    *fresh1 =
        (*fresh1 as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    let ref mut fresh2 = *buf.offset(1 as libc::c_int as isize);
    *fresh2 =
        (*fresh2 as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    let ref mut fresh3 = *buf.offset(2 as libc::c_int as isize);
    *fresh3 =
        (*fresh3 as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    let ref mut fresh4 = *buf.offset(3 as libc::c_int as isize);
    *fresh4 =
        (*fresh4 as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
}
