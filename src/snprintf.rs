use ::libc;
extern "C" {
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
pub type va_list = __builtin_va_list;
pub const _ISdigit: C2RustUnnamed = 2048;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
/*
 * Copyright Patrick Powell 1995
 * This code is based on code written by Patrick Powell (papowell@astart.com)
 * It may be used for any purpose as long as this notice remains intact
 * on all source code distributions
 */
/* *************************************************************
 * Original:
 * Patrick Powell Tue Apr 11 09:48:21 PDT 1995
 * A bombproof version of doprnt (dopr) included.
 * Sigh.  This sort of thing is always nasty do deal with.  Note that
 * the version here does not include floating point...
 *
 * snprintf() is used instead of sprintf() as it does limit checks
 * for string length.  This covers a nasty loophole.
 *
 * The other functions are there to prevent NULL pointers from
 * causing nast effects.
 *
 * More Recently:
 *  Brandon Long <blong@fiction.net> 9/15/96 for mutt 0.43
 *  This was ugly.  It is still ugly.  I opted out of floating point
 *  numbers, but the formatter understands just about everything
 *  from the normal C string format, at least as far as I can tell from
 *  the Solaris 2.5 printf(3S) man page.
 *
 *  Brandon Long <blong@fiction.net> 10/22/97 for mutt 0.87.1
 *    Ok, added some minimal floating point support, which means this
 *    probably requires libm on most operating systems.  Don't yet
 *    support the exponent (e,E) and sigfig (g,G).  Also, fmtint()
 *    was pretty badly broken, it just wasn't being exercised in ways
 *    which showed it, so that's been fixed.  Also, formated the code
 *    to mutt conventions, and removed dead code left over from the
 *    original.  Also, there is now a builtin-test, just compile with:
 *           gcc -DTEST_SNPRINTF -o snprintf snprintf.c -lm
 *    and run snprintf for results.
 * 
 *  Thomas Roessler <roessler@guug.de> 01/27/98 for mutt 0.89i
 *    The PGP code was using unsigned hexadecimal formats. 
 *    Unfortunately, unsigned formats simply didn't work.
 *
 *  Michael Elkins <me@cs.hmc.edu> 03/05/98 for mutt 0.90.8
 *    The original code assumed that both snprintf() and vsnprintf() were
 *    missing.  Some systems only have snprintf() but not vsnprintf(), so
 *    the code is now broken down under HAVE_SNPRINTF and HAVE_VSNPRINTF.
 *
 *  Andrew Tridgell (tridge@samba.org) Oct 1998
 *    fixed handling of %.0f
 *    added test for HAVE_LONG_DOUBLE
 *
 *  Russ Allbery <rra@stanford.edu> 2000-08-26
 *    fixed return value to comply with C99
 *    fixed handling of snprintf(NULL, ...)
 *
 **************************************************************/
/* * @file
 *  A snprintf implementation.
 */
/* JAM: we don't need this - #include "config.h" */
/* JAM: changed declarations to xmpp_snprintf and xmpp_vsnprintf to
   avoid namespace collision. */
/* varargs declarations: */
unsafe extern "C" fn dopr(mut buffer: *mut libc::c_char, mut maxlen: size_t,
                          mut format: *const libc::c_char,
                          mut args: ::std::ffi::VaList) -> libc::c_int {
    let mut ch: libc::c_char = 0;
    let mut value: libc::c_long = 0;
    let mut fvalue: libc::c_double = 0.;
    let mut strvalue: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut min: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut state: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut cflags: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    let mut currlen: size_t = 0;
    state = 0 as libc::c_int;
    min = 0 as libc::c_int;
    cflags = min;
    flags = cflags;
    currlen = flags as size_t;
    max = -(1 as libc::c_int);
    let fresh0 = format;
    format = format.offset(1);
    ch = *fresh0;
    total = 0 as libc::c_int;
    while state != 7 as libc::c_int {
        if ch as libc::c_int == '\u{0}' as i32 { state = 7 as libc::c_int }
        match state {
            0 => {
                if ch as libc::c_int == '%' as i32 {
                    state = 1 as libc::c_int
                } else {
                    total += dopr_outch(buffer, &mut currlen, maxlen, ch)
                }
                let fresh1 = format;
                format = format.offset(1);
                ch = *fresh1
                /* some picky compilers need this */
            }
            1 => {
                match ch as libc::c_int {
                    45 => {
                        flags |= (1 as libc::c_int) << 0 as libc::c_int;
                        let fresh2 = format;
                        format = format.offset(1);
                        ch = *fresh2
                    }
                    43 => {
                        flags |= (1 as libc::c_int) << 1 as libc::c_int;
                        let fresh3 = format;
                        format = format.offset(1);
                        ch = *fresh3
                    }
                    32 => {
                        flags |= (1 as libc::c_int) << 2 as libc::c_int;
                        let fresh4 = format;
                        format = format.offset(1);
                        ch = *fresh4
                    }
                    35 => {
                        flags |= (1 as libc::c_int) << 3 as libc::c_int;
                        let fresh5 = format;
                        format = format.offset(1);
                        ch = *fresh5
                    }
                    48 => {
                        flags |= (1 as libc::c_int) << 4 as libc::c_int;
                        let fresh6 = format;
                        format = format.offset(1);
                        ch = *fresh6
                    }
                    _ => { state = 2 as libc::c_int }
                }
            }
            2 => {
                if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as
                       libc::c_int &
                       _ISdigit as libc::c_int as libc::c_ushort as
                           libc::c_int != 0 {
                    min =
                        10 as libc::c_int * min +
                            (ch as libc::c_int - '0' as i32);
                    let fresh7 = format;
                    format = format.offset(1);
                    ch = *fresh7
                } else if ch as libc::c_int == '*' as i32 {
                    min = args.as_va_list().arg::<libc::c_int>();
                    let fresh8 = format;
                    format = format.offset(1);
                    ch = *fresh8;
                    state = 3 as libc::c_int
                } else { state = 3 as libc::c_int }
            }
            3 => {
                if ch as libc::c_int == '.' as i32 {
                    state = 4 as libc::c_int;
                    let fresh9 = format;
                    format = format.offset(1);
                    ch = *fresh9
                } else { state = 5 as libc::c_int }
            }
            4 => {
                if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as
                       libc::c_int &
                       _ISdigit as libc::c_int as libc::c_ushort as
                           libc::c_int != 0 {
                    if max < 0 as libc::c_int { max = 0 as libc::c_int }
                    max =
                        10 as libc::c_int * max +
                            (ch as libc::c_int - '0' as i32);
                    let fresh10 = format;
                    format = format.offset(1);
                    ch = *fresh10
                } else if ch as libc::c_int == '*' as i32 {
                    max = args.as_va_list().arg::<libc::c_int>();
                    let fresh11 = format;
                    format = format.offset(1);
                    ch = *fresh11;
                    state = 5 as libc::c_int
                } else { state = 5 as libc::c_int }
            }
            5 => {
                /* Currently, we don't support Long Long, bummer */
                match ch as libc::c_int {
                    104 => {
                        cflags = 1 as libc::c_int;
                        let fresh12 = format;
                        format = format.offset(1);
                        ch = *fresh12
                    }
                    108 => {
                        cflags = 2 as libc::c_int;
                        let fresh13 = format;
                        format = format.offset(1);
                        ch = *fresh13
                    }
                    76 => {
                        cflags = 3 as libc::c_int;
                        let fresh14 = format;
                        format = format.offset(1);
                        ch = *fresh14
                    }
                    _ => { }
                }
                state = 6 as libc::c_int
            }
            6 => {
                let mut current_block_115: u64;
                match ch as libc::c_int {
                    100 | 105 => {
                        if cflags == 1 as libc::c_int {
                            value =
                                args.as_va_list().arg::<libc::c_int>() as
                                    libc::c_long
                        } else if cflags == 2 as libc::c_int {
                            value = args.as_va_list().arg::<libc::c_long>()
                        } else {
                            value =
                                args.as_va_list().arg::<libc::c_int>() as
                                    libc::c_long
                        }
                        total +=
                            fmtint(buffer, &mut currlen, maxlen, value,
                                   10 as libc::c_int, min, max, flags);
                        current_block_115 = 16835199615365683821;
                    }
                    111 => {
                        flags |= (1 as libc::c_int) << 6 as libc::c_int;
                        if cflags == 1 as libc::c_int {
                            value =
                                args.as_va_list().arg::<libc::c_int>() as
                                    libc::c_long
                        } else if cflags == 2 as libc::c_int {
                            value =
                                args.as_va_list().arg::<libc::c_ulong>() as
                                    libc::c_long
                        } else {
                            value =
                                args.as_va_list().arg::<libc::c_uint>() as
                                    libc::c_long
                        }
                        total +=
                            fmtint(buffer, &mut currlen, maxlen, value,
                                   8 as libc::c_int, min, max, flags);
                        current_block_115 = 16835199615365683821;
                    }
                    117 => {
                        flags |= (1 as libc::c_int) << 6 as libc::c_int;
                        if cflags == 1 as libc::c_int {
                            value =
                                args.as_va_list().arg::<libc::c_int>() as
                                    libc::c_long
                        } else if cflags == 2 as libc::c_int {
                            value =
                                args.as_va_list().arg::<libc::c_ulong>() as
                                    libc::c_long
                        } else {
                            value =
                                args.as_va_list().arg::<libc::c_uint>() as
                                    libc::c_long
                        }
                        total +=
                            fmtint(buffer, &mut currlen, maxlen, value,
                                   10 as libc::c_int, min, max, flags);
                        current_block_115 = 16835199615365683821;
                    }
                    88 => {
                        flags |= (1 as libc::c_int) << 5 as libc::c_int;
                        current_block_115 = 11124238690025162493;
                    }
                    120 => { current_block_115 = 11124238690025162493; }
                    102 => {
                        if cflags == 3 as libc::c_int {
                            fvalue = args.as_va_list().arg::<libc::c_double>()
                        } else {
                            fvalue = args.as_va_list().arg::<libc::c_double>()
                        }
                        /* um, floating point? */
                        total +=
                            fmtfp(buffer, &mut currlen, maxlen, fvalue, min,
                                  max, flags);
                        current_block_115 = 16835199615365683821;
                    }
                    69 => {
                        flags |= (1 as libc::c_int) << 5 as libc::c_int;
                        current_block_115 = 3456962821664424094;
                    }
                    101 => { current_block_115 = 3456962821664424094; }
                    71 => {
                        flags |= (1 as libc::c_int) << 5 as libc::c_int;
                        current_block_115 = 1095268521503733605;
                    }
                    103 => { current_block_115 = 1095268521503733605; }
                    99 => {
                        total +=
                            dopr_outch(buffer, &mut currlen, maxlen,
                                       args.as_va_list().arg::<libc::c_int>()
                                           as libc::c_char);
                        current_block_115 = 16835199615365683821;
                    }
                    115 => {
                        strvalue =
                            args.as_va_list().arg::<*mut libc::c_char>();
                        total +=
                            fmtstr(buffer, &mut currlen, maxlen, strvalue,
                                   flags, min, max);
                        current_block_115 = 16835199615365683821;
                    }
                    112 => {
                        strvalue =
                            args.as_va_list().arg::<*mut libc::c_void>() as
                                *mut libc::c_char;
                        total +=
                            fmtint(buffer, &mut currlen, maxlen,
                                   strvalue as libc::c_long,
                                   16 as libc::c_int, min, max, flags);
                        current_block_115 = 16835199615365683821;
                    }
                    110 => {
                        if cflags == 1 as libc::c_int {
                            let mut num: *mut libc::c_short =
                                0 as *mut libc::c_short;
                            num =
                                args.as_va_list().arg::<*mut libc::c_short>();
                            *num = currlen as libc::c_short
                        } else if cflags == 2 as libc::c_int {
                            let mut num_0: *mut libc::c_long =
                                0 as *mut libc::c_long;
                            num_0 =
                                args.as_va_list().arg::<*mut libc::c_long>();
                            *num_0 = currlen as libc::c_long
                        } else {
                            let mut num_1: *mut libc::c_int =
                                0 as *mut libc::c_int;
                            num_1 =
                                args.as_va_list().arg::<*mut libc::c_int>();
                            *num_1 = currlen as libc::c_int
                        }
                        current_block_115 = 16835199615365683821;
                    }
                    37 => {
                        total += dopr_outch(buffer, &mut currlen, maxlen, ch);
                        current_block_115 = 16835199615365683821;
                    }
                    119 => {
                        /* not supported yet, treat as next char */
                        let fresh15 = format;
                        format = format.offset(1);
                        ch = *fresh15;
                        current_block_115 = 16835199615365683821;
                    }
                    _ => { current_block_115 = 16835199615365683821; }
                }
                match current_block_115 {
                    11124238690025162493 =>
                    //-fallthrough
                    {
                        flags |= (1 as libc::c_int) << 6 as libc::c_int;
                        if cflags == 1 as libc::c_int {
                            value =
                                args.as_va_list().arg::<libc::c_int>() as
                                    libc::c_long
                        } else if cflags == 2 as libc::c_int {
                            value =
                                args.as_va_list().arg::<libc::c_ulong>() as
                                    libc::c_long
                        } else {
                            value =
                                args.as_va_list().arg::<libc::c_uint>() as
                                    libc::c_long
                        }
                        total +=
                            fmtint(buffer, &mut currlen, maxlen, value,
                                   16 as libc::c_int, min, max, flags)
                    }
                    3456962821664424094 =>
                    //-fallthrough
                    {
                        if cflags == 3 as libc::c_int {
                            fvalue = args.as_va_list().arg::<libc::c_double>()
                        } else {
                            fvalue = args.as_va_list().arg::<libc::c_double>()
                        }
                    }
                    1095268521503733605 =>
                    //-fallthrough
                    {
                        if cflags == 3 as libc::c_int {
                            fvalue = args.as_va_list().arg::<libc::c_double>()
                        } else {
                            fvalue = args.as_va_list().arg::<libc::c_double>()
                        }
                    }
                    _ => { }
                } /* amount to pad */
                let fresh16 = format; /* strlen */
                format = format.offset(1); /* Left Justify */
                ch = *fresh16;
                state = 0 as libc::c_int;
                min = 0 as libc::c_int;
                cflags = min;
                flags = cflags;
                max = -(1 as libc::c_int)
            }
            7 | _ => { }
        }
    }
    if !buffer.is_null() && maxlen > 0 as libc::c_int as libc::c_ulong {
        if currlen < maxlen.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            *buffer.offset(currlen as isize) = '\u{0}' as i32 as libc::c_char
        } else {
            *buffer.offset(maxlen.wrapping_sub(1 as libc::c_int as
                                                   libc::c_ulong) as isize) =
                '\u{0}' as i32 as libc::c_char
        }
    }
    return total;
}
unsafe extern "C" fn fmtstr(mut buffer: *mut libc::c_char,
                            mut currlen: *mut size_t, mut maxlen: size_t,
                            mut value: *mut libc::c_char,
                            mut flags: libc::c_int, mut min: libc::c_int,
                            mut max: libc::c_int) -> libc::c_int {
    let mut padlen: libc::c_int = 0;
    let mut strln: libc::c_int = 0;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut total: libc::c_int = 0 as libc::c_int;
    if value.is_null() {
        value =
            b"<NULL>\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    }
    strln = 0 as libc::c_int;
    while *value.offset(strln as isize) != 0 { strln += 1 }
    if max >= 0 as libc::c_int && max < strln { strln = max }
    padlen = min - strln;
    if padlen < 0 as libc::c_int { padlen = 0 as libc::c_int }
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        padlen = -padlen
    }
    while padlen > 0 as libc::c_int {
        total +=
            dopr_outch(buffer, currlen, maxlen, ' ' as i32 as libc::c_char);
        padlen -= 1
    }
    while *value as libc::c_int != 0 && (max < 0 as libc::c_int || cnt < max)
          {
        let fresh17 = value;
        value = value.offset(1);
        total += dopr_outch(buffer, currlen, maxlen, *fresh17);
        cnt += 1
    }
    while padlen < 0 as libc::c_int {
        total +=
            dopr_outch(buffer, currlen, maxlen, ' ' as i32 as libc::c_char);
        padlen += 1
    }
    return total;
}
/* Have to handle DP_F_NUM (ie 0x and 0 alternates) */
unsafe extern "C" fn fmtint(mut buffer: *mut libc::c_char,
                            mut currlen: *mut size_t, mut maxlen: size_t,
                            mut value: libc::c_long, mut base: libc::c_int,
                            mut min: libc::c_int, mut max: libc::c_int,
                            mut flags: libc::c_int) -> libc::c_int {
    let mut signvalue: libc::c_int =
        0 as libc::c_int; /* amount to space pad */
    let mut uvalue: libc::c_ulong = 0; /* amount to zero pad */
    let mut convert: [libc::c_char; 20] = [0; 20];
    let mut place: libc::c_int = 0 as libc::c_int;
    let mut spadlen: libc::c_int = 0 as libc::c_int;
    let mut zpadlen: libc::c_int = 0 as libc::c_int;
    let mut caps: libc::c_int = 0 as libc::c_int;
    let mut total: libc::c_int = 0 as libc::c_int;
    if max < 0 as libc::c_int { max = 0 as libc::c_int }
    uvalue = value as libc::c_ulong;
    if flags & (1 as libc::c_int) << 6 as libc::c_int == 0 {
        if value < 0 as libc::c_int as libc::c_long {
            signvalue = '-' as i32;
            uvalue = -value as libc::c_ulong
        } else if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            /* Do a sign (+/i) */
            signvalue = '+' as i32
        } else if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            signvalue = ' ' as i32
        }
    } /* Should characters be upper case? */
    if flags & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        caps = 1 as libc::c_int
    } /* Left Justifty */
    loop  {
        let fresh18 = place;
        place = place + 1;
        convert[fresh18 as usize] =
            *if caps != 0 {
                 b"0123456789ABCDEF\x00" as *const u8 as *const libc::c_char
             } else {
                 b"0123456789abcdef\x00" as *const u8 as *const libc::c_char
             }.offset(uvalue.wrapping_rem(base as libc::c_uint as
                                              libc::c_ulong) as isize);
        uvalue = uvalue.wrapping_div(base as libc::c_uint as libc::c_ulong);
        if !(uvalue != 0 && place < 20 as libc::c_int) { break ; }
    }
    if place == 20 as libc::c_int { place -= 1 }
    convert[place as usize] = 0 as libc::c_int as libc::c_char;
    zpadlen = max - place;
    spadlen =
        min - (if max >= place { max } else { place }) -
            (if signvalue != 0 {
                 1 as libc::c_int
             } else { 0 as libc::c_int });
    if zpadlen < 0 as libc::c_int { zpadlen = 0 as libc::c_int }
    if spadlen < 0 as libc::c_int { spadlen = 0 as libc::c_int }
    if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        zpadlen = if zpadlen >= spadlen { zpadlen } else { spadlen };
        spadlen = 0 as libc::c_int
    }
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        spadlen = -spadlen
    }
    /* Spaces */
    while spadlen > 0 as libc::c_int {
        total +=
            dopr_outch(buffer, currlen, maxlen, ' ' as i32 as libc::c_char);
        spadlen -= 1
    }
    /* Sign */
    if signvalue != 0 {
        total +=
            dopr_outch(buffer, currlen, maxlen, signvalue as libc::c_char)
    }
    /* Zeros */
    if zpadlen > 0 as libc::c_int {
        while zpadlen > 0 as libc::c_int {
            total +=
                dopr_outch(buffer, currlen, maxlen,
                           '0' as i32 as libc::c_char);
            zpadlen -= 1
        }
    }
    /* Digits */
    while place > 0 as libc::c_int {
        place -= 1;
        total += dopr_outch(buffer, currlen, maxlen, convert[place as usize])
    }
    /* Left Justified spaces */
    while spadlen < 0 as libc::c_int {
        total +=
            dopr_outch(buffer, currlen, maxlen,
                       ' ' as i32 as libc::c_char); /* amount to pad */
        spadlen += 1
    }
    return total;
}
unsafe extern "C" fn abs_val(mut value: libc::c_double) -> libc::c_double {
    let mut result: libc::c_double = value;
    if value < 0 as libc::c_int as libc::c_double { result = -value }
    return result;
}
unsafe extern "C" fn _snp_pow10(mut exp: libc::c_int) -> libc::c_double {
    let mut result: libc::c_double = 1 as libc::c_int as libc::c_double;
    while exp != 0 { result *= 10 as libc::c_int as libc::c_double; exp -= 1 }
    return result;
}
unsafe extern "C" fn _snp_round(mut value: libc::c_double) -> libc::c_long {
    let mut intpart: libc::c_long = 0;
    intpart = value as libc::c_long;
    value = value - intpart as libc::c_double;
    if value >= 0.5f64 { intpart += 1 }
    return intpart;
}
unsafe extern "C" fn fmtfp(mut buffer: *mut libc::c_char,
                           mut currlen: *mut size_t, mut maxlen: size_t,
                           mut fvalue: libc::c_double, mut min: libc::c_int,
                           mut max: libc::c_int, mut flags: libc::c_int)
 -> libc::c_int {
    let mut signvalue: libc::c_int = 0 as libc::c_int;
    let mut ufvalue: libc::c_double = 0.;
    let mut iconvert: [libc::c_char; 20] = [0; 20];
    let mut fconvert: [libc::c_char; 20] = [0; 20];
    let mut iplace: libc::c_int = 0 as libc::c_int;
    let mut fplace: libc::c_int = 0 as libc::c_int;
    let mut padlen: libc::c_int = 0 as libc::c_int;
    let mut zpadlen: libc::c_int = 0 as libc::c_int;
    let mut caps: libc::c_int = 0 as libc::c_int;
    let mut total: libc::c_int = 0 as libc::c_int;
    let mut intpart: libc::c_long = 0;
    let mut fracpart: libc::c_long = 0;
    /* 
   * AIX manpage says the default is 0, but Solaris says the default
   * is 6, and sprintf on AIX defaults to 6
   */
    if max < 0 as libc::c_int { max = 6 as libc::c_int }
    ufvalue = abs_val(fvalue);
    if fvalue < 0 as libc::c_int as libc::c_double {
        signvalue = '-' as i32
    } else if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        /* Do a sign (+/i) */
        signvalue = '+' as i32
    } else if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        signvalue = ' ' as i32
    }
    intpart = ufvalue as libc::c_long;
    /* 
   * Sorry, we only support 9 digits past the decimal because of our 
   * conversion method
   */
    if max > 9 as libc::c_int { max = 9 as libc::c_int }
    /* We "cheat" by converting the fractional part to integer by
   * multiplying by a factor of 10
   */
    fracpart =
        _snp_round(_snp_pow10(max) * (ufvalue - intpart as libc::c_double));
    if fracpart as libc::c_double >= _snp_pow10(max) {
        intpart += 1;
        fracpart =
            (fracpart as libc::c_double - _snp_pow10(max)) as libc::c_long
    }
    loop 
         /* Convert integer part */
         {
        let fresh19 = iplace;
        iplace = iplace + 1;
        iconvert[fresh19 as usize] =
            *if caps != 0 {
                 b"0123456789ABCDEF\x00" as *const u8 as *const libc::c_char
             } else {
                 b"0123456789abcdef\x00" as *const u8 as *const libc::c_char
             }.offset((intpart % 10 as libc::c_int as libc::c_long) as isize);
        intpart = intpart / 10 as libc::c_int as libc::c_long;
        if !(intpart != 0 && iplace < 20 as libc::c_int) { break ; }
    }
    if iplace == 20 as libc::c_int { iplace -= 1 }
    iconvert[iplace as usize] = 0 as libc::c_int as libc::c_char;
    loop 
         /* Convert fractional part */
         {
        let fresh20 = fplace;
        fplace = fplace + 1;
        fconvert[fresh20 as usize] =
            *if caps != 0 {
                 b"0123456789ABCDEF\x00" as *const u8 as *const libc::c_char
             } else {
                 b"0123456789abcdef\x00" as *const u8 as *const libc::c_char
             }.offset((fracpart % 10 as libc::c_int as libc::c_long) as
                          isize);
        fracpart = fracpart / 10 as libc::c_int as libc::c_long;
        if !(fracpart != 0 && fplace < 20 as libc::c_int) { break ; }
    }
    if fplace == 20 as libc::c_int { fplace -= 1 }
    fconvert[fplace as usize] = 0 as libc::c_int as libc::c_char;
    /* -1 for decimal point, another -1 if we are printing a sign */
    padlen =
        min - iplace - max - 1 as libc::c_int -
            (if signvalue != 0 {
                 1 as libc::c_int
             } else { 0 as libc::c_int }); /* Left Justifty */
    zpadlen = max - fplace;
    if zpadlen < 0 as libc::c_int { zpadlen = 0 as libc::c_int }
    if padlen < 0 as libc::c_int { padlen = 0 as libc::c_int }
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        padlen = -padlen
    }
    if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 &&
           padlen > 0 as libc::c_int {
        if signvalue != 0 {
            total +=
                dopr_outch(buffer, currlen, maxlen,
                           signvalue as libc::c_char);
            padlen -= 1;
            signvalue = 0 as libc::c_int
        }
        while padlen > 0 as libc::c_int {
            total +=
                dopr_outch(buffer, currlen, maxlen,
                           '0' as i32 as libc::c_char);
            padlen -= 1
        }
    }
    while padlen > 0 as libc::c_int {
        total +=
            dopr_outch(buffer, currlen, maxlen, ' ' as i32 as libc::c_char);
        padlen -= 1
    }
    if signvalue != 0 {
        total +=
            dopr_outch(buffer, currlen, maxlen, signvalue as libc::c_char)
    }
    while iplace > 0 as libc::c_int {
        iplace -= 1;
        total +=
            dopr_outch(buffer, currlen, maxlen, iconvert[iplace as usize])
    }
    /*
   * Decimal point.  This should probably use locale to find the correct
   * char to print out.
   */
    if max > 0 as libc::c_int {
        total +=
            dopr_outch(buffer, currlen, maxlen, '.' as i32 as libc::c_char);
        while fplace > 0 as libc::c_int {
            fplace -= 1;
            total +=
                dopr_outch(buffer, currlen, maxlen, fconvert[fplace as usize])
        }
    }
    while zpadlen > 0 as libc::c_int {
        total +=
            dopr_outch(buffer, currlen, maxlen, '0' as i32 as libc::c_char);
        zpadlen -= 1
    }
    while padlen < 0 as libc::c_int {
        total +=
            dopr_outch(buffer, currlen, maxlen, ' ' as i32 as libc::c_char);
        padlen += 1
    }
    return total;
}
unsafe extern "C" fn dopr_outch(mut buffer: *mut libc::c_char,
                                mut currlen: *mut size_t, mut maxlen: size_t,
                                mut c: libc::c_char) -> libc::c_int {
    if (*currlen).wrapping_add(1 as libc::c_int as libc::c_ulong) < maxlen {
        let fresh21 = *currlen;
        *currlen = (*currlen).wrapping_add(1);
        *buffer.offset(fresh21 as isize) = c
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmpp_vsnprintf(mut str: *mut libc::c_char,
                                        mut count: size_t,
                                        mut fmt: *const libc::c_char,
                                        mut args: ::std::ffi::VaList)
 -> libc::c_int {
    if !str.is_null() && count > 0 as libc::c_int as libc::c_ulong {
        *str.offset(0 as libc::c_int as isize) =
            0 as libc::c_int as libc::c_char
    }
    return dopr(str, count, fmt, args.as_va_list());
}
/*
 * Copyright Patrick Powell 1995
 * This code is based on code written by Patrick Powell (papowell@astart.com)
 * It may be used for any purpose as long as this notice remains intact
 * on all source code distributions
 */
/* * @file
 *  Compatibility wrappers for OSes lacking snprintf(3) and/or vsnprintf(3).
 */
/* !HAVE_VSNPRINTF */
/* VARARGS3 */
#[no_mangle]
pub unsafe extern "C" fn xmpp_snprintf(mut str: *mut libc::c_char,
                                       mut count: size_t,
                                       mut fmt: *const libc::c_char,
                                       mut args: ...) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut total: libc::c_int = 0;
    ap = args.clone();
    total = xmpp_vsnprintf(str, count, fmt, ap.as_va_list());
    return total;
}
/* !HAVE_SNPRINTF */
