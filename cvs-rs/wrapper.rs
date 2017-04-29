extern {
    fn Xasprintf(format : *const u8, ...) -> *mut u8;
    fn Xstrdup(str : *const u8) -> *mut u8;
    fn __errno_location() -> *mut i32;
    fn abort() -> !;
    static mut current_parsed_root : *mut cvsroot_s;
    fn cvs_temp_name() -> *mut u8;
    fn error(
        __status : i32, __errnum : i32, __format : *const u8, ...
    );
    fn expand_path(
        name : *const u8,
        cvsroot : *const u8,
        formatsafe : bool,
        file : *const u8,
        line : i32
    ) -> *mut u8;
    fn fclose(__stream : *mut _IO_FILE) -> i32;
    fn ferror_unlocked(__stream : *mut _IO_FILE) -> i32;
    fn fnmatch(
        __pattern : *const u8, __name : *const u8, __flags : i32
    ) -> i32;
    fn fopen(
        __filename : *const u8, __modes : *const u8
    ) -> *mut _IO_FILE;
    fn free(__ptr : *mut ::std::os::raw::c_void);
    fn get_homedir() -> *mut u8;
    fn getenv(__name : *const u8) -> *mut u8;
    fn getline(
        __lineptr : *mut *mut u8,
        __n : *mut usize,
        __stream : *mut _IO_FILE
    ) -> isize;
    fn isfile(file : *const u8) -> bool;
    fn isspace(arg1 : i32) -> i32;
    fn run_exec(
        stin : *const u8, stout : *const u8, sterr : *const u8, flags : i32
    ) -> i32;
    fn run_setup(prog : *const u8);
    fn send_to_server(str : *const u8, len : usize);
    fn strcat_filename_onto_homedir(
        arg1 : *const u8, arg2 : *const u8
    ) -> *mut u8;
    fn strcmp(__s1 : *const u8, __s2 : *const u8) -> i32;
    fn xalloc_die();
    fn xmalloc(s : usize) -> *mut ::std::os::raw::c_void;
    fn xrealloc(
        p : *mut ::std::os::raw::c_void, s : usize
    ) -> *mut ::std::os::raw::c_void;
}

enum _IO_FILE {
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Enum2 {
    WRAP_MERGE,
    WRAP_COPY,
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct1 {
    pub wildCard : *mut u8,
    pub tocvsFilter : *mut u8,
    pub fromcvsFilter : *mut u8,
    pub rcsOption : *mut u8,
    pub mergeMethod : Enum2,
}

impl Clone for Struct1 {
    fn clone(&self) -> Self { *self }
}

static mut wrap_list
    : *mut *mut Struct1
    = 0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut Struct1);

static mut wrap_saved_list
    : *mut *mut Struct1
    = 0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut Struct1);

static mut wrap_size : i32 = 0i32;

static mut wrap_count : i32 = 0i32;

static mut wrap_tempcount : i32 = 0i32;

static mut wrap_saved_count : i32 = 0i32;

