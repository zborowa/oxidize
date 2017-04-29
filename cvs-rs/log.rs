extern {
    fn Make_Date(rawdate : *const u8) -> *mut u8;
    fn RCS_datecmp(date1 : *const u8, date2 : *const u8) -> i32;
    fn RCS_fully_parse(arg1 : *mut ::vers_ts::rcsnode);
    fn RCS_getbranch(
        rcs : *mut ::vers_ts::rcsnode,
        tag : *const u8,
        force_tag_match : i32
    ) -> *mut u8;
    fn RCS_getlocks(
        rcs : *mut ::vers_ts::rcsnode
    ) -> *mut ::hash::hashlist;
    fn RCS_gettag(
        rcs : *mut ::vers_ts::rcsnode,
        symtag : *const u8,
        force_tag_match : i32,
        simple_tag : *mut i32
    ) -> *mut u8;
    fn RCS_head(rcs : *mut ::vers_ts::rcsnode) -> *mut u8;
    fn RCS_nodeisbranch(
        rcs : *mut ::vers_ts::rcsnode, tag : *const u8
    ) -> i32;
    fn RCS_symbols(
        rcs : *mut ::vers_ts::rcsnode
    ) -> *mut ::hash::hashlist;
    fn RCS_whatbranch(
        rcs : *mut ::vers_ts::rcsnode, tag : *const u8
    ) -> *mut u8;
    fn Xasprintf(format : *const u8, ...) -> *mut u8;
    fn Xstrdup(str : *const u8) -> *mut u8;
    fn __assert_fail(
        __assertion : *const u8,
        __file : *const u8,
        __line : u32,
        __function : *const u8
    );
    fn __errno_location() -> *mut i32;
    fn base64_decode_alloc_ctx(
        ctx : *mut base64_decode_context,
        in_ : *const u8,
        inlen : usize,
        out : *mut *mut u8,
        outlen : *mut usize
    ) -> bool;
    fn buf_free(arg1 : *mut buffer);
    fn buf_nonio_initialize(
        arg1 : Option<unsafe extern fn(*mut buffer)>
    ) -> *mut buffer;
    fn buf_output(arg1 : *mut buffer, arg2 : *const u8, arg3 : usize);
    fn chdir(__path : *const u8) -> i32;
    fn close_module(db : *mut Struct4);
    static mut current_parsed_root : *mut cvsroot_s;
    static mut cvs_cmd_name : *const u8;
    fn cvs_output(arg1 : *const u8, arg2 : usize);
    fn cvs_output_tagged(arg1 : *const u8, arg2 : *const u8);
    fn date_to_internet(arg1 : *mut u8, arg2 : *const u8);
    fn dir_append(dir : *const u8, append : *const u8) -> *mut u8;
    fn dir_name(file : *const u8) -> *mut u8;
    fn do_module(
        db : *mut Struct4,
        mname : *mut u8,
        m_type : mtype,
        msg : *mut u8,
        callback_proc
        :
        unsafe extern fn(i32, *mut *mut u8, *mut u8, *mut u8, *mut u8, i32, i32, *mut u8, *mut u8) -> i32,
        where_ : *mut u8,
        shorten : i32,
        local_specified : i32,
        run_module_prog : i32,
        build_dirs : i32,
        extra_arg : *mut u8
    ) -> i32;
    fn error(
        __status : i32, __errnum : i32, __format : *const u8, ...
    );
    fn free(__ptr : *mut ::std::os::raw::c_void);
    fn get_responses_and_close() -> i32;
    fn getcaller() -> *mut u8;
    fn getopt(
        ___argc : i32, ___argv : *const *mut u8, __shortopts : *const u8
    ) -> i32;
    fn ign_setup();
    fn isdigit(arg1 : i32) -> i32;
    fn isdir(file : *const u8) -> bool;
    fn isspace(arg1 : i32) -> i32;
    fn last_component(file : *const u8) -> *mut u8;
    fn memcmp(
        __s1 : *const ::std::os::raw::c_void,
        __s2 : *const ::std::os::raw::c_void,
        __n : usize
    ) -> i32;
    fn numdots(s : *const u8) -> i32;
    fn open_module() -> *mut Struct4;
    static mut optarg : *mut u8;
    static mut optind : i32;
    fn parse_signature(
        bpin : *mut buffer, spout : *mut openpgp_signature
    ) -> i32;
    static mut quiet : i32;
    fn quote(name : *const u8) -> *const u8;
    static mut really_quiet : i32;
    fn send_arg(string : *const u8);
    fn send_file_names(argc : i32, argv : *mut *mut u8, flags : u32);
    fn send_files(
        argc : i32,
        argv : *mut *mut u8,
        local : i32,
        aflag : i32,
        flags : u32
    );
    fn send_to_server(str : *const u8, len : usize);
    fn sprintf(__s : *mut u8, __format : *const u8, ...) -> i32;
    fn sscanf(__s : *const u8, __format : *const u8, ...) -> i32;
    fn start_recursion(
        fileproc
        :
        Option<unsafe extern fn(*mut ::std::os::raw::c_void, *mut ::vers_ts::file_info) -> i32>,
        filesdoneproc
        :
        Option<unsafe extern fn(*mut ::std::os::raw::c_void, i32, *const u8, *const u8, *mut ::hash::hashlist) -> i32>,
        direntproc
        :
        Option<unsafe extern fn(*mut ::std::os::raw::c_void, *const u8, *const u8, *const u8, *mut ::hash::hashlist) -> direnter_type>,
        dirleaveproc
        :
        Option<unsafe extern fn(*mut ::std::os::raw::c_void, *const u8, i32, *const u8, *mut ::hash::hashlist) -> i32>,
        callerdat : *mut ::std::os::raw::c_void,
        argc : i32,
        argv : *mut *mut u8,
        local : i32,
        which : i32,
        aflag : i32,
        locktype : cvs_lock_type,
        update_preload : *const u8,
        dosrcs : i32,
        repository : *const u8
    ) -> i32;
    fn start_server();
    fn strchr(__s : *const u8, __c : i32) -> *mut u8;
    fn strcmp(__s1 : *const u8, __s2 : *const u8) -> i32;
    fn strcpy(__dest : *mut u8, __src : *const u8) -> *mut u8;
    fn strlen(__s : *const u8) -> usize;
    fn strncpy(
        __dest : *mut u8, __src : *const u8, __n : usize
    ) -> *mut u8;
    fn strrchr(__s : *const u8, __c : i32) -> *mut u8;
    fn supported_request(arg1 : *const u8) -> bool;
    fn usage(cpp : *const *const u8);
    fn wrap_setup();
    fn xalloc_die();
    fn xmalloc(s : usize) -> *mut ::std::os::raw::c_void;
}

pub enum buffer_data {
}

#[derive(Copy)]
#[repr(C)]
pub struct option_revlist {
    pub next : *mut option_revlist,
    pub first : *mut u8,
    pub last : *mut u8,
    pub branchhead : i32,
    pub inclusive : i32,
}

