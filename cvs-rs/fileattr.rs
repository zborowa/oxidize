extern {
    fn Parse_Info(
        infofile : *const u8,
        repository : *const u8,
        callproc
        :
        unsafe extern fn(*const u8, *const u8, *const u8, i32, *mut ::std::os::raw::c_void) -> i32,
        opt : i32,
        closure : *mut ::std::os::raw::c_void
    ) -> i32;
    fn Short_Repository(repository : *const u8) -> *const u8;
    fn Xasprintf(format : *const u8, ...) -> *mut u8;
    fn Xstrdup(str : *const u8) -> *mut u8;
    fn __assert_fail(
        __assertion : *const u8,
        __file : *const u8,
        __line : u32,
        __function : *const u8
    );
    fn __errno_location() -> *mut i32;
    fn abs(__x : i32) -> i32;
    static mut current_parsed_root : *mut cvsroot_s;
    static mut cvs_cmd_name : *const u8;
    fn cvs_mkdir(
        name : *const u8, update_dir : *const u8, flags : u32
    ) -> bool;
    static mut cvsumask : u32;
    fn error(
        __status : i32, __errnum : i32, __format : *const u8, ...
    );
    fn fclose(__stream : *mut _IO_FILE) -> i32;
    fn ferror_unlocked(__stream : *mut _IO_FILE) -> i32;
    fn fopen(
        __filename : *const u8, __modes : *const u8
    ) -> *mut _IO_FILE;
    fn format_cmdline(
        oldway : bool,
        srepos : *const u8,
        file : *const u8,
        line : i32,
        format : *const u8,
        ...
    ) -> *mut u8;
    fn fputs_unlocked(
        __s : *const u8, __stream : *mut _IO_FILE
    ) -> i32;
    fn free(__ptr : *mut ::std::os::raw::c_void);
    fn getline(
        __lineptr : *mut *mut u8,
        __n : *mut usize,
        __stream : *mut _IO_FILE
    ) -> isize;
    fn isdir(file : *const u8) -> bool;
    static mut noexec : i32;
    fn primary_root_inverse_translate(
        root_in : *const u8
    ) -> *const u8;
    static mut referrer : *mut cvsroot_s;
    fn rmdir(__path : *const u8) -> i32;
    fn run_exec(
        stin : *const u8, stout : *const u8, sterr : *const u8, flags : i32
    ) -> i32;
    fn run_setup(prog : *const u8);
    fn strcat(__dest : *mut u8, __src : *const u8) -> *mut u8;
    fn strchr(__s : *const u8, __c : i32) -> *mut u8;
    fn strcpy(__dest : *mut u8, __src : *const u8) -> *mut u8;
    fn strlen(__s : *const u8) -> usize;
    fn strncmp(__s1 : *const u8, __s2 : *const u8, __n : usize) -> i32;
    fn strncpy(
        __dest : *mut u8, __src : *const u8, __n : usize
    ) -> *mut u8;
    fn umask(__mask : u32) -> u32;
    fn unlink_file(f : *const u8) -> i32;
    fn xmalloc(s : usize) -> *mut ::std::os::raw::c_void;
}

enum _IO_FILE {
}

static mut fileattr_stored_repos : *mut u8 = 0 as (*mut u8);

static mut attrlist
    : *mut ::hash::hashlist
    = 0 as (*mut ::hash::hashlist);

static mut fileattr_default_attrs : *mut u8 = 0 as (*mut u8);

static mut attr_read_attempted : i32 = 0i32;

static mut attrs_modified : i32 = 0i32;

#[derive(Copy)]
#[repr(C)]
pub struct unrecog {
    pub line : *mut u8,
    pub next : *mut unrecog,
}

impl Clone for unrecog {
    fn clone(&self) -> Self { *self }
}

static mut unrecog_head : *mut unrecog = 0 as (*mut unrecog);

