// defines missing intrinsics required by core

type c_int = i32;

#[no_mangle] unsafe extern "C" fn memcpy(mut dst: *mut u8, mut src: *const u8, mut count: usize) {
    while count > 0 {
        *dst = *src;
        dst = dst.add(1);
        src = src.add(1);
        count -= 1;
    }
}

#[no_mangle] unsafe extern "C" fn memmove(dst: *mut u8, src: *const u8, mut count: usize) {
    if dst as usize <= src as usize {
        memcpy(dst, src, count);
    } else {
        while count > 0 {
            *dst.add(count) = *src.add(count);
            count -= 1;
        }
    }
}

#[no_mangle] unsafe extern "C" fn memset(mut ptr: *mut u8, value: c_int, mut count: usize) {
    let value = value as u8;
    while count > 0 {
        *ptr = value;
        ptr = ptr.add(1);
        count = count - 1;
    }
}

#[no_mangle] unsafe extern "C" fn memcmp(mut a: *const u8, mut b: *const u8, mut count: usize) -> c_int {
    while count > 0 {
        if a > b { return 1 }
        else if a < b { return -1; }
        a = a.add(1);
        b = b.add(1);
        count = count - 1;
    }
    0
}