static mut wrap_saved_tempcount : i32 = 0i32;

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Enum3 {
    null_method = 0i32,
    local_method,
    server_method,
    pserver_method,
    kserver_method,
    gserver_method,
    ext_method,
    extssh_method,
    fork_method,
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Enum4 {
    SIGN_DEFAULT,
    SIGN_ALWAYS,
    SIGN_NEVER,
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Enum5 {
    VERIFY_DEFAULT = 0i32,
    VERIFY_OFF,
    VERIFY_WARN,
    VERIFY_FATAL,
}

#[derive(Copy)]
#[repr(C)]
pub struct cvsroot_s {
    pub original : *mut u8,
    pub method : Enum3,
    pub directory : *mut u8,
    pub isremote : bool,
    pub sign : Enum4,
    pub sign_template : *mut u8,
    pub sign_args : *mut ::hash::hashlist,
    pub openpgp_textmode : *mut u8,
    pub verify : Enum5,
    pub verify_template : *mut u8,
    pub verify_args : *mut ::hash::hashlist,
    pub username : *mut u8,
    pub password : *mut u8,
    pub hostname : *mut u8,
    pub cvs_rsh : *mut u8,
    pub cvs_server : *mut u8,
    pub port : i32,
    pub proxy_hostname : *mut u8,
    pub proxy_port : i32,
    pub redirect : bool,
}

impl Clone for cvsroot_s {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn wrap_setup() {
    static mut wrap_setup_already_done : i32 = 0i32;
    let mut homedir : *mut u8;
    if wrap_setup_already_done != 0i32 {
    } else {
        wrap_setup_already_done = 1i32;
        if !(*current_parsed_root).isremote {
            let mut file : *mut u8;
            file = Xasprintf(
                       (*b"%s/%s/%s\0").as_ptr(),
                       (*current_parsed_root).directory,
                       (*b"CVSROOT\0").as_ptr(),
                       (*b"cvswrappers\0").as_ptr()
                   );
            if isfile(file as (*const u8)) {
                wrap_add_file(file as (*const u8),0i32);
            }
            free(file as (*mut ::std::os::raw::c_void));
        }
        homedir = get_homedir();
        if homedir != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            let mut file
                : *mut u8
                = strcat_filename_onto_homedir(
                      homedir as (*const u8),
                      (*b".cvswrappers\0").as_ptr()
                  );
            if isfile(file as (*const u8)) {
                wrap_add_file(file as (*const u8),0i32);
            }
            free(file as (*mut ::std::os::raw::c_void));
        }
        wrap_add(getenv((*b"CVSWRAPPERS\0").as_ptr()),0i32);
    }
}

#[no_mangle]
pub unsafe extern fn wrap_send() {
    let mut i : i32;
    i = 0i32;
    'loop1: loop {
        if !(i < wrap_count + wrap_tempcount) {
            break;
        }
        if (**wrap_list.offset(
                  i as (isize)
              )).tocvsFilter != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) || (**wrap_list.offset(
                                                                                             i as (isize)
                                                                                         )).fromcvsFilter != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            error(
                0i32,
                0i32,
                (*b"-t and -f wrapper options are not supported remotely; ignored\0").as_ptr(
                )
            );
        }
        if (**wrap_list.offset(
                  i as (isize)
              )).mergeMethod as (i32) == Enum2::WRAP_COPY as (i32) {
            error(
                0i32,
                0i32,
                (*b"-m wrapper option is not supported remotely; ignored\0").as_ptr(
                )
            );
        }
        send_to_server((*b"Argument -W\nArgument \0").as_ptr(),0usize);
        send_to_server(
            (**wrap_list.offset(i as (isize))).wildCard as (*const u8),
            0usize
        );
        send_to_server((*b" -k \'\0").as_ptr(),0usize);
        if (**wrap_list.offset(
                  i as (isize)
              )).rcsOption != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            send_to_server(
                (**wrap_list.offset(i as (isize))).rcsOption as (*const u8),
                0usize
            );
        } else {
            send_to_server((*b"kv\0").as_ptr(),0usize);
        }
        send_to_server((*b"\'\n\0").as_ptr(),0usize);
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern fn wrap_unparse_rcs_options(
    mut line : *mut *mut u8, mut first_call_p : i32
) {
    static mut i : i32 = 0i32;
    if first_call_p != 0 {
        i = 0i32;
    }
    if i >= wrap_count + wrap_tempcount {
        *line = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    } else {
        *line = Xasprintf(
                    (*b"%s -k \'%s\'\0").as_ptr(),
                    (**wrap_list.offset(i as (isize))).wildCard,
                    if !(**wrap_list.offset(i as (isize))).rcsOption.is_null() {
                        (**wrap_list.offset(i as (isize))).rcsOption as (*const u8)
                    } else {
                        (*b"kv\0").as_ptr()
                    }
                );
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern fn wrap_add_file(
    mut file : *const u8, mut temp : i32
) {
    let mut fp : *mut _IO_FILE;
    let mut line
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut line_allocated : usize = 0usize;
    wrap_restore_saved();
    wrap_kill_temp();
    fp = fopen(file,(*b"r\0").as_ptr());
    if fp == 0i32 as (*mut ::std::os::raw::c_void) as (*mut _IO_FILE) {
        if !(*__errno_location() == 2i32) {
            error(
                0i32,
                *__errno_location(),
                (*b"cannot open %s\0").as_ptr(),
                file
            );
        }
    } else {
        'loop1: loop {
            if !(getline(
                     &mut line as (*mut *mut u8),
                     &mut line_allocated as (*mut usize),
                     fp
                 ) >= 0isize) {
                break;
            }
            wrap_add(line,temp);
        }
        if !line.is_null() {
            free(line as (*mut ::std::os::raw::c_void));
        }
        if ferror_unlocked(fp) != 0 {
            error(
                0i32,
                *__errno_location(),
                (*b"cannot read %s\0").as_ptr(),
                file
            );
        }
        if fclose(fp) == -1i32 {
            error(
                0i32,
                *__errno_location(),
                (*b"cannot close %s\0").as_ptr(),
                file
            );
        }
    }
}

#[no_mangle]
pub unsafe extern fn wrap_kill() {
    wrap_kill_temp();
    'loop1: loop {
        if wrap_count == 0 {
            break;
        }
        wrap_free_entry(
            *wrap_list.offset(
                 {
                     wrap_count = wrap_count - 1;
                     wrap_count
                 } as (isize)
             )
        );
    }
}

