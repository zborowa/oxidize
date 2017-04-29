extern {
    fn Xasprintf(format : *const u8, ...) -> *mut u8;
    fn Xstrdup(str : *const u8) -> *mut u8;
    fn __assert_fail(
        __assertion : *const u8,
        __file : *const u8,
        __line : u32,
        __function : *const u8
    );
    fn __errno_location() -> *mut i32;
    fn base_remove(file : *const u8, rev : *const u8);
    fn ctime(__timer : *const isize) -> *mut u8;
    fn dir_append(dir : *const u8, append : *const u8) -> *mut u8;
    fn dir_append_dirs(dir : *const u8, ...) -> *mut u8;
    fn error(
        __status : i32, __errnum : i32, __format : *const u8, ...
    );
    fn fclose(__stream : *mut _IO_FILE) -> i32;
    fn feof_unlocked(__stream : *mut _IO_FILE) -> i32;
    fn fopen(
        __filename : *const u8, __modes : *const u8
    ) -> *mut _IO_FILE;
    fn fprintf(
        __stream : *mut _IO_FILE, __format : *const u8, ...
    ) -> i32;
    fn free(__ptr : *mut ::std::os::raw::c_void);
    fn getline(
        __lineptr : *mut *mut u8,
        __n : *mut usize,
        __stream : *mut _IO_FILE
    ) -> isize;
    fn isdir(file : *const u8) -> bool;
    fn isfile(file : *const u8) -> bool;
    static mut noexec : i32;
    fn quote(name : *const u8) -> *const u8;
    fn rename_file(from : *const u8, to : *const u8);
    static mut server_active : i32;
    fn server_clear_template(
        update_dir : *const u8, repository : *const u8
    );
    fn server_register(
        name : *const u8,
        version : *const u8,
        timestamp : *const u8,
        options : *const u8,
        tag : *const u8,
        date : *const u8,
        conflict : *const u8
    );
    fn server_scratch(name : *const u8);
    fn server_set_sticky(
        update_dir : *const u8,
        repository : *const u8,
        tag : *const u8,
        date : *const u8,
        nonbranch : i32
    );
    fn server_template(update_dir : *const u8, repository : *const u8);
    fn stat(__file : *const u8, __buf : *mut stat) -> i32;
    fn strchr(__s : *const u8, __c : i32) -> *mut u8;
    fn strcmp(__s1 : *const u8, __s2 : *const u8) -> i32;
    fn strlen(__s : *const u8) -> usize;
    fn strncmp(__s1 : *const u8, __s2 : *const u8, __n : usize) -> i32;
    fn unlink_file(f : *const u8) -> i32;
    fn xfopen(arg1 : *const u8, arg2 : *const u8) -> *mut _IO_FILE;
    fn xmalloc(s : usize) -> *mut ::std::os::raw::c_void;
    fn xzalloc(s : usize) -> *mut ::std::os::raw::c_void;
}

enum _IO_FILE {
}

static mut entfile : *mut _IO_FILE = 0 as (*mut _IO_FILE);

static mut entfilename : *mut u8 = 0 as (*mut u8);

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum ent_type {
    ENT_FILE,
    ENT_SUBDIR,
}

#[derive(Copy)]
#[repr(C)]
pub struct entnode {
    pub type_ : ent_type,
    pub user : *mut u8,
    pub version : *mut u8,
    pub timestamp : *mut u8,
    pub options : *mut u8,
    pub tag : *mut u8,
    pub date : *mut u8,
    pub conflict : *mut u8,
}

impl Clone for entnode {
    fn clone(&self) -> Self { *self }
}

unsafe extern fn fputentent(
    mut fp : *mut _IO_FILE, mut p : *mut entnode
) -> i32 {
    let switch1 = (*p).type_;
    if !(switch1 as (i32) == ent_type::ENT_FILE as (i32)) {
        if switch1 as (i32) == ent_type::ENT_SUBDIR as (i32) {
            if fprintf(fp,(*b"D\0").as_ptr()) < 0i32 {
                return 1i32;
            }
        }
    }
    if fprintf(
           fp,
           (*b"/%s/%s/%s\0").as_ptr(),
           (*p).user,
           (*p).version,
           (*p).timestamp
       ) < 0i32 {
        1i32
    } else {
        if !(*p).conflict.is_null() {
            if fprintf(fp,(*b"+%s\0").as_ptr(),(*p).conflict) < 0i32 {
                return 1i32;
            }
        }
        (if fprintf(fp,(*b"/%s/\0").as_ptr(),(*p).options) < 0i32 {
             1i32
         } else {
             if !(*p).tag.is_null() {
                 if fprintf(fp,(*b"T%s\n\0").as_ptr(),(*p).tag) < 0i32 {
                     return 1i32;
                 }
             } else if !(*p).date.is_null() {
                 if fprintf(fp,(*b"D%s\n\0").as_ptr(),(*p).date) < 0i32 {
                     return 1i32;
                 }
             } else if fprintf(fp,(*b"\n\0").as_ptr()) < 0i32 {
                 return 1i32;
             }
             0i32
         })
    }
}

