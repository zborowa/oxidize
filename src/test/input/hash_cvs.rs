extern {
    fn __assert_fail(
        __assertion : *const u8,
        __file : *const u8,
        __line : u32,
        __function : *const u8
    );
    fn free(__ptr : *mut ::std::os::raw::c_void);
    fn memset(
        __s : *mut ::std::os::raw::c_void, __c : i32, __n : usize
    ) -> *mut ::std::os::raw::c_void;
    fn qsort(
        __base : *mut ::std::os::raw::c_void,
        __nmemb : usize,
        __size : usize,
        __compar
        :
        unsafe extern fn(*const ::std::os::raw::c_void, *const ::std::os::raw::c_void) -> i32
    );
    fn strcmp(__s1 : *const u8, __s2 : *const u8) -> i32;
    fn xalloc_die();
    fn xmalloc(s : usize) -> *mut ::std::os::raw::c_void;
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum ntype {
    NT_UNKNOWN,
    HEADER,
    ENTRIES,
    FILES,
    LIST,
    RCSNODE,
    RCSVERS,
    DIRS,
    UPDATE,
    LOCK,
    NDBMNODE,
    FILEATTR,
    VARIABLE,
    RCSFIELD,
    RCSCMPFLD,
    RCSSTRING,
}

#[derive(Copy)]
#[repr(C)]
pub struct hashnode {
    pub type_ : ntype,
    pub next : *mut hashnode,
    pub prev : *mut hashnode,
    pub hashnext : *mut hashnode,
    pub hashprev : *mut hashnode,
    pub key : *mut u8,
    pub data : *mut ::std::os::raw::c_void,
    pub len : usize,
    pub delproc : Option<unsafe extern fn(*mut hashnode)>,
}

impl Clone for hashnode {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct hashlist {
    pub list : *mut hashnode,
    pub hasharray : [*mut hashnode; 151],
    pub next : *mut hashlist,
}

impl Clone for hashlist {
    fn clone(&self) -> Self { *self }
}

static mut listcache
    : *mut hashlist
    = 0i32 as (*mut ::std::os::raw::c_void) as (*mut hashlist);

static mut nodecache
    : *mut hashnode
    = 0i32 as (*mut ::std::os::raw::c_void) as (*mut hashnode);

#[no_mangle]
pub unsafe extern fn getlist() -> *mut hashlist {
    let mut i : i32;
    let mut list : *mut hashlist;
    let mut node : *mut hashnode;
    if listcache != 0i32 as (*mut ::std::os::raw::c_void) as (*mut hashlist) {
        list = listcache;
        listcache = (*listcache).next as (*mut hashlist);
        (*list).next = 0i32 as (*mut ::std::os::raw::c_void) as (*mut hashlist);
        i = 0i32;
        'loop3: loop {
            if !(i < 151i32) {
                break;
            }
            (*list).hasharray[
                i as (usize)
            ] = 0i32 as (*mut ::std::os::raw::c_void) as (*mut hashnode);
            i = i + 1;
        }
    } else {
        list = xmalloc(
                   ::std::mem::size_of::<hashlist>()
               ) as (*mut hashlist);
        memset(
            list as (*mut ::std::os::raw::c_void),
            0i32,
            ::std::mem::size_of::<hashlist>()
        );
        node = getnode();
        (*list).list = node;
        (*node).type_ = ntype::HEADER;
        (*node).next = {
                           (*node).prev = node as (*mut hashnode);
                           (*node).prev
                       };
    }
    list
}

#[no_mangle]
pub unsafe extern fn dellist(mut listp : *mut *mut hashlist) {
    let mut i : i32;
    let mut p : *mut hashnode;
    let mut tmp : *mut hashlist;
    if *listp == 0i32 as (*mut ::std::os::raw::c_void) as (*mut hashlist) {
    } else {
        tmp = *listp;
        *listp = 0i32 as (*mut ::std::os::raw::c_void) as (*mut hashlist);
        p = (*tmp).list;
        'loop2: loop {
            if !((*p).next as (*mut ::std::os::raw::c_void) != p as (*mut ::std::os::raw::c_void)) {
                break;
            }
            delnode((*p).next as (*mut hashnode));
        }
        freenode_mem(p);
        i = 0i32;
        'loop4: loop {
            if !(i < 151i32) {
                break;
            }
            if {
                   p = (*tmp).hasharray[i as (usize)];
                   p
               } != 0i32 as (*mut ::std::os::raw::c_void) as (*mut hashnode) {
                (*p).type_ = ntype::NT_UNKNOWN;
                (*p).next = nodecache as (*mut hashnode);
                nodecache = p;
            }
            i = i + 1;
        }
        (*tmp).next = listcache as (*mut hashlist);
        listcache = tmp;
    }
}