#[no_mangle]
pub unsafe extern fn wrap_kill_temp() {
    let mut temps
        : *mut *mut Struct1
        = wrap_list.offset(wrap_count as (isize));
    'loop1: loop {
        if wrap_tempcount == 0 {
            break;
        }
        wrap_free_entry(
            *temps.offset(
                 {
                     wrap_tempcount = wrap_tempcount - 1;
                     wrap_tempcount
                 } as (isize)
             )
        );
    }
}

#[no_mangle]
pub unsafe extern fn wrap_free_entry(mut e : *mut Struct1) {
    wrap_free_entry_internal(e);
    free(e as (*mut ::std::os::raw::c_void));
}

#[no_mangle]
pub unsafe extern fn wrap_free_entry_internal(mut e : *mut Struct1) {
    free((*e).wildCard as (*mut ::std::os::raw::c_void));
    if !(*e).tocvsFilter.is_null() {
        free((*e).tocvsFilter as (*mut ::std::os::raw::c_void));
    }
    if !(*e).fromcvsFilter.is_null() {
        free((*e).fromcvsFilter as (*mut ::std::os::raw::c_void));
    }
    if !(*e).rcsOption.is_null() {
        free((*e).rcsOption as (*mut ::std::os::raw::c_void));
    }
}

#[no_mangle]
pub unsafe extern fn wrap_restore_saved() {
    if wrap_saved_list.is_null() {
    } else {
        wrap_kill();
        free(wrap_list as (*mut ::std::os::raw::c_void));
        wrap_list = wrap_saved_list;
        wrap_count = wrap_saved_count;
        wrap_tempcount = wrap_saved_tempcount;
        wrap_saved_list = 0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut Struct1);
        wrap_saved_count = 0i32;
        wrap_saved_tempcount = 0i32;
    }
}

unsafe extern fn streq(
    mut a : *const u8, mut b : *const u8
) -> bool {
    *a as (i32) == *b as (i32) && (strcmp(a,b) == 0i32)
}