impl Clone for option_revlist {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct datelist {
    pub next : *mut datelist,
    pub start : *mut u8,
    pub end : *mut u8,
    pub inclusive : i32,
}

impl Clone for datelist {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct log_data {
    pub nameonly : i32,
    pub header : i32,
    pub long_header : i32,
    pub notags : i32,
    pub default_branch : i32,
    pub sup_header : i32,
    pub revlist : *mut option_revlist,
    pub datelist : *mut datelist,
    pub singledatelist : *mut datelist,
    pub statelist : *mut ::hash::hashlist,
    pub authorlist : *mut ::hash::hashlist,
}

impl Clone for log_data {
    fn clone(&self) -> Self { *self }
}

static mut global_log_data : *mut log_data = 0 as (*mut log_data);

static mut is_rlog : i32 = 0i32;

static mut log_usage : [*const u8; 28] = [
    b"Usage: %s %s [-lRhtNb] [-r[revisions]] [-d dates] [-s states]\n\0" as (*const u8),
    b"    [-w[logins]] [files...]\n\0" as (*const u8),
    b"\t-l\tLocal directory only, no recursion.\n\0" as (*const u8),
    b"\t-b\tList revisions on the default branch.\n\0" as (*const u8),
    b"\t-h\tOnly print header.\n\0" as (*const u8),
    b"\t-R\tOnly print name of RCS file.\n\0" as (*const u8),
    b"\t-t\tOnly print header and descriptive text.\n\0" as (*const u8),
    b"\t-N\tDo not list tags.\n\0" as (*const u8),
    b"\t-n\tList tags (default).\n\0" as (*const u8),
    b"\t-S\tDo not print name/header if no revisions selected.  -d, -r,\n\0" as (*const u8),
    b"\t\t-s, & -w have little effect in conjunction with -b, -h, -R, and\n\0" as (*const u8),
    b"\t\t-t without this option.\n\0" as (*const u8),
    b"\t-r[revisions]\tA comma-separated list of revisions to print:\n\0" as (*const u8),
    b"\t   rev1:rev2   Between rev1 and rev2, including rev1 and rev2.\n\0" as (*const u8),
    b"\t   rev1::rev2  Between rev1 and rev2, excluding rev1.\n\0" as (*const u8),
    b"\t   rev:        rev and following revisions on the same branch.\n\0" as (*const u8),
    b"\t   rev::       After rev on the same branch.\n\0" as (*const u8),
    b"\t   :rev        rev and previous revisions on the same branch.\n\0" as (*const u8),
    b"\t   ::rev       rev and previous revisions on the same branch.\n\0" as (*const u8),
    b"\t   rev         Just rev.\n\0" as (*const u8),
    b"\t   branch      All revisions on the branch.\n\0" as (*const u8),
    b"\t   branch.     The last revision on the branch.\n\0" as (*const u8),
    b"\t-d dates\tA semicolon-separated list of dates\n\0" as (*const u8),
    b"\t        \t(D1<D2 for range, D for latest before).\n\0" as (*const u8),
    b"\t-s states\tOnly list revisions with specified states.\n\0" as (*const u8),
    b"\t-w[logins]\tOnly list revisions checked in by specified logins.\n\0" as (*const u8),
    b"(Specify the --help global option for a list of other help options)\n\0" as (*const u8),
    0i32 as (*mut ::std::os::raw::c_void) as (*const u8)
];

unsafe extern fn streq(
    mut a : *const u8, mut b : *const u8
) -> bool {
    *a as (i32) == *b as (i32) && (strcmp(a,b) == 0i32)
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

unsafe extern fn send_one(
    mut node : *mut ::hash::hashnode,
    mut closure : *mut ::std::os::raw::c_void
) -> i32 {
    let mut option : *mut u8 = closure as (*mut u8);
    send_to_server((*b"Argument \0").as_ptr(),0usize);
    send_to_server(option as (*const u8),0usize);
    if !streq((*node).key as (*const u8),(*b"@@MYSELF\0").as_ptr()) {
        send_to_server((*node).key as (*const u8),0usize);
    }
    send_to_server((*b"\n\0").as_ptr(),0usize);
    0i32
}

unsafe extern fn send_arg_list(
    mut option : *mut u8, mut arg : *mut ::hash::hashlist
) { if arg == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashlist) {
    } else {
        ::hash::walklist(
            arg,
            send_one,
            option as (*mut ::std::os::raw::c_void)
        );
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct4 {
    pub dbm_list : *mut ::hash::hashlist,
    pub dbm_next : *mut ::hash::hashnode,
    pub name : *mut u8,
    pub modified : i32,
}

impl Clone for Struct4 {
    fn clone(&self) -> Self { *self }
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum mtype {
    CHECKOUT,
    TAG,
    PATCH,
    EXPORT,
    MISC,
}

#[no_mangle]
pub unsafe extern fn cvslog(
    mut argc : i32, mut argv : *mut *mut u8
) -> i32 {
    let mut c : i32;
    let mut err : i32 = 0i32;
    let mut local : i32 = 0i32;
    let mut prl : *mut *mut option_revlist;
    is_rlog = streq(cvs_cmd_name,(*b"rlog\0").as_ptr()) as (i32);
    if argc == -1i32 {
        usage(log_usage.as_ptr());
    }
    let mut log_data
        : log_data
        = log_data {
              nameonly: 0i32,
              header: 0i32,
              long_header: 0i32,
              notags: 0i32,
              default_branch: 0i32,
              sup_header: 0i32,
              revlist: 0 as (*mut option_revlist),
              datelist: 0 as (*mut datelist),
              singledatelist: 0 as (*mut datelist),
              statelist: 0 as (*mut ::hash::hashlist),
              authorlist: 0 as (*mut ::hash::hashlist)
          };
    prl = &mut log_data.revlist as (*mut *mut option_revlist);
    optind = 0i32;
    'loop3: loop {
        if !({
                 c = getopt(
                         argc,
                         argv as (*const *mut u8),
                         (*b"+bd:hlNnSRr::s:tw::\0").as_ptr()
                     );
                 c
             } != -1i32) {
            break;
        }
        if c == b'w' as (i32) {
            if optarg != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                log_parse_list(
                    &mut log_data.authorlist as (*mut *mut ::hash::hashlist),
                    optarg as (*const u8)
                );
            } else {
                log_parse_list(
                    &mut log_data.authorlist as (*mut *mut ::hash::hashlist),
                    (*b"@@MYSELF\0").as_ptr()
                );
            }
        } else if c == b't' as (i32) {
            log_data.long_header = 1i32;
        } else if c == b's' as (i32) {
            log_parse_list(
                &mut log_data.statelist as (*mut *mut ::hash::hashlist),
                optarg as (*const u8)
            );
        } else if c == b'r' as (i32) {
            *prl = log_parse_revlist(optarg as (*const u8));
            prl = &mut (**prl).next as (*mut *mut option_revlist) as (*mut *mut option_revlist);
        } else if c == b'R' as (i32) {
            log_data.nameonly = 1i32;
        } else if c == b'S' as (i32) {
            log_data.sup_header = 1i32;
        } else if c == b'n' as (i32) {
            log_data.notags = 0i32;
        } else if c == b'N' as (i32) {
            log_data.notags = 1i32;
        } else if c == b'l' as (i32) {
            local = 1i32;
        } else if c == b'h' as (i32) {
            log_data.header = 1i32;
        } else if c == b'd' as (i32) {
            log_parse_date(
                &mut log_data as (*mut log_data),
                optarg as (*const u8)
            );
        } else if c == b'b' as (i32) {
            log_data.default_branch = 1i32;
        } else {
            usage(log_usage.as_ptr());
        }
    }
    argc = argc - optind;
    argv = argv.offset(optind as (isize));
    wrap_setup();
    if (*current_parsed_root).isremote {
        let mut p : *mut datelist;
        let mut rp : *mut option_revlist;
        let mut datetmp = [0u8; 50];
        start_server();
        if is_rlog != 0 && !supported_request((*b"rlog\0").as_ptr()) {
            error(1i32,0i32,(*b"server does not support rlog\0").as_ptr());
        }
        ign_setup();
        if log_data.default_branch != 0 {
            send_arg((*b"-b\0").as_ptr());
        }
        'loop38: loop {
            if !(log_data.datelist != 0i32 as (*mut ::std::os::raw::c_void) as (*mut datelist)) {
                break;
            }
            p = log_data.datelist;
            log_data.datelist = (*p).next as (*mut datelist);
            if !(*p).start.is_null() && !(*p).end.is_null() {
                0i32;
            } else {
                __assert_fail(
                    (*b"p->start && p->end\0").as_ptr(),
                    file!().as_ptr(),
                    line!(),
                    (*b"cvslog\0").as_ptr()
                );
            }
            send_to_server((*b"Argument -d\n\0").as_ptr(),0usize);
            send_to_server((*b"Argument \0").as_ptr(),0usize);
            date_to_internet(datetmp.as_mut_ptr(),(*p).start as (*const u8));
            send_to_server(datetmp.as_mut_ptr() as (*const u8),0usize);
            if (*p).inclusive != 0 {
                send_to_server((*b"<=\0").as_ptr(),0usize);
            } else {
                send_to_server((*b"<\0").as_ptr(),0usize);
            }
            date_to_internet(datetmp.as_mut_ptr(),(*p).end as (*const u8));
            send_to_server(datetmp.as_mut_ptr() as (*const u8),0usize);
            send_to_server((*b"\n\0").as_ptr(),0usize);
            free((*p).start as (*mut ::std::os::raw::c_void));
            free((*p).end as (*mut ::std::os::raw::c_void));
            free(p as (*mut ::std::os::raw::c_void));
        }
        'loop39: loop {
            if !(log_data.singledatelist != 0i32 as (*mut ::std::os::raw::c_void) as (*mut datelist)) {
                break;
            }
            p = log_data.singledatelist;
            log_data.singledatelist = (*p).next as (*mut datelist);
            if !(*p).end.is_null() {
                0i32;
            } else {
                __assert_fail(
                    (*b"p->end\0").as_ptr(),
                    file!().as_ptr(),
                    line!(),
                    (*b"cvslog\0").as_ptr()
                );
            }
            send_to_server((*b"Argument -d\n\0").as_ptr(),0usize);
            send_to_server((*b"Argument \0").as_ptr(),0usize);
            date_to_internet(datetmp.as_mut_ptr(),(*p).end as (*const u8));
            send_to_server(datetmp.as_mut_ptr() as (*const u8),0usize);
            send_to_server((*b"\n\0").as_ptr(),0usize);
            free((*p).end as (*mut ::std::os::raw::c_void));
            free(p as (*mut ::std::os::raw::c_void));
        }
        if log_data.header != 0 {
            send_arg((*b"-h\0").as_ptr());
        }
        if local != 0 {
            send_arg((*b"-l\0").as_ptr());
        }
        if log_data.notags != 0 {
            send_arg((*b"-N\0").as_ptr());
        }
        if log_data.sup_header != 0 {
            send_arg((*b"-S\0").as_ptr());
        }
        if log_data.nameonly != 0 {
            send_arg((*b"-R\0").as_ptr());
        }
        if log_data.long_header != 0 {
            send_arg((*b"-t\0").as_ptr());
        }
        'loop52: loop {
            if !(log_data.revlist != 0i32 as (*mut ::std::os::raw::c_void) as (*mut option_revlist)) {
                break;
            }
            rp = log_data.revlist;
            log_data.revlist = (*rp).next as (*mut option_revlist);
            send_to_server((*b"Argument -r\0").as_ptr(),0usize);
            if (*rp).branchhead != 0 {
                if (*rp).first != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                    send_to_server((*rp).first as (*const u8),0usize);
                }
                send_to_server((*b".\0").as_ptr(),1usize);
            } else {
                if (*rp).first != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                    send_to_server((*rp).first as (*const u8),0usize);
                }
                send_to_server((*b":\0").as_ptr(),1usize);
                if (*rp).inclusive == 0 {
                    send_to_server((*b":\0").as_ptr(),1usize);
                }
                if (*rp).last != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                    send_to_server((*rp).last as (*const u8),0usize);
                }
            }
            send_to_server((*b"\n\0").as_ptr(),0usize);
            if !(*rp).first.is_null() {
                free((*rp).first as (*mut ::std::os::raw::c_void));
            }
            if !(*rp).last.is_null() {
                free((*rp).last as (*mut ::std::os::raw::c_void));
            }
            free(rp as (*mut ::std::os::raw::c_void));
        }
        send_arg_list((*b"-s\0").as_ptr() as (*mut u8),log_data.statelist);
        ::hash::dellist(
            &mut log_data.statelist as (*mut *mut ::hash::hashlist)
        );
        send_arg_list(
            (*b"-w\0").as_ptr() as (*mut u8),
            log_data.authorlist
        );
        ::hash::dellist(
            &mut log_data.authorlist as (*mut *mut ::hash::hashlist)
        );
        send_arg((*b"--\0").as_ptr());
        if is_rlog != 0 {
            let mut i : i32;
            i = 0i32;
            'loop56: loop {
                if !(i < argc) {
                    break;
                }
                send_arg(*argv.offset(i as (isize)) as (*const u8));
                i = i + 1;
            }
            send_to_server((*b"rlog\n\0").as_ptr(),0usize);
        } else {
            send_files(argc,argv,local,0i32,(1i32 << 2i32) as (u32));
            send_file_names(argc,argv,1u32);
            send_to_server((*b"log\n\0").as_ptr(),0usize);
        }
        err = get_responses_and_close();
        err
    } else {
        if ::hash::findnode(
               log_data.authorlist,
               (*b"@@MYSELF\0").as_ptr()
           ) != 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode) {
            log_parse_list(
                &mut log_data.authorlist as (*mut *mut ::hash::hashlist),
                getcaller() as (*const u8)
            );
        }
        global_log_data = &mut log_data as (*mut log_data);
        if is_rlog != 0 {
            let mut db : *mut Struct4;
            let mut i : i32;
            db = open_module();
            i = 0i32;
            'loop10: loop {
                if !(i < argc) {
                    break;
                }
                err = err + do_module(
                                db,
                                *argv.offset(i as (isize)),
                                mtype::MISC,
                                (*b"Logging\0").as_ptr() as (*mut u8),
                                rlog_proc,
                                0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                                0i32,
                                local,
                                0i32,
                                0i32,
                                0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                            );
                i = i + 1;
            }
            close_module(db);
        } else {
            err = rlog_proc(
                      argc + 1i32,
                      argv.offset(-1isize),
                      0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                      0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                      0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                      0i32,
                      local,
                      0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                      0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                  );
        }
        global_log_data = 0i32 as (*mut log_data);
        'loop13: loop {
            if log_data.revlist.is_null() {
                break;
            }
            let mut rl
                : *mut option_revlist
                = (*log_data.revlist).next as (*mut option_revlist);
            if !(*log_data.revlist).first.is_null() {
                free((*log_data.revlist).first as (*mut ::std::os::raw::c_void));
            }
            if !(*log_data.revlist).last.is_null() {
                free((*log_data.revlist).last as (*mut ::std::os::raw::c_void));
            }
            free(log_data.revlist as (*mut ::std::os::raw::c_void));
            log_data.revlist = rl;
        }
        'loop14: loop {
            if log_data.datelist.is_null() {
                break;
            }
            let mut nd
                : *mut datelist
                = (*log_data.datelist).next as (*mut datelist);
            if !(*log_data.datelist).start.is_null() {
                free((*log_data.datelist).start as (*mut ::std::os::raw::c_void));
            }
            if !(*log_data.datelist).end.is_null() {
                free((*log_data.datelist).end as (*mut ::std::os::raw::c_void));
            }
            free(log_data.datelist as (*mut ::std::os::raw::c_void));
            log_data.datelist = nd;
        }
        'loop15: loop {
            if log_data.singledatelist.is_null() {
                break;
            }
            let mut nd
                : *mut datelist
                = (*log_data.singledatelist).next as (*mut datelist);
            if !(*log_data.singledatelist).start.is_null() {
                free(
                    (*log_data.singledatelist).start as (*mut ::std::os::raw::c_void)
                );
            }
            if !(*log_data.singledatelist).end.is_null() {
                free(
                    (*log_data.singledatelist).end as (*mut ::std::os::raw::c_void)
                );
            }
            free(log_data.singledatelist as (*mut ::std::os::raw::c_void));
            log_data.singledatelist = nd;
        }
        ::hash::dellist(
            &mut log_data.statelist as (*mut *mut ::hash::hashlist)
        );
        ::hash::dellist(
            &mut log_data.authorlist as (*mut *mut ::hash::hashlist)
        );
        err
    }
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum direnter_type {
    R_PROCESS = 1i32,
    R_SKIP_FILES,
    R_SKIP_DIRS,
    R_SKIP_ALL,
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum cvs_lock_type {
    CVS_LOCK_NONE,
    CVS_LOCK_READ,
    CVS_LOCK_WRITE,
}

unsafe extern fn rlog_proc(
    mut argc : i32,
    mut argv : *mut *mut u8,
    mut xwhere : *mut u8,
    mut mwhere : *mut u8,
    mut mfile : *mut u8,
    mut shorten : i32,
    mut local : i32,
    mut mname : *mut u8,
    mut msg : *mut u8
) -> i32 {
    let mut myargv : [*mut u8; 2];
    let mut err : i32 = 0i32;
    let mut which : i32;
    let mut repository : *mut u8;
    let mut where_ : *mut u8;
    if is_rlog != 0 {
        repository = dir_append(
                         (*current_parsed_root).directory as (*const u8),
                         *argv.offset(0isize) as (*const u8)
                     );
        where_ = Xstrdup(*argv.offset(0isize) as (*const u8));
        if !mfile.is_null() {
            let mut path
                : *mut u8
                = dir_append(repository as (*const u8),mfile as (*const u8));
            if isdir(path as (*const u8)) {
                let mut tmp
                    : *mut ::std::os::raw::c_void
                    = repository as (*mut ::std::os::raw::c_void);
                repository = dir_append(
                                 repository as (*const u8),
                                 mfile as (*const u8)
                             );
                free(tmp);
                let mut tmp
                    : *mut ::std::os::raw::c_void
                    = where_ as (*mut ::std::os::raw::c_void);
                where_ = dir_append(
                             repository as (*const u8),
                             mfile as (*const u8)
                         );
                free(tmp);
            } else {
                let mut d : *mut u8 = dir_name(mfile as (*const u8));
                let mut tmp
                    : *mut ::std::os::raw::c_void
                    = repository as (*mut ::std::os::raw::c_void);
                repository = dir_append(
                                 repository as (*const u8),
                                 d as (*const u8)
                             );
                free(tmp);
                let mut tmp
                    : *mut ::std::os::raw::c_void
                    = where_ as (*mut ::std::os::raw::c_void);
                where_ = dir_append(where_ as (*const u8),d as (*const u8));
                free(tmp);
                myargv = [
                    *argv.offset(0isize),
                    last_component(mfile as (*const u8)),
                ];
                argc = 2i32;
                argv = myargv.as_mut_ptr();
                free(d as (*mut ::std::os::raw::c_void));
            }
            free(path as (*mut ::std::os::raw::c_void));
        }
        if chdir(repository as (*const u8)) < 0i32 {
            error(
                0i32,
                *__errno_location(),
                (*b"cannot chdir to %s\0").as_ptr(),
                quote(repository as (*const u8))
            );
            free(repository as (*mut ::std::os::raw::c_void));
            free(where_ as (*mut ::std::os::raw::c_void));
            return 1i32;
        } else {
            which = 1i32 << 1i32 | 1i32 << 2i32;
        }
    } else {
        repository = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        where_ = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        which = 1i32 << 0i32 | 1i32 << 1i32 | 1i32 << 2i32;
    }
    err = start_recursion(
              Some(log_fileproc),
              None,
              Some(log_dirproc),
              None,
              global_log_data as (*mut ::std::os::raw::c_void),
              argc - 1i32,
              argv.offset(1isize),
              local,
              which,
              0i32,
              cvs_lock_type::CVS_LOCK_READ,
              where_ as (*const u8),
              1i32,
              repository as (*const u8)
          );
    if !repository.is_null() {
        free(repository as (*mut ::std::os::raw::c_void));
    }
    if !where_.is_null() {
        free(where_ as (*mut ::std::os::raw::c_void));
    }
    err
}

unsafe extern fn log_parse_revlist(
    mut argstring : *const u8
) -> *mut option_revlist {
    let mut orig_copy : *mut u8;
    let mut copy : *mut u8;
    let mut ret : *mut option_revlist;
    let mut pr : *mut *mut option_revlist;
    if argstring == 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
        argstring = (*b"\0").as_ptr();
    }
    ret = 0i32 as (*mut ::std::os::raw::c_void) as (*mut option_revlist);
    pr = &mut ret as (*mut *mut option_revlist);
    orig_copy = {
                    copy = Xstrdup(argstring);
                    copy
                };
    'loop3: loop {
        if !(copy != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)) {
            break;
        }
        let mut comma : *mut u8;
        let mut r : *mut option_revlist;
        comma = strchr(copy as (*const u8),b',' as (i32));
        if comma != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            *{
                 let _old = comma;
                 comma = comma.offset(1isize);
                 _old
             } = b'\0';
        }
        r = xmalloc(
                ::std::mem::size_of::<option_revlist>()
            ) as (*mut option_revlist);
        (*r).next = 0i32 as (*mut ::std::os::raw::c_void) as (*mut option_revlist);
        (*r).first = copy;
        (*r).branchhead = 0i32;
        (*r).last = strchr(copy as (*const u8),b':' as (i32));
        if (*r).last != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            *{
                 let _old = (*r).last;
                 (*r).last = (*r).last.offset(1isize);
                 _old
             } = b'\0';
            (*r).inclusive = (*(*r).last as (i32) != b':' as (i32)) as (i32);
            if (*r).inclusive == 0 {
                (*r).last = (*r).last.offset(1isize);
            }
        } else {
            (*r).last = (*r).first;
            (*r).inclusive = 1i32;
            if *(*r).first.offset(
                    0isize
                ) as (i32) != b'\0' as (i32) && (*(*r).first.offset(
                                                      strlen(
                                                          (*r).first as (*const u8)
                                                      ).wrapping_sub(
                                                          1usize
                                                      ) as (isize)
                                                  ) as (i32) == b'.' as (i32)) {
                (*r).branchhead = 1i32;
                *(*r).first.offset(
                     strlen((*r).first as (*const u8)).wrapping_sub(1usize) as (isize)
                 ) = b'\0';
            }
        }
        if *(*r).first as (i32) == b'\0' as (i32) {
            (*r).first = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        }
        if *(*r).last as (i32) == b'\0' as (i32) {
            (*r).last = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        }
        if (*r).first != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            (*r).first = Xstrdup((*r).first as (*const u8));
        }
        if (*r).last != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            (*r).last = Xstrdup((*r).last as (*const u8));
        }
        *pr = r;
        pr = &mut (*r).next as (*mut *mut option_revlist) as (*mut *mut option_revlist);
        copy = comma;
    }
    free(orig_copy as (*mut ::std::os::raw::c_void));
    ret
}

