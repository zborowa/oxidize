extern {
    fn RCS_getexpand(arg1 : *mut rcsnode) -> *mut u8;
    fn RCS_getrevtime(
        rcs : *mut rcsnode, rev : *const u8, date : *mut u8, fudge : i32
    ) -> isize;
    fn RCS_getversion(
        rcs : *mut rcsnode,
        tag : *const u8,
        date : *const u8,
        force_tag_match : i32,
        simple_tag : *mut i32
    ) -> *mut u8;
    fn RCS_parse(file : *const u8, repos : *const u8) -> *mut rcsnode;
    fn Xasprintf(format : *const u8, ...) -> *mut u8;
    fn Xstrdup(str : *const u8) -> *mut u8;
    fn __errno_location() -> *mut i32;
    fn abort();
    fn asctime(__tp : *const tm) -> *mut u8;
    fn ctime(__timer : *const isize) -> *mut u8;
    fn error(
        __status : i32, __errnum : i32, __format : *const u8, ...
    );
    fn free(__ptr : *mut ::std::os::raw::c_void);
    fn freercsnode(rnodep : *mut *mut rcsnode);
    fn gmtime(__timer : *const isize) -> *mut tm;
    fn lstat(__file : *const u8, __buf : *mut stat) -> i32;
    fn memset(
        __s : *mut ::std::os::raw::c_void, __c : i32, __n : usize
    ) -> *mut ::std::os::raw::c_void;
    static mut server_active : i32;
    fn server_modtime(finfo : *mut file_info, vers_ts : *mut vers_ts);
    fn stat(__file : *const u8, __buf : *mut stat) -> i32;
    fn strcat(__dest : *mut u8, __src : *const u8) -> *mut u8;
    fn strcmp(__s1 : *const u8, __s2 : *const u8) -> i32;
    fn strcpy(__dest : *mut u8, __src : *const u8) -> *mut u8;
    fn strlen(__s : *const u8) -> usize;
    fn time(__timer : *mut isize) -> isize;
    fn utime(__file : *const u8, __file_times : *const utimbuf) -> i32;
    fn xmalloc(s : usize) -> *mut ::std::os::raw::c_void;
}

#[derive(Copy)]
#[repr(C)]
pub struct rcsnode {
    pub refcount : i32,
    pub flags : i32,
    pub path : *mut u8,
    pub print_path : *mut u8,
    pub head : *mut u8,
    pub branch : *mut u8,
    pub symbols_data : *mut u8,
    pub expand : *mut u8,
    pub symbols : *mut ::hash::hashlist,
    pub versions : *mut ::hash::hashlist,
    pub access : *mut u8,
    pub locks_data : *mut u8,
    pub locks : *mut ::hash::hashlist,
    pub strict_locks : i32,
    pub comment : *mut u8,
    pub desc : *mut u8,
    pub delta_pos : isize,
    pub other : *mut ::hash::hashlist,
}

