extern {
    fn Checkin(
        type_ : i32,
        finfo : *mut ::vers_ts::file_info,
        rev : *mut u8,
        tag : *mut u8,
        options : *mut u8,
        message : *mut u8
    ) -> i32;
    fn Classify_File(
        finfo : *mut ::vers_ts::file_info,
        tag : *mut u8,
        date : *mut u8,
        options : *mut u8,
        force_tag_match : i32,
        aflag : i32,
        versp : *mut *mut ::vers_ts::vers_ts,
        pipeout : i32
    ) -> classify_type;
    fn Lock_Cleanup();
    fn Name_Repository(
        dir : *const u8, update_dir : *const u8
    ) -> *mut u8;
    fn Parse_Info(
        infofile : *const u8,
        repository : *const u8,
        callproc
        :
        unsafe extern fn(*const u8, *const u8, *const u8, i32, *mut ::std::os::raw::c_void) -> i32,
        opt : i32,
        closure : *mut ::std::os::raw::c_void
    ) -> i32;
    fn RCS_checkin(
        rcs : *mut ::vers_ts::rcsnode,
        update_dir : *const u8,
        workfile : *const u8,
        message : *const u8,
        rev : *const u8,
        citime : isize,
        flags : i32
    ) -> i32;
    fn RCS_checkout(
        arg1 : *mut ::vers_ts::rcsnode,
        arg2 : *const u8,
        arg3 : *const u8,
        arg4 : *const u8,
        arg5 : *const u8,
        arg6 : *const u8,
        arg7
        :
        Option<unsafe extern fn(*mut ::std::os::raw::c_void, *const u8, usize)>,
        arg8 : *mut ::std::os::raw::c_void
    ) -> i32;
    fn RCS_deltag(
        arg1 : *mut ::vers_ts::rcsnode, arg2 : *const u8
    ) -> i32;
    fn RCS_getbranch(
        rcs : *mut ::vers_ts::rcsnode,
        tag : *const u8,
        force_tag_match : i32
    ) -> *mut u8;
    fn RCS_getexpand(arg1 : *mut ::vers_ts::rcsnode) -> *mut u8;
    fn RCS_getrevtime(
        rcs : *mut ::vers_ts::rcsnode,
        rev : *const u8,
        date : *mut u8,
        fudge : i32
    ) -> isize;
    fn RCS_gettag(
        rcs : *mut ::vers_ts::rcsnode,
        symtag : *const u8,
        force_tag_match : i32,
        simple_tag : *mut i32
    ) -> *mut u8;
    fn RCS_getversion(
        rcs : *mut ::vers_ts::rcsnode,
        tag : *const u8,
        date : *const u8,
        force_tag_match : i32,
        simple_tag : *mut i32
    ) -> *mut u8;
    fn RCS_head(rcs : *mut ::vers_ts::rcsnode) -> *mut u8;
    fn RCS_isbranch(
        rcs : *mut ::vers_ts::rcsnode, rev : *const u8
    ) -> i32;
    fn RCS_isdead(
        arg1 : *mut ::vers_ts::rcsnode, arg2 : *const u8
    ) -> i32;
    fn RCS_lock(
        arg1 : *mut ::vers_ts::rcsnode, arg2 : *const u8, arg3 : i32
    ) -> i32;
    fn RCS_magicrev(
        rcs : *mut ::vers_ts::rcsnode, rev : *mut u8
    ) -> *mut u8;
    fn RCS_nodeisbranch(
        rcs : *mut ::vers_ts::rcsnode, tag : *const u8
    ) -> i32;
    fn RCS_parse(
        file : *const u8, repos : *const u8
    ) -> *mut ::vers_ts::rcsnode;
    fn RCS_parsercsfile(
        rcsfile : *const u8
    ) -> *mut ::vers_ts::rcsnode;
    fn RCS_rewrite(
        arg1 : *mut ::vers_ts::rcsnode,
        arg2 : *mut deltatext,
        arg3 : *mut u8
    );
    fn RCS_setattic(arg1 : *mut ::vers_ts::rcsnode, arg2 : i32) -> i32;
    fn RCS_setbranch(
        arg1 : *mut ::vers_ts::rcsnode, arg2 : *const u8
    ) -> i32;
    fn RCS_setexpand(arg1 : *mut ::vers_ts::rcsnode, arg2 : *const u8);
    fn RCS_settag(
        arg1 : *mut ::vers_ts::rcsnode, arg2 : *const u8, arg3 : *const u8
    ) -> i32;
    fn RCS_unlock(
        arg1 : *mut ::vers_ts::rcsnode, arg2 : *mut u8, arg3 : i32
    ) -> i32;
    fn RCS_whatbranch(
        rcs : *mut ::vers_ts::rcsnode, tag : *const u8
    ) -> *mut u8;
    fn SIG_beginCrSect();
    fn SIG_endCrSect();
    fn SIG_inCrSect() -> i32;
    fn Scratch_Entry(list : *mut ::hash::hashlist, fname : *const u8);
    fn Short_Repository(repository : *const u8) -> *const u8;
    fn Update_Logfile(
        repository : *const u8,
        xmessage : *const u8,
        xlogfp : *mut _IO_FILE,
        xchanges : *mut ::hash::hashlist
    );
    fn WriteTag(
        dir : *const u8,
        tag : *const u8,
        date : *const u8,
        nonbranch : i32,
        update_dir : *const u8,
        repository : *const u8
    );
    fn WriteTemplate(
        update_dir : *const u8, dotemplate : i32, repository : *const u8
    );
    fn Xasprintf(format : *const u8, ...) -> *mut u8;
    fn Xstrdup(str : *const u8) -> *mut u8;
    fn __assert_fail(
        __assertion : *const u8,
        __file : *const u8,
        __line : u32,
        __function : *const u8
    );
    fn __errno_location() -> *mut i32;
    fn add_rcs_file(
        arg1 : *const u8,
        arg2 : *const u8,
        arg3 : *const u8,
        arg4 : *const u8,
        arg5 : *const u8,
        arg6 : *const u8,
        arg7 : *const u8,
        arg8 : i32,
        arg9 : *mut *mut u8,
        arg10 : *const u8,
        arg11 : usize,
        arg12 : *mut _IO_FILE,
        arg13 : bool
    ) -> i32;
    fn atoi(__nptr : *const u8) -> i32;
    fn cmdlineescape(quotes : u8, s : *mut u8) -> *mut u8;
    fn cmdlinequote(quotes : u8, s : *mut u8) -> *mut u8;
    static mut config : *mut config;
    static mut current_parsed_root : *mut cvsroot_s;
    static mut cvs_cmd_name : *const u8;
    fn cvs_output(arg1 : *const u8, arg2 : usize);
    fn cvs_temp_file(filename : *mut *mut u8) -> *mut _IO_FILE;
    fn cvs_xmkdir(
        name : *const u8, update_dir : *const u8, flags : u32
    ) -> bool;
    fn do_editor(
        dir : *const u8,
        messagep : *mut *mut u8,
        repository : *const u8,
        changes : *mut ::hash::hashlist
    );
    fn do_verify(
        messagep : *mut *mut u8,
        repository : *const u8,
        changes : *mut ::hash::hashlist
    ) -> i32;
    fn error(
        __status : i32, __errnum : i32, __format : *const u8, ...
    );
    fn expand_string(
        arg1 : *mut *mut u8, arg2 : *mut usize, arg3 : usize
    );
    fn fclose(__stream : *mut _IO_FILE) -> i32;
    fn file_contains_keyword(
        finfo : *const ::vers_ts::file_info
    ) -> bool;
    fn file_has_markers(arg1 : *const ::vers_ts::file_info) -> i32;
    fn fileattr_get0(
        filename : *const u8, attrname : *const u8
    ) -> *mut u8;
    fn fileattr_newfile(filename : *const u8);
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
    fn free(__ptr : *mut ::std::os::raw::c_void);
    fn freercsnode(rnodep : *mut *mut ::vers_ts::rcsnode);
    fn fwrite_unlocked(
        __ptr : *const ::std::os::raw::c_void,
        __size : usize,
        __n : usize,
        __stream : *mut _IO_FILE
    ) -> usize;
    fn get_file(
        arg1 : *const u8,
        arg2 : *const u8,
        arg3 : *const u8,
        arg4 : *mut *mut u8,
        arg5 : *mut usize,
        arg6 : *mut usize
    );
    fn get_responses_and_close() -> i32;
    fn get_sign_commits(server_support : bool) -> bool;
    fn getcaller() -> *mut u8;
    fn geteuid() -> u32;
    fn getopt(
        ___argc : i32, ___argv : *const *mut u8, __shortopts : *const u8
    ) -> i32;
    fn getpwnam(__name : *const u8) -> *mut passwd;
    fn gmtime(__timer : *const isize) -> *mut tm;
    fn have_sigfile(fn_ : *const u8) -> bool;
    fn history_write(
        type_ : i32,
        update_dir : *const u8,
        revs : *const u8,
        name : *const u8,
        repository : *const u8
    );
    static mut ign_inhibit_server : i32;
    fn ign_setup();
    fn ignore_files(
        arg1 : *mut ::hash::hashlist,
        arg2 : *mut ::hash::hashlist,
        arg3 : *const u8,
        arg4 : unsafe extern fn(*const u8, *const u8)
    );
    fn isdigit(arg1 : i32) -> i32;
    fn isdir(file : *const u8) -> bool;
    fn isfile(file : *const u8) -> bool;
    fn lock_tree_promotably(
        argc : i32,
        argv : *mut *mut u8,
        local : i32,
        which : i32,
        aflag : i32
    ) -> i32;
    fn mkmodules(dir : *mut u8) -> i32;
    static mut noexec : i32;
    fn notify_do(
        type_ : i32,
        filename : *const u8,
        upadte_dir : *const u8,
        who : *const u8,
        val : *const u8,
        watches : *const u8,
        repository : *const u8
    );
    fn numdots(s : *const u8) -> i32;
    static mut optarg : *mut u8;
    static mut optind : i32;
    fn option_with_arg(option : *const u8, arg : *const u8);
    static mut program_name : *const u8;
    static mut quiet : i32;
    fn quote(name : *const u8) -> *const u8;
    fn quote_n(n : i32, name : *const u8) -> *const u8;
    static mut really_quiet : i32;
    static mut referrer : *mut cvsroot_s;
    fn rename_file(from : *const u8, to : *const u8);
    fn run_exec(
        stin : *const u8, stout : *const u8, sterr : *const u8, flags : i32
    ) -> i32;
    fn run_setup(prog : *const u8);
    fn send_a_repository(
        arg1 : *const u8, arg2 : *const u8, arg3 : *const u8
    );
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
    static mut server_active : i32;
    fn server_scratch_entry_only();
    fn server_updated(
        finfo : *mut ::vers_ts::file_info,
        vers : *mut ::vers_ts::vers_ts,
        updated : server_updated_arg4,
        mode : u32,
        checksum : *mut u8,
        filebuf : *mut buffer
    );
    fn sleep_past(desttime : isize);
    fn sprintf(__s : *mut u8, __format : *const u8, ...) -> i32;
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
    fn strlen(__s : *const u8) -> usize;
    fn strncmp(__s1 : *const u8, __s2 : *const u8, __n : usize) -> i32;
    fn strncpy(
        __dest : *mut u8, __src : *const u8, __n : usize
    ) -> *mut u8;
    fn strrchr(__s : *const u8, __c : i32) -> *mut u8;
    fn supported_request(arg1 : *const u8) -> bool;
    fn tag_check_valid(
        arg1 : *const u8,
        arg2 : i32,
        arg3 : *mut *mut u8,
        arg4 : i32,
        arg5 : i32,
        arg6 : *mut u8,
        arg7 : bool
    );
    fn time(__timer : *mut isize) -> isize;
    fn unlink_file(f : *const u8) -> i32;
    fn usage(cpp : *const *const u8);
    static mut use_editor : i32;
    fn wrap_setup();
    fn xalloc_die();
    fn xmalloc(s : usize) -> *mut ::std::os::raw::c_void;
}

enum _IO_FILE {
}

enum buffer {
}

static mut check_valid_edit : i32 = 0i32;

static mut force_ci : i32 = 0i32;

static mut aflag : i32 = 0i32;

static mut saved_tag : *mut u8 = 0 as (*mut u8);

static mut write_dirtag : *mut u8 = 0 as (*mut u8);

static mut write_dirnonbranch : i32 = 0i32;

static mut logfile : *mut u8 = 0 as (*mut u8);

static mut mulist
    : *mut ::hash::hashlist
    = 0 as (*mut ::hash::hashlist);

static mut last_register_time : isize = 0isize;

static mut commit_usage : [*const u8; 11] = [
    b"Usage: %s %s [-cRlf] [-m msg | -F logfile] [-r rev] [files...]\n\0" as (*const u8),
    b"\n\0" as (*const u8),
    b"    -c          Check for valid edits before committing.\n\0" as (*const u8),
    b"    -R          Process directories recursively.\n\0" as (*const u8),
    b"    -l          Local directory only (not recursive).\n\0" as (*const u8),
    b"    -f          Force the file to be committed; disables recursion.\n\0" as (*const u8),
    b"    -F logfile  Read the log message from file.\n\0" as (*const u8),
    b"    -m msg      Log message.\n\0" as (*const u8),
    b"    -r rev      Commit to this branch or trunk revision.\n\0" as (*const u8),
    b"(Specify the --help global option for a list of other help options)\n\0" as (*const u8),
    0i32 as (*mut ::std::os::raw::c_void) as (*const u8)
];