unsafe extern fn log_parse_date(
    mut log_data : *mut log_data, mut argstring : *const u8
) {
    let mut orig_copy : *mut u8;
    let mut copy : *mut u8;
    orig_copy = {
                    copy = Xstrdup(argstring);
                    copy
                };
    'loop1: loop {
        if !(copy != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)) {
            break;
        }
        let mut nd : *mut datelist;
        let mut pd : *mut *mut datelist;
        let mut cpend : *mut u8;
        let mut cp : *mut u8;
        let mut ds : *mut u8;
        let mut de : *mut u8;
        nd = xmalloc(::std::mem::size_of::<datelist>()) as (*mut datelist);
        cpend = strchr(copy as (*const u8),b';' as (i32));
        if cpend != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            *{
                 let _old = cpend;
                 cpend = cpend.offset(1isize);
                 _old
             } = b'\0';
        }
        pd = &mut (*log_data).datelist as (*mut *mut datelist);
        (*nd).inclusive = 0i32;
        if {
               cp = strchr(copy as (*const u8),b'>' as (i32));
               cp
           } != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            *{
                 let _old = cp;
                 cp = cp.offset(1isize);
                 _old
             } = b'\0';
            if *cp as (i32) == b'=' as (i32) {
                cp = cp.offset(1isize);
                (*nd).inclusive = 1i32;
            }
            ds = cp;
            de = copy;
        } else if {
                      cp = strchr(copy as (*const u8),b'<' as (i32));
                      cp
                  } != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            *{
                 let _old = cp;
                 cp = cp.offset(1isize);
                 _old
             } = b'\0';
            if *cp as (i32) == b'=' as (i32) {
                cp = cp.offset(1isize);
                (*nd).inclusive = 1i32;
            }
            ds = copy;
            de = cp;
        } else {
            ds = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
            de = copy;
            pd = &mut (*log_data).singledatelist as (*mut *mut datelist);
        }
        if ds == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            (*nd).start = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        } else if *ds as (i32) != b'\0' as (i32) {
            (*nd).start = Make_Date(ds as (*const u8));
        } else {
            (*nd).start = Make_Date((*b"1/1/1970 UTC\0").as_ptr());
        }
        if *de as (i32) != b'\0' as (i32) {
            (*nd).end = Make_Date(de as (*const u8));
        } else {
            (*nd).end = Make_Date((*b"2038-01-01\0").as_ptr());
        }
        (*nd).next = *pd as (*mut datelist);
        *pd = nd;
        copy = cpend;
    }
    free(orig_copy as (*mut ::std::os::raw::c_void));
}