#[no_mangle]
pub unsafe extern fn fileattr_startdir(mut repos : *const u8) {
    if fileattr_stored_repos == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        0i32;
    } else {
        __assert_fail(
            (*b"fileattr_stored_repos == NULL\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"fileattr_startdir\0").as_ptr()
        );
    }
    fileattr_stored_repos = Xstrdup(repos);
    if attrlist == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashlist) {
        0i32;
    } else {
        __assert_fail(
            (*b"attrlist == NULL\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"fileattr_startdir\0").as_ptr()
        );
    }
    attr_read_attempted = 0i32;
    if unrecog_head == 0i32 as (*mut ::std::os::raw::c_void) as (*mut unrecog) {
        0i32;
    } else {
        __assert_fail(
            (*b"unrecog_head == NULL\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"fileattr_startdir\0").as_ptr()
        );
    }
}

unsafe extern fn fileattr_delproc(
    mut node : *mut ::hash::hashnode
) {
    if !(*node).data.is_null() {
        0i32;
    } else {
        __assert_fail(
            (*b"node->data\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"fileattr_delproc\0").as_ptr()
        );
    }
    free((*node).data);
    (*node).data = 0i32 as (*mut ::std::os::raw::c_void);
}

unsafe extern fn fileattr_read() {
    let mut fname : *mut u8;
    let mut fp : *mut _IO_FILE;
    let mut line
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut line_len : usize = 0usize;
    if attr_read_attempted != 0 {
    } else {
        if !fileattr_stored_repos.is_null() {
            0i32;
        } else {
            __assert_fail(
                (*b"fileattr_stored_repos\0").as_ptr(),
                file!().as_ptr(),
                line!(),
                (*b"fileattr_read\0").as_ptr()
            );
        }
        fname = Xasprintf(
                    (*b"%s/%s\0").as_ptr(),
                    fileattr_stored_repos,
                    (*b"CVS/fileattr\0").as_ptr()
                );
        attr_read_attempted = 1i32;
        fp = fopen(fname as (*const u8),(*b"rb\0").as_ptr());
        (if fp == 0i32 as (*mut ::std::os::raw::c_void) as (*mut _IO_FILE) {
             if !(*__errno_location() == 2i32) {
                 error(
                     0i32,
                     *__errno_location(),
                     (*b"cannot read %s\0").as_ptr(),
                     fname
                 );
             }
             free(fname as (*mut ::std::os::raw::c_void));
         } else {
             attrlist = ::hash::getlist();
             'loop3: loop {
                 let mut nread : i32;
                 nread = getline(
                             &mut line as (*mut *mut u8),
                             &mut line_len as (*mut usize),
                             fp
                         ) as (i32);
                 if nread < 0i32 {
                     break;
                 }
                 if *line.offset(
                         (nread - 1i32) as (isize)
                     ) as (i32) == b'\n' as (i32) {
                     *line.offset((nread - 1i32) as (isize)) = b'\0';
                 }
                 if *line.offset(0isize) as (i32) == b'F' as (i32) {
                     let mut p : *mut u8;
                     let mut newnode : *mut ::hash::hashnode;
                     p = strchr(line as (*const u8),b'\t' as (i32));
                     if p == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                         error(
                             1i32,
                             0i32,
                             (*b"file attribute database corruption: tab missing in %s\0").as_ptr(
                             ),
                             primary_root_inverse_translate(fname as (*const u8))
                         );
                     }
                     *{
                          let _old = p;
                          p = p.offset(1isize);
                          _old
                      } = b'\0';
                     newnode = ::hash::getnode();
                     (*newnode).type_ = ::hash::ntype::FILEATTR;
                     (*newnode).delproc = Some(fileattr_delproc);
                     (*newnode).key = Xstrdup(line.offset(1isize) as (*const u8));
                     (*newnode).data = Xstrdup(
                                           p as (*const u8)
                                       ) as (*mut ::std::os::raw::c_void);
                     if !(::hash::addnode(attrlist,newnode) != 0i32) {
                         continue;
                     }
                     ::hash::freenode(newnode);
                 } else if *line.offset(0isize) as (i32) == b'D' as (i32) {
                     let mut p : *mut u8;
                     p = strchr(line as (*const u8),b'\t' as (i32));
                     if p == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                         error(
                             1i32,
                             0i32,
                             (*b"file attribute database corruption: tab missing in %s\0").as_ptr(
                             ),
                             fname
                         );
                     }
                     p = p.offset(1isize);
                     if !fileattr_default_attrs.is_null() {
                         free(fileattr_default_attrs as (*mut ::std::os::raw::c_void));
                     }
                     fileattr_default_attrs = Xstrdup(p as (*const u8));
                 } else {
                     let mut new : *mut unrecog;
                     new = xmalloc(::std::mem::size_of::<unrecog>()) as (*mut unrecog);
                     (*new).line = Xstrdup(line as (*const u8));
                     (*new).next = unrecog_head as (*mut unrecog);
                     unrecog_head = new;
                 }
             }
             if ferror_unlocked(fp) != 0 {
                 error(
                     0i32,
                     *__errno_location(),
                     (*b"cannot read %s\0").as_ptr(),
                     fname
                 );
             }
             if line != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                 free(line as (*mut ::std::os::raw::c_void));
             }
             if fclose(fp) < 0i32 {
                 error(
                     0i32,
                     *__errno_location(),
                     (*b"cannot close %s\0").as_ptr(),
                     fname
                 );
             }
             attrs_modified = 0i32;
             free(fname as (*mut ::std::os::raw::c_void));
         })
    }
}