#[derive(Copy)]
#[repr(C)]
pub struct question {
    pub dir : *mut u8,
    pub repos : *mut u8,
    pub file : *mut u8,
    pub next : *mut question,
}

impl Clone for question {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct find_data {
    pub ulist : *mut ::hash::hashlist,
    pub argc : i32,
    pub argv : *mut *mut u8,
    pub ignlist : *mut ::hash::hashlist,
    pub questionables : *mut question,
    pub repository : *const u8,
    pub force : i32,
}

impl Clone for find_data {
    fn clone(&self) -> Self { *self }
}

static mut find_data_static
    : *mut find_data
    = 0 as (*mut find_data);

#[derive(Copy)]
#[repr(C)]
pub struct commit_data {
    pub saved_message : *mut u8,
}

impl Clone for commit_data {
    fn clone(&self) -> Self { *self }
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

#[derive(Copy)]
#[repr(C)]
pub struct passwd {
    pub pw_name : *mut u8,
    pub pw_passwd : *mut u8,
    pub pw_uid : u32,
    pub pw_gid : u32,
    pub pw_gecos : *mut u8,
    pub pw_dir : *mut u8,
    pub pw_shell : *mut u8,
}

impl Clone for passwd {
    fn clone(&self) -> Self { *self }
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

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum classify_type {
    T_UNKNOWN = 1i32,
    T_CONFLICT,
    T_NEEDS_MERGE,
    T_MODIFIED,
    T_CHECKOUT,
    T_ADDED,
    T_REMOVED,
    T_REMOVE_ENTRY,
    T_UPTODATE,
    T_PATCH,
    T_TITLE,
}

#[derive(Copy)]
#[repr(C)]
pub struct logfile_info {
    pub type_ : classify_type,
    pub tag : *mut u8,
    pub rev_old : *mut u8,
    pub rev_new : *mut u8,
}

impl Clone for logfile_info {
    fn clone(&self) -> Self { *self }
}

unsafe extern fn streq(
    mut a : *const u8, mut b : *const u8
) -> bool {
    *a as (i32) == *b as (i32) && (strcmp(a,b) == 0i32)
}

unsafe extern fn find_fileproc(
    mut callerdat : *mut ::std::os::raw::c_void,
    mut finfo : *mut ::vers_ts::file_info
) -> i32 {
    let mut vers : *mut ::vers_ts::vers_ts;
    let mut status : classify_type;
    let mut node : *mut ::hash::hashnode;
    let mut args : *mut find_data = callerdat as (*mut find_data);
    let mut data : *mut logfile_info;
    let mut xfinfo : ::vers_ts::file_info;
    if !(*args).ignlist.is_null() {
        let mut p : *mut ::hash::hashnode;
        p = ::hash::getnode();
        (*p).type_ = ::hash::ntype::FILES;
        (*p).key = Xstrdup((*finfo).file);
        if ::hash::addnode((*args).ignlist,p) != 0i32 {
            ::hash::freenode(p);
        }
    }
    xfinfo = *finfo;
    xfinfo.repository = 0i32 as (*mut ::std::os::raw::c_void) as (*const u8);
    xfinfo.rcs = 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::vers_ts::rcsnode);
    vers = ::vers_ts::Version_TS(
               &mut xfinfo as (*mut ::vers_ts::file_info),
               0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
               saved_tag as (*const u8),
               0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
               0i32,
               0i32
           );
    if (*vers).vn_user.is_null() {
        if (*vers).ts_user.is_null() {
            error(
                0i32,
                0i32,
                (*b"Nothing known about %s\0").as_ptr(),
                quote((*finfo).fullname)
            );
        } else {
            let mut cmd
                : *mut u8
                = Xasprintf((*b"%s add\0").as_ptr(),program_name);
            error(
                0i32,
                0i32,
                (*b"Use %s to create an entry for %s\0").as_ptr(),
                quote_n(0i32,cmd as (*const u8)),
                quote_n(1i32,(*finfo).fullname)
            );
            free(cmd as (*mut ::std::os::raw::c_void));
        }
        ::vers_ts::freevers_ts(
            &mut vers as (*mut *mut ::vers_ts::vers_ts)
        );
        1i32
    } else {
        if *(*vers).vn_user.offset(0isize) as (i32) == b'-' as (i32) {
            if !(*vers).ts_user.is_null() {
                error(
                    0i32,
                    0i32,
                    (*b"%s should be removed and is still there (or is back again)\0").as_ptr(
                    ),
                    quote((*finfo).fullname)
                );
                ::vers_ts::freevers_ts(
                    &mut vers as (*mut *mut ::vers_ts::vers_ts)
                );
                return 1i32;
            } else {
                status = classify_type::T_REMOVED;
            }
        } else if streq(
                      (*vers).vn_user as (*const u8),
                      (*b"0\0").as_ptr()
                  ) {
            if (*vers).ts_user.is_null() {
                if really_quiet == 0 {
                    error(
                        0i32,
                        0i32,
                        (*b"warning: new-born %s has disappeared\0").as_ptr(),
                        (*finfo).fullname
                    );
                }
                status = classify_type::T_REMOVE_ENTRY;
            } else {
                status = classify_type::T_ADDED;
            }
        } else if (*vers).ts_user.is_null() {
            ::vers_ts::freevers_ts(
                &mut vers as (*mut *mut ::vers_ts::vers_ts)
            );
            return 0i32;
        } else if !(*vers).ts_rcs.is_null(
                   ) && ((*args).force != 0 || !streq(
                                                    (*vers).ts_user as (*const u8),
                                                    (*vers).ts_rcs as (*const u8)
                                                )) {
            status = classify_type::T_MODIFIED;
        } else {
            ::vers_ts::freevers_ts(
                &mut vers as (*mut *mut ::vers_ts::vers_ts)
            );
            return 0i32;
        }
        node = ::hash::getnode();
        (*node).key = Xstrdup((*finfo).fullname);
        data = xmalloc(
                   ::std::mem::size_of::<logfile_info>()
               ) as (*mut logfile_info);
        (*data).type_ = status;
        (*data).tag = Xstrdup((*vers).tag as (*const u8));
        (*data).rev_old = {
                              (*data).rev_new = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                              (*data).rev_new
                          };
        (*node).type_ = ::hash::ntype::UPDATE;
        (*node).delproc = Some(update_delproc);
        (*node).data = data as (*mut ::std::os::raw::c_void);
        ::hash::addnode((*args).ulist,node);
        (*args).argc = (*args).argc + 1;
        ::vers_ts::freevers_ts(
            &mut vers as (*mut *mut ::vers_ts::vers_ts)
        );
        0i32
    }
}

unsafe extern fn find_ignproc(
    mut file : *const u8, mut dir : *const u8
) {
    let mut p : *mut question;
    p = xmalloc(::std::mem::size_of::<question>()) as (*mut question);
    (*p).dir = Xstrdup(dir);
    (*p).repos = Xstrdup((*find_data_static).repository);
    (*p).file = Xstrdup(file);
    (*p).next = (*find_data_static).questionables as (*mut question);
    (*find_data_static).questionables = p;
}

unsafe extern fn find_filesdoneproc(
    mut callerdat : *mut ::std::os::raw::c_void,
    mut err : i32,
    mut repository : *const u8,
    mut update_dir : *const u8,
    mut entries : *mut ::hash::hashlist
) -> i32 {
    let mut find_data : *mut find_data = callerdat as (*mut find_data);
    (*find_data).repository = repository;
    if !(*find_data).ignlist.is_null() {
        find_data_static = find_data;
        ignore_files((*find_data).ignlist,entries,update_dir,find_ignproc);
        ::hash::dellist(
            &mut (*find_data).ignlist as (*mut *mut ::hash::hashlist)
        );
    }
    (*find_data).repository = 0i32 as (*mut ::std::os::raw::c_void) as (*const u8);
    err
}

unsafe extern fn find_dirent_proc(
    mut callerdat : *mut ::std::os::raw::c_void,
    mut dir : *const u8,
    mut repository : *const u8,
    mut update_dir : *const u8,
    mut entries : *mut ::hash::hashlist
) -> direnter_type {
    let mut find_data : *mut find_data = callerdat as (*mut find_data);
    if !isdir(dir) {
        direnter_type::R_SKIP_ALL
    } else {
        (*find_data).ignlist = ::hash::getlist();
        if quiet == 0 {
            error(
                0i32,
                0i32,
                (*b"Examining %s\0").as_ptr(),
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

unsafe extern fn xnmalloc(
    mut n : usize, mut s : usize
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
    xmalloc(n.wrapping_mul(s))
}

unsafe extern fn copy_ulist(
    mut node : *mut ::hash::hashnode,
    mut data : *mut ::std::os::raw::c_void
) -> i32 {
    let mut args : *mut find_data = data as (*mut find_data);
    *(*args).argv.offset(
         {
             let _old = (*args).argc;
             (*args).argc = (*args).argc + 1;
             _old
         } as (isize)
     ) = (*node).key;
    0i32
}

#[no_mangle]
pub unsafe extern fn commit(
    mut argc : i32, mut argv : *mut *mut u8
) -> i32 {
    let mut c : i32;
    let mut err : i32 = 0i32;
    let mut local : i32 = 0i32;
    let mut commit_data : commit_data = ::std::mem::zeroed();
    let mut flags : i32;
    let mut short_options : [u8; 16] = *b"+cgG:lRm:fF:r:n\0";
    if argc == -1i32 {
        usage(commit_usage.as_ptr());
    }
    if geteuid() == 0u32 && !(*current_parsed_root).isremote {
        let mut pw : *mut passwd;
        if {
               pw = getpwnam(getcaller() as (*const u8));
               pw
           } == 0i32 as (*mut ::std::os::raw::c_void) as (*mut passwd) {
            error(
                1i32,
                0i32,
                (*b"your apparent username (%s) is unknown to this system\0").as_ptr(
                ),
                getcaller()
            );
        }
        if (*pw).pw_uid == 0u32 {
            error(
                1i32,
                0i32,
                (*b"\'root\' is not allowed to commit files\0").as_ptr()
            );
        }
    }
    optind = 0i32;
    'loop8: loop {
        if !({
                 c = getopt(argc,argv as (*const *mut u8),short_options.as_ptr());
                 c
             } != -1i32) {
            break;
        }
        if c == b'F' as (i32) {
            use_editor = 0i32;
            logfile = optarg;
        } else if c == b'f' as (i32) {
            force_ci = 1i32;
            check_valid_edit = 0i32;
            local = 1i32;
        } else if c == b'R' as (i32) {
            local = 0i32;
        } else if c == b'l' as (i32) {
            local = 1i32;
        } else if c == b'r' as (i32) {
            if !saved_tag.is_null() {
                free(saved_tag as (*mut ::std::os::raw::c_void));
            }
            saved_tag = Xstrdup(optarg as (*const u8));
        } else if c == b'm' as (i32) {
            use_editor = 0i32;
            if !commit_data.saved_message.is_null() {
                free(commit_data.saved_message as (*mut ::std::os::raw::c_void));
                commit_data.saved_message = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
            }
            commit_data.saved_message = Xstrdup(optarg as (*const u8));
        } else if c == b'n' as (i32) {
            if !(server_active == 0) {
                continue;
            }
            error(0i32,0i32,(*b"the `-n\' option is obsolete\0").as_ptr());
        } else if c == b'c' as (i32) {
            check_valid_edit = 1i32;
        } else {
            usage(commit_usage.as_ptr());
        }
    }
    argc = argc - optind;
    argv = argv.offset(optind as (isize));
    if !saved_tag.is_null() && (isdigit(*saved_tag as (i32)) != 0) {
        let mut p
            : *mut u8
            = saved_tag.offset(strlen(saved_tag as (*const u8)) as (isize));
        aflag = 1i32;
        'loop11: loop {
            if !(*{
                      p = p.offset(-1isize);
                      p
                  } as (i32) == b'.' as (i32)) {
                break;
            }
        }
        *p.offset(1isize) = b'\0';
        'loop13: loop {
            if !(*saved_tag.offset(
                      0isize
                  ) as (i32) == b'0' as (i32) && (isdigit(
                                                      *saved_tag.offset(1isize) as (i32)
                                                  ) != 0)) {
                break;
            }
            saved_tag = saved_tag.offset(1isize);
        }
    }
    if !logfile.is_null() {
        let mut size : usize = 0usize;
        let mut len : usize = 0;
        if !commit_data.saved_message.is_null() {
            error(
                1i32,
                0i32,
                (*b"cannot specify both a message and a log file\0").as_ptr()
            );
        }
        get_file(
            logfile as (*const u8),
            logfile as (*const u8),
            (*b"r\0").as_ptr(),
            &mut commit_data.saved_message as (*mut *mut u8),
            &mut size as (*mut usize),
            &mut len as (*mut usize)
        );
    }
    if (*current_parsed_root).isremote {
        ign_setup();
        let mut find_args : find_data = find_data {
            ulist: ::hash::getlist(),
            argc: 0i32,
            argv: 0 as (*mut *mut u8),
            questionables: 0i32 as (*mut ::std::os::raw::c_void) as (*mut question),
            ignlist: 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashlist),
            repository: 0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
            force: (force_ci != 0 || saved_tag != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)) as (i32),
        };
        err = start_recursion(
                  Some(find_fileproc),
                  Some(find_filesdoneproc),
                  Some(find_dirent_proc),
                  None,
                  &mut find_args as (*mut find_data) as (*mut ::std::os::raw::c_void),
                  argc,
                  argv,
                  local,
                  1i32 << 0i32,
                  0i32,
                  cvs_lock_type::CVS_LOCK_NONE,
                  0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                  0i32,
                  0i32 as (*mut ::std::os::raw::c_void) as (*const u8)
              );
        if err != 0 {
            error(1i32,0i32,(*b"correct above errors first!\0").as_ptr());
        }
        (if find_args.argc == 0i32 {
             ::hash::dellist(
                 &mut find_args.ulist as (*mut *mut ::hash::hashlist)
             );
             0i32
         } else if if find_args.argc as (u64) <= 18446744073709551615u64.wrapping_div(
                                                     ::std::mem::size_of::<*mut *mut u8>() as (u64)
                                                 ) {
                       (find_args.argc as (usize)).wrapping_mul(
                           ::std::mem::size_of::<*mut *mut u8>()
                       ) as (u64)
                   } else {
                       18446744073709551615u64
                   } == 18446744073709551615u64 {
             find_args.argc = 0i32;
             0i32
         } else {
             find_args.argv = xnmalloc(
                                  find_args.argc as (usize),
                                  ::std::mem::size_of::<*mut *mut u8>()
                              ) as (*mut *mut u8);
             find_args.argc = 0i32;
             ::hash::walklist(
                 find_args.ulist,
                 copy_ulist,
                 &mut find_args as (*mut find_data) as (*mut ::std::os::raw::c_void)
             );
             start_server();
             if use_editor != 0 {
                 do_editor(
                     0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                     &mut commit_data.saved_message as (*mut *mut u8),
                     0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                     find_args.ulist
                 );
             }
             option_with_arg(
                 (*b"-m\0").as_ptr(),
                 if !commit_data.saved_message.is_null() {
                     commit_data.saved_message as (*const u8)
                 } else {
                     (*b"\0").as_ptr()
                 }
             );
             let mut p : *mut question;
             let mut q : *mut question;
             p = find_args.questionables;
             'loop41: loop {
                 if !(p != 0i32 as (*mut ::std::os::raw::c_void) as (*mut question)) {
                     break;
                 }
                 if ign_inhibit_server != 0 || !supported_request(
                                                    (*b"Questionable\0").as_ptr()
                                                ) {
                     cvs_output((*b"? \0").as_ptr(),2usize);
                     if *(*p).dir.offset(0isize) as (i32) != b'\0' as (i32) {
                         cvs_output((*p).dir as (*const u8),0usize);
                         cvs_output((*b"/\0").as_ptr(),1usize);
                     }
                     cvs_output((*p).file as (*const u8),0usize);
                     cvs_output((*b"\n\0").as_ptr(),1usize);
                 } else {
                     send_a_repository(
                         (*p).dir as (*const u8),
                         (*p).repos as (*const u8),
                         (*p).dir as (*const u8)
                     );
                     send_to_server((*b"Questionable \0").as_ptr(),0usize);
                     send_to_server((*p).file as (*const u8),0usize);
                     send_to_server((*b"\n\0").as_ptr(),1usize);
                 }
                 free((*p).dir as (*mut ::std::os::raw::c_void));
                 free((*p).repos as (*mut ::std::os::raw::c_void));
                 free((*p).file as (*mut ::std::os::raw::c_void));
                 q = (*p).next as (*mut question);
                 free(p as (*mut ::std::os::raw::c_void));
                 p = q;
             }
             if local != 0 {
                 send_arg((*b"-l\0").as_ptr());
             }
             if check_valid_edit != 0 {
                 send_arg((*b"-c\0").as_ptr());
             }
             if force_ci != 0 {
                 send_arg((*b"-f\0").as_ptr());
             }
             option_with_arg((*b"-r\0").as_ptr(),saved_tag as (*const u8));
             send_arg((*b"--\0").as_ptr());
             flags = if find_args.force != 0 { 1i32 << 1i32 } else { 0i32 };
             send_files(
                 find_args.argc,
                 find_args.argv,
                 local,
                 0i32,
                 flags as (u32)
             );
             send_file_names(find_args.argc,find_args.argv,0u32);
             free(find_args.argv as (*mut ::std::os::raw::c_void));
             ::hash::dellist(
                 &mut find_args.ulist as (*mut *mut ::hash::hashlist)
             );
             send_to_server((*b"ci\n\0").as_ptr(),0usize);
             err = get_responses_and_close();
             if err != 0i32 && (use_editor != 0) && (commit_data.saved_message != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)) {
                 let mut fname : *mut u8 = 0 as (*mut u8);
                 let mut fp : *mut _IO_FILE;
                 fp = cvs_temp_file(&mut fname as (*mut *mut u8));
                 if fp == 0i32 as (*mut ::std::os::raw::c_void) as (*mut _IO_FILE) {
                     error(
                         1i32,
                         0i32,
                         (*b"cannot create temporary file %s\0").as_ptr(),
                         if !fname.is_null() {
                             fname as (*const u8)
                         } else {
                             (*b"(null)\0").as_ptr()
                         }
                     );
                 }
                 if fwrite_unlocked(
                        commit_data.saved_message as (*const ::std::os::raw::c_void),
                        1usize,
                        strlen(commit_data.saved_message as (*const u8)),
                        fp
                    ) != strlen(commit_data.saved_message as (*const u8)) {
                     error(
                         1i32,
                         *__errno_location(),
                         (*b"cannot write temporary file %s\0").as_ptr(),
                         fname
                     );
                 }
                 if fclose(fp) < 0i32 {
                     error(
                         0i32,
                         *__errno_location(),
                         (*b"cannot close temporary file %s\0").as_ptr(),
                         fname
                     );
                 }
                 error(0i32,0i32,(*b"saving log message in %s\0").as_ptr(),fname);
                 free(fname as (*mut ::std::os::raw::c_void));
             }
             err
         })
    } else {
        if saved_tag != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            tag_check_valid(
                saved_tag as (*const u8),
                argc,
                argv,
                local,
                aflag,
                (*b"\0").as_ptr() as (*mut u8),
                false
            );
        }
        if argc <= 0i32 {
            write_dirtag = saved_tag;
        }
        wrap_setup();
        if lock_tree_promotably(argc,argv,local,1i32 << 0i32,aflag) != 0 {
            error(1i32,0i32,(*b"correct above errors first!\0").as_ptr());
        }
        mulist = ::hash::getlist();
        if start_recursion(
               Some(check_fileproc),
               Some(check_filesdoneproc),
               Some(check_direntproc),
               None,
               &mut commit_data as (*mut commit_data) as (*mut ::std::os::raw::c_void),
               argc,
               argv,
               local,
               1i32 << 0i32,
               aflag,
               cvs_lock_type::CVS_LOCK_NONE,
               0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
               1i32,
               0i32 as (*mut ::std::os::raw::c_void) as (*const u8)
           ) != 0 {
            error(1i32,0i32,(*b"correct above errors first!\0").as_ptr());
        }
        write_dirnonbranch = 0i32;
        if noexec == 0i32 {
            err = start_recursion(
                      Some(commit_fileproc),
                      Some(commit_filesdoneproc),
                      Some(commit_direntproc),
                      Some(commit_dirleaveproc),
                      &mut commit_data as (*mut commit_data) as (*mut ::std::os::raw::c_void),
                      argc,
                      argv,
                      local,
                      1i32 << 0i32,
                      aflag,
                      cvs_lock_type::CVS_LOCK_WRITE,
                      0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                      1i32,
                      0i32 as (*mut ::std::os::raw::c_void) as (*const u8)
                  );
        }
        Lock_Cleanup();
        ::hash::dellist(&mut mulist as (*mut *mut ::hash::hashlist));
        if !commit_data.saved_message.is_null() {
            free(commit_data.saved_message as (*mut ::std::os::raw::c_void));
            commit_data.saved_message = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        }
        if server_active == 0 && (last_register_time != 0) {
            sleep_past(last_register_time);
        }
        err
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct commit_info {
    pub status : classify_type,
    pub rev : *mut u8,
    pub tag : *mut u8,
    pub options : *mut u8,
}