unsafe extern fn log_parse_list(
    mut plist : *mut *mut ::hash::hashlist, mut argstring : *const u8
) {
    'loop0: loop {
        let mut p : *mut ::hash::hashnode;
        let mut cp : *mut u8;
        p = ::hash::getnode();
        cp = strchr(argstring,b',' as (i32));
        if cp == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            (*p).key = Xstrdup(argstring);
        } else {
            let mut len : usize;
            len = ((cp as (isize)).wrapping_sub(
                       argstring as (isize)
                   ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
            (*p).key = xmalloc(len.wrapping_add(1usize)) as (*mut u8);
            strncpy((*p).key,argstring,len);
            *(*p).key.offset(len as (isize)) = b'\0';
        }
        if *plist == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashlist) {
            *plist = ::hash::getlist();
        }
        if ::hash::addnode(*plist,p) != 0i32 {
            ::hash::freenode(p);
        }
        if cp == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            break;
        }
        argstring = cp.offset(1isize) as (*const u8);
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct revlist {
    pub next : *mut revlist,
    pub first : *mut u8,
    pub last : *mut u8,
    pub fields : i32,
    pub inclusive : i32,
}

impl Clone for revlist {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct log_data_and_rcs {
    pub log_data : *mut log_data,
    pub revlist : *mut revlist,
    pub rcs : *mut ::vers_ts::rcsnode,
}

impl Clone for log_data_and_rcs {
    fn clone(&self) -> Self { *self }
}

unsafe extern fn printlock_proc(
    mut lock : *mut ::hash::hashnode,
    mut foo : *mut ::std::os::raw::c_void
) -> i32 {
    cvs_output((*b"\n\t\0").as_ptr(),2usize);
    cvs_output((*lock).data as (*const u8),0usize);
    cvs_output((*b": \0").as_ptr(),2usize);
    cvs_output((*lock).key as (*const u8),0usize);
    0i32
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

#[derive(Copy)]
#[repr(C)]
pub struct rcsversnode {
    pub version : *mut u8,
    pub date : *mut u8,
    pub author : *mut u8,
    pub state : *mut u8,
    pub next : *mut u8,
    pub dead : i32,
    pub outdated : i32,
    pub text : *mut deltatext,
    pub branches : *mut ::hash::hashlist,
    pub other : *mut ::hash::hashlist,
    pub other_delta : *mut ::hash::hashlist,
}

impl Clone for rcsversnode {
    fn clone(&self) -> Self { *self }
}

unsafe extern fn log_fileproc(
    mut callerdat : *mut ::std::os::raw::c_void,
    mut finfo : *mut ::vers_ts::file_info
) -> i32 {
    let mut log_data : *mut log_data = callerdat as (*mut log_data);
    let mut p : *mut ::hash::hashnode;
    let mut baserev : *mut u8;
    let mut selrev : i32 = -1i32;
    let mut rcsfile : *mut ::vers_ts::rcsnode;
    let mut buf = [0u8; 50];
    let mut revlist
        : *mut revlist
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut revlist);
    rcsfile = (*finfo).rcs;
    p = ::hash::findnode((*finfo).entries,(*finfo).file);
    if p != 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode) {
        let mut e
            : *mut ::entries::entnode
            = (*p).data as (*mut ::entries::entnode);
        baserev = (*e).version;
        if *baserev.offset(0isize) as (i32) == b'-' as (i32) {
            baserev = baserev.offset(1isize);
        }
    } else {
        baserev = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    }
    if rcsfile == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::vers_ts::rcsnode) {
        if baserev != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            if *baserev.offset(
                    0isize
                ) as (i32) == b'0' as (i32) && (*baserev.offset(
                                                     1isize
                                                 ) as (i32) == b'\0' as (i32)) {
                if really_quiet == 0 {
                    error(
                        0i32,
                        0i32,
                        (*b"%s has been added, but not committed\0").as_ptr(),
                        (*finfo).file
                    );
                }
                return 0i32;
            }
        }
        if really_quiet == 0 {
            error(
                0i32,
                0i32,
                (*b"Nothing known about %s\0").as_ptr(),
                quote((*finfo).file)
            );
        }
        1i32
    } else {
        if (*log_data).sup_header != 0 || (*log_data).nameonly == 0 {
            RCS_fully_parse(rcsfile);
            revlist = log_expand_revlist(
                          rcsfile,
                          baserev,
                          (*log_data).revlist,
                          (*log_data).default_branch
                      );
            if (*log_data).sup_header != 0 || (*log_data).header == 0 && ((*log_data).long_header == 0) {
                let mut log_data_and_rcs = log_data_and_rcs {
                    log_data: log_data,
                    revlist: revlist,
                    rcs: rcsfile,
                };
                if (*log_data).singledatelist != 0i32 as (*mut ::std::os::raw::c_void) as (*mut datelist) {
                    ::hash::walklist(
                        (*rcsfile).versions,
                        log_fix_singledate,
                        &mut log_data_and_rcs as (*mut log_data_and_rcs) as (*mut ::std::os::raw::c_void)
                    );
                }
                selrev = ::hash::walklist(
                             (*rcsfile).versions,
                             log_count_print,
                             &mut log_data_and_rcs as (*mut log_data_and_rcs) as (*mut ::std::os::raw::c_void)
                         );
                if (*log_data).sup_header != 0 && (selrev == 0i32) {
                    log_free_revlist(revlist);
                    return 0i32;
                }
            }
        }
        (if (*log_data).nameonly != 0 {
             cvs_output((*rcsfile).print_path as (*const u8),0usize);
             cvs_output((*b"\n\0").as_ptr(),1usize);
             log_free_revlist(revlist);
             0i32
         } else {
             cvs_output((*b"\n\0").as_ptr(),1usize);
             cvs_output((*b"RCS file: \0").as_ptr(),0usize);
             cvs_output((*rcsfile).print_path as (*const u8),0usize);
             if is_rlog == 0 {
                 cvs_output((*b"\nWorking file: \0").as_ptr(),0usize);
                 if *(*finfo).update_dir.offset(0isize) as (i32) != b'\0' as (i32) {
                     cvs_output((*finfo).update_dir,0usize);
                     cvs_output((*b"/\0").as_ptr(),0usize);
                 }
                 cvs_output((*finfo).file,0usize);
             }
             cvs_output((*b"\nhead:\0").as_ptr(),0usize);
             if (*rcsfile).head != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                 cvs_output((*b" \0").as_ptr(),1usize);
                 cvs_output((*rcsfile).head as (*const u8),0usize);
             }
             cvs_output((*b"\nbranch:\0").as_ptr(),0usize);
             if (*rcsfile).branch != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                 cvs_output((*b" \0").as_ptr(),1usize);
                 cvs_output((*rcsfile).branch as (*const u8),0usize);
             }
             cvs_output((*b"\nlocks:\0").as_ptr(),0usize);
             if (*rcsfile).strict_locks != 0 {
                 cvs_output((*b" strict\0").as_ptr(),0usize);
             }
             ::hash::walklist(
                 RCS_getlocks(rcsfile),
                 printlock_proc,
                 0i32 as (*mut ::std::os::raw::c_void)
             );
             cvs_output((*b"\naccess list:\0").as_ptr(),0usize);
             if (*rcsfile).access != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                 let mut cp : *const u8;
                 cp = (*rcsfile).access as (*const u8);
                 'loop23: loop {
                     if !(*cp as (i32) != b'\0' as (i32)) {
                         break;
                     }
                     let mut cp2 : *const u8;
                     cvs_output((*b"\n\t\0").as_ptr(),2usize);
                     cp2 = cp;
                     'loop52: loop {
                         if !(isspace(
                                  *cp2 as (i32)
                              ) == 0 && (*cp2 as (i32) != b'\0' as (i32))) {
                             break;
                         }
                         cp2 = cp2.offset(1isize);
                     }
                     cvs_output(
                         cp,
                         ((cp2 as (isize)).wrapping_sub(
                              cp as (isize)
                          ) / ::std::mem::size_of::<u8>() as (isize)) as (usize)
                     );
                     cp = cp2;
                     'loop54: loop {
                         if !(isspace(
                                  *cp as (i32)
                              ) != 0 && (*cp as (i32) != b'\0' as (i32))) {
                             break;
                         }
                         cp = cp.offset(1isize);
                     }
                 }
             }
             if (*log_data).notags == 0 {
                 let mut syms : *mut ::hash::hashlist;
                 cvs_output((*b"\nsymbolic names:\0").as_ptr(),0usize);
                 syms = RCS_symbols(rcsfile);
                 ::hash::walklist(
                     syms,
                     log_symbol,
                     0i32 as (*mut ::std::os::raw::c_void)
                 );
             }
             cvs_output((*b"\nkeyword substitution: \0").as_ptr(),0usize);
             if (*rcsfile).expand == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                 cvs_output((*b"kv\0").as_ptr(),2usize);
             } else {
                 cvs_output((*rcsfile).expand as (*const u8),0usize);
             }
             cvs_output((*b"\ntotal revisions: \0").as_ptr(),0usize);
             sprintf(
                 buf.as_mut_ptr(),
                 (*b"%d\0").as_ptr(),
                 ::hash::walklist(
                     (*rcsfile).versions,
                     log_count,
                     0i32 as (*mut ::std::os::raw::c_void)
                 )
             );
             cvs_output(buf.as_mut_ptr() as (*const u8),0usize);
             if selrev >= 0i32 {
                 cvs_output((*b";\tselected revisions: \0").as_ptr(),0usize);
                 sprintf(buf.as_mut_ptr(),(*b"%d\0").as_ptr(),selrev);
                 cvs_output(buf.as_mut_ptr() as (*const u8),0usize);
             }
             cvs_output((*b"\n\0").as_ptr(),1usize);
             if (*log_data).header == 0 || (*log_data).long_header != 0 {
                 cvs_output((*b"description:\n\0").as_ptr(),0usize);
                 if (*rcsfile).desc != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                     cvs_output((*rcsfile).desc as (*const u8),0usize);
                 }
             }
             if (*log_data).header == 0 && ((*log_data).long_header == 0) && ((*rcsfile).head != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)) {
                 p = ::hash::findnode(
                         (*rcsfile).versions,
                         (*rcsfile).head as (*const u8)
                     );
                 if p == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode) {
                     error(
                         1i32,
                         0i32,
                         (*b"can not find head revision in `%s\'\0").as_ptr(),
                         (*finfo).fullname
                     );
                 }
                 'loop37: loop {
                     if !(p != 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode)) {
                         break;
                     }
                     let mut vers : *mut rcsversnode = (*p).data as (*mut rcsversnode);
                     log_version(log_data,revlist,rcsfile,vers,1i32);
                     if (*vers).next == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                         p = 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode);
                     } else {
                         p = ::hash::findnode(
                                 (*rcsfile).versions,
                                 (*vers).next as (*const u8)
                             );
                         if !(p == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode)) {
                             continue;
                         }
                         error(
                             1i32,
                             0i32,
                             (*b"can not find next revision `%s\' in `%s\'\0").as_ptr(),
                             (*vers).next,
                             (*finfo).fullname
                         );
                     }
                 }
                 log_tree(log_data,revlist,rcsfile,(*rcsfile).head as (*const u8));
             }
             cvs_output(
                 (*b"=============================================================================\n\0").as_ptr(
                 ),
                 0usize
             );
             log_free_revlist(revlist);
             if (*log_data).singledatelist != 0i32 as (*mut ::std::os::raw::c_void) as (*mut datelist) {
                 let mut d : *mut datelist;
                 d = (*log_data).singledatelist;
                 'loop41: loop {
                     if !(d != 0i32 as (*mut ::std::os::raw::c_void) as (*mut datelist)) {
                         break;
                     }
                     if (*d).start != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                         free((*d).start as (*mut ::std::os::raw::c_void));
                     }
                     (*d).start = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                     d = (*d).next as (*mut datelist);
                 }
             }
             0i32
         })
    }
}

