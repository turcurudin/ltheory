use libc;
use std::ffi::VaListImpl;

extern "C" {
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::size_t,
        _: *const libc::c_char,
        _: __builtin_va_list,
    ) -> libc::c_int;
}
pub type __builtin_va_list = *mut libc::c_char;
pub type va_list = __builtin_va_list;

#[inline]
pub unsafe extern "C" fn MemAlloc(mut size: libc::size_t) -> *mut libc::c_void {
    return libc::malloc(size);
}

#[inline]
pub unsafe extern "C" fn MemAllocZero(mut size: libc::size_t) -> *mut libc::c_void {
    return libc::calloc(1, size);
}

#[inline]
pub unsafe extern "C" fn MemFree(mut ptr: *const libc::c_void) {
    libc::free(ptr as *mut libc::c_void);
}

#[inline]
pub unsafe extern "C" fn MemRealloc(
    mut ptr: *mut libc::c_void,
    mut newSize: libc::size_t,
) -> *mut libc::c_void {
    return libc::realloc(ptr, newSize);
}

#[inline]
pub unsafe extern "C" fn MemCpy(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
    mut size: libc::size_t,
) {
    libc::memcpy(dst, src, size as libc::size_t);
}

#[inline]
pub unsafe extern "C" fn MemMove(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
    mut size: usize,
) {
    libc::memmove(dst, src, size as libc::size_t);
}

#[inline]
pub unsafe extern "C" fn MemZero(mut dst: *mut libc::c_void, mut size: usize) {
    libc::memset(dst, 0 as libc::c_int, size);
}

#[inline]
pub unsafe extern "C" fn MemSet(
    mut dst: *mut libc::c_void,
    mut value: libc::c_int,
    mut size: libc::size_t,
) {
    libc::memset(dst, value, size);
}

#[inline]
pub unsafe extern "C" fn StrAlloc(mut len: libc::size_t) -> *mut libc::c_char {
    return libc::malloc(len) as *mut libc::c_char;
}

#[inline]
pub unsafe extern "C" fn StrFree(mut s: *const libc::c_char) {
    libc::free(s as *mut libc::c_void);
}

#[inline]
pub unsafe extern "C" fn StrDup(mut s: *const libc::c_char) -> *const libc::c_char {
    if s.is_null() {
        return 0 as *const libc::c_char;
    }
    let mut len: libc::size_t = (StrLen(s)).wrapping_add(1 as libc::size_t);
    let mut buf: *mut libc::c_char = StrAlloc(len);
    libc::memcpy(buf as *mut libc::c_void, s as *const libc::c_void, len);
    return buf as *const libc::c_char;
}

#[inline]
pub unsafe extern "C" fn StrLen(mut s: *const libc::c_char) -> libc::size_t {
    if s.is_null() {
        return 0 as libc::c_int as libc::size_t;
    }
    let mut begin: *const libc::c_char = s;
    while *s != 0 {
        s = s.offset(1);
    }
    return s.offset_from(begin) as libc::c_long as libc::size_t;
}

#[inline]
pub unsafe extern "C" fn StrEqual(mut a: *const libc::c_char, mut b: *const libc::c_char) -> bool {
    return libc::strcmp(a, b) == 0 as libc::c_int;
}

#[inline]
pub unsafe extern "C" fn StrFormat(mut fmt: *const libc::c_char, mut args: ...) -> *const libc::c_char {
    let mut args_0: va_list = 0 as *mut libc::c_char;
    args_0 = &args as *const VaListImpl as va_list;
    let mut len: libc::size_t = (vsnprintf(
        0 as *mut libc::c_char,
        0 as libc::size_t,
        fmt,
        args_0,
    ) + 1 as libc::c_int) as libc::size_t;
    let mut buf: *mut libc::c_char = StrAlloc(len);
    args_0 = &args as *const VaListImpl as va_list;
    vsnprintf(buf, len, fmt, args_0);
    return buf as *const libc::c_char;
}

#[inline]
pub unsafe extern "C" fn StrReplace(
    mut s: *const libc::c_char,
    mut search: *const libc::c_char,
    mut replace: *const libc::c_char,
) -> *const libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ins: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len_search: libc::size_t = 0;
    let mut len_replace: libc::size_t = 0;
    let mut len_front: libc::size_t = 0;
    let mut count: libc::size_t = 0;
    if s.is_null() || search.is_null() {
        return 0 as *const libc::c_char;
    }
    len_search = StrLen(search);
    if len_search == 0 as libc::size_t {
        return 0 as *const libc::c_char;
    }
    if replace.is_null() {
        replace = b"\0" as *const u8 as *const libc::c_char;
    }
    len_replace = StrLen(replace);
    ins = s as *mut libc::c_char;
    count = 0 as libc::c_int as libc::size_t;
    loop {
        tmp = libc::strstr(ins, search);
        if tmp.is_null() {
            break;
        }
        ins = tmp.offset(len_search as isize);
        count = count.wrapping_add(1);
    }
    result = StrAlloc(
        (StrLen(s))
            .wrapping_add(len_replace.wrapping_sub(len_search).wrapping_mul(count))
            .wrapping_add(1 as libc::size_t),
    );
    tmp = result;
    loop {
        let fresh0 = count;
        count = count.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        ins = libc::strstr(s, search);
        len_front = ins.offset_from(s) as libc::c_long as libc::size_t;
        tmp = (libc::strncpy(tmp, s, len_front)).offset(len_front as isize);
        tmp = (libc::strcpy(tmp, replace)).offset(len_replace as isize);
        s = s.offset(len_front.wrapping_add(len_search) as isize);
    }
    libc::strcpy(tmp, s);
    return result as *const libc::c_char;
}