impl Clone for commit_info {
    fn clone(&self) -> Self { *self }
}

unsafe extern fn strneq(
    mut a : *const u8, mut b : *const u8, mut n : usize
) -> bool {
    n == 0 || *a as (i32) == *b as (i32) && (strncmp(a,b,n) == 0i32)
}

unsafe extern fn classify_file_internal(
    mut finfo : *mut ::vers_ts::file_info,
    mut vers : *mut *mut ::vers_ts::vers_ts
) -> classify_type {
    let mut save_noexec : i32;
    let mut save_quiet : i32;
    let mut save_really_quiet : i32;
    let mut status : classify_type;
    save_noexec = noexec;
    save_quiet = quiet;
    save_really_quiet = really_quiet;
    noexec = {
                 quiet = {
                             really_quiet = 1i32;
                             really_quiet
                         };
                 quiet
             };
    if !saved_tag.is_null() && (isdigit(*saved_tag as (i32)) != 0) {
        if numdots(saved_tag as (*const u8)) < 2i32 {
            status = Classify_File(
                         finfo,
                         0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                         0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                         0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                         1i32,
                         aflag,
                         vers,
                         0i32
                     );
            if status as (i32) == classify_type::T_UPTODATE as (i32) || status as (i32) == classify_type::T_MODIFIED as (i32) || status as (i32) == classify_type::T_ADDED as (i32) {
                let mut xstatus : classify_type;
                ::vers_ts::freevers_ts(vers);
                xstatus = Classify_File(
                              finfo,
                              saved_tag,
                              0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                              0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                              1i32,
                              aflag,
                              vers,
                              0i32
                          );
                if xstatus as (i32) == classify_type::T_REMOVE_ENTRY as (i32) {
                    status = classify_type::T_MODIFIED;
                } else if status as (i32) == classify_type::T_MODIFIED as (i32) && (xstatus as (i32) == classify_type::T_CONFLICT as (i32)) {
                    status = classify_type::T_MODIFIED;
                } else {
                    status = xstatus;
                }
            }
        } else {
            let mut xtag : *mut u8;
            let mut cp : *mut u8 = 0 as (*mut u8);
            xtag = Xstrdup(saved_tag as (*const u8));
            if numdots(xtag as (*const u8)) & 1i32 != 0i32 {
                cp = strrchr(xtag as (*const u8),b'.' as (i32));
                *cp = b'\0';
            }
            status = Classify_File(
                         finfo,
                         xtag,
                         0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                         0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                         1i32,
                         aflag,
                         vers,
                         0i32
                     );
            if (status as (i32) == classify_type::T_REMOVE_ENTRY as (i32) || status as (i32) == classify_type::T_CONFLICT as (i32)) && ({
                                                                                                                                            cp = strrchr(
                                                                                                                                                     xtag as (*const u8),
                                                                                                                                                     b'.' as (i32)
                                                                                                                                                 );
                                                                                                                                            cp
                                                                                                                                        } != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)) {
                *cp = b'\0';
                ::vers_ts::freevers_ts(vers);
                status = Classify_File(
                             finfo,
                             xtag,
                             0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                             0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                             1i32,
                             aflag,
                             vers,
                             0i32
                         );
                if status as (i32) == classify_type::T_UPTODATE as (i32) || status as (i32) == classify_type::T_REMOVE_ENTRY as (i32) {
                    status = classify_type::T_MODIFIED;
                }
            }
            free((**vers).tag as (*mut ::std::os::raw::c_void));
            (**vers).tag = Xstrdup(saved_tag as (*const u8));
            free(xtag as (*mut ::std::os::raw::c_void));
        }
    } else {
        status = Classify_File(
                     finfo,
                     saved_tag,
                     0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                     0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                     1i32,
                     0i32,
                     vers,
                     0i32
                 );
    }
    noexec = save_noexec;
    quiet = save_quiet;
    really_quiet = save_really_quiet;
    status
}

#[derive(Copy)]
#[repr(C)]
pub struct master_lists {
    pub ulist : *mut ::hash::hashlist,
    pub cilist : *mut ::hash::hashlist,
}

impl Clone for master_lists {
    fn clone(&self) -> Self { *self }
}