unsafe extern fn log_expand_revlist(
    mut rcs : *mut ::vers_ts::rcsnode,
    mut baserev : *mut u8,
    mut revlist : *mut option_revlist,
    mut default_branch : i32
) -> *mut revlist {
    let mut r : *mut option_revlist;
    let mut ret : *mut revlist;
    let mut pr : *mut *mut revlist;
    ret = 0i32 as (*mut ::std::os::raw::c_void) as (*mut revlist);
    pr = &mut ret as (*mut *mut revlist);
    r = revlist;
    'loop1: loop {
        if !(r != 0i32 as (*mut ::std::os::raw::c_void) as (*mut option_revlist)) {
            break;
        }
        let mut nr : *mut revlist;
        nr = xmalloc(::std::mem::size_of::<revlist>()) as (*mut revlist);
        (*nr).inclusive = (*r).inclusive;
        if (*r).first == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) && ((*r).last == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)) {
            (*nr).first = RCS_head(rcs);
            if (*nr).first.is_null() {
                if really_quiet == 0 {
                    error(
                        0i32,
                        0i32,
                        (*b"No head revision in archive `%s\'.\0").as_ptr(),
                        (*rcs).path
                    );
                }
                (*nr).last = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                (*nr).fields = 0i32;
            } else {
                (*nr).last = Xstrdup((*nr).first as (*const u8));
                (*nr).fields = numdots((*nr).first as (*const u8)) + 1i32;
            }
        } else if (*r).branchhead != 0 {
            let mut branch : *mut u8;
            if !(*r).first.is_null() {
                0i32;
            } else {
                __assert_fail(
                    (*b"r->first\0").as_ptr(),
                    file!().as_ptr(),
                    line!(),
                    (*b"log_expand_revlist\0").as_ptr()
                );
            }
            if isdigit(*(*r).first.offset(0isize) as (i32)) != 0 {
                (*nr).first = RCS_getbranch(rcs,(*r).first as (*const u8),1i32);
            } else {
                branch = RCS_whatbranch(rcs,(*r).first as (*const u8));
                if branch == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                    (*nr).first = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                } else {
                    (*nr).first = RCS_getbranch(rcs,branch as (*const u8),1i32);
                    free(branch as (*mut ::std::os::raw::c_void));
                }
            }
            if (*nr).first.is_null() {
                if really_quiet == 0 {
                    error(
                        0i32,
                        0i32,
                        (*b"warning: no branch `%s\' in `%s\'\0").as_ptr(),
                        (*r).first,
                        (*rcs).print_path
                    );
                }
                (*nr).last = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                (*nr).fields = 0i32;
            } else {
                (*nr).last = Xstrdup((*nr).first as (*const u8));
                (*nr).fields = numdots((*nr).first as (*const u8)) + 1i32;
            }
        } else {
            if (*r).first == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) || isdigit(
                                                                                       *(*r).first.offset(
                                                                                            0isize
                                                                                        ) as (i32)
                                                                                   ) != 0 {
                (*nr).first = Xstrdup((*r).first as (*const u8));
            } else {
                if !baserev.is_null() && streq(
                                             (*r).first as (*const u8),
                                             (*b"BASE\0").as_ptr()
                                         ) {
                    (*nr).first = Xstrdup(baserev as (*const u8));
                } else if RCS_nodeisbranch(rcs,(*r).first as (*const u8)) != 0 {
                    (*nr).first = RCS_whatbranch(rcs,(*r).first as (*const u8));
                } else {
                    (*nr).first = RCS_gettag(
                                      rcs,
                                      (*r).first as (*const u8),
                                      1i32,
                                      0i32 as (*mut ::std::os::raw::c_void) as (*mut i32)
                                  );
                }
                if (*nr).first == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) && (really_quiet == 0) {
                    error(
                        0i32,
                        0i32,
                        (*b"warning: no revision `%s\' in `%s\'\0").as_ptr(),
                        (*r).first,
                        (*rcs).print_path
                    );
                }
            }
            if (*r).last == (*r).first || (*r).last != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) && ((*r).first != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)) && streq(
                                                                                                                                                                                       (*r).last as (*const u8),
                                                                                                                                                                                       (*r).first as (*const u8)
                                                                                                                                                                                   ) {
                (*nr).last = Xstrdup((*nr).first as (*const u8));
            } else if (*r).last == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) || isdigit(
                                                                                             *(*r).last.offset(
                                                                                                  0isize
                                                                                              ) as (i32)
                                                                                         ) != 0 {
                (*nr).last = Xstrdup((*r).last as (*const u8));
            } else {
                if !baserev.is_null() && streq(
                                             (*r).last as (*const u8),
                                             (*b"BASE\0").as_ptr()
                                         ) {
                    (*nr).last = Xstrdup(baserev as (*const u8));
                } else if RCS_nodeisbranch(rcs,(*r).last as (*const u8)) != 0 {
                    (*nr).last = RCS_whatbranch(rcs,(*r).last as (*const u8));
                } else {
                    (*nr).last = RCS_gettag(
                                     rcs,
                                     (*r).last as (*const u8),
                                     1i32,
                                     0i32 as (*mut ::std::os::raw::c_void) as (*mut i32)
                                 );
                }
                if (*nr).last == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) && (really_quiet == 0) {
                    error(
                        0i32,
                        0i32,
                        (*b"warning: no revision `%s\' in `%s\'\0").as_ptr(),
                        (*r).last,
                        (*rcs).print_path
                    );
                }
            }
            if (*r).first == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) && ((*nr).last != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)) {
                (*nr).fields = numdots((*nr).last as (*const u8)) + 1i32;
                if (*nr).fields < 2i32 {
                    (*nr).first = Xstrdup((*b".0\0").as_ptr());
                } else {
                    let mut cp : *mut u8;
                    (*nr).first = Xstrdup((*nr).last as (*const u8));
                    cp = strrchr((*nr).first as (*const u8),b'.' as (i32));
                    if !cp.is_null() {
                        0i32;
                    } else {
                        __assert_fail(
                            (*b"cp\0").as_ptr(),
                            file!().as_ptr(),
                            line!(),
                            (*b"log_expand_revlist\0").as_ptr()
                        );
                    }
                    strcpy(cp.offset(1isize),(*b"0\0").as_ptr());
                }
            } else if (*r).last == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) && ((*nr).first != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)) {
                (*nr).fields = numdots((*nr).first as (*const u8)) + 1i32;
                (*nr).last = Xstrdup((*nr).first as (*const u8));
                if (*nr).fields < 2i32 {
                    *(*nr).last.offset(0isize) = b'\0';
                } else {
                    let mut cp : *mut u8;
                    cp = strrchr((*nr).last as (*const u8),b'.' as (i32));
                    if !cp.is_null() {
                        0i32;
                    } else {
                        __assert_fail(
                            (*b"cp\0").as_ptr(),
                            file!().as_ptr(),
                            line!(),
                            (*b"log_expand_revlist\0").as_ptr()
                        );
                    }
                    *cp = b'\0';
                }
            } else if (*nr).first == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) || (*nr).last == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                (*nr).fields = 0i32;
            } else if streq(
                          (*nr).first as (*const u8),
                          (*nr).last as (*const u8)
                      ) {
                (*nr).fields = numdots((*nr).last as (*const u8)) + 1i32;
            } else {
                let mut ord : i32;
                let mut dots1 : i32 = numdots((*nr).first as (*const u8));
                let mut dots2 : i32 = numdots((*nr).last as (*const u8));
                if dots1 > dots2 || dots1 == dots2 && (version_compare(
                                                           (*nr).first as (*const u8),
                                                           (*nr).last as (*const u8),
                                                           dots1 + 1i32
                                                       ) > 0i32) {
                    let mut tmp : *mut u8 = (*nr).first;
                    (*nr).first = (*nr).last;
                    (*nr).last = tmp;
                    (*nr).fields = dots2 + 1i32;
                    dots2 = dots1;
                    dots1 = (*nr).fields - 1i32;
                } else {
                    (*nr).fields = dots1 + 1i32;
                }
                dots1 = dots1 + ((*nr).fields & 1i32);
                ord = version_compare(
                          (*nr).first as (*const u8),
                          (*nr).last as (*const u8),
                          dots1
                      );
                if ord > 0i32 || (*nr).fields > 2i32 && (ord < 0i32) {
                    error(
                        0i32,
                        0i32,
                        (*b"invalid branch or revision pair %s:%s in `%s\'\0").as_ptr(),
                        (*r).first,
                        (*r).last,
                        (*rcs).print_path
                    );
                    free((*nr).first as (*mut ::std::os::raw::c_void));
                    (*nr).first = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                    free((*nr).last as (*mut ::std::os::raw::c_void));
                    (*nr).last = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                    (*nr).fields = 0i32;
                } else {
                    if (*nr).fields <= dots2 && ((*nr).fields & 1i32 != 0) {
                        let mut p : *mut u8 = Xasprintf((*b"%s.0\0").as_ptr(),(*nr).first);
                        free((*nr).first as (*mut ::std::os::raw::c_void));
                        (*nr).first = p;
                        (*nr).fields = (*nr).fields + 1;
                    }
                    'loop40: loop {
                        if !((*nr).fields <= dots2) {
                            break;
                        }
                        let mut p : *mut u8;
                        let mut i : i32;
                        (*nr).next = 0i32 as (*mut ::std::os::raw::c_void) as (*mut revlist);
                        *pr = nr;
                        nr = xmalloc(::std::mem::size_of::<revlist>()) as (*mut revlist);
                        (*nr).inclusive = 1i32;
                        (*nr).first = Xstrdup((**pr).last as (*const u8));
                        (*nr).last = Xstrdup((**pr).last as (*const u8));
                        (*nr).fields = (**pr).fields;
                        p = (**pr).last;
                        i = 0i32;
                        'loop42: loop {
                            if !(i < (*nr).fields) {
                                break;
                            }
                            p = strchr(p as (*const u8),b'.' as (i32)).offset(1isize);
                            i = i + 1;
                        }
                        *p.offset(-1isize) = b'\0';
                        p = strchr(
                                (*nr).first.offset(
                                    (p as (isize)).wrapping_sub(
                                        (**pr).last as (isize)
                                    ) / ::std::mem::size_of::<u8>() as (isize)
                                ) as (*const u8),
                                b'.' as (i32)
                            );
                        if p != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                            *{
                                 p = p.offset(1isize);
                                 p
                             } = b'0';
                            *{
                                 p = p.offset(1isize);
                                 p
                             } = b'\0';
                            (*nr).fields = (*nr).fields + 2i32;
                        } else {
                            (*nr).fields = (*nr).fields + 1;
                        }
                        pr = &mut (**pr).next as (*mut *mut revlist) as (*mut *mut revlist);
                    }
                }
            }
        }
        (*nr).next = 0i32 as (*mut ::std::os::raw::c_void) as (*mut revlist);
        *pr = nr;
        pr = &mut (*nr).next as (*mut *mut revlist) as (*mut *mut revlist);
        r = (*r).next as (*mut option_revlist);
    }
    if default_branch != 0 && ((*rcs).head != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) || (*rcs).branch != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)) {
        let mut nr : *mut revlist;
        nr = xmalloc(::std::mem::size_of::<revlist>()) as (*mut revlist);
        if (*rcs).branch != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            (*nr).first = Xstrdup((*rcs).branch as (*const u8));
        } else {
            let mut cp : *mut u8;
            (*nr).first = Xstrdup((*rcs).head as (*const u8));
            if !(*nr).first.is_null() {
                0i32;
            } else {
                __assert_fail(
                    (*b"nr->first\0").as_ptr(),
                    file!().as_ptr(),
                    line!(),
                    (*b"log_expand_revlist\0").as_ptr()
                );
            }
            cp = strrchr((*nr).first as (*const u8),b'.' as (i32));
            if !cp.is_null() {
                0i32;
            } else {
                __assert_fail(
                    (*b"cp\0").as_ptr(),
                    file!().as_ptr(),
                    line!(),
                    (*b"log_expand_revlist\0").as_ptr()
                );
            }
            *cp = b'\0';
        }
        (*nr).last = Xstrdup((*nr).first as (*const u8));
        (*nr).fields = numdots((*nr).first as (*const u8)) + 1i32;
        (*nr).inclusive = 1i32;
        (*nr).next = 0i32 as (*mut ::std::os::raw::c_void) as (*mut revlist);
        *pr = nr;
    }
    ret
}

