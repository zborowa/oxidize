extern {
    fn __assert_fail(
        __assertion : *const u8,
        __file : *const u8,
        __line : u32,
        __function : *const u8
    );
    fn call_diff(out : *const u8) -> i32;
    fn call_diff_add_arg(s : *const u8);
    fn call_diff_setup(
        prog : *const u8, argc : i32, argv : *const *mut u8
    );
    fn cvs_output(arg1 : *const u8, arg2 : usize);
    fn quotearg_style(s : quoting_style, arg : *const u8) -> *mut u8;
}

#[no_mangle]
pub unsafe extern fn diff_exec(
    mut file1 : *const u8,
    mut file2 : *const u8,
    mut label1 : *const u8,
    mut label2 : *const u8,
    mut dargc : i32,
    mut dargv : *const *mut u8,
    mut out : *const u8
) -> i32 {
    call_diff_setup((*b"diff\0").as_ptr(),dargc,dargv);
    if !label1.is_null() {
        call_diff_add_arg(label1);
    }
    if !label2.is_null() {
        call_diff_add_arg(label2);
    }
    call_diff_add_arg((*b"--\0").as_ptr());
    call_diff_add_arg(file1);
    call_diff_add_arg(file2);
    call_diff(out)
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum quoting_style {
    literal_quoting_style,
    shell_quoting_style,
    shell_always_quoting_style,
    c_quoting_style,
    c_maybe_quoting_style,
    escape_quoting_style,
    locale_quoting_style,
    clocale_quoting_style,
}

#[no_mangle]
pub unsafe extern fn RCS_output_diff_options(
    mut diff_argc : i32,
    mut diff_argv : *const *mut u8,
    mut devnull : bool,
    mut rev1 : *const u8,
    mut rev2 : *const u8,
    mut workfile : *const u8
) {
    let mut i : i32;
    cvs_output((*b"diff\0").as_ptr(),0usize);
    i = 0i32;
    'loop1: loop {
        if !(i < diff_argc) {
            break;
        }
        cvs_output((*b" \0").as_ptr(),1usize);
        cvs_output(
            quotearg_style(
                quoting_style::shell_quoting_style,
                *diff_argv.offset(i as (isize)) as (*const u8)
            ) as (*const u8),
            0usize
        );
        i = i + 1;
    }
    if devnull {
        cvs_output((*b" -N\0").as_ptr(),3usize);
    }
    if !rev1.is_null() {
        cvs_output((*b" -r\0").as_ptr(),3usize);
        cvs_output(rev1,0usize);
    }
    if !rev2.is_null() {
        cvs_output((*b" -r\0").as_ptr(),3usize);
        cvs_output(rev2,0usize);
    }
    if !workfile.is_null() {
        0i32;
    } else {
        __assert_fail(
            (*b"workfile\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"RCS_output_diff_options\0").as_ptr()
        );
    }
    cvs_output((*b" \0").as_ptr(),1usize);
    cvs_output(workfile,0usize);
    cvs_output((*b"\n\0").as_ptr(),1usize);
}