#[no_mangle]
pub unsafe extern fn removenode(mut p : *mut hashnode) {
    if p.is_null() {
    } else {
        (*(*p).next).prev = (*p).prev;
        (*(*p).prev).next = (*p).next;
        if !(*p).hashnext.is_null() {
            (*(*p).hashnext).hashprev = (*p).hashprev;
            (*(*p).hashprev).hashnext = (*p).hashnext;
        }
    }
}

#[no_mangle]
pub unsafe extern fn mergelists(
    mut dest : *mut hashlist, mut src : *mut *mut hashlist
) {
    let mut head : *mut hashnode;
    let mut p : *mut hashnode;
    let mut n : *mut hashnode;
    head = (**src).list;
    p = (*head).next as (*mut hashnode);
    'loop1: loop {
        if !(p != head) {
            break;
        }
        n = (*p).next as (*mut hashnode);
        removenode(p);
        if addnode(dest,p) == -1i32 {
            freenode(p);
        }
        p = n;
    }
    dellist(src);
}

#[no_mangle]
pub unsafe extern fn getnode() -> *mut hashnode {
    let mut p : *mut hashnode;
    if nodecache != 0i32 as (*mut ::std::os::raw::c_void) as (*mut hashnode) {
        p = nodecache;
        nodecache = (*p).next as (*mut hashnode);
    } else {
        p = xmalloc(::std::mem::size_of::<hashnode>()) as (*mut hashnode);
    }
    memset(
        p as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<hashnode>()
    );
    (*p).type_ = ntype::NT_UNKNOWN;
    p
}

#[no_mangle]
pub unsafe extern fn delnode(mut p : *mut hashnode) {
    if p.is_null() {
    } else {
        removenode(p);
        freenode(p);
    }
}

unsafe extern fn freenode_mem(mut p : *mut hashnode) {
    if let Some(delproc) = (*p).delproc {
        delproc(p as (*mut hashnode));
    } else if (*p).data != 0i32 as (*mut ::std::os::raw::c_void) {
        free((*p).data);
    }
    if (*p).key != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        free((*p).key as (*mut ::std::os::raw::c_void));
    }
    (*p).key = {
                   (*p).data = 0i32 as (*mut ::std::os::raw::c_void);
                   (*p).data
               } as (*mut u8);
    (*p).delproc = None;
}

#[no_mangle]
pub unsafe extern fn freenode(mut p : *mut hashnode) {
    freenode_mem(p);
    (*p).type_ = ntype::NT_UNKNOWN;
    (*p).next = nodecache as (*mut hashnode);
    nodecache = p;
}

unsafe extern fn hashp(mut key : *const u8) -> i32 {
    let mut h : u32 = 0u32;
    let mut g : u32;
    if key != 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) {
        0i32;
    } else {
        __assert_fail(
            (*b"key != NULL\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"hashp\0").as_ptr()
        );
    }
    'loop1: loop {
        if !(*key as (i32) != 0i32) {
            break;
        }
        let mut c
            : u32
            = *{
                   let _old = key;
                   key = key.offset(1isize);
                   _old
               } as (u32);
        h = (h << 4i32).wrapping_add(c);
        if !({
                 g = h & 0xf0000000u32;
                 g
             } != 0u32) {
            continue;
        }
        h = h ^ g >> 24i32 ^ g;
    }
    h.wrapping_rem(151u32) as (i32)
}

unsafe extern fn streq(
    mut a : *const u8, mut b : *const u8
) -> bool {
    *a as (i32) == *b as (i32) && (strcmp(a,b) == 0i32)
}

