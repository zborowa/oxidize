
fn looping(){
    'loop0: loop {
        if !(*v1 as (i32) == b'0' as (i32)) {
            break;
        }
        v1 = v1.offset(1isize);
    }
}

fn looping2(){
    'loop1: loop {
        if !(i < argc) {
            break;
        }
        send_arg(*argv.offset(i as (isize)) as (*const u8));
        i = i + 1;
    }
}

fn empty_if(){
    if true{}
}

fn looping3(){
    'loop2: loop{
        if true{
            println!("Hello World!");
        }
    }
}

fn whileling(){
	while true{
		println!("Hello World!");
	}
}

fn null_checking(){
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
}