unsafe extern fn write_ent_proc(
    mut node : *mut ::hash::hashnode,
    mut closure : *mut ::std::os::raw::c_void
) -> i32 {
    let mut entnode : *mut entnode = (*node).data as (*mut entnode);
    if closure != 0i32 as (*mut ::std::os::raw::c_void) && ((*entnode).type_ as (i32) != ent_type::ENT_FILE as (i32)) {
        *(closure as (*mut i32)) = 1i32;
    }
    if fputentent(entfile,entnode) != 0 {
        error(
            1i32,
            *__errno_location(),
            (*b"cannot write %s\0").as_ptr(),
            entfilename
        );
    }
    0i32
}

#[no_mangle]
pub unsafe extern fn Scratch_Entry(
    mut list : *mut ::hash::hashlist, mut fname : *const u8
) {
    let mut node : *mut ::hash::hashnode;
    if {
           node = ::hash::findnode_fn(list,fname);
           node
       } != 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode) {
        if noexec == 0 {
            let mut e : *mut entnode = (*node).data as (*mut entnode);
            base_remove(fname,(*e).version as (*const u8));
            entfilename = (*b"CVS/Entries.Log\0").as_ptr() as (*mut u8);
            entfile = xfopen(entfilename as (*const u8),(*b"a\0").as_ptr());
            if fprintf(entfile,(*b"R \0").as_ptr()) < 0i32 {
                error(
                    1i32,
                    *__errno_location(),
                    (*b"cannot write %s\0").as_ptr(),
                    entfilename
                );
            }
            write_ent_proc(node,0i32 as (*mut ::std::os::raw::c_void));
            if fclose(entfile) == -1i32 {
                error(
                    1i32,
                    *__errno_location(),
                    (*b"error closing %s\0").as_ptr(),
                    entfilename
                );
            }
        }
        ::hash::delnode(node);
        if server_active != 0 {
            server_scratch(fname);
        }
    }
}

unsafe extern fn Entnode_Create(
    mut type_ : ent_type,
    mut user : *const u8,
    mut vn : *const u8,
    mut ts : *const u8,
    mut options : *const u8,
    mut tag : *const u8,
    mut date : *const u8,
    mut ts_conflict : *const u8
) -> *mut entnode {
    let mut ent : *mut entnode;
    ent = xmalloc(::std::mem::size_of::<entnode>()) as (*mut entnode);
    (*ent).type_ = type_;
    (*ent).user = Xstrdup(user);
    (*ent).version = Xstrdup(vn);
    (*ent).timestamp = Xstrdup(
                           if !ts.is_null() { ts } else { (*b"\0").as_ptr() }
                       );
    (*ent).options = Xstrdup(
                         if !options.is_null() { options } else { (*b"\0").as_ptr() }
                     );
    (*ent).tag = Xstrdup(tag);
    (*ent).date = Xstrdup(date);
    (*ent).conflict = Xstrdup(ts_conflict);
    ent
}

unsafe extern fn Entnode_Destroy(mut ent : *mut entnode) {
    free((*ent).user as (*mut ::std::os::raw::c_void));
    free((*ent).version as (*mut ::std::os::raw::c_void));
    free((*ent).timestamp as (*mut ::std::os::raw::c_void));
    free((*ent).options as (*mut ::std::os::raw::c_void));
    if !(*ent).tag.is_null() {
        free((*ent).tag as (*mut ::std::os::raw::c_void));
    }
    if !(*ent).date.is_null() {
        free((*ent).date as (*mut ::std::os::raw::c_void));
    }
    if !(*ent).conflict.is_null() {
        free((*ent).conflict as (*mut ::std::os::raw::c_void));
    }
    free(ent as (*mut ::std::os::raw::c_void));
}

unsafe extern fn Entries_delproc(
    mut node : *mut ::hash::hashnode
) {
    let mut p : *mut entnode = (*node).data as (*mut entnode);
    Entnode_Destroy(p);
}

unsafe extern fn AddEntryNode(
    mut list : *mut ::hash::hashlist, mut entdata : *mut entnode
) -> *mut ::hash::hashnode {
    let mut p : *mut ::hash::hashnode;
    if {
           p = ::hash::findnode_fn(list,(*entdata).user as (*const u8));
           p
       } != 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode) {
        ::hash::delnode(p);
    }
    p = ::hash::getnode();
    (*p).type_ = ::hash::ntype::ENTRIES;
    (*p).delproc = Some(Entries_delproc);
    (*p).key = Xstrdup((*entdata).user as (*const u8));
    (*p).data = entdata as (*mut ::std::os::raw::c_void);
    ::hash::addnode(list,p);
    p
}

