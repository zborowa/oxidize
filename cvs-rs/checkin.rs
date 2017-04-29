extern {
    fn RCS_checkin(
        rcs : *mut ::vers_ts::rcsnode,
        update_dir : *const u8,
        workfile : *const u8,
        message : *const u8,
        rev : *const u8,
        citime : isize,
        flags : i32
    ) -> i32;
    fn RCS_cmp_file(
        arg1 : *mut ::vers_ts::rcsnode,
        arg2 : *const u8,
        arg3 : *const u8,
        arg4 : *mut *mut u8,
        arg5 : *const u8,
        arg6 : *const u8,
        arg7 : *const u8
    ) -> i32;
    fn RCS_rewrite(
        arg1 : *mut ::vers_ts::rcsnode,
        arg2 : *mut deltatext,
        arg3 : *mut u8
    );
    fn RCS_unlock(
        arg1 : *mut ::vers_ts::rcsnode, arg2 : *mut u8, arg3 : i32
    ) -> i32;
    fn __assert_fail(
        __assertion : *const u8,
        __file : *const u8,
        __line : u32,
        __function : *const u8
    );
    fn __errno_location() -> *mut i32;
    fn base_checkout(
        rcs : *mut ::vers_ts::rcsnode,
        finfo : *mut ::vers_ts::file_info,
        prev : *const u8,
        rev : *const u8,
        ptag : *const u8,
        tag : *const u8,
        poptions : *const u8,
        options : *const u8
    ) -> i32;
    fn base_copy(
        finfo : *mut ::vers_ts::file_info,
        rev : *const u8,
        flags : *const u8
    );
    fn base_remove(file : *const u8, rev : *const u8);
    fn copy_file(from : *const u8, to : *const u8);
    fn cvs_xmkdir(
        name : *const u8, update_dir : *const u8, flags : u32
    ) -> bool;
    static mut cvswrite : i32;
    fn error(
        __status : i32, __errnum : i32, __format : *const u8, ...
    );
    fn fileattr_get(
        filename : *const u8, attrname : *const u8
    ) -> *mut u8;
    fn free(__ptr : *mut ::std::os::raw::c_void);
    fn history_write(
        type_ : i32,
        update_dir : *const u8,
        revs : *const u8,
        name : *const u8,
        repository : *const u8
    );
    fn make_base_file_name(
        filename : *const u8, rev : *const u8
    ) -> *mut u8;
    fn mark_up_to_date(update_dir : *const u8, file : *const u8);
    static mut noexec : i32;
    fn rename_file(from : *const u8, to : *const u8);
    static mut server_active : i32;
    fn server_checked_in(
        file : *const u8, update_dir : *const u8, repository : *const u8
    );
    fn server_updated(
        finfo : *mut ::vers_ts::file_info,
        vers : *mut ::vers_ts::vers_ts,
        updated : server_updated_arg4,
        mode : u32,
        checksum : *mut u8,
        filebuf : *mut buffer
    );
    fn strcmp(__s1 : *const u8, __s2 : *const u8) -> i32;
    static mut suppress_bases : bool;
    fn unlink_file_dir(f : *const u8) -> i32;
    fn wrap_fromcvs_process_file(fileName : *const u8);
    fn wrap_tocvs_process_file(fileName : *const u8) -> *mut u8;
    fn xchmod(fname : *const u8, writable : bool);
}

enum buffer {
}

unsafe extern fn streq(
    mut a : *const u8, mut b : *const u8
) -> bool {
    *a as (i32) == *b as (i32) && (strcmp(a,b) == 0i32)
}

#[derive(Copy)]
#[repr(C)]
pub struct deltatext {
    pub version : *mut u8,
    pub log : *mut u8,
    pub text : *mut u8,
    pub len : usize,
    pub other : *mut ::hash::hashlist,
}

