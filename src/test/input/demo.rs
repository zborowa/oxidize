
// fn looping(){
//     'loop2: loop {
//         if !(*v1 as (i32) == b'0' as (i32)) {
//             break;
//         }
//         v1 = v1.offset(1isize);
//     }
// }

fn looping2(){
    'loop56: loop {
        if !(i < argc) {
            break;
        }
        send_arg(*argv.offset(i as (isize)) as (*const u8));
        i = i + 1;
    }
}

// fn looping3(){
//     'loop2: loop{
//         if(true){
//             println!("Hello World!");
//         }
//     }
// }

// fn whileling(){
// 	while true{
// 		println!("Hello World!");
// 	}
// }
