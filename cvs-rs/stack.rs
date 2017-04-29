extern {
    fn Xstrdup(str : *const u8) -> *mut u8;
}

unsafe extern fn do_push(
    mut stack : *mut ::hash::hashlist,
    mut elem : *mut ::std::os::raw::c_void,
    mut isstring : i32
) {
    let mut p : *mut ::hash::hashnode = ::hash::getnode();
    if isstring != 0 {
        (*p).key = elem as (*mut u8);
    } else {
        (*p).data = elem;
    }
    ::hash::addnode(stack,p);
}

#[no_mangle]
pub unsafe extern fn push(
    mut stack : *mut ::hash::hashlist,
    mut elem : *mut ::std::os::raw::c_void
) {
    do_push(stack,elem,0i32);
}

#[no_mangle]
pub unsafe extern fn push_string(
    mut stack : *mut ::hash::hashlist, mut elem : *mut u8
) {
    do_push(stack,elem as (*mut ::std::os::raw::c_void),1i32);
}

unsafe extern fn do_pop(
    mut stack : *mut ::hash::hashlist, mut isstring : i32
) -> *mut ::std::os::raw::c_void {
    let mut elem : *mut ::std::os::raw::c_void;
    if isempty(stack) != 0 {
        0i32 as (*mut ::std::os::raw::c_void)
    } else {
        if isstring != 0 {
            elem = (*(*(*stack).list).prev).key as (*mut ::std::os::raw::c_void);
            (*(*(*stack).list).prev).key = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        } else {
            elem = (*(*(*stack).list).prev).data;
            (*(*(*stack).list).prev).data = 0i32 as (*mut ::std::os::raw::c_void);
        }
        ::hash::delnode((*(*stack).list).prev as (*mut ::hash::hashnode));
        elem
    }
}

#[no_mangle]
pub unsafe extern fn pop(
    mut stack : *mut ::hash::hashlist
) -> *mut ::std::os::raw::c_void {
    do_pop(stack,0i32)
}

#[no_mangle]
pub unsafe extern fn pop_string(
    mut stack : *mut ::hash::hashlist
) -> *mut u8 {
    do_pop(stack,1i32) as (*mut u8)
}

unsafe extern fn do_unshift(
    mut stack : *mut ::hash::hashlist,
    mut elem : *mut ::std::os::raw::c_void,
    mut isstring : i32
) {
    let mut p : *mut ::hash::hashnode = ::hash::getnode();
    if isstring != 0 {
        (*p).key = elem as (*mut u8);
    } else {
        (*p).data = elem;
    }
    ::hash::addnode_at_front(stack,p);
}

#[no_mangle]
pub unsafe extern fn unshift(
    mut stack : *mut ::hash::hashlist,
    mut elem : *mut ::std::os::raw::c_void
) {
    do_unshift(stack,elem,0i32);
}

#[no_mangle]
pub unsafe extern fn unshift_string(
    mut stack : *mut ::hash::hashlist, mut elem : *mut u8
) {
    do_unshift(stack,elem as (*mut ::std::os::raw::c_void),1i32);
}

unsafe extern fn do_shift(
    mut stack : *mut ::hash::hashlist, mut isstring : i32
) -> *mut ::std::os::raw::c_void {
    let mut elem : *mut ::std::os::raw::c_void;
    if isempty(stack) != 0 {
        0i32 as (*mut ::std::os::raw::c_void)
    } else {
        if isstring != 0 {
            elem = (*(*(*stack).list).next).key as (*mut ::std::os::raw::c_void);
            (*(*(*stack).list).next).key = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        } else {
            elem = (*(*(*stack).list).next).data;
            (*(*(*stack).list).next).data = 0i32 as (*mut ::std::os::raw::c_void);
        }
        ::hash::delnode((*(*stack).list).next as (*mut ::hash::hashnode));
        elem
    }
}

#[no_mangle]
pub unsafe extern fn shift(
    mut stack : *mut ::hash::hashlist
) -> *mut ::std::os::raw::c_void {
    do_shift(stack,0i32)
}

#[no_mangle]
pub unsafe extern fn shift_string(
    mut stack : *mut ::hash::hashlist
) -> *mut u8 {
    do_shift(stack,1i32) as (*mut u8)
}

#[no_mangle]
pub unsafe extern fn isempty(
    mut stack : *mut ::hash::hashlist
) -> i32 {
    if (*stack).list as (*mut ::std::os::raw::c_void) == (*(*stack).list).next as (*mut ::std::os::raw::c_void) {
        1i32
    } else {
        0i32
    }
}

#[no_mangle]
pub unsafe extern fn unshift_string_array(
    mut list : *mut ::hash::hashlist,
    mut argv : *mut *mut u8,
    mut argc : i32
) {
    'loop0: loop {
        if !({
                 let _old = argc;
                 argc = argc - 1;
                 _old
             } > 0i32) {
            break;
        }
        unshift_string(
            list,
            Xstrdup(*argv.offset(argc as (isize)) as (*const u8))
        );
    }
}

#[no_mangle]
pub unsafe extern fn init_string_list(
    mut argv : *mut *mut u8, mut argc : i32
) -> *mut ::hash::hashlist {
    let mut newlist : *mut ::hash::hashlist = ::hash::getlist();
    unshift_string_array(newlist,argv,argc);
    newlist
}