impl Clone for rcsnode {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct vers_ts {
    pub vn_user : *mut u8,
    pub vn_rcs : *mut u8,
    pub vn_tag : *mut u8,
    pub ts_user : *mut u8,
    pub ts_rcs : *mut u8,
    pub options : *mut u8,
    pub ts_conflict : *mut u8,
    pub tag : *mut u8,
    pub date : *mut u8,
    pub nonbranch : i32,
    pub entdata : *mut ::entries::entnode,
    pub srcfile : *mut rcsnode,
}

impl Clone for vers_ts {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct file_info {
    pub file : *const u8,
    pub update_dir : *const u8,
    pub fullname : *const u8,
    pub repository : *const u8,
    pub entries : *mut ::hash::hashlist,
    pub rcs : *mut rcsnode,
}

impl Clone for file_info {
    fn clone(&self) -> Self { *self }
}

unsafe extern fn streq(
    mut a : *const u8, mut b : *const u8
) -> bool {
    *a as (i32) == *b as (i32) && (strcmp(a,b) == 0i32)
}

#[derive(Copy)]
#[repr(C)]
pub struct utimbuf {
    pub actime : isize,
    pub modtime : isize,
}

impl Clone for utimbuf {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn Version_TS(
    mut finfo : *mut file_info,
    mut options : *const u8,
    mut tag : *const u8,
    mut date : *const u8,
    mut force_tag_match : i32,
    mut set_time : i32
) -> *mut vers_ts {
    let mut p : *mut ::hash::hashnode;
    let mut rcsdata : *mut rcsnode;
    let mut vers_ts : *mut vers_ts;
    let mut entdata : *mut ::entries::entnode;
    let mut rcsexpand
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    vers_ts = xmalloc(
                  ::std::mem::size_of::<vers_ts>()
              ) as (*mut vers_ts);
    memset(
        vers_ts as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<vers_ts>()
    );
    if (*finfo).entries == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashlist) {
        p = 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode);
    } else {
        p = ::hash::findnode_fn((*finfo).entries,(*finfo).file);
    }
    if p == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode) {
        entdata = 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::entries::entnode);
    } else {
        entdata = (*p).data as (*mut ::entries::entnode);
        if (*entdata).type_ as (i32) == ::entries::ent_type::ENT_SUBDIR as (i32) {
            (*vers_ts).entdata = entdata;
        } else if !streq(
                       (*entdata).timestamp as (*const u8),
                       (*b"D\0").as_ptr()
                   ) {
            (*vers_ts).vn_user = Xstrdup((*entdata).version as (*const u8));
            (*vers_ts).ts_rcs = Xstrdup((*entdata).timestamp as (*const u8));
            (*vers_ts).ts_conflict = Xstrdup(
                                         (*entdata).conflict as (*const u8)
                                     );
            if !(!tag.is_null() || !date.is_null(
                                    )) && !::entries::entriesGetAflag((*finfo).entries) {
                (*vers_ts).tag = Xstrdup((*entdata).tag as (*const u8));
                (*vers_ts).date = Xstrdup((*entdata).date as (*const u8));
            }
            (*vers_ts).entdata = entdata;
        }
        if (options.is_null(
            ) || *options == 0) && !::entries::entriesGetAflag(
                                        (*finfo).entries
                                    ) {
            (*vers_ts).options = Xstrdup((*entdata).options as (*const u8));
        }
    }
    if (*finfo).rcs != 0i32 as (*mut ::std::os::raw::c_void) as (*mut rcsnode) {
        rcsexpand = RCS_getexpand((*finfo).rcs);
    }
    if !options.is_null() && (*options != 0) {
        if !(*vers_ts).options.is_null() {
            free((*vers_ts).options as (*mut ::std::os::raw::c_void));
        }
        if !rcsexpand.is_null() && streq(
                                       rcsexpand as (*const u8),
                                       (*b"b\0").as_ptr()
                                   ) {
            (*vers_ts).options = Xstrdup((*b"-kb\0").as_ptr());
        } else {
            (*vers_ts).options = Xstrdup(options);
        }
    } else if ((*vers_ts).options.is_null(
               ) || *(*vers_ts).options == 0) && !rcsexpand.is_null() {
        if (*vers_ts).options != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            free((*vers_ts).options as (*mut ::std::os::raw::c_void));
        }
        (*vers_ts).options = xmalloc(
                                 strlen(rcsexpand as (*const u8)).wrapping_add(3usize)
                             ) as (*mut u8);
        strcpy((*vers_ts).options,(*b"-k\0").as_ptr());
        strcat((*vers_ts).options,rcsexpand as (*const u8));
    }
    if (*vers_ts).options.is_null() {
        (*vers_ts).options = Xstrdup((*b"\0").as_ptr());
    }
    if !tag.is_null() || !date.is_null() {
        (*vers_ts).tag = Xstrdup(tag);
        (*vers_ts).date = Xstrdup(date);
    } else if (*vers_ts).entdata.is_null(
              ) && !::entries::entriesGetAflag(
                        (*finfo).entries
                    ) && ::entries::entriesHasSticky((*finfo).entries) {
        if (*vers_ts).tag.is_null() {
            (*vers_ts).tag = Xstrdup(
                                 ::entries::entriesGetTag((*finfo).entries)
                             );
            (*vers_ts).nonbranch = ::entries::entriesGetNonbranch(
                                       (*finfo).entries
                                   );
        }
        if (*vers_ts).date.is_null() {
            (*vers_ts).date = Xstrdup(
                                  ::entries::entriesGetDate((*finfo).entries)
                              );
        }
    }
    if (*finfo).rcs != 0i32 as (*mut ::std::os::raw::c_void) as (*mut rcsnode) {
        rcsdata = (*finfo).rcs;
        (*rcsdata).refcount = (*rcsdata).refcount + 1;
    } else if (*finfo).repository != 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
        rcsdata = RCS_parse((*finfo).file,(*finfo).repository);
    } else {
        rcsdata = 0i32 as (*mut ::std::os::raw::c_void) as (*mut rcsnode);
    }
    if rcsdata != 0i32 as (*mut ::std::os::raw::c_void) as (*mut rcsnode) {
        (*vers_ts).srcfile = rcsdata;
        if (*finfo).rcs.is_null() {
            (*rcsdata).refcount = (*rcsdata).refcount + 1;
            (*finfo).rcs = rcsdata;
        }
        if !(*vers_ts).tag.is_null() && streq(
                                            (*vers_ts).tag as (*const u8),
                                            (*b"BASE\0").as_ptr()
                                        ) {
            (*vers_ts).vn_rcs = Xstrdup((*vers_ts).vn_user as (*const u8));
            (*vers_ts).vn_tag = Xstrdup((*vers_ts).vn_user as (*const u8));
        } else {
            let mut simple : i32 = 0;
            (*vers_ts).vn_rcs = RCS_getversion(
                                    rcsdata,
                                    (*vers_ts).tag as (*const u8),
                                    (*vers_ts).date as (*const u8),
                                    force_tag_match,
                                    &mut simple as (*mut i32)
                                );
            if (*vers_ts).vn_rcs == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                (*vers_ts).vn_tag = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
            } else if simple != 0 {
                (*vers_ts).vn_tag = Xstrdup((*vers_ts).tag as (*const u8));
            } else {
                (*vers_ts).vn_tag = Xstrdup((*vers_ts).vn_rcs as (*const u8));
            }
        }
        if set_time != 0 && ((*vers_ts).vn_rcs != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)) {
            if server_active != 0 {
                server_modtime(finfo,vers_ts);
            } else {
                let mut t : utimbuf = ::std::mem::zeroed();
                t.modtime = RCS_getrevtime(
                                rcsdata,
                                (*vers_ts).vn_rcs as (*const u8),
                                0i32 as (*mut u8),
                                0i32
                            );
                if t.modtime != -1isize {
                    time(&mut t.actime as (*mut isize));
                    utime((*finfo).file,&mut t as (*mut utimbuf) as (*const utimbuf));
                }
            }
        }
    }
    if (*finfo).entries != 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashlist) {
        if server_active != 0 {
            time_stamp_server((*finfo).file,vers_ts,entdata);
        } else {
            (*vers_ts).ts_user = time_stamp((*finfo).file);
        }
    }
    vers_ts
}