unsafe extern fn log_free_revlist(mut revlist : *mut revlist) {
    let mut r : *mut revlist;
    r = revlist;
    'loop1: loop {
        if !(r != 0i32 as (*mut ::std::os::raw::c_void) as (*mut revlist)) {
            break;
        }
        let mut next : *mut revlist;
        if (*r).first != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            free((*r).first as (*mut ::std::os::raw::c_void));
        }
        if (*r).last != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            free((*r).last as (*mut ::std::os::raw::c_void));
        }
        next = (*r).next as (*mut revlist);
        free(r as (*mut ::std::os::raw::c_void));
        r = next;
    }
}

unsafe extern fn log_symbol(
    mut p : *mut ::hash::hashnode,
    mut closure : *mut ::std::os::raw::c_void
) -> i32 {
    cvs_output((*b"\n\t\0").as_ptr(),2usize);
    cvs_output((*p).key as (*const u8),0usize);
    cvs_output((*b": \0").as_ptr(),2usize);
    cvs_output((*p).data as (*const u8),0usize);
    0i32
}

unsafe extern fn log_count(
    mut p : *mut ::hash::hashnode,
    mut closure : *mut ::std::os::raw::c_void
) -> i32 {
    1i32
}

unsafe extern fn log_version_requested(
    mut log_data : *mut log_data,
    mut revlist : *mut revlist,
    mut rcs : *mut ::vers_ts::rcsnode,
    mut vnode : *mut rcsversnode
) -> i32 {
    let mut _currentBlock;
    if (*log_data).statelist != 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashlist) && (::hash::findnode(
                                                                                                         (*log_data).statelist,
                                                                                                         (*vnode).state as (*const u8)
                                                                                                     ) == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode)) {
        0i32
    } else {
        if (*log_data).authorlist != 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashlist) {
            if (*vnode).author != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) && (::hash::findnode(
                                                                                             (*log_data).authorlist,
                                                                                             (*vnode).author as (*const u8)
                                                                                         ) == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode)) {
                return 0i32;
            }
        }
        if (*log_data).datelist != 0i32 as (*mut ::std::os::raw::c_void) as (*mut datelist) || (*log_data).singledatelist != 0i32 as (*mut ::std::os::raw::c_void) as (*mut datelist) {
            let mut d : *mut datelist;
            d = (*log_data).datelist;
            'loop5: loop {
                if !(d != 0i32 as (*mut ::std::os::raw::c_void) as (*mut datelist)) {
                    break;
                }
                let mut cmp : i32;
                cmp = RCS_datecmp(
                          (*vnode).date as (*const u8),
                          (*d).start as (*const u8)
                      );
                if cmp > 0i32 || cmp == 0i32 && ((*d).inclusive != 0) {
                    cmp = RCS_datecmp(
                              (*vnode).date as (*const u8),
                              (*d).end as (*const u8)
                          );
                    if cmp < 0i32 || cmp == 0i32 && ((*d).inclusive != 0) {
                        break;
                    }
                }
                d = (*d).next as (*mut datelist);
            }
            if d == 0i32 as (*mut ::std::os::raw::c_void) as (*mut datelist) {
                d = (*log_data).singledatelist;
                'loop11: loop {
                    if !(d != 0i32 as (*mut ::std::os::raw::c_void) as (*mut datelist)) {
                        break;
                    }
                    if (*d).start != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) && (RCS_datecmp(
                                                                                                (*vnode).date as (*const u8),
                                                                                                (*d).start as (*const u8)
                                                                                            ) == 0i32) {
                        break;
                    }
                    d = (*d).next as (*mut datelist);
                }
                if d == 0i32 as (*mut ::std::os::raw::c_void) as (*mut datelist) {
                    return 0i32;
                }
            }
        }
        (if revlist != 0i32 as (*mut ::std::os::raw::c_void) as (*mut revlist) {
             let mut v : *mut u8;
             let mut vfields : i32;
             let mut r : *mut revlist;
             v = (*vnode).version;
             vfields = numdots(v as (*const u8)) + 1i32;
             r = revlist;
             'loop18: loop {
                 if !(r != 0i32 as (*mut ::std::os::raw::c_void) as (*mut revlist)) {
                     _currentBlock = 19;
                     break;
                 }
                 if vfields == (*r).fields + ((*r).fields & 1i32) && (if (*r).inclusive != 0 {
                                                                          (version_compare(
                                                                               v as (*const u8),
                                                                               (*r).first as (*const u8),
                                                                               (*r).fields
                                                                           ) >= 0i32) as (i32)
                                                                      } else {
                                                                          (version_compare(
                                                                               v as (*const u8),
                                                                               (*r).first as (*const u8),
                                                                               (*r).fields
                                                                           ) > 0i32) as (i32)
                                                                      } != 0) && (version_compare(
                                                                                      v as (*const u8),
                                                                                      (*r).last as (*const u8),
                                                                                      (*r).fields
                                                                                  ) <= 0i32) {
                     _currentBlock = 22;
                     break;
                 }
                 r = (*r).next as (*mut revlist);
             }
             (if _currentBlock == 19 { 0i32 } else { 1i32 })
         } else {
             1i32
         })
    }
}