unsafe extern fn strneq(
    mut a : *const u8, mut b : *const u8, mut n : usize
) -> bool {
    n == 0 || *a as (i32) == *b as (i32) && (strncmp(a,b,n) == 0i32)
}

#[no_mangle]
pub unsafe extern fn fileattr_get(
    mut filename : *const u8, mut attrname : *const u8
) -> *mut u8 {
    let mut _currentBlock;
    let mut node : *mut ::hash::hashnode;
    let mut attrname_len : usize = strlen(attrname);
    let mut p : *mut u8;
    if attrlist == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashlist) {
        fileattr_read();
    }
    if attrlist == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashlist) {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        if filename == 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
            p = fileattr_default_attrs;
        } else {
            node = ::hash::findnode(attrlist,filename);
            if node == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode) {
                return 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
            } else {
                p = (*node).data as (*mut u8);
            }
        }
        'loop8: loop {
            if p.is_null() {
                _currentBlock = 12;
                break;
            }
            if strneq(attrname,p as (*const u8),attrname_len) && (*p.offset(
                                                                       attrname_len as (isize)
                                                                   ) as (i32) == b'=' as (i32)) {
                _currentBlock = 13;
                break;
            }
            p = strchr(p as (*const u8),b';' as (i32));
            if p == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                _currentBlock = 12;
                break;
            }
            p = p.offset(1isize);
        }
        (if _currentBlock == 12 {
             0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
         } else {
             p.offset(attrname_len as (isize)).offset(1isize)
         })
    }
}

#[no_mangle]
pub unsafe extern fn fileattr_get0(
    mut filename : *const u8, mut attrname : *const u8
) -> *mut u8 {
    let mut cp : *mut u8;
    let mut cpend : *mut u8;
    let mut retval : *mut u8;
    cp = fileattr_get(filename,attrname);
    if cp == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        cpend = strchr(cp as (*const u8),b';' as (i32));
        if cpend == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            cpend = cp.offset(strlen(cp as (*const u8)) as (isize));
        }
        retval = xmalloc(
                     ((cpend as (isize)).wrapping_sub(
                          cp as (isize)
                      ) / ::std::mem::size_of::<u8>() as (isize) + 1isize) as (usize)
                 ) as (*mut u8);
        strncpy(
            retval,
            cp as (*const u8),
            ((cpend as (isize)).wrapping_sub(
                 cp as (isize)
             ) / ::std::mem::size_of::<u8>() as (isize)) as (usize)
        );
        *retval.offset(
             (cpend as (isize)).wrapping_sub(
                 cp as (isize)
             ) / ::std::mem::size_of::<u8>() as (isize)
         ) = b'\0';
        retval
    }
}

