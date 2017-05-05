//fn pe_macro_expr(){
//	macro_rules! check {
//	($(($name:expr, $val:expr),)*) => {
//	    if value == "1" {
//	        $(
//	            if key == concat!("CFG_ENABLE_", $name) {
//	                $val = true;
//	                continue
//	            }
//	            if key == concat!("CFG_DISABLE_", $name) {
//	                    $val = false;
//	                    continue
//	                }
//	            )*
//	        }
//	    }
//	}
//}

// Solved by `> Expression "&" !>> "&" Expression`
//fn amb_andand(){
//	if true && false {
//        {}
//    }
//}

fn amb_lambda(){
	do_op(|p| fs::remove_dir(p));
}