impl Clone for deltatext {
    fn clone(&self) -> Self { *self }
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum server_updated_arg4 {
    SERVER_UPDATED,
    SERVER_MERGED,
    SERVER_PATCHED,
    SERVER_RCS_DIFF,
}

#[no_mangle]
pub unsafe extern fn Checkin(
    mut type_ : i32,
    mut finfo : *mut ::vers_ts::file_info,
    mut rev : *mut u8,
    mut tag : *mut u8,
    mut options : *mut u8,
    mut message : *mut u8
) -> i32 {
    let mut vers : *mut ::vers_ts::vers_ts;
    let mut pvers : *mut ::vers_ts::vers_ts;
    let mut set_time : i32;
    let mut tocvsPath
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    tocvsPath = wrap_tocvs_process_file((*finfo).file);
    if noexec == 0 {
        if !tocvsPath.is_null() {
            if unlink_file_dir((*finfo).file) < 0i32 {
                if !(*__errno_location() == 2i32) {
                    error(
                        1i32,
                        *__errno_location(),
                        (*b"cannot remove %s\0").as_ptr(),
                        (*finfo).fullname
                    );
                }
            }
            rename_file(tocvsPath as (*const u8),(*finfo).file);
        }
    }
    if (*finfo).rcs != 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::vers_ts::rcsnode) {
        0i32;
    } else {
        __assert_fail(
            (*b"finfo->rcs != NULL\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"Checkin\0").as_ptr()
        );
    }
    pvers = ::vers_ts::Version_TS(
                finfo,
                0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                tag as (*const u8),
                0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                1i32,
                0i32
            );
    let switch1
        = RCS_checkin(
              (*finfo).rcs,
              (*finfo).update_dir,
              (*finfo).file,
              message as (*const u8),
              rev as (*const u8),
              0isize,
              16i32
          );
    if switch1 == -1i32 {
        if !tocvsPath.is_null() {
            if unlink_file_dir(tocvsPath as (*const u8)) < 0i32 {
                error(
                    0i32,
                    *__errno_location(),
                    (*b"cannot remove %s\0").as_ptr(),
                    tocvsPath
                );
            }
        }
        if noexec == 0 {
            error(
                1i32,
                *__errno_location(),
                (*b"could not check in %s -- fork failed\0").as_ptr(),
                (*finfo).fullname
            );
        }
        1i32
    } else if switch1 == 0i32 {
        if !options.is_null() && streq(
                                     options as (*const u8),
                                     (*b"-V4\0").as_ptr()
                                 ) {
            *options.offset(0isize) = b'\0';
        }
        if true && !options.is_null() && (streq(
                                              options as (*const u8),
                                              (*b"-ko\0").as_ptr()
                                          ) || streq(
                                                   options as (*const u8),
                                                   (*b"-kb\0").as_ptr()
                                               )) || RCS_cmp_file(
                                                         (*finfo).rcs,
                                                         (*pvers).tag as (*const u8),
                                                         rev as (*const u8),
                                                         0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut u8),
                                                         0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                                                         options as (*const u8),
                                                         (*finfo).file
                                                     ) == 0 {
            set_time = 0i32;
        } else {
            set_time = 1i32;
        }
        vers = ::vers_ts::Version_TS(
                   finfo,
                   0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                   tag as (*const u8),
                   0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                   1i32,
                   set_time
               );
        if set_time != 0 {
            if base_checkout(
                   (*finfo).rcs,
                   finfo,
                   (*pvers).vn_user as (*const u8),
                   (*vers).vn_rcs as (*const u8),
                   (*(*pvers).entdata).tag as (*const u8),
                   (*vers).tag as (*const u8),
                   (*(*pvers).entdata).options as (*const u8),
                   options as (*const u8)
               ) != 0 {
                error(
                    1i32,
                    0i32,
                    (*b"failed when checking out new copy of %s\0").as_ptr(),
                    (*finfo).fullname
                );
            }
            base_copy(
                finfo,
                (*vers).vn_rcs as (*const u8),
                if cvswrite != 0 && fileattr_get(
                                        (*finfo).file,
                                        (*b"_watched\0").as_ptr()
                                    ).is_null(
                                    ) {
                    (*b"yy\0").as_ptr()
                } else {
                    (*b"yn\0").as_ptr()
                }
            );
            set_time = 1i32;
        } else if !suppress_bases {
            let mut basefile : *mut u8;
            cvs_xmkdir(
                (*b"CVS/Base\0").as_ptr(),
                0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                (1i32 << 4i32) as (u32)
            );
            basefile = make_base_file_name(
                           (*finfo).file,
                           (*vers).vn_rcs as (*const u8)
                       );
            copy_file((*finfo).file,basefile as (*const u8));
            free(basefile as (*mut ::std::os::raw::c_void));
        }
        base_remove((*finfo).file,(*pvers).vn_user as (*const u8));
        ::vers_ts::freevers_ts(
            &mut pvers as (*mut *mut ::vers_ts::vers_ts)
        );
        wrap_fromcvs_process_file((*finfo).file);
        if cvswrite == 0 || !fileattr_get(
                                 (*finfo).file,
                                 (*b"_watched\0").as_ptr()
                             ).is_null(
                             ) {
            xchmod((*finfo).file,false);
        }
        if streq((*vers).options as (*const u8),(*b"-V4\0").as_ptr()) {
            *(*vers).options.offset(0isize) = b'\0';
        }
        ::entries::Register(
            finfo as (*const ::vers_ts::file_info),
            (*vers).vn_rcs as (*const u8),
            (*vers).ts_user as (*const u8),
            (*vers).options as (*const u8),
            (*vers).tag as (*const u8),
            (*vers).date as (*const u8),
            0i32 as (*mut ::std::os::raw::c_void) as (*const u8)
        );
        history_write(
            type_,
            0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
            (*vers).vn_rcs as (*const u8),
            (*finfo).file,
            (*finfo).repository
        );
        if !tocvsPath.is_null() {
            if unlink_file_dir(tocvsPath as (*const u8)) < 0i32 {
                error(
                    0i32,
                    *__errno_location(),
                    (*b"cannot remove %s\0").as_ptr(),
                    tocvsPath
                );
            }
        }
        if !rev.is_null() {
            RCS_unlock(
                (*finfo).rcs,
                0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                1i32
            );
            RCS_rewrite(
                (*finfo).rcs,
                0i32 as (*mut ::std::os::raw::c_void) as (*mut deltatext),
                0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
            );
        }
        if server_active != 0 {
            if set_time != 0 {
                server_updated(
                    finfo,
                    vers,
                    server_updated_arg4::SERVER_UPDATED,
                    -1i32 as (u32),
                    0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                    0i32 as (*mut ::std::os::raw::c_void) as (*mut buffer)
                );
            } else {
                server_checked_in(
                    (*finfo).file,
                    (*finfo).update_dir,
                    (*finfo).repository
                );
            }
        } else {
            mark_up_to_date((*finfo).update_dir,(*finfo).file);
        }
        ::vers_ts::freevers_ts(
            &mut vers as (*mut *mut ::vers_ts::vers_ts)
        );
        0i32
    } else {
        if !tocvsPath.is_null() {
            if unlink_file_dir(tocvsPath as (*const u8)) < 0i32 {
                error(
                    0i32,
                    *__errno_location(),
                    (*b"cannot remove %s\0").as_ptr(),
                    tocvsPath
                );
            }
        }
        if noexec == 0 {
            error(
                0i32,
                0i32,
                (*b"could not check in %s\0").as_ptr(),
                (*finfo).fullname
            );
        }
        1i32
    }
}