unsafe extern fn log_fix_singledate(
    mut p : *mut ::hash::hashnode,
    mut closure : *mut ::std::os::raw::c_void
) -> i32 {
    let mut data
        : *mut log_data_and_rcs
        = closure as (*mut log_data_and_rcs);
    let mut pv : *mut ::hash::hashnode;
    let mut vnode : *mut rcsversnode;
    let mut holdsingle : *mut datelist;
    let mut holddate : *mut datelist;
    let mut requested : i32;
    pv = ::hash::findnode(
             (*(*data).rcs).versions,
             (*p).key as (*const u8)
         );
    if pv == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode) {
        error(
            1i32,
            0i32,
            (*b"missing version `%s\' in RCS file `%s\'\0").as_ptr(),
            (*p).key,
            (*(*data).rcs).print_path
        );
    }
    vnode = (*pv).data as (*mut rcsversnode);
    holdsingle = (*(*data).log_data).singledatelist;
    (*(*data).log_data).singledatelist = 0i32 as (*mut ::std::os::raw::c_void) as (*mut datelist);
    holddate = (*(*data).log_data).datelist;
    (*(*data).log_data).datelist = 0i32 as (*mut ::std::os::raw::c_void) as (*mut datelist);
    requested = log_version_requested(
                    (*data).log_data,
                    (*data).revlist,
                    (*data).rcs,
                    vnode
                );
    (*(*data).log_data).singledatelist = holdsingle;
    (*(*data).log_data).datelist = holddate;
    if requested != 0 {
        let mut d : *mut datelist;
        d = (*(*data).log_data).singledatelist;
        'loop4: loop {
            if !(d != 0i32 as (*mut ::std::os::raw::c_void) as (*mut datelist)) {
                break;
            }
            if RCS_datecmp(
                   (*vnode).date as (*const u8),
                   (*d).end as (*const u8)
               ) <= 0i32 && ((*d).start == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) || RCS_datecmp(
                                                                                                     (*vnode).date as (*const u8),
                                                                                                     (*d).start as (*const u8)
                                                                                                 ) > 0i32) {
                if (*d).start != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                    free((*d).start as (*mut ::std::os::raw::c_void));
                }
                (*d).start = Xstrdup((*vnode).date as (*const u8));
            }
            d = (*d).next as (*mut datelist);
        }
    }
    0i32
}

unsafe extern fn log_count_print(
    mut p : *mut ::hash::hashnode,
    mut closure : *mut ::std::os::raw::c_void
) -> i32 {
    let mut data
        : *mut log_data_and_rcs
        = closure as (*mut log_data_and_rcs);
    let mut pv : *mut ::hash::hashnode;
    pv = ::hash::findnode(
             (*(*data).rcs).versions,
             (*p).key as (*const u8)
         );
    if pv == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode) {
        error(
            1i32,
            0i32,
            (*b"missing version `%s\' in RCS file `%s\'\0").as_ptr(),
            (*p).key,
            (*(*data).rcs).print_path
        );
    }
    if log_version_requested(
           (*data).log_data,
           (*data).revlist,
           (*data).rcs,
           (*pv).data as (*mut rcsversnode)
       ) != 0 {
        1i32
    } else {
        0i32
    }
}

unsafe extern fn log_tree(
    mut log_data : *mut log_data,
    mut revlist : *mut revlist,
    mut rcs : *mut ::vers_ts::rcsnode,
    mut ver : *const u8
) {
    let mut p : *mut ::hash::hashnode;
    let mut vnode : *mut rcsversnode;
    p = ::hash::findnode((*rcs).versions,ver);
    if p == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode) {
        error(
            1i32,
            0i32,
            (*b"missing version `%s\' in RCS file `%s\'\0").as_ptr(),
            ver,
            (*rcs).print_path
        );
    }
    vnode = (*p).data as (*mut rcsversnode);
    if (*vnode).next != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        log_tree(log_data,revlist,rcs,(*vnode).next as (*const u8));
    }
    if (*vnode).branches != 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashlist) {
        let mut head : *mut ::hash::hashnode;
        let mut branch : *mut ::hash::hashnode;
        head = (*(*vnode).branches).list;
        branch = (*head).prev as (*mut ::hash::hashnode);
        'loop6: loop {
            if !(branch != head) {
                break;
            }
            log_abranch(log_data,revlist,rcs,(*branch).key as (*const u8));
            log_tree(log_data,revlist,rcs,(*branch).key as (*const u8));
            branch = (*branch).prev as (*mut ::hash::hashnode);
        }
    }
}

unsafe extern fn log_abranch(
    mut log_data : *mut log_data,
    mut revlist : *mut revlist,
    mut rcs : *mut ::vers_ts::rcsnode,
    mut ver : *const u8
) {
    let mut p : *mut ::hash::hashnode;
    let mut vnode : *mut rcsversnode;
    p = ::hash::findnode((*rcs).versions,ver);
    if p == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode) {
        error(
            1i32,
            0i32,
            (*b"missing version `%s\' in RCS file `%s\'\0").as_ptr(),
            ver,
            (*rcs).print_path
        );
    }
    vnode = (*p).data as (*mut rcsversnode);
    if (*vnode).next != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        log_abranch(log_data,revlist,rcs,(*vnode).next as (*const u8));
    }
    log_version(log_data,revlist,rcs,vnode,0i32);
}

#[derive(Copy)]
#[repr(C)]
pub struct buffer {
    pub data : *mut buffer_data,
    pub last : *mut buffer_data,
    pub nonblocking : bool,
    pub input : unsafe extern fn(*mut ::std::os::raw::c_void, *mut u8, usize, usize, *mut usize) -> i32,
    pub output : unsafe extern fn(*mut ::std::os::raw::c_void, *const u8, usize, *mut usize) -> i32,
    pub flush : unsafe extern fn(*mut ::std::os::raw::c_void) -> i32,
    pub block : unsafe extern fn(*mut ::std::os::raw::c_void, bool) -> i32,
    pub get_fd : unsafe extern fn(*mut ::std::os::raw::c_void) -> i32,
    pub shutdown : unsafe extern fn(*mut buffer) -> i32,
    pub closure : *mut ::std::os::raw::c_void,
    pub memory_error : unsafe extern fn(*mut buffer),
}