#[no_mangle]
pub unsafe extern fn fileattr_modify(
    mut list : *mut u8,
    mut attrname : *const u8,
    mut attrval : *const u8,
    mut namevalsep : i32,
    mut entsep : i32
) -> *mut u8 {
    let mut _currentBlock;
    let mut retval : *mut u8;
    let mut rp : *mut u8;
    let mut attrname_len : usize = strlen(attrname);
    let mut pre : *mut u8;
    let mut preend : *mut u8;
    let mut post : *mut u8;
    let mut p : *mut u8;
    let mut p2 : *mut u8;
    p = list;
    pre = list;
    preend = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    post = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    p2 = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    if list != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        _currentBlock = 1;
    } else {
        _currentBlock = 11;
    }
    'loop1: loop {
        if _currentBlock == 1 {
            p2 = strchr(p as (*const u8),entsep);
            if p2 == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                p2 = p.offset(strlen(p as (*const u8)) as (isize));
                if preend == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                    preend = p2;
                }
            } else {
                p2 = p2.offset(1isize);
            }
            if strneq(attrname,p as (*const u8),attrname_len) && (*p.offset(
                                                                       attrname_len as (isize)
                                                                   ) as (i32) == namevalsep) {
                preend = p;
                if preend > list {
                    preend = preend.offset(-1isize);
                }
                post = p2;
            }
            if *p2.offset(0isize) as (i32) == b'\0' as (i32) {
                _currentBlock = 11;
                continue;
            }
            p = p2;
            _currentBlock = 1;
        } else if post == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            _currentBlock = 12;
            break;
        } else {
            _currentBlock = 13;
            break;
        }
    }
    if _currentBlock == 12 {
        post = p2;
    }
    if preend == pre && (attrval == 0i32 as (*mut ::std::os::raw::c_void) as (*const u8)) && (post == p2) {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        retval = xmalloc(
                     (((preend as (isize)).wrapping_sub(
                           pre as (isize)
                       ) / ::std::mem::size_of::<u8>(
                           ) as (isize) + 1isize) as (usize)).wrapping_add(
                         if attrval == 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
                             0usize
                         } else {
                             attrname_len.wrapping_add(1usize).wrapping_add(strlen(attrval))
                         }
                     ).wrapping_add(
                         1usize
                     ).wrapping_add(
                         ((p2 as (isize)).wrapping_sub(
                              post as (isize)
                          ) / ::std::mem::size_of::<u8>() as (isize)) as (usize)
                     ).wrapping_add(
                         1usize
                     )
                 ) as (*mut u8);
        if preend != pre {
            strncpy(
                retval,
                pre as (*const u8),
                ((preend as (isize)).wrapping_sub(
                     pre as (isize)
                 ) / ::std::mem::size_of::<u8>() as (isize)) as (usize)
            );
            rp = retval.offset(
                     (preend as (isize)).wrapping_sub(
                         pre as (isize)
                     ) / ::std::mem::size_of::<u8>() as (isize)
                 );
            if attrval != 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
                *{
                     let _old = rp;
                     rp = rp.offset(1isize);
                     _old
                 } = entsep as (u8);
            }
            *rp = b'\0';
        } else {
            *retval.offset(0isize) = b'\0';
        }
        if attrval != 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
            strcat(retval,attrname);
            rp = retval.offset(strlen(retval as (*const u8)) as (isize));
            *{
                 let _old = rp;
                 rp = rp.offset(1isize);
                 _old
             } = namevalsep as (u8);
            strcpy(rp,attrval);
        }
        if post != p2 {
            rp = retval.offset(strlen(retval as (*const u8)) as (isize));
            if preend != pre || attrval != 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
                *{
                     let _old = rp;
                     rp = rp.offset(1isize);
                     _old
                 } = entsep as (u8);
            }
            strncpy(
                rp,
                post as (*const u8),
                ((p2 as (isize)).wrapping_sub(
                     post as (isize)
                 ) / ::std::mem::size_of::<u8>() as (isize)) as (usize)
            );
            rp = rp.offset(
                     (p2 as (isize)).wrapping_sub(
                         post as (isize)
                     ) / ::std::mem::size_of::<u8>() as (isize)
                 );
            *rp = b'\0';
        }
        retval
    }
}