#[no_mangle]
pub unsafe extern fn wrap_add(mut line : *mut u8, mut isTemp : i32) {
    let mut temp : *mut u8;
    let mut ctemp : u8;
    let mut e : Struct1 = ::std::mem::zeroed();
    let mut opt : u8;
    if line.is_null() || *line.offset(
                              0isize
                          ) as (i32) == b'#' as (i32) {
    } else {
        'loop2: loop {
            if !(*line != 0 && (isspace(*line as (i32)) != 0)) {
                break;
            }
            line = line.offset(1isize);
        }
        temp = line;
        'loop4: loop {
            if !(*line != 0 && (isspace(*line as (i32)) == 0)) {
                break;
            }
            line = line.offset(1isize);
        }
        (if temp == line {
         } else {
             ctemp = *line;
             *line = b'\0';
             e.wildCard = Xstrdup(temp as (*const u8));
             *line = ctemp;
             'loop7: loop {
                 if *line == 0 {
                     break;
                 }
                 'loop8: loop {
                     if !(*line != 0 && (*line as (i32) != b'-' as (i32))) {
                         break;
                     }
                     line = line.offset(1isize);
                 }
                 if *line == 0 {
                     break;
                 }
                 line = line.offset(1isize);
                 if *line == 0 {
                     break;
                 }
                 opt = *line;
                 line = line.offset(1isize);
                 'loop12: loop {
                     if !(*line != 0 && (*line as (i32) != b'\'' as (i32))) {
                         break;
                     }
                     line = line.offset(1isize);
                 }
                 if *line == 0 {
                     break;
                 }
                 temp = {
                            line = line.offset(1isize);
                            line
                        };
                 'loop15: loop {
                     if !(*line != 0 && (*line as (i32) != b'\'' as (i32) || *line.offset(
                                                                                  -1isize
                                                                              ) as (i32) == b'\\' as (i32))) {
                         break;
                     }
                     line = line.offset(1isize);
                 }
                 if line == temp {
                     break;
                 }
                 ctemp = *line;
                 *line = b'\0';
                 if opt as (i32) == b'k' as (i32) {
                     if !e.rcsOption.is_null() {
                         free(e.rcsOption as (*mut ::std::os::raw::c_void));
                     }
                     e.rcsOption = if streq(temp as (*const u8),(*b"kv\0").as_ptr()) {
                                       0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                                   } else {
                                       Xstrdup(temp as (*const u8))
                                   };
                 } else if opt as (i32) == b'm' as (i32) {
                     if *temp as (i32) == b'C' as (i32) || *temp as (i32) == b'c' as (i32) {
                         e.mergeMethod = Enum2::WRAP_COPY;
                     } else {
                         e.mergeMethod = Enum2::WRAP_MERGE;
                     }
                 } else if opt as (i32) == b't' as (i32) {
                     error(
                         1i32,
                         0i32,
                         (*b"-t/-f wrappers not supported by this version of CVS\0").as_ptr(
                         )
                     );
                     if !e.tocvsFilter.is_null() {
                         free(e.tocvsFilter as (*mut ::std::os::raw::c_void));
                     }
                     e.tocvsFilter = expand_path(
                                         temp as (*const u8),
                                         (*current_parsed_root).directory as (*const u8),
                                         false,
                                         (*b"<wrapper>\0").as_ptr(),
                                         0i32
                                     );
                     if e.tocvsFilter.is_null() {
                         error(1i32,0i32,(*b"Correct above errors first\0").as_ptr());
                     }
                 } else if opt as (i32) == b'f' as (i32) {
                     error(
                         1i32,
                         0i32,
                         (*b"-t/-f wrappers not supported by this version of CVS\0").as_ptr(
                         )
                     );
                     if !e.fromcvsFilter.is_null() {
                         free(e.fromcvsFilter as (*mut ::std::os::raw::c_void));
                     }
                     e.fromcvsFilter = expand_path(
                                           temp as (*const u8),
                                           (*current_parsed_root).directory as (*const u8),
                                           false,
                                           (*b"<wrapper>\0").as_ptr(),
                                           0i32
                                       );
                     if e.fromcvsFilter.is_null() {
                         error(1i32,0i32,(*b"Correct above errors first\0").as_ptr());
                     }
                 }
                 *line = ctemp;
                 if *line == 0 {
                     break;
                 }
                 line = line.offset(1isize);
             }
             wrap_add_entry(&mut e as (*mut Struct1),isTemp);
         })
    }
}