unsafe extern fn check_fileproc(
    mut callerdat : *mut ::std::os::raw::c_void,
    mut finfo : *mut ::vers_ts::file_info
) -> i32 {
    let mut _currentBlock;
    let mut status : classify_type;
    let mut xdir : *const u8;
    let mut p : *mut ::hash::hashnode;
    let mut ulist : *mut ::hash::hashlist;
    let mut cilist : *mut ::hash::hashlist;
    let mut vers
        : *mut ::vers_ts::vers_ts
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::vers_ts::vers_ts);
    let mut ci : *mut commit_info;
    let mut li : *mut logfile_info;
    let mut retval : i32 = 1i32;
    let mut cvsroot_len
        : usize
        = strlen((*current_parsed_root).directory as (*const u8));
    if (*finfo).repository.is_null() {
        error(
            0i32,
            0i32,
            (*b"Nothing known about %s\0").as_ptr(),
            quote((*finfo).fullname)
        );
        retval = 1i32;
    } else {
        if strneq(
               (*finfo).repository,
               (*current_parsed_root).directory as (*const u8),
               cvsroot_len
           ) && (*(*finfo).repository.offset(
                      cvsroot_len as (isize)
                  ) as (i32) == b'/' as (i32)) && strneq(
                                                      (*finfo).repository.offset(
                                                          cvsroot_len as (isize)
                                                      ).offset(
                                                          1isize
                                                      ),
                                                      (*b"CVSROOT\0").as_ptr(),
                                                      ::std::mem::size_of::<[u8; 8]>().wrapping_sub(
                                                          1usize
                                                      )
                                                  ) && (*(*finfo).repository.offset(
                                                             cvsroot_len.wrapping_add(
                                                                 ::std::mem::size_of::<[u8; 8]>()
                                                             ) as (isize)
                                                         ) as (i32) == b'/' as (i32)) && streq(
                                                                                             (*finfo).repository.offset(
                                                                                                 cvsroot_len as (isize)
                                                                                             ).offset(
                                                                                                 ::std::mem::size_of::<[u8; 8]>(
                                                                                                 ) as (isize)
                                                                                             ).offset(
                                                                                                 1isize
                                                                                             ),
                                                                                             (*b"Emptydir\0").as_ptr(
                                                                                             )
                                                                                         ) {
            error(
                1i32,
                0i32,
                (*b"cannot check in to %s\0").as_ptr(),
                (*finfo).repository
            );
        }
        status = classify_file_internal(
                     finfo,
                     &mut vers as (*mut *mut ::vers_ts::vers_ts)
                 );
        if force_ci != 0 && (status as (i32) == classify_type::T_UPTODATE as (i32)) {
            status = classify_type::T_MODIFIED;
        }
        if status as (i32) == classify_type::T_UPTODATE as (i32) {
            _currentBlock = 66;
        } else if status as (i32) == classify_type::T_UNKNOWN as (i32) {
            if (*vers).ts_user.is_null() {
                error(
                    0i32,
                    0i32,
                    (*b"Nothing known about %s\0").as_ptr(),
                    quote((*finfo).fullname)
                );
            } else {
                let mut cmd
                    : *mut u8
                    = Xasprintf((*b"%s add\0").as_ptr(),program_name);
                error(
                    0i32,
                    0i32,
                    (*b"Use %s to create an entry for %s\0").as_ptr(),
                    quote_n(0i32,cmd as (*const u8)),
                    quote_n(1i32,(*finfo).fullname)
                );
                free(cmd as (*mut ::std::os::raw::c_void));
            }
            _currentBlock = 68;
        } else if status as (i32) == classify_type::T_REMOVED as (i32) || status as (i32) == classify_type::T_ADDED as (i32) || status as (i32) == classify_type::T_MODIFIED as (i32) || status as (i32) == classify_type::T_CONFLICT as (i32) {
            let mut editor : *mut u8 = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
            if saved_tag.is_null() || isdigit(*saved_tag as (i32)) == 0 {
                if !(*vers).date.is_null() {
                    error(
                        0i32,
                        0i32,
                        (*b"cannot commit with sticky date for file `%s\'\0").as_ptr(),
                        (*finfo).fullname
                    );
                    _currentBlock = 68;
                } else if status as (i32) == classify_type::T_MODIFIED as (i32) && !(*vers).tag.is_null(
                                                                                    ) && (RCS_isbranch(
                                                                                              (*finfo).rcs,
                                                                                              (*vers).tag as (*const u8)
                                                                                          ) == 0) {
                    error(
                        0i32,
                        0i32,
                        (*b"sticky tag `%s\' for file `%s\' is not a branch\0").as_ptr(),
                        (*vers).tag,
                        (*finfo).fullname
                    );
                    _currentBlock = 68;
                } else {
                    _currentBlock = 14;
                }
            } else {
                _currentBlock = 14;
            }
            if _currentBlock == 68 {
            } else if status as (i32) == classify_type::T_CONFLICT as (i32) && (force_ci == 0) {
                error(
                    0i32,
                    0i32,
                    (*b"file `%s\' had a conflict and has not been modified\0").as_ptr(
                    ),
                    (*finfo).fullname
                );
                _currentBlock = 68;
            } else {
                if status as (i32) == classify_type::T_MODIFIED as (i32) && (force_ci == 0) && (really_quiet == 0) && (file_has_markers(
                                                                                                                           finfo as (*const ::vers_ts::file_info)
                                                                                                                       ) != 0) {
                    error(
                        0i32,
                        0i32,
                        (*b"warning: file %s seems to still contain conflict indicators\0").as_ptr(
                        ),
                        quote((*finfo).fullname)
                    );
                }
                if (status as (i32) == classify_type::T_ADDED as (i32) || status as (i32) == classify_type::T_MODIFIED as (i32)) && (force_ci == 0) && (really_quiet == 0) && (get_sign_commits(
                                                                                                                                                                                   true
                                                                                                                                                                               ) || have_sigfile(
                                                                                                                                                                                        (*finfo).file
                                                                                                                                                                                    )) && file_contains_keyword(
                                                                                                                                                                                              finfo as (*const ::vers_ts::file_info)
                                                                                                                                                                                          ) {
                    if quiet == 0 {
                        error(
                            0i32,
                            0i32,
                            (*b"warning: signed file `%s\' contains at least one RCS keyword\0").as_ptr(
                            ),
                            (*finfo).fullname
                        );
                    }
                }
                if status as (i32) == classify_type::T_REMOVED as (i32) {
                    if !(*vers).ts_user.is_null() {
                        error(
                            0i32,
                            0i32,
                            (*b"`%s\' should be removed and is still there (or is back again)\0").as_ptr(
                            ),
                            (*finfo).fullname
                        );
                        _currentBlock = 68;
                    } else if !(*vers).tag.is_null() && (isdigit(
                                                             *(*vers).tag as (i32)
                                                         ) != 0) {
                        error(
                            0i32,
                            0i32,
                            (*b"cannot remove file `%s\' which has a numeric sticky tag of `%s\'\0").as_ptr(
                            ),
                            (*finfo).fullname,
                            (*vers).tag
                        );
                        ::vers_ts::freevers_ts(
                            &mut vers as (*mut *mut ::vers_ts::vers_ts)
                        );
                        _currentBlock = 68;
                    } else {
                        _currentBlock = 23;
                    }
                } else {
                    _currentBlock = 23;
                }
                if _currentBlock == 68 {
                } else {
                    if status as (i32) == classify_type::T_ADDED as (i32) {
                        if (*vers).tag.is_null() {
                            if !(*finfo).rcs.is_null() && (RCS_isdead(
                                                               (*finfo).rcs,
                                                               (*(*finfo).rcs).head as (*const u8)
                                                           ) == 0) {
                                error(
                                    0i32,
                                    0i32,
                                    (*b"cannot add file %s when RCS file %s already exists\0").as_ptr(
                                    ),
                                    quote_n(0i32,(*finfo).fullname),
                                    quote_n(1i32,(*(*finfo).rcs).path as (*const u8))
                                );
                                _currentBlock = 68;
                            } else {
                                _currentBlock = 28;
                            }
                        } else if isdigit(*(*vers).tag as (i32)) != 0 && (numdots(
                                                                              (*vers).tag as (*const u8)
                                                                          ) > 1i32) {
                            error(
                                0i32,
                                0i32,
                                (*b"cannot add file %s with revision %s; must be on trunk\0").as_ptr(
                                ),
                                quote_n(0i32,(*finfo).fullname),
                                quote_n(1i32,(*vers).tag as (*const u8))
                            );
                            _currentBlock = 68;
                        } else {
                            _currentBlock = 28;
                        }
                    } else {
                        _currentBlock = 28;
                    }
                    if _currentBlock == 68 {
                    } else {
                        if *(*finfo).update_dir == 0 {
                            xdir = (*b".\0").as_ptr();
                        } else {
                            xdir = (*finfo).update_dir;
                        }
                        if {
                               p = ::hash::findnode(mulist,xdir);
                               p
                           } != 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode) {
                            ulist = (*((*p).data as (*mut master_lists))).ulist;
                            cilist = (*((*p).data as (*mut master_lists))).cilist;
                        } else {
                            let mut ml : *mut master_lists;
                            ml = xmalloc(
                                     ::std::mem::size_of::<master_lists>()
                                 ) as (*mut master_lists);
                            ulist = {
                                        (*ml).ulist = ::hash::getlist();
                                        (*ml).ulist
                                    };
                            cilist = {
                                         (*ml).cilist = ::hash::getlist();
                                         (*ml).cilist
                                     };
                            p = ::hash::getnode();
                            (*p).key = Xstrdup(xdir);
                            (*p).type_ = ::hash::ntype::UPDATE;
                            (*p).data = ml as (*mut ::std::os::raw::c_void);
                            (*p).delproc = Some(masterlist_delproc);
                            ::hash::addnode(mulist,p);
                        }
                        p = ::hash::getnode();
                        (*p).key = Xstrdup((*finfo).file);
                        (*p).type_ = ::hash::ntype::UPDATE;
                        (*p).delproc = Some(update_delproc);
                        li = xmalloc(
                                 ::std::mem::size_of::<logfile_info>()
                             ) as (*mut logfile_info);
                        (*li).type_ = status;
                        if check_valid_edit != 0 {
                            let mut editors
                                : *mut u8
                                = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                            editors = fileattr_get0((*finfo).file,(*b"_editors\0").as_ptr());
                            if !editors.is_null() {
                                let mut caller : *mut u8 = getcaller();
                                let mut p
                                    : *mut u8
                                    = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                                let mut p0
                                    : *mut u8
                                    = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                                p = editors;
                                p0 = p;
                                'loop37: loop {
                                    if !(*p as (i32) != b'\0' as (i32)) {
                                        break;
                                    }
                                    p = strchr(p as (*const u8),b'>' as (i32));
                                    if p.is_null() {
                                        break;
                                    }
                                    *p = b'\0';
                                    if streq(caller as (*const u8),p0 as (*const u8)) {
                                        break;
                                    }
                                    p = strchr(p.offset(1isize) as (*const u8),b',' as (i32));
                                    if p.is_null() {
                                        break;
                                    }
                                    p = p.offset(1isize);
                                    p0 = p;
                                }
                                if streq(caller as (*const u8),p0 as (*const u8)) {
                                    editor = caller;
                                }
                                free(editors as (*mut ::std::os::raw::c_void));
                            }
                        }
                        if check_valid_edit != 0 && editor.is_null() {
                            error(
                                0i32,
                                0i32,
                                (*b"Valid edit does not exist for %s\0").as_ptr(),
                                quote((*finfo).fullname)
                            );
                            if !li.is_null() {
                                free(li as (*mut ::std::os::raw::c_void));
                            }
                            if !p.is_null() {
                                ::hash::freenode(p);
                            }
                            retval = 1i32;
                            _currentBlock = 68;
                        } else {
                            (*li).tag = Xstrdup((*vers).tag as (*const u8));
                            (*li).rev_old = Xstrdup((*vers).vn_rcs as (*const u8));
                            (*li).rev_new = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                            (*p).data = li as (*mut ::std::os::raw::c_void);
                            ::hash::addnode(ulist,p);
                            p = ::hash::getnode();
                            (*p).key = Xstrdup((*finfo).file);
                            (*p).type_ = ::hash::ntype::UPDATE;
                            (*p).delproc = Some(ci_delproc);
                            ci = xmalloc(
                                     ::std::mem::size_of::<commit_info>()
                                 ) as (*mut commit_info);
                            (*ci).status = status;
                            if !(*vers).tag.is_null() {
                                if isdigit(*(*vers).tag as (i32)) != 0 {
                                    (*ci).rev = Xstrdup((*vers).tag as (*const u8));
                                } else {
                                    (*ci).rev = RCS_whatbranch(
                                                    (*finfo).rcs,
                                                    (*vers).tag as (*const u8)
                                                );
                                }
                            } else {
                                (*ci).rev = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                            }
                            (*ci).tag = Xstrdup((*vers).tag as (*const u8));
                            (*ci).options = Xstrdup((*vers).options as (*const u8));
                            (*p).data = ci as (*mut ::std::os::raw::c_void);
                            ::hash::addnode(cilist,p);
                            _currentBlock = 66;
                        }
                    }
                }
            }
        } else if status as (i32) == classify_type::T_REMOVE_ENTRY as (i32) || status as (i32) == classify_type::T_NEEDS_MERGE as (i32) || status as (i32) == classify_type::T_PATCH as (i32) || status as (i32) == classify_type::T_CHECKOUT as (i32) {
            error(
                0i32,
                0i32,
                (*b"Up-to-date check failed for `%s\'\0").as_ptr(),
                (*finfo).fullname
            );
            _currentBlock = 68;
        } else {
            error(
                0i32,
                0i32,
                (*b"CVS internal error: unknown status %d\0").as_ptr(),
                status as (i32)
            );
            _currentBlock = 66;
        }
        if _currentBlock == 68 {
        } else {
            retval = 0i32;
        }
    }
    if !vers.is_null() {
        ::vers_ts::freevers_ts(
            &mut vers as (*mut *mut ::vers_ts::vers_ts)
        );
    }
    retval
}