#[no_mangle]
pub unsafe extern fn fileattr_set(
    mut filename : *const u8,
    mut attrname : *const u8,
    mut attrval : *const u8
) {
    let mut node : *mut ::hash::hashnode;
    let mut p : *mut u8;
    if !fileattr_stored_repos.is_null() {
        0i32;
    } else {
        __assert_fail(
            (*b"fileattr_stored_repos\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"fileattr_set\0").as_ptr()
        );
    }
    if filename == 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
        p = fileattr_modify(
                fileattr_default_attrs,
                attrname,
                attrval,
                b'=' as (i32),
                b';' as (i32)
            );
        if fileattr_default_attrs != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            free(fileattr_default_attrs as (*mut ::std::os::raw::c_void));
        }
        fileattr_default_attrs = p;
        attrs_modified = 1i32;
    } else {
        if attrlist == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashlist) {
            fileattr_read();
        }
        if attrlist == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashlist) {
            attrlist = ::hash::getlist();
        }
        node = ::hash::findnode(attrlist,filename);
        if node == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode) {
            if attrval == 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
                return;
            } else {
                node = ::hash::getnode();
                (*node).type_ = ::hash::ntype::FILEATTR;
                (*node).delproc = Some(fileattr_delproc);
                (*node).key = Xstrdup(filename);
                (*node).data = Xasprintf(
                                   (*b"%s=%s\0").as_ptr(),
                                   attrname,
                                   attrval
                               ) as (*mut ::std::os::raw::c_void);
                ::hash::addnode(attrlist,node);
            }
        }
        p = fileattr_modify(
                (*node).data as (*mut u8),
                attrname,
                attrval,
                b'=' as (i32),
                b';' as (i32)
            );
        if p == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            ::hash::delnode(node);
        } else {
            free((*node).data);
            (*node).data = p as (*mut ::std::os::raw::c_void);
        }
        attrs_modified = 1i32;
    }
}

#[no_mangle]
pub unsafe extern fn fileattr_getall(
    mut filename : *const u8
) -> *mut u8 {
    let mut node : *mut ::hash::hashnode;
    let mut p : *mut u8;
    if attrlist == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashlist) {
        fileattr_read();
    }
    if attrlist == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashlist) {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    } else {
        if filename == 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
            p = fileattr_default_attrs;
        } else {
            node = ::hash::findnode(attrlist,filename);
            if node == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode) {
                return 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
            } else {
                p = (*node).data as (*mut u8);
            }
        }
        Xstrdup(p as (*const u8))
    }
}

#[no_mangle]
pub unsafe extern fn fileattr_setall(
    mut filename : *const u8, mut attrs : *const u8
) {
    let mut node : *mut ::hash::hashnode;
    if filename == 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
        if fileattr_default_attrs != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            free(fileattr_default_attrs as (*mut ::std::os::raw::c_void));
        }
        fileattr_default_attrs = Xstrdup(attrs);
        attrs_modified = 1i32;
    } else {
        if attrlist == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashlist) {
            fileattr_read();
        }
        if attrlist == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashlist) {
            attrlist = ::hash::getlist();
        }
        node = ::hash::findnode(attrlist,filename);
        if node == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode) {
            if attrs != 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
                node = ::hash::getnode();
                (*node).type_ = ::hash::ntype::FILEATTR;
                (*node).delproc = Some(fileattr_delproc);
                (*node).key = Xstrdup(filename);
                (*node).data = Xstrdup(attrs) as (*mut ::std::os::raw::c_void);
                ::hash::addnode(attrlist,node);
            }
        } else if attrs == 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
            ::hash::delnode(node);
        } else {
            free((*node).data);
            (*node).data = Xstrdup(attrs) as (*mut ::std::os::raw::c_void);
        }
        attrs_modified = 1i32;
    }
}

#[no_mangle]
pub unsafe extern fn fileattr_newfile(mut filename : *const u8) {
    let mut node : *mut ::hash::hashnode;
    if attrlist == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashlist) {
        fileattr_read();
    }
    if fileattr_default_attrs == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
    } else {
        if attrlist == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashlist) {
            attrlist = ::hash::getlist();
        }
        node = ::hash::getnode();
        (*node).type_ = ::hash::ntype::FILEATTR;
        (*node).delproc = Some(fileattr_delproc);
        (*node).key = Xstrdup(filename);
        (*node).data = Xstrdup(
                           fileattr_default_attrs as (*const u8)
                       ) as (*mut ::std::os::raw::c_void);
        ::hash::addnode(attrlist,node);
        attrs_modified = 1i32;
    }
}