#[derive(Copy)]
#[repr(C)]
pub struct timespec {
    pub tv_sec : isize,
    pub tv_nsec : isize,
}

impl Clone for timespec {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct stat {
    pub st_dev : usize,
    pub st_ino : usize,
    pub st_nlink : usize,
    pub st_mode : u32,
    pub st_uid : u32,
    pub st_gid : u32,
    pub __pad0 : i32,
    pub st_rdev : usize,
    pub st_size : isize,
    pub st_blksize : isize,
    pub st_blocks : isize,
    pub st_atim : timespec,
    pub st_mtim : timespec,
    pub st_ctim : timespec,
    pub __glibc_reserved : [isize; 3],
}

impl Clone for stat {
    fn clone(&self) -> Self { *self }
}

unsafe extern fn time_stamp_server(
    mut file : *const u8,
    mut vers_ts : *mut vers_ts,
    mut entdata : *mut ::entries::entnode
) {
    let mut sb : stat = ::std::mem::zeroed();
    if lstat(file,&mut sb as (*mut stat)) < 0i32 {
        if !(*__errno_location() == 2i32) {
            error(
                1i32,
                *__errno_location(),
                (*b"cannot stat temp file\0").as_ptr()
            );
        }
        if entdata == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::entries::entnode) {
            (*vers_ts).ts_user = 0i32 as (*mut u8);
        } else if !(*entdata).timestamp.is_null(
                   ) && (*(*entdata).timestamp.offset(
                              0isize
                          ) as (i32) == b'=' as (i32)) && (*(*entdata).timestamp.offset(
                                                                1isize
                                                            ) as (i32) == b'\0' as (i32)) {
            (*vers_ts).ts_user = Xstrdup((*vers_ts).ts_rcs as (*const u8));
        } else if !(*entdata).conflict.is_null(
                   ) && (*(*entdata).conflict.offset(
                              0isize
                          ) as (i32) == b'=' as (i32)) {
            (*vers_ts).ts_user = Xstrdup((*b"Is-modified\0").as_ptr());
            (*vers_ts).ts_conflict = Xstrdup((*b"Is-modified\0").as_ptr());
        } else if !(*entdata).timestamp.is_null(
                   ) && (*(*entdata).timestamp.offset(
                              0isize
                          ) as (i32) == b'M' as (i32) || *(*entdata).timestamp.offset(
                                                              0isize
                                                          ) as (i32) == b'D' as (i32)) && (*(*entdata).timestamp.offset(
                                                                                                1isize
                                                                                            ) as (i32) == b'\0' as (i32)) {
            (*vers_ts).ts_user = Xstrdup((*b"Is-modified\0").as_ptr());
        } else {
            (*vers_ts).ts_user = 0i32 as (*mut u8);
        }
    } else if sb.st_mtim.tv_sec == 0isize {
        abort();
    } else {
        (*vers_ts).ts_user = entries_time(sb.st_mtim.tv_sec);
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct tm {
    pub tm_sec : i32,
    pub tm_min : i32,
    pub tm_hour : i32,
    pub tm_mday : i32,
    pub tm_mon : i32,
    pub tm_year : i32,
    pub tm_wday : i32,
    pub tm_yday : i32,
    pub tm_isdst : i32,
    pub tm_gmtoff : isize,
    pub tm_zone : *const u8,
}

impl Clone for tm {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn entries_time(mut unixtime : isize) -> *mut u8 {
    let mut tm_p : *mut tm;
    let mut cp : *mut u8;
    tm_p = gmtime(&mut unixtime as (*mut isize) as (*const isize));
    cp = if !tm_p.is_null() {
             asctime(tm_p as (*const tm))
         } else {
             ctime(&mut unixtime as (*mut isize) as (*const isize))
         };
    *cp.offset(24isize) = b'\0';
    if *cp.offset(8isize) as (i32) == b'0' as (i32) {
        *cp.offset(8isize) = b' ';
    }
    Xasprintf((*b"%s\0").as_ptr(),cp)
}

#[no_mangle]
pub unsafe extern fn unix_time_stamp(mut file : *const u8) -> isize {
    let mut sb : stat = ::std::mem::zeroed();
    let mut mtime : isize = 0isize;
    if lstat(file,&mut sb as (*mut stat)) == 0 {
        mtime = sb.st_mtim.tv_sec;
    } else if !(*__errno_location() == 2i32) {
        error(
            0i32,
            *__errno_location(),
            (*b"cannot lstat %s\0").as_ptr(),
            file
        );
    }
    if stat(file,&mut sb as (*mut stat)) == 0 {
        if mtime < sb.st_mtim.tv_sec {
            mtime = sb.st_mtim.tv_sec;
        }
    } else if !(*__errno_location() == 2i32) {
        error(
            0i32,
            *__errno_location(),
            (*b"cannot stat %s\0").as_ptr(),
            file
        );
    }
    mtime
}

#[no_mangle]
pub unsafe extern fn time_stamp(mut file : *const u8) -> *mut u8 {
    let mut mtime : isize = unix_time_stamp(file);
    if mtime != 0 {
        entries_time(mtime)
    } else {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    }
}

#[no_mangle]
pub unsafe extern fn freevers_ts(mut versp : *mut *mut vers_ts) {
    if !(**versp).srcfile.is_null() {
        freercsnode(&mut (**versp).srcfile as (*mut *mut rcsnode));
    }
    if !(**versp).vn_user.is_null() {
        free((**versp).vn_user as (*mut ::std::os::raw::c_void));
    }
    if !(**versp).vn_rcs.is_null() {
        free((**versp).vn_rcs as (*mut ::std::os::raw::c_void));
    }
    if !(**versp).vn_tag.is_null() {
        free((**versp).vn_tag as (*mut ::std::os::raw::c_void));
    }
    if !(**versp).ts_user.is_null() {
        free((**versp).ts_user as (*mut ::std::os::raw::c_void));
    }
    if !(**versp).ts_rcs.is_null() {
        free((**versp).ts_rcs as (*mut ::std::os::raw::c_void));
    }
    if !(**versp).options.is_null() {
        free((**versp).options as (*mut ::std::os::raw::c_void));
    }
    if !(**versp).tag.is_null() {
        free((**versp).tag as (*mut ::std::os::raw::c_void));
    }
    if !(**versp).date.is_null() {
        free((**versp).date as (*mut ::std::os::raw::c_void));
    }
    if !(**versp).ts_conflict.is_null() {
        free((**versp).ts_conflict as (*mut ::std::os::raw::c_void));
    }
    free(*versp as (*mut u8) as (*mut ::std::os::raw::c_void));
    *versp = 0i32 as (*mut ::std::os::raw::c_void) as (*mut vers_ts);
}