#[no_mangle]
pub unsafe extern fn insert_before(
    mut list : *mut hashlist,
    mut marker : *mut hashnode,
    mut p : *mut hashnode
) -> i32 {
    let mut _currentBlock;
    if (*p).key != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        let mut hashval : i32;
        let mut q : *mut hashnode;
        hashval = hashp((*p).key as (*const u8));
        if (*list).hasharray[
               hashval as (usize)
           ] == 0i32 as (*mut ::std::os::raw::c_void) as (*mut hashnode) {
            q = getnode();
            (*q).type_ = ntype::HEADER;
            (*list).hasharray[hashval as (usize)] = {
                                                        (*q).hashnext = {
                                                                            (*q).hashprev = q as (*mut hashnode);
                                                                            (*q).hashprev
                                                                        };
                                                        (*q).hashnext
                                                    } as (*mut hashnode);
        }
        q = (*(*list).hasharray[
                  hashval as (usize)
              ]).hashnext as (*mut hashnode);
        'loop4: loop {
            if !(q != (*list).hasharray[hashval as (usize)]) {
                _currentBlock = 5;
                break;
            }
            if streq((*p).key as (*const u8),(*q).key as (*const u8)) {
                _currentBlock = 9;
                break;
            }
            q = (*q).hashnext as (*mut hashnode);
        }
        if _currentBlock == 5 {
            q = (*list).hasharray[hashval as (usize)];
            (*p).hashprev = (*q).hashprev;
            (*p).hashnext = q as (*mut hashnode);
            (*(*p).hashprev).hashnext = p as (*mut hashnode);
            (*q).hashprev = p as (*mut hashnode);
        } else {
            return -1i32;
        }
    }
    (*p).next = marker as (*mut hashnode);
    (*p).prev = (*marker).prev;
    (*(*marker).prev).next = p as (*mut hashnode);
    (*marker).prev = p as (*mut hashnode);
    0i32
}

#[no_mangle]
pub unsafe extern fn addnode(
    mut list : *mut hashlist, mut p : *mut hashnode
) -> i32 {
    insert_before(list,(*list).list,p)
}

#[no_mangle]
pub unsafe extern fn addnode_at_front(
    mut list : *mut hashlist, mut p : *mut hashnode
) -> i32 {
    insert_before(list,(*(*list).list).next as (*mut hashnode),p)
}

#[no_mangle]
pub unsafe extern fn findnode(
    mut list : *mut hashlist, mut key : *const u8
) -> *mut hashnode {
    let mut _currentBlock;
    let mut head : *mut hashnode;
    let mut p : *mut hashnode;
    if !key.is_null() {
        0i32;
    } else {
        __assert_fail(
            (*b"key\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"findnode\0").as_ptr()
        );
    }
    if list_isempty(list) != 0 {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut hashnode)
    } else {
        head = (*list).hasharray[hashp(key) as (usize)];
        (if head.is_null() {
             0i32 as (*mut ::std::os::raw::c_void) as (*mut hashnode)
         } else {
             p = (*head).hashnext as (*mut hashnode);
             'loop3: loop {
                 if !(p != head) {
                     _currentBlock = 4;
                     break;
                 }
                 if streq((*p).key as (*const u8),key) {
                     _currentBlock = 7;
                     break;
                 }
                 p = (*p).hashnext as (*mut hashnode);
             }
             (if _currentBlock == 4 {
                  0i32 as (*mut ::std::os::raw::c_void) as (*mut hashnode)
              } else {
                  p
              })
         })
    }
}

#[no_mangle]
pub unsafe extern fn findnode_fn(
    mut list : *mut hashlist, mut key : *const u8
) -> *mut hashnode {
    let mut _currentBlock;
    let mut head : *mut hashnode;
    let mut p : *mut hashnode;
    if !key.is_null() {
        0i32;
    } else {
        __assert_fail(
            (*b"key\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"findnode_fn\0").as_ptr()
        );
    }
    if list_isempty(list) != 0 {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut hashnode)
    } else {
        head = (*list).hasharray[hashp(key) as (usize)];
        (if head.is_null() {
             0i32 as (*mut ::std::os::raw::c_void) as (*mut hashnode)
         } else {
             p = (*head).hashnext as (*mut hashnode);
             'loop3: loop {
                 if !(p != head) {
                     _currentBlock = 4;
                     break;
                 }
                 if strcmp((*p).key as (*const u8),key) == 0i32 {
                     _currentBlock = 7;
                     break;
                 }
                 p = (*p).hashnext as (*mut hashnode);
             }
             (if _currentBlock == 4 {
                  0i32 as (*mut ::std::os::raw::c_void) as (*mut hashnode)
              } else {
                  p
              })
         })
    }
}