#[inline]
pub unsafe extern "C" fn StrFind(mut s: *const libc::c_char, mut sub: *const libc::c_char) -> *const libc::c_char {
    return libc::strstr(s, sub) as *const libc::c_char;
}

#[inline]
pub unsafe extern "C" fn StrSubStr(mut begin: *const libc::c_char, mut end: *const libc::c_char) -> *const libc::c_char {
    let mut len: libc::size_t = end.offset_from(begin) as libc::c_long as libc::size_t;
    let mut result: *mut libc::c_char = StrAlloc(
        len.wrapping_add(1 as libc::size_t),
    );
    let mut pResult: *mut libc::c_char = result;
    while begin != end {
        let fresh1 = begin;
        begin = begin.offset(1);
        let fresh2 = pResult;
        pResult = pResult.offset(1);
        *fresh2 = *fresh1;
    }
    *result.offset(len as isize) = 0 as libc::c_int as libc::c_char;
    return result as *const libc::c_char;
}
#[inline]
pub unsafe extern "C" fn StrSub(
    mut s: *const libc::c_char,
    mut begin: *const libc::c_char,
    mut end: *const libc::c_char,
    mut replace: *const libc::c_char,
) -> *const libc::c_char {
    let mut len: libc::size_t = begin
        .offset((StrLen(s)).wrapping_add(StrLen(replace)) as isize)
        .offset_from(end) as libc::c_long as libc::size_t;
    let mut result: *mut libc::c_char = StrAlloc(
        len.wrapping_add(1 as libc::size_t),
    );
    let mut pResult: *mut libc::c_char = result;
    while s != begin {
        let fresh3 = s;
        s = s.offset(1);
        let fresh4 = pResult;
        pResult = pResult.offset(1);
        *fresh4 = *fresh3;
    }
    while *replace != 0 {
        let fresh5 = replace;
        replace = replace.offset(1);
        let fresh6 = pResult;
        pResult = pResult.offset(1);
        *fresh6 = *fresh5;
    }
    while *end != 0 {
        let fresh7 = end;
        end = end.offset(1);
        let fresh8 = pResult;
        pResult = pResult.offset(1);
        *fresh8 = *fresh7;
    }
    *pResult = 0 as libc::c_int as libc::c_char;
    return result as *const libc::c_char;
}

#[inline]
pub unsafe extern "C" fn StrAdd(mut a: *const libc::c_char, mut b: *const libc::c_char) -> *const libc::c_char {
    let mut buf: *mut libc::c_char = StrAlloc(
        (StrLen(a))
            .wrapping_add(StrLen(b))
            .wrapping_add(1 as libc::size_t),
    );
    let mut cur: *mut libc::c_char = buf;
    while *a != 0 {
        let fresh9 = a;
        a = a.offset(1);
        let fresh10 = cur;
        cur = cur.offset(1);
        *fresh10 = *fresh9;
    }
    while *b != 0 {
        let fresh11 = b;
        b = b.offset(1);
        let fresh12 = cur;
        cur = cur.offset(1);
        *fresh12 = *fresh11;
    }
    *cur = 0 as libc::c_int as libc::c_char;
    return buf as *const libc::c_char;
}

#[inline]
pub unsafe extern "C" fn StrAdd3(mut a: *const libc::c_char, mut b: *const libc::c_char, mut c: *const libc::c_char) -> *const libc::c_char {
    let mut buf: *mut libc::c_char = StrAlloc(
        (StrLen(a))
            .wrapping_add(StrLen(b))
            .wrapping_add(StrLen(c))
            .wrapping_add(1 as libc::size_t),
    );
    let mut cur: *mut libc::c_char = buf;
    while *a != 0 {
        let fresh0 = a;
        a = a.offset(1);
        let fresh1 = cur;
        cur = cur.offset(1);
        *fresh1 = *fresh0;
    }
    while *b != 0 {
        let fresh2 = b;
        b = b.offset(1);
        let fresh3 = cur;
        cur = cur.offset(1);
        *fresh3 = *fresh2;
    }
    while *c != 0 {
        let fresh4 = c;
        c = c.offset(1);
        let fresh5 = cur;
        cur = cur.offset(1);
        *fresh5 = *fresh4;
    }
    *cur = 0 as libc::c_int as libc::c_char;
    return buf as *const libc::c_char;
}