unsafe extern fn check_direntproc(
    mut callerdat : *mut ::std::os::raw::c_void,
    mut dir : *const u8,
    mut repos : *const u8,
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
                (*b"Examining %s\0").as_ptr(),
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

#[derive(Copy)]
#[repr(C)]
pub struct format_cmdline_walklist_closure {
    pub file : *const u8,
    pub line : i32,
    pub format : *const u8,
    pub buf : *mut *mut u8,
    pub length : *mut usize,
    pub d : *mut *mut u8,
    pub quotes : u8,
    pub onearg : i32,
    pub firstpass : i32,
    pub srepos : *const u8,
    pub closure : *mut ::std::os::raw::c_void,
}

impl Clone for format_cmdline_walklist_closure {
    fn clone(&self) -> Self { *self }
}

unsafe extern fn precommit_list_to_args_proc(
    mut p : *mut ::hash::hashnode,
    mut closure : *mut ::std::os::raw::c_void
) -> i32 {
    let mut c
        : *mut format_cmdline_walklist_closure
        = closure as (*mut format_cmdline_walklist_closure);
    let mut li : *mut logfile_info;
    let mut arg
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut f : *const u8;
    let mut d : *mut u8;
    let mut doff : usize;
    if (*p).data == 0i32 as (*mut ::std::os::raw::c_void) {
        1i32
    } else {
        f = (*c).format;
        d = *(*c).d;
        'loop2: loop {
            if *f == 0 {
                break;
            }
            let switch4
                = *{
                       let _old = f;
                       f = f.offset(1isize);
                       _old
                   };
            if switch4 as (i32) == b'a' as (i32) {
                li = (*p).data as (*mut logfile_info);
                let switch5 = (*li).type_;
                if switch5 as (i32) == classify_type::T_TITLE as (i32) {
                    arg = (*b"imported\0").as_ptr() as (*mut u8);
                } else if switch5 as (i32) == classify_type::T_REMOVED as (i32) {
                    arg = (*b"removed\0").as_ptr() as (*mut u8);
                } else if switch5 as (i32) == classify_type::T_CONFLICT as (i32) || switch5 as (i32) == classify_type::T_MODIFIED as (i32) {
                    arg = (*b"modified\0").as_ptr() as (*mut u8);
                } else if switch5 as (i32) == classify_type::T_ADDED as (i32) {
                    arg = (*b"added\0").as_ptr() as (*mut u8);
                } else {
                    error(
                        1i32,
                        0i32,
                        (*b"Unexpected action type %d.\0").as_ptr(),
                        (*li).type_ as (i32)
                    );
                }
            } else if switch4 as (i32) == b'T' as (i32) {
                li = (*p).data as (*mut logfile_info);
                arg = if !(*li).tag.is_null() {
                          (*li).tag as (*const u8)
                      } else {
                          (*b"\0").as_ptr()
                      } as (*mut u8);
            } else if switch4 as (i32) == b's' as (i32) {
                li = (*p).data as (*mut logfile_info);
                if (*li).type_ as (i32) == classify_type::T_ADDED as (i32) || (*li).type_ as (i32) == classify_type::T_MODIFIED as (i32) || (*li).type_ as (i32) == classify_type::T_REMOVED as (i32) {
                    arg = (*p).key;
                }
            } else {
                error(
                    1i32,
                    0i32,
                    (*b"Unknown format character or not a list attribute: %c\0").as_ptr(
                    ),
                    *f.offset(-1isize) as (i32)
                );
            }
            if (*c).quotes != 0 {
                arg = cmdlineescape((*c).quotes,arg);
            } else {
                arg = cmdlinequote(b'\"',arg);
            }
            doff = ((d as (isize)).wrapping_sub(
                        *(*c).buf as (isize)
                    ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
            expand_string(
                (*c).buf,
                (*c).length,
                doff.wrapping_add(strlen(arg as (*const u8)))
            );
            d = (*(*c).buf).offset(doff as (isize));
            strncpy(d,arg as (*const u8),strlen(arg as (*const u8)));
            d = d.offset(strlen(arg as (*const u8)) as (isize));
            free(arg as (*mut ::std::os::raw::c_void));
            doff = ((d as (isize)).wrapping_sub(
                        *(*c).buf as (isize)
                    ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
            expand_string((*c).buf,(*c).length,doff.wrapping_add(1usize));
            d = (*(*c).buf).offset(doff as (isize));
            *{
                 let _old = d;
                 d = d.offset(1isize);
                 _old
             } = b' ';
        }
        *(*c).d = d;
        0i32
    }
}

unsafe extern fn precommit_proc(
    mut repository : *const u8,
    mut filter : *const u8,
    mut file : *const u8,
    mut line : i32,
    mut closure : *mut ::std::os::raw::c_void
) -> i32 {
    let mut newfilter
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut cmdline : *mut u8;
    let mut srepos : *const u8 = Short_Repository(repository);
    let mut ulist
        : *mut ::hash::hashlist
        = closure as (*mut ::hash::hashlist);
    if strchr(filter,b'%' as (i32)).is_null() {
        error(
            0i32,
            0i32,
            (*b"%s:%d: warning: commitinfo line contains no format strings.\nAppending defaults (%s), but please be aware that this usage is\ndeprecated.\0").as_ptr(
            ),
            file,
            line,
            quote((*b" %r/%p %s\0").as_ptr())
        );
        newfilter = Xasprintf((*b"%s %%r/%%p %%s\0").as_ptr(),filter);
        filter = newfilter as (*const u8);
    }
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
                  (*b"saT\0").as_ptr(),
                  (*b",\0").as_ptr(),
                  ulist,
                  precommit_list_to_args_proc as (*mut ::std::os::raw::c_void),
                  0i32 as (*mut ::std::os::raw::c_void),
                  0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
              );
    if !newfilter.is_null() {
        free(newfilter as (*mut ::std::os::raw::c_void));
    }
    if cmdline.is_null() || strlen(cmdline as (*const u8)) == 0 {
        if !cmdline.is_null() {
            free(cmdline as (*mut ::std::os::raw::c_void));
        }
        error(
            0i32,
            0i32,
            (*b"%s:%d: precommit proc resolved to the empty string!\0").as_ptr(
            ),
            file,
            line
        );
        1i32
    } else {
        run_setup(cmdline as (*const u8));
        free(cmdline as (*mut ::std::os::raw::c_void));
        run_exec(
            0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
            0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
            0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
            0x0i32 | 0x1i32 << 1i32
        )
    }
}

unsafe extern fn check_filesdoneproc(
    mut callerdat : *mut ::std::os::raw::c_void,
    mut err : i32,
    mut repos : *const u8,
    mut update_dir : *const u8,
    mut entries : *mut ::hash::hashlist
) -> i32 {
    let mut n : i32;
    let mut p : *mut ::hash::hashnode;
    let mut saved_ulist : *mut ::hash::hashlist;
    let mut commit_data
        : *mut commit_data
        = callerdat as (*mut commit_data);
    p = ::hash::findnode(
            mulist,
            if !update_dir.is_null() && (*update_dir != 0) {
                update_dir
            } else {
                (*b".\0").as_ptr()
            }
        );
    if !p.is_null() {
        saved_ulist = (*((*p).data as (*mut master_lists))).ulist;
    } else {
        saved_ulist = 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashlist);
    }
    if ::hash::list_isempty(saved_ulist) != 0 {
        err
    } else {
        n = Parse_Info(
                (*b"commitinfo\0").as_ptr(),
                repos,
                precommit_proc,
                1i32,
                saved_ulist as (*mut ::std::os::raw::c_void)
            );
        if n > 0i32 {
            error(0i32,0i32,(*b"Pre-commit check failed\0").as_ptr());
            err = err + n;
        }
        if server_active == 0 && (use_editor != 0) {
            do_editor(
                update_dir,
                &mut (*commit_data).saved_message as (*mut *mut u8),
                repos,
                saved_ulist
            );
        }
        err = err + do_verify(
                        &mut (*commit_data).saved_message as (*mut *mut u8),
                        repos,
                        saved_ulist
                    );
        err
    }
}

static mut maxrev : i32 = 0i32;

static mut sbranch : *mut u8 = 0 as (*mut u8);

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum server_updated_arg4 {
    SERVER_UPDATED,
    SERVER_MERGED,
    SERVER_PATCHED,
    SERVER_RCS_DIFF,
}

#[derive(Copy)]
#[repr(C)]
pub struct config {
    pub keywords : *mut ::std::os::raw::c_void,
    pub top_level_admin : bool,
    pub lock_dir : *mut u8,
    pub logHistory : *mut u8,
    pub usingDefaultLogHistory : bool,
    pub HistoryLogPath : *mut u8,
    pub HistorySearchPath : *mut u8,
    pub TmpDir : *mut u8,
    pub RereadLogAfterVerify : i32,
    pub FirstVerifyLogErrorFatal : bool,
    pub UserAdminOptions : *mut u8,
    pub ImportNewFilesToVendorBranchOnly : bool,
    pub MaxCommentLeaderLength : usize,
    pub UseArchiveCommentLeader : bool,
    pub system_auth : bool,
    pub UseNewInfoFmtStrings : bool,
    pub PrimaryServer : *mut cvsroot_s,
    pub MaxProxyBufferSize : usize,
    pub MinCompressionLevel : usize,
    pub MaxCompressionLevel : usize,
    pub VerifyCommits : Enum3,
    pub VerifyTemplate : *mut u8,
    pub OpenPGPTextmode : *mut u8,
    pub VerifyArgs : *mut ::hash::hashlist,
}

impl Clone for config {
    fn clone(&self) -> Self { *self }
}

unsafe extern fn commit_fileproc(
    mut callerdat : *mut ::std::os::raw::c_void,
    mut finfo : *mut ::vers_ts::file_info
) -> i32 {
    let mut _currentBlock;
    let mut p : *mut ::hash::hashnode;
    let mut err : i32 = 0i32;
    let mut ulist : *mut ::hash::hashlist;
    let mut cilist : *mut ::hash::hashlist;
    let mut ci : *mut commit_info;
    let mut commit_data
        : *mut commit_data
        = callerdat as (*mut commit_data);
    if write_dirtag != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) && ((*finfo).rcs != 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::vers_ts::rcsnode)) {
        let mut rev
            : *mut u8
            = RCS_getversion(
                  (*finfo).rcs,
                  write_dirtag as (*const u8),
                  0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                  1i32,
                  0i32 as (*mut ::std::os::raw::c_void) as (*mut i32)
              );
        if rev != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) && (RCS_nodeisbranch(
                                                                             (*finfo).rcs,
                                                                             write_dirtag as (*const u8)
                                                                         ) == 0) {
            write_dirnonbranch = 1i32;
        }
        if rev != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            free(rev as (*mut ::std::os::raw::c_void));
        }
    }
    if *(*finfo).update_dir.offset(0isize) as (i32) == b'\0' as (i32) {
        p = ::hash::findnode(mulist,(*b".\0").as_ptr());
    } else {
        p = ::hash::findnode(mulist,(*finfo).update_dir);
    }
    if p == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode) {
        0i32
    } else {
        ulist = (*((*p).data as (*mut master_lists))).ulist;
        cilist = (*((*p).data as (*mut master_lists))).cilist;
        p = ::hash::findnode(cilist,(*finfo).file);
        (if p == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode) {
             0i32
         } else {
             ci = (*p).data as (*mut commit_info);
             if (*ci).status as (i32) == classify_type::T_MODIFIED as (i32) {
                 if (*finfo).rcs == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::vers_ts::rcsnode) {
                     error(
                         1i32,
                         0i32,
                         (*b"internal error: no parsed RCS file\0").as_ptr()
                     );
                 }
                 if lock_RCS(
                        (*finfo).file,
                        (*finfo).rcs,
                        (*ci).rev as (*const u8),
                        (*finfo).repository
                    ) != 0i32 {
                     unlockrcs((*finfo).rcs);
                     err = 1i32;
                     _currentBlock = 45;
                 } else {
                     _currentBlock = 27;
                 }
             } else if (*ci).status as (i32) == classify_type::T_ADDED as (i32) {
                 if checkaddfile(
                        (*finfo).file,
                        (*finfo).repository,
                        (*ci).tag as (*const u8),
                        (*ci).options as (*const u8),
                        &mut (*finfo).rcs as (*mut *mut ::vers_ts::rcsnode)
                    ) != 0i32 {
                     if (*finfo).rcs != 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::vers_ts::rcsnode) {
                         fixaddfile((*(*finfo).rcs).path as (*const u8));
                     }
                     err = 1i32;
                     _currentBlock = 45;
                 } else if !(*ci).tag.is_null() && (isdigit(
                                                        *(*ci).tag.offset(0isize) as (i32)
                                                    ) == 0) {
                     if (*finfo).rcs == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::vers_ts::rcsnode) {
                         error(
                             1i32,
                             0i32,
                             (*b"internal error: no parsed RCS file\0").as_ptr()
                         );
                     }
                     if !(*ci).rev.is_null() {
                         free((*ci).rev as (*mut ::std::os::raw::c_void));
                     }
                     (*ci).rev = RCS_whatbranch((*finfo).rcs,(*ci).tag as (*const u8));
                     err = Checkin(
                               b'A' as (i32),
                               finfo,
                               (*ci).rev,
                               (*ci).tag,
                               (*ci).options,
                               (*commit_data).saved_message
                           );
                     if err != 0i32 {
                         unlockrcs((*finfo).rcs);
                         fixbranch((*finfo).rcs,sbranch);
                     }
                     time(&mut last_register_time as (*mut isize));
                     (*ci).status = classify_type::T_UPTODATE;
                     _currentBlock = 27;
                 } else {
                     _currentBlock = 27;
                 }
             } else {
                 _currentBlock = 27;
             }
             if _currentBlock == 27 {
                 if (*ci).status as (i32) == classify_type::T_ADDED as (i32) {
                     let mut xrev
                         : *mut u8
                         = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                     if (*ci).rev == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                         maxrev = 0i32;
                         ::hash::walklist(
                             (*finfo).entries,
                             findmaxrev,
                             0i32 as (*mut ::std::os::raw::c_void)
                         );
                         if !(*(*finfo).rcs).head.is_null() {
                             let mut thisrev : i32 = atoi((*(*finfo).rcs).head as (*const u8));
                             if thisrev > maxrev {
                                 maxrev = thisrev;
                             }
                         }
                         if maxrev == 0i32 {
                             maxrev = 1i32;
                         }
                         xrev = Xasprintf((*b"%d\0").as_ptr(),maxrev);
                     }
                     err = finaladd(
                               finfo,
                               if !(*ci).rev.is_null() { (*ci).rev } else { xrev },
                               (*ci).tag,
                               (*ci).options,
                               (*commit_data).saved_message
                           );
                     if !xrev.is_null() {
                         free(xrev as (*mut ::std::os::raw::c_void));
                     }
                 } else if (*ci).status as (i32) == classify_type::T_MODIFIED as (i32) {
                     err = Checkin(
                               b'M' as (i32),
                               finfo,
                               (*ci).rev,
                               (*ci).tag,
                               (*ci).options,
                               (*commit_data).saved_message
                           );
                     time(&mut last_register_time as (*mut isize));
                     if err != 0i32 {
                         unlockrcs((*finfo).rcs);
                         fixbranch((*finfo).rcs,sbranch);
                     }
                 } else if (*ci).status as (i32) == classify_type::T_REMOVED as (i32) {
                     err = remove_file(finfo,(*ci).tag,(*commit_data).saved_message);
                     if server_active != 0 {
                         server_scratch_entry_only();
                         server_updated(
                             finfo,
                             0i32 as (*mut ::std::os::raw::c_void) as (*mut ::vers_ts::vers_ts),
                             server_updated_arg4::SERVER_UPDATED,
                             -1i32 as (u32),
                             0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                             0i32 as (*mut ::std::os::raw::c_void) as (*mut buffer)
                         );
                     }
                 }
                 notify_do(
                     b'C' as (i32),
                     (*finfo).file,
                     (*finfo).update_dir,
                     getcaller() as (*const u8),
                     0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                     0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                     (*finfo).repository
                 );
             }
             if err != 0i32 {
                 p = ::hash::findnode(ulist,(*finfo).file);
                 if !p.is_null() {
                     ::hash::delnode(p);
                 }
             } else if (*ci).status as (i32) != classify_type::T_REMOVED as (i32) || (*config).UseNewInfoFmtStrings {
                 p = ::hash::findnode(ulist,(*finfo).file);
                 if !p.is_null() {
                     let mut vers : *mut ::vers_ts::vers_ts = 0 as (*mut ::vers_ts::vers_ts);
                     let mut li : *mut logfile_info;
                     classify_file_internal(
                         finfo,
                         &mut vers as (*mut *mut ::vers_ts::vers_ts)
                     );
                     li = (*p).data as (*mut logfile_info);
                     (*li).rev_new = Xstrdup((*vers).vn_rcs as (*const u8));
                     ::vers_ts::freevers_ts(
                         &mut vers as (*mut *mut ::vers_ts::vers_ts)
                     );
                 }
             }
             if SIG_inCrSect() != 0 {
                 SIG_endCrSect();
             }
             err
         })
    }
}