#[no_mangle]
pub unsafe extern fn Register(
    mut finfo : *const ::vers_ts::file_info,
    mut vn : *const u8,
    mut ts : *const u8,
    mut options : *const u8,
    mut tag : *const u8,
    mut date : *const u8,
    mut ts_conflict : *const u8
) {
    let mut entnode : *mut entnode;
    let mut node : *mut ::hash::hashnode;
    if server_active != 0 {
        server_register((*finfo).file,vn,ts,options,tag,date,ts_conflict);
    }
    entnode = Entnode_Create(
                  ent_type::ENT_FILE,
                  (*finfo).file,
                  vn,
                  ts,
                  options,
                  tag,
                  date,
                  ts_conflict
              );
    node = AddEntryNode((*finfo).entries,entnode);
    if noexec == 0 {
        entfilename = (*b"CVS/Entries.Log\0").as_ptr() as (*mut u8);
        entfile = fopen(entfilename as (*const u8),(*b"a\0").as_ptr());
        if entfile.is_null() {
            error(
                0i32,
                *__errno_location(),
                (*b"cannot open %s\0").as_ptr(),
                entfilename
            );
            return;
        } else {
            if fprintf(entfile,(*b"A \0").as_ptr()) < 0i32 {
                error(
                    1i32,
                    *__errno_location(),
                    (*b"cannot write %s\0").as_ptr(),
                    entfilename
                );
            }
            write_ent_proc(node,0i32 as (*mut ::std::os::raw::c_void));
            if fclose(entfile) == -1i32 {
                error(
                    1i32,
                    *__errno_location(),
                    (*b"error closing %s\0").as_ptr(),
                    entfilename
                );
            }
        }
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct stickydirtag {
    pub aflag : i32,
    pub tag : *mut u8,
    pub date : *mut u8,
    pub nonbranch : i32,
    pub subdirs : i32,
}

impl Clone for stickydirtag {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn entriesHasSticky(
    mut entries : *mut ::hash::hashlist
) -> bool {
    let mut sdtp : *mut stickydirtag;
    if entries.is_null() {
        false
    } else {
        if !(*entries).list.is_null() {
            0i32;
        } else {
            __assert_fail(
                (*b"entries->list\0").as_ptr(),
                file!().as_ptr(),
                line!(),
                (*b"entriesHasSticky\0").as_ptr()
            );
        }
        sdtp = (*(*entries).list).data as (*mut stickydirtag);
        !sdtp.is_null() && (!(*sdtp).tag.is_null(
                             ) || !(*sdtp).date.is_null())
    }
}

#[no_mangle]
pub unsafe extern fn entriesHasAllSubdirs(
    mut entries : *mut ::hash::hashlist
) -> bool {
    let mut sdtp : *mut stickydirtag;
    if !entries.is_null() {
        0i32;
    } else {
        __assert_fail(
            (*b"entries\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"entriesHasAllSubdirs\0").as_ptr()
        );
    }
    if !(*entries).list.is_null() {
        0i32;
    } else {
        __assert_fail(
            (*b"entries->list\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"entriesHasAllSubdirs\0").as_ptr()
        );
    }
    sdtp = (*(*entries).list).data as (*mut stickydirtag);
    sdtp.is_null() || (*sdtp).subdirs != 0
}

#[no_mangle]
pub unsafe extern fn entriesGetAflag(
    mut entries : *mut ::hash::hashlist
) -> bool {
    let mut sdtp : *mut stickydirtag;
    if entries.is_null() {
        false
    } else {
        if !(*entries).list.is_null() {
            0i32;
        } else {
            __assert_fail(
                (*b"entries->list\0").as_ptr(),
                file!().as_ptr(),
                line!(),
                (*b"entriesGetAflag\0").as_ptr()
            );
        }
        sdtp = (*(*entries).list).data as (*mut stickydirtag);
        (if sdtp.is_null() { false } else { (*sdtp).aflag != 0 })
    }
}

#[no_mangle]
pub unsafe extern fn entriesGetTag(
    mut entries : *mut ::hash::hashlist
) -> *const u8 {
    let mut sdtp : *mut stickydirtag;
    if !entries.is_null() {
        0i32;
    } else {
        __assert_fail(
            (*b"entries\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"entriesGetTag\0").as_ptr()
        );
    }
    if !(*entries).list.is_null() {
        0i32;
    } else {
        __assert_fail(
            (*b"entries->list\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"entriesGetTag\0").as_ptr()
        );
    }
    if !(*(*entries).list).data.is_null() {
        0i32;
    } else {
        __assert_fail(
            (*b"entries->list->data\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"entriesGetTag\0").as_ptr()
        );
    }
    sdtp = (*(*entries).list).data as (*mut stickydirtag);
    (*sdtp).tag as (*const u8)
}

#[no_mangle]
pub unsafe extern fn entriesGetNonbranch(
    mut entries : *mut ::hash::hashlist
) -> i32 {
    let mut sdtp : *mut stickydirtag;
    if !entries.is_null() {
        0i32;
    } else {
        __assert_fail(
            (*b"entries\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"entriesGetNonbranch\0").as_ptr()
        );
    }
    if !(*entries).list.is_null() {
        0i32;
    } else {
        __assert_fail(
            (*b"entries->list\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"entriesGetNonbranch\0").as_ptr()
        );
    }
    if !(*(*entries).list).data.is_null() {
        0i32;
    } else {
        __assert_fail(
            (*b"entries->list->data\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"entriesGetNonbranch\0").as_ptr()
        );
    }
    sdtp = (*(*entries).list).data as (*mut stickydirtag);
    (*sdtp).nonbranch
}

#[no_mangle]
pub unsafe extern fn entriesGetDate(
    mut entries : *mut ::hash::hashlist
) -> *const u8 {
    let mut sdtp : *mut stickydirtag;
    if !entries.is_null() {
        0i32;
    } else {
        __assert_fail(
            (*b"entries\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"entriesGetDate\0").as_ptr()
        );
    }
    if !(*entries).list.is_null() {
        0i32;
    } else {
        __assert_fail(
            (*b"entries->list\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"entriesGetDate\0").as_ptr()
        );
    }
    if !(*(*entries).list).data.is_null() {
        0i32;
    } else {
        __assert_fail(
            (*b"entries->list->data\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"entriesGetDate\0").as_ptr()
        );
    }
    sdtp = (*(*entries).list).data as (*mut stickydirtag);
    (*sdtp).date as (*const u8)
}

unsafe extern fn freesdt(mut p : *mut ::hash::hashnode) {
    let mut sdtp
        : *mut stickydirtag
        = (*p).data as (*mut stickydirtag);
    if !(*sdtp).tag.is_null() {
        free((*sdtp).tag as (*mut ::std::os::raw::c_void));
    }
    if !(*sdtp).date.is_null() {
        free((*sdtp).date as (*mut ::std::os::raw::c_void));
    }
    free(sdtp as (*mut u8) as (*mut ::std::os::raw::c_void));
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

unsafe extern fn strneq(
    mut a : *const u8, mut b : *const u8, mut n : usize
) -> bool {
    n == 0 || *a as (i32) == *b as (i32) && (strncmp(a,b,n) == 0i32)
}

unsafe extern fn fgetentent(
    mut fpin : *mut _IO_FILE, mut cmd : *mut u8, mut sawdir : *mut i32
) -> *mut entnode {
    let mut _currentBlock;
    let mut ent : *mut entnode;
    let mut line : *mut u8;
    let mut line_chars_allocated : usize;
    let mut cp : *mut u8 = ::std::ptr::null_mut();
    let mut type_ : ent_type = ent_type::ENT_FILE;
    let mut l : *mut u8;
    let mut user : *mut u8 = ::std::ptr::null_mut();
    let mut vn : *mut u8 = ::std::ptr::null_mut();
    let mut ts : *mut u8 = ::std::ptr::null_mut();
    let mut options : *mut u8 = ::std::ptr::null_mut();
    let mut tag_or_date : *mut u8 = ::std::ptr::null_mut();
    let mut tag : *mut u8;
    let mut date : *mut u8;
    let mut ts_conflict : *mut u8;
    let mut line_length : i32;
    line = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    line_chars_allocated = 0usize;
    ent = 0i32 as (*mut ::std::os::raw::c_void) as (*mut entnode);
    'loop1: loop {
        if !({
                 line_length = getline(
                                   &mut line as (*mut *mut u8),
                                   &mut line_chars_allocated as (*mut usize),
                                   fpin
                               ) as (i32);
                 line_length
             } > 0i32) {
            _currentBlock = 27;
            break;
        }
        l = line;
        if cmd != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            if *l.offset(1isize) as (i32) != b' ' as (i32) {
                *cmd = b'A';
            } else {
                *cmd = *l.offset(0isize);
                l = l.offset(2isize);
            }
        }
        type_ = ent_type::ENT_FILE;
        if *l.offset(0isize) as (i32) == b'D' as (i32) {
            type_ = ent_type::ENT_SUBDIR;
            *sawdir = 1i32;
            l = l.offset(1isize);
        }
        if *l.offset(0isize) as (i32) != b'/' as (i32) {
            continue;
        }
        user = l.offset(1isize);
        if {
               cp = strchr(user as (*const u8),b'/' as (i32));
               cp
           } == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            continue;
        }
        *{
             let _old = cp;
             cp = cp.offset(1isize);
             _old
         } = b'\0';
        vn = cp;
        if {
               cp = strchr(vn as (*const u8),b'/' as (i32));
               cp
           } == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            continue;
        }
        *{
             let _old = cp;
             cp = cp.offset(1isize);
             _old
         } = b'\0';
        ts = cp;
        if {
               cp = strchr(ts as (*const u8),b'/' as (i32));
               cp
           } == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            continue;
        }
        *{
             let _old = cp;
             cp = cp.offset(1isize);
             _old
         } = b'\0';
        options = cp;
        if {
               cp = strchr(options as (*const u8),b'/' as (i32));
               cp
           } == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            continue;
        }
        *{
             let _old = cp;
             cp = cp.offset(1isize);
             _old
         } = b'\0';
        tag_or_date = cp;
        if !({
                 cp = strchr(tag_or_date as (*const u8),b'\n' as (i32));
                 cp
             } == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)) {
            _currentBlock = 14;
            break;
        }
    }
    if _currentBlock == 14 {
        *cp = b'\0';
        tag = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        date = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        if *tag_or_date as (i32) == b'T' as (i32) {
            tag = tag_or_date.offset(1isize);
        } else if *tag_or_date as (i32) == b'D' as (i32) {
            date = tag_or_date.offset(1isize);
        }
        if !{
                ts_conflict = strchr(ts as (*const u8),b'+' as (i32));
                ts_conflict
            }.is_null(
            ) {
            *{
                 let _old = ts_conflict;
                 ts_conflict = ts_conflict.offset(1isize);
                 _old
             } = b'\0';
        }
        let mut sb : stat = ::std::mem::zeroed();
        if strlen(ts as (*const u8)) > 30usize && (stat(
                                                       user as (*const u8),
                                                       &mut sb as (*mut stat)
                                                   ) == 0i32) {
            let mut c
                : *mut u8
                = ctime(&mut sb.st_mtim.tv_sec as (*mut isize) as (*const isize));
            if *c.offset(8isize) as (i32) == b'0' as (i32) {
                *c.offset(8isize) = b' ';
            }
            if strneq(
                   ts.offset(25isize) as (*const u8),
                   c as (*const u8),
                   24usize
               ) {
                ts = ::vers_ts::time_stamp(user as (*const u8));
            } else {
                ts = ts.offset(24isize);
                *ts.offset(0isize) = b'*';
            }
        }
        ent = Entnode_Create(
                  type_,
                  user as (*const u8),
                  vn as (*const u8),
                  ts as (*const u8),
                  options as (*const u8),
                  tag as (*const u8),
                  date as (*const u8),
                  ts_conflict as (*const u8)
              );
    }
    if line_length < 0i32 && (feof_unlocked(fpin) == 0) {
        error(
            0i32,
            *__errno_location(),
            (*b"cannot read entries file\0").as_ptr()
        );
    }
    free(line as (*mut ::std::os::raw::c_void));
    ent
}

unsafe extern fn write_entries(
    mut list : *mut ::hash::hashlist,
    mut update_dir : *const u8,
    mut dir : *const u8
) {
    let mut sawdir : i32;
    let mut update_file : *mut u8;
    let mut bakfilename : *mut u8;
    if !update_dir.is_null() {
        0i32;
    } else {
        __assert_fail(
            (*b"update_dir\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"write_entries\0").as_ptr()
        );
    }
    if !dir.is_null() {
        0i32;
    } else {
        __assert_fail(
            (*b"dir\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"write_entries\0").as_ptr()
        );
    }
    sawdir = 0i32;
    bakfilename = dir_append(dir,(*b"CVS/Entries.Backup\0").as_ptr());
    update_file = dir_append_dirs(
                      update_dir,
                      dir,
                      (*b"CVS/Entries.Backup\0").as_ptr(),
                      0i32 as (*mut ::std::os::raw::c_void)
                  );
    entfile = fopen(bakfilename as (*const u8),(*b"w+\0").as_ptr());
    if entfile.is_null() {
        error(
            0i32,
            *__errno_location(),
            (*b"cannot rewrite %s\0").as_ptr(),
            quote(update_file as (*const u8))
        );
    } else {
        ::hash::walklist(
            list,
            write_ent_proc,
            &mut sawdir as (*mut i32) as (*mut ::std::os::raw::c_void)
        );
        if sawdir == 0 {
            if entriesHasAllSubdirs(list) {
                if fprintf(entfile,(*b"D\n\0").as_ptr()) < 0i32 {
                    error(
                        1i32,
                        *__errno_location(),
                        (*b"cannot write %s\0").as_ptr(),
                        quote(update_file as (*const u8))
                    );
                }
            }
        }
        if fclose(entfile) == -1i32 {
            error(
                1i32,
                *__errno_location(),
                (*b"error closing %s\0").as_ptr(),
                quote(update_file as (*const u8))
            );
        }
        let mut filename
            : *mut u8
            = dir_append(dir,(*b"CVS/Entries\0").as_ptr());
        rename_file(bakfilename as (*const u8),filename as (*const u8));
        free(filename as (*mut ::std::os::raw::c_void));
        filename = dir_append(dir,(*b"CVS/Entries.Log\0").as_ptr());
        if unlink_file(
               filename as (*const u8)
           ) < 0i32 && !(*__errno_location() == 2i32) {
            let mut newupdate_file
                : *mut u8
                = dir_append_dirs(
                      update_dir,
                      dir,
                      (*b"CVS/Entries.Log\0").as_ptr(),
                      0i32 as (*mut ::std::os::raw::c_void)
                  );
            error(
                0i32,
                *__errno_location(),
                (*b"cannot remove %s\0").as_ptr(),
                quote(newupdate_file as (*const u8))
            );
            free(newupdate_file as (*mut ::std::os::raw::c_void));
        }
        free(filename as (*mut ::std::os::raw::c_void));
    }
    free(bakfilename as (*mut ::std::os::raw::c_void));
    free(update_file as (*mut ::std::os::raw::c_void));
}

#[no_mangle]
pub unsafe extern fn Entries_Open_Dir(
    mut aflag : i32, mut update_dir_i : *const u8, mut dir : *const u8
) -> *mut ::hash::hashlist {
    let mut entries : *mut ::hash::hashlist;
    let mut sdtp
        : *mut stickydirtag
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut stickydirtag);
    let mut ent : *mut entnode;
    let mut dirtag : *mut u8 = ::std::ptr::null_mut();
    let mut dirdate : *mut u8 = ::std::ptr::null_mut();
    let mut dirnonbranch : i32 = 0;
    let mut do_rewrite : i32 = 0i32;
    let mut fpin : *mut _IO_FILE;
    let mut sawdir : i32;
    let mut filename : *mut u8;
    let mut update_dir : *mut u8;
    if !update_dir_i.is_null() {
        0i32;
    } else {
        __assert_fail(
            (*b"update_dir_i\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"Entries_Open_Dir\0").as_ptr()
        );
    }
    if !dir.is_null() {
        0i32;
    } else {
        __assert_fail(
            (*b"dir\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"Entries_Open_Dir\0").as_ptr()
        );
    }
    entries = ::hash::getlist();
    ParseTag(
        &mut dirtag as (*mut *mut u8),
        &mut dirdate as (*mut *mut u8),
        &mut dirnonbranch as (*mut i32)
    );
    if aflag != 0 || !dirtag.is_null() || !dirdate.is_null() {
        sdtp = xzalloc(
                   ::std::mem::size_of::<stickydirtag>()
               ) as (*mut stickydirtag);
        (*sdtp).aflag = aflag;
        (*sdtp).tag = Xstrdup(dirtag as (*const u8));
        (*sdtp).date = Xstrdup(dirdate as (*const u8));
        (*sdtp).nonbranch = dirnonbranch;
        (*(*entries).list).data = sdtp as (*mut ::std::os::raw::c_void);
        (*(*entries).list).delproc = Some(freesdt);
    }
    sawdir = 0i32;
    update_dir = dir_append(update_dir_i,dir);
    filename = dir_append(dir,(*b"CVS/Entries\0").as_ptr());
    fpin = fopen(filename as (*const u8),(*b"r\0").as_ptr());
    if !fpin.is_null() {
        'loop4: loop {
            if !({
                     ent = fgetentent(
                               fpin,
                               0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                               &mut sawdir as (*mut i32)
                           );
                     ent
                 } != 0i32 as (*mut ::std::os::raw::c_void) as (*mut entnode)) {
                break;
            }
            AddEntryNode(entries,ent);
        }
        if fclose(fpin) < 0i32 {
            let mut update_file
                : *mut u8
                = dir_append(
                      update_dir as (*const u8),
                      (*b"CVS/Entries\0").as_ptr()
                  );
            error(
                0i32,
                *__errno_location(),
                (*b"cannot close %s\0").as_ptr(),
                quote(update_file as (*const u8))
            );
            free(update_file as (*mut ::std::os::raw::c_void));
        }
    } else {
        let mut update_file
            : *mut u8
            = dir_append(
                  update_dir as (*const u8),
                  (*b"CVS/Entries\0").as_ptr()
              );
        error(
            0i32,
            *__errno_location(),
            (*b"cannot open %s for reading\0").as_ptr(),
            quote(update_file as (*const u8))
        );
        free(update_file as (*mut ::std::os::raw::c_void));
    }
    free(filename as (*mut ::std::os::raw::c_void));
    filename = dir_append(dir,(*b"CVS/Entries.Log\0").as_ptr());
    fpin = fopen(filename as (*const u8),(*b"r\0").as_ptr());
    if !fpin.is_null() {
        let mut cmd : u8 = 0;
        let mut node : *mut ::hash::hashnode;
        'loop9: loop {
            if !({
                     ent = fgetentent(
                               fpin,
                               &mut cmd as (*mut u8),
                               &mut sawdir as (*mut i32)
                           );
                     ent
                 } != 0i32 as (*mut ::std::os::raw::c_void) as (*mut entnode)) {
                break;
            }
            if cmd as (i32) == b'R' as (i32) {
                node = ::hash::findnode_fn(entries,(*ent).user as (*const u8));
                if !node.is_null() {
                    ::hash::delnode(node);
                }
                Entnode_Destroy(ent);
            } else if cmd as (i32) == b'A' as (i32) {
                AddEntryNode(entries,ent);
            } else {
                Entnode_Destroy(ent);
            }
        }
        do_rewrite = 1i32;
        if fclose(fpin) < 0i32 {
            let mut update_file
                : *mut u8
                = dir_append(
                      update_dir as (*const u8),
                      (*b"CVS/Entries.Log\0").as_ptr()
                  );
            error(
                0i32,
                *__errno_location(),
                (*b"cannot close %s\0").as_ptr(),
                quote(update_file as (*const u8))
            );
            free(update_file as (*mut ::std::os::raw::c_void));
        }
    }
    free(filename as (*mut ::std::os::raw::c_void));
    if !sdtp.is_null() {
        (*sdtp).subdirs = sawdir;
    } else if sawdir == 0 {
        sdtp = xzalloc(
                   ::std::mem::size_of::<stickydirtag>()
               ) as (*mut stickydirtag);
        (*sdtp).subdirs = 0i32;
        (*(*entries).list).data = sdtp as (*mut ::std::os::raw::c_void);
        (*(*entries).list).delproc = Some(freesdt);
    }
    if do_rewrite != 0 && (noexec == 0) {
        write_entries(entries,update_dir_i,dir);
    }
    free(update_dir as (*mut ::std::os::raw::c_void));
    if !dirtag.is_null() {
        free(dirtag as (*mut ::std::os::raw::c_void));
    }
    if !dirdate.is_null() {
        free(dirdate as (*mut ::std::os::raw::c_void));
    }
    entries
}

#[no_mangle]
pub unsafe extern fn Entries_Open(
    mut aflag : i32, mut update_dir : *const u8
) -> *mut ::hash::hashlist {
    Entries_Open_Dir(aflag,update_dir,(*b".\0").as_ptr())
}

#[no_mangle]
pub unsafe extern fn Entries_Close_Dir(
    mut list : *mut ::hash::hashlist,
    mut update_dir : *const u8,
    mut dir : *const u8
) {
    if !dir.is_null() {
        0i32;
    } else {
        __assert_fail(
            (*b"dir\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"Entries_Close_Dir\0").as_ptr()
        );
    }
    if !update_dir.is_null() {
        0i32;
    } else {
        __assert_fail(
            (*b"update_dir\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"Entries_Close_Dir\0").as_ptr()
        );
    }
    if !list.is_null() {
        if noexec == 0 {
            let mut filename
                : *mut u8
                = dir_append(dir,(*b"CVS/Entries.Log\0").as_ptr());
            if isfile(filename as (*const u8)) {
                write_entries(list,update_dir,dir);
            }
            free(filename as (*mut ::std::os::raw::c_void));
        }
        ::hash::dellist(&mut list as (*mut *mut ::hash::hashlist));
    }
}

#[no_mangle]
pub unsafe extern fn Entries_Close(
    mut list : *mut ::hash::hashlist, mut update_dir : *const u8
) {
    Entries_Close_Dir(list,update_dir,(*b".\0").as_ptr());
}

#[no_mangle]
pub unsafe extern fn WriteTemplate(
    mut update_dir : *const u8,
    mut xdotemplate : i32,
    mut repository : *const u8
) { if noexec != 0 {
    } else if server_active != 0 && (xdotemplate != 0) {
        server_clear_template(update_dir,repository);
        server_template(update_dir,repository);
    }
}

#[no_mangle]
pub unsafe extern fn WriteTag(
    mut dir : *const u8,
    mut tag : *const u8,
    mut date : *const u8,
    mut nonbranch : i32,
    mut update_dir : *const u8,
    mut repository : *const u8
) {
    let mut fout : *mut _IO_FILE;
    let mut tmp : *mut u8;
    if noexec != 0 {
    } else {
        if dir != 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
            tmp = Xasprintf(
                      (*b"%s/%s\0").as_ptr(),
                      dir,
                      (*b"CVS/Tag\0").as_ptr()
                  );
        } else {
            tmp = Xstrdup((*b"CVS/Tag\0").as_ptr());
        }
        if unlink_file(tmp as (*const u8)) < 0i32 && !(*__errno_location(
                                                        ) == 2i32) {
            error(
                1i32,
                *__errno_location(),
                (*b"cannot remove %s\0").as_ptr(),
                tmp
            );
        }
        if !tag.is_null() || !date.is_null() {
            fout = xfopen(tmp as (*const u8),(*b"w\0").as_ptr());
            if !tag.is_null() {
                if fprintf(
                       fout,
                       (*b"%c%s\n\0").as_ptr(),
                       if nonbranch != 0 { b'N' as (i32) } else { b'T' as (i32) },
                       tag
                   ) < 0i32 {
                    error(
                        1i32,
                        *__errno_location(),
                        (*b"write to %s failed\0").as_ptr(),
                        tmp
                    );
                }
            } else if fprintf(fout,(*b"D%s\n\0").as_ptr(),date) < 0i32 {
                error(
                    1i32,
                    *__errno_location(),
                    (*b"write to %s failed\0").as_ptr(),
                    tmp
                );
            }
            if fclose(fout) == -1i32 {
                error(
                    1i32,
                    *__errno_location(),
                    (*b"cannot close %s\0").as_ptr(),
                    tmp
                );
            }
        }
        free(tmp as (*mut ::std::os::raw::c_void));
        if server_active != 0 {
            server_set_sticky(update_dir,repository,tag,date,nonbranch);
        }
    }
}

#[no_mangle]
pub unsafe extern fn ParseTag(
    mut tagp : *mut *mut u8,
    mut datep : *mut *mut u8,
    mut nonbranchp : *mut i32
) {
    let mut fp : *mut _IO_FILE;
    if !tagp.is_null() {
        *tagp = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    }
    if !datep.is_null() {
        *datep = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    }
    if nonbranchp != 0i32 as (*mut ::std::os::raw::c_void) as (*mut i32) {
        *nonbranchp = 0i32;
    }
    fp = fopen((*b"CVS/Tag\0").as_ptr(),(*b"r\0").as_ptr());
    if !fp.is_null() {
        let mut line : *mut u8;
        let mut line_length : i32;
        let mut line_chars_allocated : usize;
        line = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        line_chars_allocated = 0usize;
        if {
               line_length = getline(
                                 &mut line as (*mut *mut u8),
                                 &mut line_chars_allocated as (*mut usize),
                                 fp
                             ) as (i32);
               line_length
           } > 0i32 {
            if *line.offset(
                    (line_length - 1i32) as (isize)
                ) as (i32) == b'\n' as (i32) {
                *line.offset(
                     {
                         line_length = line_length - 1;
                         line_length
                     } as (isize)
                 ) = b'\0';
            }
            let switch2 = *line;
            if switch2 as (i32) == b'N' as (i32) {
                if tagp != 0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut u8) {
                    *tagp = Xstrdup(line.offset(1isize) as (*const u8));
                }
                if nonbranchp != 0i32 as (*mut ::std::os::raw::c_void) as (*mut i32) {
                    *nonbranchp = 1i32;
                }
            } else if switch2 as (i32) == b'D' as (i32) {
                if datep != 0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut u8) {
                    *datep = Xstrdup(line.offset(1isize) as (*const u8));
                }
            } else if switch2 as (i32) == b'T' as (i32) {
                if tagp != 0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut u8) {
                    *tagp = Xstrdup(line.offset(1isize) as (*const u8));
                }
            }
        }
        if line_length < 0i32 {
            if feof_unlocked(fp) != 0 {
                error(
                    0i32,
                    0i32,
                    (*b"cannot read %s: end of file\0").as_ptr(),
                    (*b"CVS/Tag\0").as_ptr()
                );
            } else {
                error(
                    0i32,
                    *__errno_location(),
                    (*b"cannot read %s\0").as_ptr(),
                    (*b"CVS/Tag\0").as_ptr()
                );
            }
        }
        if fclose(fp) < 0i32 {
            error(
                0i32,
                *__errno_location(),
                (*b"cannot close %s\0").as_ptr(),
                (*b"CVS/Tag\0").as_ptr()
            );
        }
        free(line as (*mut ::std::os::raw::c_void));
    } else if !(*__errno_location() == 2i32) {
        error(
            0i32,
            *__errno_location(),
            (*b"cannot open %s\0").as_ptr(),
            (*b"CVS/Tag\0").as_ptr()
        );
    }
}

#[no_mangle]
pub unsafe extern fn Subdirs_Known(
    mut entries : *mut ::hash::hashlist
) {
    let mut sdtp
        : *mut stickydirtag
        = (*(*entries).list).data as (*mut stickydirtag);
    if sdtp != 0i32 as (*mut ::std::os::raw::c_void) as (*mut stickydirtag) && ((*sdtp).subdirs == 0) {
        let mut fp : *mut _IO_FILE;
        (*sdtp).subdirs = 1i32;
        if noexec == 0 {
            entfilename = (*b"CVS/Entries.Log\0").as_ptr() as (*mut u8);
            fp = fopen(entfilename as (*const u8),(*b"a\0").as_ptr());
            if fp == 0i32 as (*mut ::std::os::raw::c_void) as (*mut _IO_FILE) {
                let mut save_errno : i32 = *__errno_location();
                if !isdir((*b"CVS\0").as_ptr()) {
                    return;
                } else {
                    error(1i32,save_errno,(*b"cannot open %s\0").as_ptr(),entfilename);
                }
            } else if fclose(fp) == -1i32 {
                error(
                    1i32,
                    *__errno_location(),
                    (*b"cannot close %s\0").as_ptr(),
                    entfilename
                );
            }
        }
    }
}

unsafe extern fn subdir_record(
    mut cmd : i32, mut parent : *const u8, mut dir : *const u8
) -> *mut entnode {
    let mut entnode : *mut entnode;
    entnode = Entnode_Create(
                  ent_type::ENT_SUBDIR,
                  dir,
                  (*b"\0").as_ptr(),
                  (*b"\0").as_ptr(),
                  (*b"\0").as_ptr(),
                  0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                  0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                  0i32 as (*mut ::std::os::raw::c_void) as (*const u8)
              );
    if noexec == 0 {
        if parent == 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
            entfilename = (*b"CVS/Entries.Log\0").as_ptr() as (*mut u8);
        } else {
            entfilename = Xasprintf(
                              (*b"%s/%s\0").as_ptr(),
                              parent,
                              (*b"CVS/Entries.Log\0").as_ptr()
                          );
        }
        entfile = fopen(entfilename as (*const u8),(*b"a\0").as_ptr());
        if entfile == 0i32 as (*mut ::std::os::raw::c_void) as (*mut _IO_FILE) {
            let mut save_errno : i32 = *__errno_location();
            if parent == 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
                if !isdir((*b"CVS\0").as_ptr()) {
                    return entnode;
                }
            } else {
                free(entfilename as (*mut ::std::os::raw::c_void));
                entfilename = Xasprintf(
                                  (*b"%s/%s\0").as_ptr(),
                                  parent,
                                  (*b"CVS\0").as_ptr()
                              );
                if !isdir(entfilename as (*const u8)) {
                    free(entfilename as (*mut ::std::os::raw::c_void));
                    entfilename = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                    return entnode;
                }
            }
            error(1i32,save_errno,(*b"cannot open %s\0").as_ptr(),entfilename);
        }
        if fprintf(entfile,(*b"%c \0").as_ptr(),cmd) < 0i32 {
            error(
                1i32,
                *__errno_location(),
                (*b"cannot write %s\0").as_ptr(),
                entfilename
            );
        }
        if fputentent(entfile,entnode) != 0i32 {
            error(
                1i32,
                *__errno_location(),
                (*b"cannot write %s\0").as_ptr(),
                entfilename
            );
        }
        if fclose(entfile) == -1i32 {
            error(
                1i32,
                *__errno_location(),
                (*b"error closing %s\0").as_ptr(),
                entfilename
            );
        }
        if parent != 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
            free(entfilename as (*mut ::std::os::raw::c_void));
            entfilename = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        }
    }
    entnode
}

unsafe extern fn streq(
    mut a : *const u8, mut b : *const u8
) -> bool {
    *a as (i32) == *b as (i32) && (strcmp(a,b) == 0i32)
}

#[no_mangle]
pub unsafe extern fn Subdir_Register(
    mut entries : *mut ::hash::hashlist,
    mut parent : *const u8,
    mut dir : *const u8
) {
    let mut entnode : *mut entnode;
    if *dir.offset(0isize) as (i32) == b'.' as (i32) && (*dir.offset(
                                                              1isize
                                                          ) as (i32) == b'\0' as (i32)) {
    } else {
        entnode = subdir_record(b'A' as (i32),parent,dir);
        if !entries.is_null() && (parent.is_null() || streq(
                                                          parent,
                                                          (*b".\0").as_ptr()
                                                      )) {
            AddEntryNode(entries,entnode);
        } else {
            Entnode_Destroy(entnode);
        }
    }
}

#[no_mangle]
pub unsafe extern fn Subdir_Deregister(
    mut entries : *mut ::hash::hashlist,
    mut parent : *const u8,
    mut dir : *const u8
) {
    let mut entnode : *mut entnode;
    entnode = subdir_record(b'R' as (i32),parent,dir);
    Entnode_Destroy(entnode);
    if !entries.is_null() && (parent.is_null() || streq(
                                                      parent,
                                                      (*b".\0").as_ptr()
                                                  )) {
        let mut p : *mut ::hash::hashnode;
        p = ::hash::findnode_fn(entries,dir);
        if p != 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode) {
            ::hash::delnode(p);
        }
    }
}