unsafe extern fn writeattr_proc(
    mut node : *mut ::hash::hashnode,
    mut data : *mut ::std::os::raw::c_void
) -> i32 {
    let mut fp : *mut _IO_FILE = data as (*mut _IO_FILE);
    fputs_unlocked((*b"F\0").as_ptr(),fp);
    fputs_unlocked((*node).key as (*const u8),fp);
    fputs_unlocked((*b"\t\0").as_ptr(),fp);
    fputs_unlocked((*node).data as (*const u8),fp);
    fputs_unlocked((*b"\n\0").as_ptr(),fp);
    0i32
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Enum1 {
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
pub enum Enum2 {
    SIGN_DEFAULT,
    SIGN_ALWAYS,
    SIGN_NEVER,
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Enum3 {
    VERIFY_DEFAULT = 0i32,
    VERIFY_OFF,
    VERIFY_WARN,
    VERIFY_FATAL,
}

#[derive(Copy)]
#[repr(C)]
pub struct cvsroot_s {
    pub original : *mut u8,
    pub method : Enum1,
    pub directory : *mut u8,
    pub isremote : bool,
    pub sign : Enum2,
    pub sign_template : *mut u8,
    pub sign_args : *mut ::hash::hashlist,
    pub openpgp_textmode : *mut u8,
    pub verify : Enum3,
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

unsafe extern fn postwatch_proc(
    mut repository : *const u8,
    mut filter : *const u8,
    mut file : *const u8,
    mut line : i32,
    mut closure : *mut ::std::os::raw::c_void
) -> i32 {
    let mut cmdline : *mut u8;
    let mut srepos : *const u8 = Short_Repository(repository);
    cmdline = format_cmdline(
                  false,
                  srepos,
                  file,
                  line,
                  filter,
                  (*b"c\0").as_ptr(),
                  (*b"s\0").as_ptr(),
                  cvs_cmd_name,
                  (*b"R\0").as_ptr(),
                  (*b"s\0").as_ptr(),
                  if !referrer.is_null() {
                      (*referrer).original as (*const u8)
                  } else {
                      (*b"NONE\0").as_ptr()
                  },
                  (*b"p\0").as_ptr(),
                  (*b"s\0").as_ptr(),
                  srepos,
                  (*b"r\0").as_ptr(),
                  (*b"s\0").as_ptr(),
                  (*current_parsed_root).directory,
                  0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
              );
    if cmdline.is_null() || strlen(cmdline as (*const u8)) == 0 {
        if !cmdline.is_null() {
            free(cmdline as (*mut ::std::os::raw::c_void));
        }
        error(
            0i32,
            0i32,
            (*b"%s:%d: postwatch proc resolved to the empty string!\0").as_ptr(
            ),
            file,
            line
        );
        1i32
    } else {
        run_setup(cmdline as (*const u8));
        free(cmdline as (*mut ::std::os::raw::c_void));
        abs(
            run_exec(
                0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                0x0i32 | 0x1i32 << 4i32
            )
        )
    }
}

#[no_mangle]
pub unsafe extern fn fileattr_write() {
    let mut fp : *mut _IO_FILE;
    let mut fname : *mut u8;
    let mut omask : u32;
    let mut p : *mut unrecog;
    if attrs_modified == 0 {
    } else if noexec != 0 {
    } else {
        if !fileattr_stored_repos.is_null() {
            0i32;
        } else {
            __assert_fail(
                (*b"fileattr_stored_repos\0").as_ptr(),
                file!().as_ptr(),
                line!(),
                (*b"fileattr_write\0").as_ptr()
            );
        }
        fname = Xasprintf(
                    (*b"%s/%s\0").as_ptr(),
                    fileattr_stored_repos,
                    (*b"CVS/fileattr\0").as_ptr()
                );
        (if ::hash::list_isempty(
                attrlist
            ) != 0 && (fileattr_default_attrs == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)) && (unrecog_head == 0i32 as (*mut ::std::os::raw::c_void) as (*mut unrecog)) {
             if unlink_file(fname as (*const u8)) < 0i32 {
                 if !(*__errno_location() == 2i32) {
                     error(
                         0i32,
                         *__errno_location(),
                         (*b"cannot remove %s\0").as_ptr(),
                         fname
                     );
                 }
             }
             strcpy(fname,fileattr_stored_repos as (*const u8));
             strcat(fname,(*b"/\0").as_ptr());
             strcat(fname,(*b"CVS\0").as_ptr());
             if rmdir(fname as (*const u8)) < 0i32 {
                 if *__errno_location() != 39i32 && !(*__errno_location() == 2i32) {
                     error(
                         0i32,
                         *__errno_location(),
                         (*b"cannot remove %s\0").as_ptr(),
                         fname
                     );
                 }
             }
             free(fname as (*mut ::std::os::raw::c_void));
         } else {
             omask = umask(cvsumask);
             fp = fopen(fname as (*const u8),(*b"wb\0").as_ptr());
             if fp == 0i32 as (*mut ::std::os::raw::c_void) as (*mut _IO_FILE) {
                 if *__errno_location() == 2i32 {
                     let mut repname : *mut u8;
                     repname = Xasprintf(
                                   (*b"%s/%s\0").as_ptr(),
                                   fileattr_stored_repos,
                                   (*b"CVS\0").as_ptr()
                               );
                     if !isdir(repname as (*const u8)) && !cvs_mkdir(
                                                               repname as (*const u8),
                                                               0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                                                               (1i32 << 2i32) as (u32)
                                                           ) {
                         umask(omask);
                         free(fname as (*mut ::std::os::raw::c_void));
                         free(repname as (*mut ::std::os::raw::c_void));
                         return;
                     } else {
                         free(repname as (*mut ::std::os::raw::c_void));
                         fp = fopen(fname as (*const u8),(*b"wb\0").as_ptr());
                     }
                 }
                 if fp == 0i32 as (*mut ::std::os::raw::c_void) as (*mut _IO_FILE) {
                     error(
                         0i32,
                         *__errno_location(),
                         (*b"cannot write %s\0").as_ptr(),
                         fname
                     );
                     umask(omask);
                     free(fname as (*mut ::std::os::raw::c_void));
                     return;
                 }
             }
             umask(omask);
             ::hash::walklist(
                 attrlist,
                 writeattr_proc,
                 fp as (*mut ::std::os::raw::c_void)
             );
             if fileattr_default_attrs != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                 fputs_unlocked((*b"D\t\0").as_ptr(),fp);
                 fputs_unlocked(fileattr_default_attrs as (*const u8),fp);
                 fputs_unlocked((*b"\n\0").as_ptr(),fp);
             }
             p = unrecog_head;
             'loop11: loop {
                 if !(p != 0i32 as (*mut ::std::os::raw::c_void) as (*mut unrecog)) {
                     break;
                 }
                 fputs_unlocked((*p).line as (*const u8),fp);
                 fputs_unlocked((*b"\n\0").as_ptr(),fp);
                 p = (*p).next as (*mut unrecog);
             }
             if fclose(fp) < 0i32 {
                 error(
                     0i32,
                     *__errno_location(),
                     (*b"cannot close %s\0").as_ptr(),
                     fname
                 );
             }
             attrs_modified = 0i32;
             free(fname as (*mut ::std::os::raw::c_void));
             Parse_Info(
                 (*b"postwatch\0").as_ptr(),
                 fileattr_stored_repos as (*const u8),
                 postwatch_proc,
                 1i32,
                 0i32 as (*mut ::std::os::raw::c_void)
             );
         })
    }
}

#[no_mangle]
pub unsafe extern fn fileattr_free() {
    ::hash::dellist(&mut attrlist as (*mut *mut ::hash::hashlist));
    if fileattr_stored_repos != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        free(fileattr_stored_repos as (*mut ::std::os::raw::c_void));
    }
    fileattr_stored_repos = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    if fileattr_default_attrs != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        free(fileattr_default_attrs as (*mut ::std::os::raw::c_void));
    }
    fileattr_default_attrs = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    'loop5: loop {
        if unrecog_head.is_null() {
            break;
        }
        let mut p : *mut unrecog = unrecog_head;
        unrecog_head = (*p).next as (*mut unrecog);
        free((*p).line as (*mut ::std::os::raw::c_void));
        free(p as (*mut ::std::os::raw::c_void));
    }
}