unsafe extern fn commit_filesdoneproc(
    mut callerdat : *mut ::std::os::raw::c_void,
    mut err : i32,
    mut repository : *const u8,
    mut update_dir : *const u8,
    mut entries : *mut ::hash::hashlist
) -> i32 {
    let mut p : *mut ::hash::hashnode;
    let mut ulist : *mut ::hash::hashlist;
    let mut commit_data
        : *mut commit_data
        = callerdat as (*mut commit_data);
    if !repository.is_null() {
        0i32;
    } else {
        __assert_fail(
            (*b"repository\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"commit_filesdoneproc\0").as_ptr()
        );
    }
    p = ::hash::findnode(
            mulist,
            if !update_dir.is_null() && (*update_dir != 0) {
                update_dir
            } else {
                (*b".\0").as_ptr()
            }
        );
    if p == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode) {
        err
    } else {
        ulist = (*((*p).data as (*mut master_lists))).ulist;
        let mut p : *const u8;
        if !strneq(
                (*current_parsed_root).directory as (*const u8),
                repository,
                strlen((*current_parsed_root).directory as (*const u8))
            ) {
            error(
                0i32,
                0i32,
                (*b"internal error: repository (%s) doesn\'t begin with root (%s)\0").as_ptr(
                ),
                repository,
                (*current_parsed_root).directory
            );
        }
        p = repository.offset(
                strlen((*current_parsed_root).directory as (*const u8)) as (isize)
            );
        if *p as (i32) == b'/' as (i32) {
            p = p.offset(1isize);
        }
        if streq((*b"CVSROOT\0").as_ptr(),p) || strneq(
                                                    (*b"CVSROOT/\0").as_ptr(),
                                                    p,
                                                    strlen((*b"CVSROOT/\0").as_ptr())
                                                ) {
            let mut admin_dir : *mut u8 = Xstrdup(repository);
            let mut cvsrootlen
                : i32
                = strlen((*b"CVSROOT\0").as_ptr()) as (i32);
            if *admin_dir.offset(
                    (p as (isize)).wrapping_sub(
                        repository as (isize)
                    ) / ::std::mem::size_of::<u8>() as (isize) + cvsrootlen as (isize)
                ) as (i32) == b'\0' as (i32) || *admin_dir.offset(
                                                     (p as (isize)).wrapping_sub(
                                                         repository as (isize)
                                                     ) / ::std::mem::size_of::<u8>(
                                                         ) as (isize) + cvsrootlen as (isize)
                                                 ) as (i32) == b'/' as (i32) {
                0i32;
            } else {
                __assert_fail(
                    (*b"admin_dir[p - repository + cvsrootlen] == \'\\0\' || admin_dir[p - repository + cvsrootlen] == \'/\'\0").as_ptr(
                    ),
                    file!().as_ptr(),
                    line!(),
                    (*b"commit_filesdoneproc\0").as_ptr()
                );
            }
            *admin_dir.offset(
                 (p as (isize)).wrapping_sub(
                     repository as (isize)
                 ) / ::std::mem::size_of::<u8>() as (isize) + cvsrootlen as (isize)
             ) = b'\0';
            if really_quiet == 0 {
                cvs_output(program_name,0usize);
                cvs_output((*b" \0").as_ptr(),1usize);
                cvs_output(cvs_cmd_name,0usize);
                cvs_output(
                    (*b": Rebuilding administrative file database\n\0").as_ptr(),
                    0usize
                );
            }
            mkmodules(admin_dir);
            free(admin_dir as (*mut ::std::os::raw::c_void));
            WriteTemplate((*b".\0").as_ptr(),1i32,repository);
        }
        Update_Logfile(
            repository,
            (*commit_data).saved_message as (*const u8),
            0i32 as (*mut ::std::os::raw::c_void) as (*mut _IO_FILE),
            ulist
        );
        err
    }
}

unsafe extern fn commit_direntproc(
    mut callerdat : *mut ::std::os::raw::c_void,
    mut dir : *const u8,
    mut repos : *const u8,
    mut update_dir : *const u8,
    mut entries : *mut ::hash::hashlist
) -> direnter_type {
    let mut p : *mut ::hash::hashnode;
    let mut ulist : *mut ::hash::hashlist;
    if !isdir(dir) {
        direnter_type::R_SKIP_ALL
    } else {
        p = ::hash::findnode(
                mulist,
                if !update_dir.is_null() && (*update_dir != 0) {
                    update_dir
                } else {
                    (*b".\0").as_ptr()
                }
            );
        if p != 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashnode) {
            ulist = (*((*p).data as (*mut master_lists))).ulist;
        } else {
            ulist = 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashlist);
        }
        (if ulist == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::hash::hashlist) || (*(*ulist).list).next as (*mut ::std::os::raw::c_void) == (*ulist).list as (*mut ::std::os::raw::c_void) {
             direnter_type::R_SKIP_FILES
         } else {
             direnter_type::R_PROCESS
         })
    }
}

unsafe extern fn commit_dirleaveproc(
    mut callerdat : *mut ::std::os::raw::c_void,
    mut dir : *const u8,
    mut err : i32,
    mut update_dir : *const u8,
    mut entries : *mut ::hash::hashlist
) -> i32 {
    if err == 0i32 && (write_dirtag != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)) {
        let mut repos
            : *mut u8
            = Name_Repository(
                  0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                  update_dir
              );
        WriteTag(
            0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
            write_dirtag as (*const u8),
            0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
            write_dirnonbranch,
            update_dir,
            repos as (*const u8)
        );
        free(repos as (*mut ::std::os::raw::c_void));
    }
    err
}