unsafe extern fn xnrealloc(
    mut p : *mut ::std::os::raw::c_void, mut n : usize, mut s : usize
) -> *mut ::std::os::raw::c_void {
    if (if ::std::mem::size_of::<isize>(
           ) <= ::std::mem::size_of::<usize>() {
            -1i32
        } else {
            -2i32
        } as (usize)).wrapping_div(
           s
       ) < n {
        xalloc_die();
    }
    xrealloc(p,n.wrapping_mul(s))
}

#[no_mangle]
pub unsafe extern fn wrap_add_entry(
    mut e : *mut Struct1, mut temp : i32
) {
    let mut x : i32;
    if wrap_count + wrap_tempcount >= wrap_size {
        wrap_size = wrap_size + 8i32;
        wrap_list = xnrealloc(
                        wrap_list as (*mut ::std::os::raw::c_void),
                        wrap_size as (usize),
                        ::std::mem::size_of::<*mut Struct1>()
                    ) as (*mut *mut Struct1);
    }
    if temp == 0 && (wrap_tempcount != 0) {
        x = wrap_count + wrap_tempcount - 1i32;
        'loop4: loop {
            if !(x >= wrap_count) {
                break;
            }
            *wrap_list.offset((x + 1i32) as (isize)) = *wrap_list.offset(
                                                            x as (isize)
                                                        );
            x = x - 1;
        }
    }
    x = if temp != 0 {
            wrap_count + {
                             let _old = wrap_tempcount;
                             wrap_tempcount = wrap_tempcount + 1;
                             _old
                         }
        } else {
            ({
                 let _old = wrap_count;
                 wrap_count = wrap_count + 1;
                 _old
             })
        };
    *wrap_list.offset(x as (isize)) = xmalloc(
                                          ::std::mem::size_of::<Struct1>()
                                      ) as (*mut Struct1);
    **wrap_list.offset(x as (isize)) = *e;
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Enum6 {
    WRAP_TOCVS,
    WRAP_FROMCVS,
    WRAP_RCSOPTION,
}

#[no_mangle]
pub unsafe extern fn wrap_name_has(
    mut name : *const u8, mut has : Enum6
) -> i32 {
    let mut _currentBlock;
    let mut x : i32;
    let mut count : i32 = wrap_count + wrap_tempcount;
    let mut temp : *mut u8;
    x = 0i32;
    'loop1: loop {
        if !(x < count) {
            _currentBlock = 2;
            break;
        }
        if fnmatch(
               (**wrap_list.offset(x as (isize))).wildCard as (*const u8),
               name,
               0i32
           ) == 0i32 {
            _currentBlock = 5;
            break;
        }
        x = x + 1;
    }
    if _currentBlock == 2 {
        0i32
    } else {
        if has as (i32) == Enum6::WRAP_RCSOPTION as (i32) {
            temp = (**wrap_list.offset(x as (isize))).rcsOption;
        } else if has as (i32) == Enum6::WRAP_FROMCVS as (i32) {
            temp = (**wrap_list.offset(x as (isize))).fromcvsFilter;
        } else if has as (i32) == Enum6::WRAP_TOCVS as (i32) {
            temp = (**wrap_list.offset(x as (isize))).tocvsFilter;
        } else {
            abort();
        }
        (if temp == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
             0i32
         } else {
             1i32
         })
    }
}

