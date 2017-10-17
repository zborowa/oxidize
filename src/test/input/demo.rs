
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

fn whileing1(){
	'while0: while true {
		println!("Hello World!");
	}
}

fn whileing2(){
	let w = 0;
	'while1: while w <= 9 {
		println!(w);
		w = w + 1;
	}
}

fn whileling3(){
    println!("Hello, ");

	while true{
		println!("magnificent ");
	}

    println!("world!");
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

fn ptr_der1(){
    if !p.is_null() { /* p is not null in this block */ }
}

fn ptr_der2(){
	let x = *p;
}

fn ptr_der3(){
    let q = &x as (*const _); // q is not null, by construction
}