unsafe extern fn findmaxrev(
    mut p : *mut ::hash::hashnode,
    mut closure : *mut ::std::os::raw::c_void
) -> i32 {
    let mut thisrev : i32;
    let mut entdata
        : *mut ::entries::entnode
        = (*p).data as (*mut ::entries::entnode);
    if (*entdata).type_ as (i32) != ::entries::ent_type::ENT_FILE as (i32) {
        0i32
    } else {
        thisrev = atoi((*entdata).version as (*const u8));
        if thisrev > maxrev {
            maxrev = thisrev;
        }
        0i32
    }
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

unsafe extern fn remove_file(
    mut finfo : *mut ::vers_ts::file_info,
    mut tag : *mut u8,
    mut message : *mut u8
) -> i32 {
    let mut retcode : i32;
    let mut branch : i32;
    let mut lockflag : i32;
    let mut corev : *mut u8;
    let mut rev : *mut u8;
    let mut prev_rev : *mut u8;
    let mut old_path : *mut u8;
    corev = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    rev = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    prev_rev = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    retcode = 0i32;
    if (*finfo).rcs == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::vers_ts::rcsnode) {
        error(
            1i32,
            0i32,
            (*b"internal error: no parsed RCS file\0").as_ptr()
        );
    }
    branch = 0i32;
    if !tag.is_null() && ({
                              branch = RCS_nodeisbranch((*finfo).rcs,tag as (*const u8));
                              branch
                          } == 0) {
        (if {
                retcode = RCS_deltag((*finfo).rcs,tag as (*const u8));
                retcode
            } != 0i32 {
             if quiet == 0 {
                 error(
                     0i32,
                     if retcode == -1i32 { *__errno_location() } else { 0i32 },
                     (*b"failed to remove tag `%s\' from `%s\'\0").as_ptr(),
                     tag,
                     (*finfo).fullname
                 );
             }
             1i32
         } else {
             RCS_rewrite(
                 (*finfo).rcs,
                 0i32 as (*mut ::std::os::raw::c_void) as (*mut deltatext),
                 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
             );
             Scratch_Entry((*finfo).entries,(*finfo).file);
             0i32
         })
    } else {
        rev = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        lockflag = 1i32;
        if branch != 0 {
            let mut branchname : *mut u8;
            rev = RCS_whatbranch((*finfo).rcs,tag as (*const u8));
            if rev == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                error(0i32,0i32,(*b"cannot find branch \"%s\".\0").as_ptr(),tag);
                return 1i32;
            } else {
                branchname = RCS_getbranch((*finfo).rcs,rev as (*const u8),1i32);
                if branchname == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                    corev = RCS_gettag(
                                (*finfo).rcs,
                                tag as (*const u8),
                                1i32,
                                0i32 as (*mut ::std::os::raw::c_void) as (*mut i32)
                            );
                    prev_rev = Xstrdup(corev as (*const u8));
                    lockflag = 0i32;
                } else {
                    corev = Xstrdup(rev as (*const u8));
                    prev_rev = Xstrdup(branchname as (*const u8));
                    free(branchname as (*mut ::std::os::raw::c_void));
                }
            }
        } else {
            prev_rev = RCS_head((*finfo).rcs);
        }
        if tag.is_null() && (branch == 0) {
            if RCS_setbranch(
                   (*finfo).rcs,
                   0i32 as (*mut ::std::os::raw::c_void) as (*const u8)
               ) != 0i32 {
                error(
                    0i32,
                    0i32,
                    (*b"cannot change branch to default for %s\0").as_ptr(),
                    (*finfo).fullname
                );
                return 1i32;
            } else {
                RCS_rewrite(
                    (*finfo).rcs,
                    0i32 as (*mut ::std::os::raw::c_void) as (*mut deltatext),
                    0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                );
            }
        }
        retcode = RCS_checkout(
                      (*finfo).rcs,
                      (*finfo).file,
                      if !rev.is_null() {
                          corev
                      } else {
                          0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                      } as (*const u8),
                      0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                      0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                      0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                      None,
                      0i32 as (*mut ::std::os::raw::c_void)
                  );
        (if retcode != 0i32 {
             error(
                 0i32,
                 0i32,
                 (*b"failed to check out `%s\'\0").as_ptr(),
                 (*finfo).fullname
             );
             1i32
         } else {
             if lockflag != 0 {
                 if RCS_lock(
                        (*finfo).rcs,
                        if !rev.is_null() {
                            corev
                        } else {
                            0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                        } as (*const u8),
                        1i32
                    ) == 0i32 {
                     RCS_rewrite(
                         (*finfo).rcs,
                         0i32 as (*mut ::std::os::raw::c_void) as (*mut deltatext),
                         0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                     );
                 }
             }
             if corev != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                 free(corev as (*mut ::std::os::raw::c_void));
             }
             retcode = RCS_checkin(
                           (*finfo).rcs,
                           0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                           (*finfo).file,
                           message as (*const u8),
                           rev as (*const u8),
                           0isize,
                           2i32 | 4i32
                       );
             (if retcode != 0i32 {
                  if quiet == 0 {
                      error(
                          0i32,
                          if retcode == -1i32 { *__errno_location() } else { 0i32 },
                          (*b"failed to commit dead revision for `%s\'\0").as_ptr(),
                          (*finfo).fullname
                      );
                  }
                  if prev_rev != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                      free(prev_rev as (*mut ::std::os::raw::c_void));
                  }
                  1i32
              } else {
                  corev = if !rev.is_null() {
                              RCS_getbranch((*finfo).rcs,rev as (*const u8),1i32)
                          } else {
                              RCS_head((*finfo).rcs)
                          };
                  history_write(
                      b'R' as (i32),
                      0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                      corev as (*const u8),
                      (*finfo).file,
                      (*finfo).repository
                  );
                  free(corev as (*mut ::std::os::raw::c_void));
                  if rev != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                      free(rev as (*mut ::std::os::raw::c_void));
                  }
                  old_path = Xstrdup((*(*finfo).rcs).path as (*const u8));
                  if branch == 0 {
                      RCS_setattic((*finfo).rcs,1i32);
                  }
                  if really_quiet == 0 {
                      cvs_output(old_path as (*const u8),0usize);
                      cvs_output((*b"  <--  \0").as_ptr(),0usize);
                      if !(*finfo).update_dir.is_null() && (strlen(
                                                                (*finfo).update_dir
                                                            ) != 0) {
                          cvs_output((*finfo).update_dir,0usize);
                          cvs_output((*b"/\0").as_ptr(),1usize);
                      }
                      cvs_output((*finfo).file,0usize);
                      cvs_output(
                          (*b"\nnew revision: delete; previous revision: \0").as_ptr(),
                          0usize
                      );
                      cvs_output(prev_rev as (*const u8),0usize);
                      cvs_output((*b"\n\0").as_ptr(),0usize);
                  }
                  free(prev_rev as (*mut ::std::os::raw::c_void));
                  free(old_path as (*mut ::std::os::raw::c_void));
                  Scratch_Entry((*finfo).entries,(*finfo).file);
                  0i32
              })
         })
    }
}

unsafe extern fn finaladd(
    mut finfo : *mut ::vers_ts::file_info,
    mut rev : *mut u8,
    mut tag : *mut u8,
    mut options : *mut u8,
    mut msg : *mut u8
) -> i32 {
    let mut ret : i32;
    ret = Checkin(b'A' as (i32),finfo,rev,tag,options,msg);
    if ret == 0i32 {
        let mut tmp
            : *mut u8
            = Xasprintf(
                  (*b"%s/%s%s\0").as_ptr(),
                  (*b"CVS\0").as_ptr(),
                  (*finfo).file,
                  (*b",t\0").as_ptr()
              );
        if unlink_file(tmp as (*const u8)) < 0i32 && !(*__errno_location(
                                                        ) == 2i32) {
            error(
                0i32,
                *__errno_location(),
                (*b"cannot remove %s\0").as_ptr(),
                tmp
            );
        }
        free(tmp as (*mut ::std::os::raw::c_void));
    } else if (*finfo).rcs != 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::vers_ts::rcsnode) {
        fixaddfile((*(*finfo).rcs).path as (*const u8));
    }
    time(&mut last_register_time as (*mut isize));
    ret
}

unsafe extern fn unlockrcs(mut rcs : *mut ::vers_ts::rcsnode) {
    let mut retcode : i32;
    if {
           retcode = RCS_unlock(
                         rcs,
                         0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                         1i32
                     );
           retcode
       } != 0i32 {
        error(
            if retcode == -1i32 { 1i32 } else { 0i32 },
            if retcode == -1i32 { *__errno_location() } else { 0i32 },
            (*b"could not unlock %s\0").as_ptr(),
            (*rcs).path
        );
    } else {
        RCS_rewrite(
            rcs,
            0i32 as (*mut ::std::os::raw::c_void) as (*mut deltatext),
            0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
        );
    }
}

unsafe extern fn fixaddfile(mut rcs : *const u8) {
    let mut rcsfile : *mut ::vers_ts::rcsnode;
    let mut save_really_quiet : i32;
    save_really_quiet = really_quiet;
    really_quiet = 1i32;
    if {
           rcsfile = RCS_parsercsfile(rcs);
           rcsfile
       } == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::vers_ts::rcsnode) {
        if unlink_file(rcs) < 0i32 {
            error(
                0i32,
                *__errno_location(),
                (*b"cannot remove %s\0").as_ptr(),
                rcs
            );
        }
    } else {
        freercsnode(&mut rcsfile as (*mut *mut ::vers_ts::rcsnode));
    }
    really_quiet = save_really_quiet;
}