impl Clone for buffer {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct openpgp_signature {
    pub ctime : isize,
    pub keyid : usize,
    pub raw : *mut u8,
    pub rawlen : usize,
}

impl Clone for openpgp_signature {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct base64_decode_context {
    pub i : u32,
    pub buf : [u8; 4],
}

impl Clone for base64_decode_context {
    fn clone(&self) -> Self { *self }
}

unsafe extern fn log_version(
    mut log_data : *mut log_data,
    mut revlist : *mut revlist,
    mut rcs : *mut ::vers_ts::rcsnode,
    mut ver : *mut rcsversnode,
    mut trunk : i32
) {
    let mut p : *mut ::hash::hashnode;
    let mut year : i32 = 0;
    let mut mon : i32 = 0;
    let mut mday : i32 = 0;
    let mut hour : i32 = 0;
    let mut min : i32 = 0;
    let mut sec : i32 = 0;
    let mut buf = [0u8; 100];
    let mut padd : *mut ::hash::hashnode;
    let mut pdel : *mut ::hash::hashnode;
    if log_version_requested(log_data,revlist,rcs,ver) == 0 {
    } else {
        cvs_output(
            (*b"----------------------------\nrevision \0").as_ptr(),
            0usize
        );
        cvs_output((*ver).version as (*const u8),0usize);
        p = ::hash::findnode(
                RCS_getlocks(rcs),
                (*ver).version as (*const u8)
            );
        if p != 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode) {
            cvs_output((*b"\tlocked by: \0").as_ptr(),0usize);
            cvs_output((*p).data as (*const u8),0usize);
            cvs_output((*b";\0").as_ptr(),1usize);
        }
        cvs_output((*b"\n\0").as_ptr(),1usize);
        cvs_output_tagged((*b"text\0").as_ptr(),(*b"date: \0").as_ptr());
        sscanf(
            (*ver).date as (*const u8),
            (*b"%d.%d.%d.%d.%d.%d\0").as_ptr(),
            &mut year as (*mut i32),
            &mut mon as (*mut i32),
            &mut mday as (*mut i32),
            &mut hour as (*mut i32),
            &mut min as (*mut i32),
            &mut sec as (*mut i32)
        );
        if year < 1900i32 {
            year = year + 1900i32;
        }
        sprintf(
            buf.as_mut_ptr(),
            (*b"%04d-%02d-%02d %02d:%02d:%02d +0000\0").as_ptr(),
            year,
            mon,
            mday,
            hour,
            min,
            sec
        );
        cvs_output_tagged(
            (*b"date\0").as_ptr(),
            buf.as_mut_ptr() as (*const u8)
        );
        cvs_output_tagged(
            (*b"text\0").as_ptr(),
            (*b";  author: \0").as_ptr()
        );
        cvs_output_tagged(
            (*b"text\0").as_ptr(),
            (*ver).author as (*const u8)
        );
        cvs_output_tagged(
            (*b"text\0").as_ptr(),
            (*b";  state: \0").as_ptr()
        );
        cvs_output_tagged(
            (*b"text\0").as_ptr(),
            (*ver).state as (*const u8)
        );
        cvs_output_tagged((*b"text\0").as_ptr(),(*b";\0").as_ptr());
        if trunk == 0 {
            padd = ::hash::findnode((*ver).other,(*b";add\0").as_ptr());
            pdel = ::hash::findnode((*ver).other,(*b";delete\0").as_ptr());
        } else if (*ver).next == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            padd = 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode);
            pdel = 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode);
        } else {
            let mut nextp : *mut ::hash::hashnode;
            let mut nextver : *mut rcsversnode;
            nextp = ::hash::findnode(
                        (*rcs).versions,
                        (*ver).next as (*const u8)
                    );
            if nextp == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode) {
                error(
                    1i32,
                    0i32,
                    (*b"missing version `%s\' in `%s\'\0").as_ptr(),
                    (*ver).next,
                    (*rcs).print_path
                );
            }
            nextver = (*nextp).data as (*mut rcsversnode);
            pdel = ::hash::findnode((*nextver).other,(*b";add\0").as_ptr());
            padd = ::hash::findnode((*nextver).other,(*b";delete\0").as_ptr());
        }
        if padd != 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode) {
            if !pdel.is_null() {
                0i32;
            } else {
                __assert_fail(
                    (*b"pdel\0").as_ptr(),
                    file!().as_ptr(),
                    line!(),
                    (*b"log_version\0").as_ptr()
                );
            }
            cvs_output_tagged(
                (*b"text\0").as_ptr(),
                (*b"  lines: +\0").as_ptr()
            );
            cvs_output_tagged(
                (*b"text\0").as_ptr(),
                (*padd).data as (*const u8)
            );
            cvs_output_tagged((*b"text\0").as_ptr(),(*b" -\0").as_ptr());
            cvs_output_tagged(
                (*b"text\0").as_ptr(),
                (*pdel).data as (*const u8)
            );
            cvs_output_tagged((*b"text\0").as_ptr(),(*b";\0").as_ptr());
        }
        p = ::hash::findnode((*ver).other_delta,(*b"commitid\0").as_ptr());
        if !p.is_null() && !(*p).data.is_null() {
            cvs_output_tagged(
                (*b"text\0").as_ptr(),
                (*b"  commitid: \0").as_ptr()
            );
            cvs_output_tagged((*b"text\0").as_ptr(),(*p).data as (*const u8));
            cvs_output_tagged((*b"text\0").as_ptr(),(*b";\0").as_ptr());
        }
        cvs_output_tagged(
            (*b"newline\0").as_ptr(),
            0i32 as (*mut ::std::os::raw::c_void) as (*const u8)
        );
        if (*ver).branches != 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashlist) {
            cvs_output((*b"branches:\0").as_ptr(),0usize);
            ::hash::walklist(
                (*ver).branches,
                log_branch,
                0i32 as (*mut ::std::os::raw::c_void)
            );
            cvs_output((*b"\n\0").as_ptr(),1usize);
        }
        p = ::hash::findnode(
                (*ver).other_delta,
                (*b"openpgp-signatures\0").as_ptr()
            );
        if !p.is_null() {
            let mut rawsig : *mut u8 = 0 as (*mut u8);
            let mut rawsiglen : usize = 0;
            let mut membuf : *mut buffer;
            let mut sig : openpgp_signature = ::std::mem::zeroed();
            let mut rc : i32;
            if !base64_decode_alloc_ctx(
                    0i32 as (*mut ::std::os::raw::c_void) as (*mut base64_decode_context),
                    (*p).data as (*const u8),
                    (*p).len,
                    &mut rawsig as (*mut *mut u8),
                    &mut rawsiglen as (*mut usize)
                ) {
                error(
                    0i32,
                    0i32,
                    (*b"Unable to base64-decode OpenPGP signature.\0").as_ptr()
                );
            }
            if rawsig.is_null() {
                xalloc_die();
            }
            membuf = buf_nonio_initialize(None);
            buf_output(membuf,rawsig as (*const u8),rawsiglen);
            'loop24: loop {
                if !({
                         rc = parse_signature(membuf,&mut sig as (*mut openpgp_signature));
                         rc
                     } == 0) {
                    break;
                }
                let mut hexsig : *mut u8;
                cvs_output_tagged(
                    (*b"openpgp-keyid-header\0").as_ptr(),
                    (*b"OpenPGP signature using key ID \0").as_ptr()
                );
                hexsig = Xasprintf(
                             (*b"0x%lx\0").as_ptr(),
                             sig.keyid & 0xffffffffusize
                         );
                cvs_output_tagged(
                    (*b"openpgp-keyid\0").as_ptr(),
                    hexsig as (*const u8)
                );
                free(hexsig as (*mut ::std::os::raw::c_void));
                cvs_output_tagged(
                    (*b"openpgp-keyid-footer\0").as_ptr(),
                    (*b";\0").as_ptr()
                );
                cvs_output_tagged(
                    (*b"newline\0").as_ptr(),
                    0i32 as (*mut ::std::os::raw::c_void) as (*const u8)
                );
            }
            if rc == -2i32 {
                error(
                    1i32,
                    0i32,
                    (*b"Memory allocation failure parsing signature.\0").as_ptr()
                );
            }
            buf_free(membuf);
        }
        p = ::hash::findnode((*ver).other,(*b"log\0").as_ptr());
        if p == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode) || (*p).data == 0i32 as (*mut ::std::os::raw::c_void) || *((*p).data as (*mut u8)) as (i32) == b'\0' as (i32) {
            cvs_output((*b"*** empty log message ***\n\0").as_ptr(),0usize);
        } else {
            cvs_output((*p).data as (*const u8),0usize);
            if *((*p).data as (*mut u8)).offset(
                    strlen((*p).data as (*const u8)).wrapping_sub(1usize) as (isize)
                ) as (i32) != b'\n' as (i32) {
                cvs_output((*b"\n\0").as_ptr(),1usize);
            }
        }
    }
}

unsafe extern fn log_branch(
    mut p : *mut ::hash::hashnode,
    mut closure : *mut ::std::os::raw::c_void
) -> i32 {
    cvs_output((*b"  \0").as_ptr(),2usize);
    if numdots((*p).key as (*const u8)) & 1i32 == 0i32 {
        cvs_output((*p).key as (*const u8),0usize);
    } else {
        let mut f : *mut u8;
        let mut cp : *mut u8;
        f = Xstrdup((*p).key as (*const u8));
        cp = strrchr(f as (*const u8),b'.' as (i32));
        *cp = b'\0';
        cvs_output(f as (*const u8),0usize);
        free(f as (*mut ::std::os::raw::c_void));
    }
    cvs_output((*b";\0").as_ptr(),1usize);
    0i32
}

unsafe extern fn log_dirproc(
    mut callerdat : *mut ::std::os::raw::c_void,
    mut dir : *const u8,
    mut repository : *const u8,
    mut update_dir : *const u8,
    mut entries : *mut ::hash::hashlist
) -> direnter_type {
    if !isdir(dir) {
        direnter_type::R_SKIP_ALL
    } else {
        if quiet == 0 {
            error(
                0i32,
                0i32,
                (*b"Logging %s\0").as_ptr(),
                if !update_dir.is_null() && (*update_dir != 0) {
                    update_dir
                } else {
                    (*b".\0").as_ptr()
                }
            );
        }
        direnter_type::R_PROCESS
    }
}

unsafe extern fn version_compare(
    mut v1 : *const u8, mut v2 : *const u8, mut len : i32
) -> i32 {
    let mut _currentBlock;
    let mut d1 : i32 = 0;
    let mut d2 : i32 = 0;
    let mut r : i32 = 0;
    'loop0: loop {
        if *v1 as (i32) == b'\0' as (i32) {
            _currentBlock = 23;
            break;
        }
        if *v2 as (i32) == b'\0' as (i32) {
            _currentBlock = 22;
            break;
        }
        'loop2: loop {
            if !(*v1 as (i32) == b'0' as (i32)) {
                break;
            }
            v1 = v1.offset(1isize);
        }
        d1 = 0i32;
        'loop4: loop {
            if isdigit(*v1.offset(d1 as (isize)) as (i32)) == 0 {
                break;
            }
            d1 = d1 + 1;
        }
        'loop5: loop {
            if !(*v2 as (i32) == b'0' as (i32)) {
                break;
            }
            v2 = v2.offset(1isize);
        }
        d2 = 0i32;
        'loop7: loop {
            if isdigit(*v2.offset(d2 as (isize)) as (i32)) == 0 {
                break;
            }
            d2 = d2 + 1;
        }
        if d1 != d2 {
            _currentBlock = 17;
            break;
        }
        r = memcmp(
                v1 as (*const ::std::os::raw::c_void),
                v2 as (*const ::std::os::raw::c_void),
                d1 as (usize)
            );
        if r != 0i32 {
            _currentBlock = 16;
            break;
        }
        len = len - 1;
        if len == 0i32 {
            _currentBlock = 15;
            break;
        }
        v1 = v1.offset(d1 as (isize));
        v2 = v2.offset(d1 as (isize));
        if *v1 as (i32) == b'.' as (i32) {
            v1 = v1.offset(1isize);
        }
        if !(*v2 as (i32) == b'.' as (i32)) {
            continue;
        }
        v2 = v2.offset(1isize);
    }
    if _currentBlock == 15 {
        0i32
    } else if _currentBlock == 16 {
        r
    } else if _currentBlock == 17 {
        (if d1 < d2 { -1i32 } else { 1i32 })
    } else if _currentBlock == 22 {
        -1i32
    } else {
        1i32
    }
}