unsafe extern fn wrap_matching_entry(
    mut name : *const u8
) -> *mut Struct1 {
    let mut _currentBlock;
    let mut x : i32;
    let mut count : i32 = wrap_count + wrap_tempcount;
    x = 0i32;
    'loop1: loop {
        if !(x < count) {
            _currentBlock = 2;
            break;
        }
        if fnmatch(
               (**wrap_list.offset(x as (isize))).wildCard as (*const u8),
               name,
               0i32
           ) == 0i32 {
            _currentBlock = 5;
            break;
        }
        x = x + 1;
    }
    if _currentBlock == 2 {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut Struct1)
    } else {
        *wrap_list.offset(x as (isize))
    }
}

#[no_mangle]
pub unsafe extern fn wrap_rcsoption(
    mut filename : *const u8, mut asflag : i32
) -> *mut u8 {
    let mut e : *mut Struct1 = wrap_matching_entry(filename);
    if e == 0i32 as (*mut ::std::os::raw::c_void) as (*mut Struct1) || (*e).rcsOption == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) || *(*e).rcsOption as (i32) == b'\0' as (i32) {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        Xasprintf(
            (*b"%s%s\0").as_ptr(),
            if asflag != 0 { (*b"-k\0").as_ptr() } else { (*b"\0").as_ptr() },
            (*e).rcsOption
        )
    }
}

unsafe extern fn wrap_clean_fmt_str(
    mut fmt : *mut u8, mut max_s : i32
) {
    'loop0: loop {
        if *fmt == 0 {
            break;
        }
        if *fmt.offset(0isize) as (i32) == b'%' as (i32) && (*fmt.offset(
                                                                  1isize
                                                              ) != 0) {
            if *fmt.offset(1isize) as (i32) == b'%' as (i32) {
                fmt = fmt.offset(1isize);
            } else if *fmt.offset(
                           1isize
                       ) as (i32) == b's' as (i32) && (max_s > 0i32) {
                max_s = max_s - 1;
                fmt = fmt.offset(1isize);
            } else {
                *fmt = b' ';
            }
        }
        fmt = fmt.offset(1isize);
    }
}

#[no_mangle]
pub unsafe extern fn wrap_tocvs_process_file(
    mut fileName : *const u8
) -> *mut u8 {
    let mut e : *mut Struct1 = wrap_matching_entry(fileName);
    static mut buf
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut args : *mut u8;
    if e == 0i32 as (*mut ::std::os::raw::c_void) as (*mut Struct1) || (*e).tocvsFilter == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        if buf != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            free(buf as (*mut ::std::os::raw::c_void));
        }
        buf = cvs_temp_name();
        wrap_clean_fmt_str((*e).tocvsFilter,2i32);
        args = Xasprintf((*e).tocvsFilter as (*const u8),fileName,buf);
        run_setup(args as (*const u8));
        run_exec(
            0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
            0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
            0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
            0x0i32 | 0x1i32 << 1i32
        );
        free(args as (*mut ::std::os::raw::c_void));
        buf
    }
}

#[no_mangle]
pub unsafe extern fn wrap_merge_is_copy(
    mut fileName : *const u8
) -> i32 {
    let mut e : *mut Struct1 = wrap_matching_entry(fileName);
    if e == 0i32 as (*mut ::std::os::raw::c_void) as (*mut Struct1) || (*e).mergeMethod as (i32) == Enum2::WRAP_MERGE as (i32) {
        0i32
    } else {
        1i32
    }
}

#[no_mangle]
pub unsafe extern fn wrap_fromcvs_process_file(
    mut fileName : *const u8
) {
    let mut args : *mut u8;
    let mut e : *mut Struct1 = wrap_matching_entry(fileName);
    if e != 0i32 as (*mut ::std::os::raw::c_void) as (*mut Struct1) && ((*e).fromcvsFilter != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)) {
        wrap_clean_fmt_str((*e).fromcvsFilter,1i32);
        args = Xasprintf((*e).fromcvsFilter as (*const u8),fileName);
        run_setup(args as (*const u8));
        run_exec(
            0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
            0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
            0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
            0x0i32
        );
        free(args as (*mut ::std::os::raw::c_void));
    }
}