unsafe extern fn fixbranch(
    mut rcs : *mut ::vers_ts::rcsnode, mut branch : *mut u8
) {
    let mut retcode : i32;
    if branch != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        if {
               retcode = RCS_setbranch(rcs,branch as (*const u8));
               retcode
           } != 0i32 {
            error(
                if retcode == -1i32 { 1i32 } else { 0i32 },
                if retcode == -1i32 { *__errno_location() } else { 0i32 },
                (*b"cannot restore branch to %s for %s\0").as_ptr(),
                branch,
                (*rcs).path
            );
        }
        RCS_rewrite(
            rcs,
            0i32 as (*mut ::std::os::raw::c_void) as (*mut deltatext),
            0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
        );
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

unsafe extern fn checkaddfile(
    mut file : *const u8,
    mut repository : *const u8,
    mut tag : *const u8,
    mut options : *const u8,
    mut rcsnode : *mut *mut ::vers_ts::rcsnode
) -> i32 {
    let mut _currentBlock;
    let mut rcs : *mut ::vers_ts::rcsnode = 0 as (*mut ::vers_ts::rcsnode);
    let mut fname : *mut u8;
    let mut newfile : i32 = 0i32;
    let mut retval : i32 = 1i32;
    let mut adding_on_branch : i32;
    if rcsnode != 0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut ::vers_ts::rcsnode) {
        0i32;
    } else {
        __assert_fail(
            (*b"rcsnode != NULL\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"checkaddfile\0").as_ptr()
        );
    }
    if options != 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) && (*options.offset(
                                                                                0isize
                                                                            ) as (i32) == b'\0' as (i32)) {
        options = 0i32 as (*mut ::std::os::raw::c_void) as (*const u8);
    }
    if options != 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
        if *options.offset(
                0isize
            ) as (i32) == b'-' as (i32) && (*options.offset(
                                                 1isize
                                             ) as (i32) == b'k' as (i32)) {
            0i32;
        } else {
            __assert_fail(
                (*b"options[0] == \'-\' && options[1] == \'k\'\0").as_ptr(),
                file!().as_ptr(),
                line!(),
                (*b"checkaddfile\0").as_ptr()
            );
        }
    }
    adding_on_branch = (tag != 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) && (isdigit(
                                                                                            *tag.offset(
                                                                                                 0isize
                                                                                             ) as (i32)
                                                                                        ) == 0)) as (i32);
    if *rcsnode == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::vers_ts::rcsnode) {
        let mut rcsname : *mut u8;
        let mut desc
            : *mut u8
            = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        let mut descalloc : usize = 0usize;
        let mut desclen : usize = 0usize;
        let mut opt : *const u8;
        if adding_on_branch != 0 {
            rcsname = xmalloc(
                          strlen(repository).wrapping_add(
                              ::std::mem::size_of::<[u8; 6]>()
                          ).wrapping_add(
                              strlen(file)
                          ).wrapping_add(
                              ::std::mem::size_of::<[u8; 3]>()
                          ).wrapping_add(
                              3usize
                          )
                      ) as (*mut u8);
            sprintf(
                rcsname,
                (*b"%s/%s\0").as_ptr(),
                repository,
                (*b"Attic\0").as_ptr()
            );
            if !isdir(rcsname as (*const u8)) {
                cvs_xmkdir(
                    rcsname as (*const u8),
                    0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                    (1i32 << 2i32) as (u32)
                );
            }
            sprintf(
                rcsname,
                (*b"%s/%s/%s%s\0").as_ptr(),
                repository,
                (*b"Attic\0").as_ptr(),
                file,
                (*b",v\0").as_ptr()
            );
        } else {
            rcsname = Xasprintf(
                          (*b"%s/%s%s\0").as_ptr(),
                          repository,
                          file,
                          (*b",v\0").as_ptr()
                      );
        }
        fname = Xasprintf(
                    (*b"%s/%s%s\0").as_ptr(),
                    (*b"CVS\0").as_ptr(),
                    file,
                    (*b",t\0").as_ptr()
                );
        if isfile(fname as (*const u8)) {
            get_file(
                fname as (*const u8),
                fname as (*const u8),
                (*b"r\0").as_ptr(),
                &mut desc as (*mut *mut u8),
                &mut descalloc as (*mut usize),
                &mut desclen as (*mut usize)
            );
        }
        free(fname as (*mut ::std::os::raw::c_void));
        if desclen > 0usize {
            expand_string(
                &mut desc as (*mut *mut u8),
                &mut descalloc as (*mut usize),
                desclen.wrapping_add(1usize)
            );
            *desc.offset(
                 {
                     let _old = desclen;
                     desclen = desclen.wrapping_add(1usize);
                     _old
                 } as (isize)
             ) = b'\n';
        }
        if options != 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
            opt = options.offset(2isize);
        } else {
            opt = 0i32 as (*mut ::std::os::raw::c_void) as (*const u8);
        }
        if add_rcs_file(
               0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
               rcsname as (*const u8),
               file,
               0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
               opt,
               0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
               0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
               0i32,
               0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut u8),
               desc as (*const u8),
               desclen,
               0i32 as (*mut ::std::os::raw::c_void) as (*mut _IO_FILE),
               false
           ) != 0i32 {
            if rcsname != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                free(rcsname as (*mut ::std::os::raw::c_void));
                _currentBlock = 70;
            } else {
                _currentBlock = 70;
            }
        } else {
            rcs = RCS_parsercsfile(rcsname as (*const u8));
            newfile = 1i32;
            if rcsname != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                free(rcsname as (*mut ::std::os::raw::c_void));
            }
            if desc != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                free(desc as (*mut ::std::os::raw::c_void));
            }
            *rcsnode = rcs;
            _currentBlock = 34;
        }
    } else {
        let mut rev : *mut u8;
        let mut oldexpand : *mut u8;
        rcs = *rcsnode;
        oldexpand = RCS_getexpand(rcs);
        if oldexpand != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) && (options != 0i32 as (*mut ::std::os::raw::c_void) as (*const u8)) && !streq(
                                                                                                                                                        options.offset(
                                                                                                                                                            2isize
                                                                                                                                                        ),
                                                                                                                                                        oldexpand as (*const u8)
                                                                                                                                                    ) || oldexpand == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) && (options != 0i32 as (*mut ::std::os::raw::c_void) as (*const u8)) {
            error(
                0i32,
                0i32,
                (*b"changing keyword expansion mode of `%s\' from `-k%s\' to `%s\'\0").as_ptr(
                ),
                file,
                if !oldexpand.is_null() {
                    oldexpand as (*const u8)
                } else {
                    (*b"kv\0").as_ptr()
                },
                options
            );
            RCS_setexpand(rcs,options.offset(2isize));
        }
        if adding_on_branch == 0 {
            if (*rcs).flags & 0x2i32 == 0 {
                error(
                    0i32,
                    0i32,
                    (*b"warning: expected %s to be in Attic\0").as_ptr(),
                    (*rcs).path
                );
            }
            SIG_beginCrSect();
            if RCS_setattic(rcs,0i32) != 0 {
                _currentBlock = 70;
            } else {
                _currentBlock = 11;
            }
        } else {
            _currentBlock = 11;
        }
        if _currentBlock == 70 {
        } else {
            rev = RCS_getversion(
                      rcs,
                      tag,
                      0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                      1i32,
                      0i32 as (*mut ::std::os::raw::c_void) as (*mut i32)
                  );
            if lock_RCS(file,rcs,rev as (*const u8),repository) != 0 {
                error(
                    0i32,
                    0i32,
                    (*b"cannot lock revision %s in `%s\'.\0").as_ptr(),
                    if !rev.is_null() {
                        rev as (*const u8)
                    } else if !tag.is_null() {
                        tag
                    } else {
                        (*b"HEAD\0").as_ptr()
                    },
                    (*rcs).path
                );
                if rev != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                    free(rev as (*mut ::std::os::raw::c_void));
                    _currentBlock = 70;
                } else {
                    _currentBlock = 70;
                }
            } else if rev != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                free(rev as (*mut ::std::os::raw::c_void));
                _currentBlock = 34;
            } else {
                _currentBlock = 34;
            }
        }
    }
    if _currentBlock == 34 {
        if adding_on_branch != 0 {
            if newfile != 0 {
                let mut tmp : *mut u8;
                let mut fp : *mut _IO_FILE;
                let mut retcode : i32;
                fname = Xasprintf(
                            (*b"%s/%s%s\0").as_ptr(),
                            (*b"CVS\0").as_ptr(),
                            (*b",,\0").as_ptr(),
                            file
                        );
                rename_file(file,fname as (*const u8));
                fp = fopen(file,(*b"w\0").as_ptr());
                if fp == 0i32 as (*mut ::std::os::raw::c_void) as (*mut _IO_FILE) {
                    error(
                        1i32,
                        *__errno_location(),
                        (*b"cannot open %s for writing\0").as_ptr(),
                        file
                    );
                }
                if fclose(fp) < 0i32 {
                    error(
                        0i32,
                        *__errno_location(),
                        (*b"cannot close %s\0").as_ptr(),
                        file
                    );
                }
                tmp = Xasprintf(
                          (*b"file %s was initially added on branch %s.\0").as_ptr(),
                          file,
                          tag
                      );
                retcode = RCS_checkin(
                              rcs,
                              0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                              0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                              tmp as (*const u8),
                              0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                              0isize,
                              2i32 | 4i32
                          );
                free(tmp as (*mut ::std::os::raw::c_void));
                if retcode != 0i32 {
                    error(
                        if retcode == -1i32 { 1i32 } else { 0i32 },
                        if retcode == -1i32 { *__errno_location() } else { 0i32 },
                        (*b"could not create initial dead revision %s\0").as_ptr(),
                        (*rcs).path
                    );
                    free(fname as (*mut ::std::os::raw::c_void));
                    _currentBlock = 70;
                } else {
                    rename_file(fname as (*const u8),file);
                    free(fname as (*mut ::std::os::raw::c_void));
                    freercsnode(&mut rcs as (*mut *mut ::vers_ts::rcsnode));
                    rcs = RCS_parse(file,repository);
                    if rcs == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::vers_ts::rcsnode) {
                        error(
                            0i32,
                            0i32,
                            (*b"could not read %s in %s\0").as_ptr(),
                            file,
                            repository
                        );
                        _currentBlock = 70;
                    } else {
                        *rcsnode = rcs;
                        if lock_RCS(
                               file,
                               rcs,
                               0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                               repository
                           ) != 0 {
                            error(
                                0i32,
                                0i32,
                                (*b"cannot lock initial revision in `%s\'.\0").as_ptr(),
                                (*rcs).path
                            );
                            _currentBlock = 70;
                        } else {
                            _currentBlock = 43;
                        }
                    }
                }
            } else {
                _currentBlock = 43;
            }
            if _currentBlock == 70 {
            } else {
                if RCS_nodeisbranch(rcs,tag) == 0 {
                    let mut head : *mut u8;
                    let mut magicrev : *mut u8;
                    let mut retcode : i32;
                    let mut headtime : isize = -1isize;
                    let mut revnum : *mut u8;
                    let mut tmp : *mut u8;
                    let mut fp : *mut _IO_FILE;
                    let mut t : isize = -1isize;
                    let mut ct : *mut tm;
                    fixbranch(rcs,sbranch);
                    head = RCS_getversion(
                               rcs,
                               0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                               0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                               0i32,
                               0i32 as (*mut ::std::os::raw::c_void) as (*mut i32)
                           );
                    if head.is_null() {
                        error(
                            1i32,
                            0i32,
                            (*b"No head revision in archive file `%s\'.\0").as_ptr(),
                            (*rcs).print_path
                        );
                    }
                    magicrev = RCS_magicrev(rcs,head);
                    if newfile == 0 {
                        headtime = RCS_getrevtime(
                                       rcs,
                                       head as (*const u8),
                                       0i32 as (*mut u8),
                                       0i32
                                   );
                    }
                    retcode = RCS_settag(rcs,tag,magicrev as (*const u8));
                    RCS_rewrite(
                        rcs,
                        0i32 as (*mut ::std::os::raw::c_void) as (*mut deltatext),
                        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                    );
                    free(head as (*mut ::std::os::raw::c_void));
                    free(magicrev as (*mut ::std::os::raw::c_void));
                    if retcode != 0i32 {
                        error(
                            if retcode == -1i32 { 1i32 } else { 0i32 },
                            if retcode == -1i32 { *__errno_location() } else { 0i32 },
                            (*b"could not stub branch %s for %s\0").as_ptr(),
                            tag,
                            (*rcs).path
                        );
                        _currentBlock = 70;
                    } else if newfile == 0 && (headtime != -1isize) {
                        fname = Xasprintf(
                                    (*b"%s/%s%s\0").as_ptr(),
                                    (*b"CVS\0").as_ptr(),
                                    (*b",,\0").as_ptr(),
                                    file
                                );
                        rename_file(file,fname as (*const u8));
                        fp = fopen(file,(*b"w\0").as_ptr());
                        if fp == 0i32 as (*mut ::std::os::raw::c_void) as (*mut _IO_FILE) {
                            error(
                                1i32,
                                *__errno_location(),
                                (*b"cannot open %s for writing\0").as_ptr(),
                                file
                            );
                        }
                        if fclose(fp) < 0i32 {
                            error(
                                0i32,
                                *__errno_location(),
                                (*b"cannot close %s\0").as_ptr(),
                                file
                            );
                        }
                        t = time(0i32 as (*mut ::std::os::raw::c_void) as (*mut isize));
                        ct = gmtime(&mut t as (*mut isize) as (*const isize));
                        tmp = Xasprintf(
                                  (*b"file %s was added on branch %s on %d-%02d-%02d %02d:%02d:%02d +0000\0").as_ptr(
                                  ),
                                  file,
                                  tag,
                                  (*ct).tm_year + if (*ct).tm_year < 100i32 {
                                                      0i32
                                                  } else {
                                                      1900i32
                                                  },
                                  (*ct).tm_mon + 1i32,
                                  (*ct).tm_mday,
                                  (*ct).tm_hour,
                                  (*ct).tm_min,
                                  (*ct).tm_sec
                              );
                        revnum = RCS_whatbranch(rcs,tag);
                        retcode = RCS_checkin(
                                      rcs,
                                      0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                                      0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                                      tmp as (*const u8),
                                      revnum as (*const u8),
                                      headtime,
                                      2i32 | 4i32 | 32i32
                                  );
                        free(revnum as (*mut ::std::os::raw::c_void));
                        free(tmp as (*mut ::std::os::raw::c_void));
                        if retcode != 0i32 {
                            error(
                                if retcode == -1i32 { 1i32 } else { 0i32 },
                                if retcode == -1i32 { *__errno_location() } else { 0i32 },
                                (*b"could not created dead stub %s for %s\0").as_ptr(),
                                tag,
                                (*rcs).path
                            );
                            _currentBlock = 70;
                        } else {
                            rename_file(fname as (*const u8),file);
                            free(fname as (*mut ::std::os::raw::c_void));
                            freercsnode(&mut rcs as (*mut *mut ::vers_ts::rcsnode));
                            rcs = RCS_parse(file,repository);
                            if rcs == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ::vers_ts::rcsnode) {
                                error(0i32,0i32,(*b"could not read %s\0").as_ptr(),(*rcs).path);
                                _currentBlock = 70;
                            } else {
                                *rcsnode = rcs;
                                _currentBlock = 59;
                            }
                        }
                    } else {
                        _currentBlock = 59;
                    }
                } else if lock_RCS(
                              file,
                              rcs,
                              0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                              repository
                          ) != 0 {
                    error(
                        0i32,
                        0i32,
                        (*b"cannot lock head revision in `%s\'.\0").as_ptr(),
                        (*rcs).path
                    );
                    _currentBlock = 70;
                } else {
                    _currentBlock = 59;
                }
                if _currentBlock == 70 {
                } else if *rcsnode != rcs {
                    freercsnode(rcsnode);
                    *rcsnode = rcs;
                    _currentBlock = 61;
                } else {
                    _currentBlock = 61;
                }
            }
        } else {
            _currentBlock = 61;
        }
        if _currentBlock == 70 {
        } else {
            fileattr_newfile(file);
            retval = 0i32;
        }
    }
    if retval != 0i32 && (SIG_inCrSect() != 0) {
        SIG_endCrSect();
    }
    retval
}

unsafe extern fn lock_RCS(
    mut user : *const u8,
    mut rcs : *mut ::vers_ts::rcsnode,
    mut rev : *const u8,
    mut repository : *const u8
) -> i32 {
    let mut branch
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut err : i32 = 0i32;
    if rev == 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) || !rev.is_null(
                                                                       ) && (isdigit(
                                                                                 *rev as (i32)
                                                                             ) != 0) && (numdots(
                                                                                             rev
                                                                                         ) < 2i32) {
        branch = Xstrdup((*rcs).branch as (*const u8));
        if branch != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            if RCS_setbranch(
                   rcs,
                   0i32 as (*mut ::std::os::raw::c_void) as (*const u8)
               ) != 0i32 {
                error(
                    0i32,
                    0i32,
                    (*b"cannot change branch to default for %s\0").as_ptr(),
                    (*rcs).path
                );
                if !branch.is_null() {
                    free(branch as (*mut ::std::os::raw::c_void));
                }
                return 1i32;
            }
        }
        err = RCS_lock(
                  rcs,
                  0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
                  1i32
              );
    } else {
        RCS_lock(rcs,rev,1i32);
    }
    if err == 0i32 {
        if sbranch != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            free(sbranch as (*mut ::std::os::raw::c_void));
        }
        sbranch = branch;
        0i32
    } else {
        if branch != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
            fixbranch(rcs,branch);
        }
        if !branch.is_null() {
            free(branch as (*mut ::std::os::raw::c_void));
        }
        1i32
    }
}

#[no_mangle]
pub unsafe extern fn update_delproc(mut p : *mut ::hash::hashnode) {
    let mut li : *mut logfile_info = (*p).data as (*mut logfile_info);
    if li.is_null() {
    } else {
        if !(*li).tag.is_null() {
            free((*li).tag as (*mut ::std::os::raw::c_void));
        }
        if !(*li).rev_old.is_null() {
            free((*li).rev_old as (*mut ::std::os::raw::c_void));
        }
        if !(*li).rev_new.is_null() {
            free((*li).rev_new as (*mut ::std::os::raw::c_void));
        }
        free(li as (*mut ::std::os::raw::c_void));
    }
}

unsafe extern fn ci_delproc(mut p : *mut ::hash::hashnode) {
    let mut ci : *mut commit_info = (*p).data as (*mut commit_info);
    if !(*ci).rev.is_null() {
        free((*ci).rev as (*mut ::std::os::raw::c_void));
    }
    if !(*ci).tag.is_null() {
        free((*ci).tag as (*mut ::std::os::raw::c_void));
    }
    if !(*ci).options.is_null() {
        free((*ci).options as (*mut ::std::os::raw::c_void));
    }
    free(ci as (*mut ::std::os::raw::c_void));
}

unsafe extern fn masterlist_delproc(
    mut p : *mut ::hash::hashnode
) {
    let mut ml : *mut master_lists = (*p).data as (*mut master_lists);
    ::hash::dellist(&mut (*ml).ulist as (*mut *mut ::hash::hashlist));
    ::hash::dellist(&mut (*ml).cilist as (*mut *mut ::hash::hashlist));
    free(ml as (*mut ::std::os::raw::c_void));
}