#[no_mangle]
pub unsafe extern fn walklist(
    mut list : *mut hashlist,
    mut
    proc_
    :
    unsafe extern fn(*mut hashnode, *mut ::std::os::raw::c_void) -> i32,
    mut closure : *mut ::std::os::raw::c_void
) -> i32 {
    let mut head : *mut hashnode;
    let mut p : *mut hashnode;
    let mut err : i32 = 0i32;
    if list.is_null() {
        0i32
    } else {
        head = (*list).list;
        p = (*head).next as (*mut hashnode);
        'loop2: loop {
            if !(p != head) {
                break;
            }
            let mut next : *mut hashnode = (*p).next as (*mut hashnode);
            err = err + proc_(p,closure);
            p = next;
        }
        err
    }
}

#[no_mangle]
pub unsafe extern fn list_isempty(mut list : *mut hashlist) -> i32 {
    (list == 0i32 as (*mut ::std::os::raw::c_void) as (*mut hashlist) || (*(*list).list).next as (*mut ::std::os::raw::c_void) == (*list).list as (*mut ::std::os::raw::c_void)) as (i32)
}

static mut client_comp
    : Option<unsafe extern fn(*const hashnode, *const hashnode) -> i32>
    = None;

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

unsafe extern fn qsort_comp(
    mut elem1 : *const ::std::os::raw::c_void,
    mut elem2 : *const ::std::os::raw::c_void
) -> i32 {
    let mut node1 : *mut *mut hashnode = elem1 as (*mut *mut hashnode);
    let mut node2 : *mut *mut hashnode = elem2 as (*mut *mut hashnode);
    client_comp.unwrap()(
        *node1 as (*const hashnode),
        *node2 as (*const hashnode)
    )
}

#[no_mangle]
pub unsafe extern fn sortlist(
    mut list : *mut hashlist,
    mut
    comp
    :
    Option<unsafe extern fn(*const hashnode, *const hashnode) -> i32>
) {
    let mut head : *mut hashnode;
    let mut remain : *mut hashnode;
    let mut p : *mut hashnode;
    let mut array : *mut *mut hashnode;
    let mut i : i32;
    let mut n : i32;
    if list == 0i32 as (*mut ::std::os::raw::c_void) as (*mut hashlist) {
    } else {
        head = (*list).list;
        remain = (*head).next as (*mut hashnode);
        n = 0i32;
        p = remain;
        'loop2: loop {
            if !(p != head) {
                break;
            }
            n = n + 1;
            p = (*p).next as (*mut hashnode);
        }
        array = xnmalloc(
                    n as (usize),
                    ::std::mem::size_of::<*mut hashnode>()
                ) as (*mut *mut hashnode);
        i = 0i32;
        p = remain;
        'loop4: loop {
            if !(p != head) {
                break;
            }
            *array.offset(
                 {
                     let _old = i;
                     i = i + 1;
                     _old
                 } as (isize)
             ) = p;
            p = (*p).next as (*mut hashnode);
        }
        client_comp = comp;
        qsort(
            array as (*mut ::std::os::raw::c_void),
            n as (usize),
            ::std::mem::size_of::<*mut hashnode>(),
            qsort_comp
        );
        (*head).next = {
                           (*head).prev = head as (*mut hashnode);
                           (*head).prev
                       };
        i = 0i32;
        'loop6: loop {
            if !(i < n) {
                break;
            }
            p = *array.offset(i as (isize));
            (*p).next = head as (*mut hashnode);
            (*p).prev = (*head).prev;
            (*(*p).prev).next = p as (*mut hashnode);
            (*head).prev = p as (*mut hashnode);
            i = i + 1;
        }
        free(array as (*mut ::std::os::raw::c_void));
    }
}

#[no_mangle]
pub unsafe extern fn fsortcmp(
    mut p : *const hashnode, mut q : *const hashnode
) -> i32 {
    strcmp((*p).key as (*const u8),(*q).key as (*const u8))
}
