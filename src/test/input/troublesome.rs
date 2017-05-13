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

//fn amb_lambda(){
//	do_op(|p| fs::remove_dir(p));
//}

//fn default() -> Cow<'a, B> {
//	Owned(<B as ToOwned>::Owned::default())
//}

//macro_rules! check {
//    ($(($name:expr, $val:expr),)*) => {
//        if value == "1" {
//            $(
//                if key == concat!("CFG_ENABLE_", $name) {
//                    $val = true;
//                    continue
//                }
//                if key == concat!("CFG_DISABLE_", $name) {
//                    $val = false;
//                    continue
//                }
//            )*
//        }
//    }
//}

//fn main(){
//	let rtf = r"{\rtf1\ansi\deff0{\fonttbl{\f0\fnil\fcharset0 Arial;}}\nowwrap\fs18";
//}

//        macro_rules! matches {
//            ( $($x:pat),+ ) => (
//                match rustc_tok.tok {
//                    $($x => match antlr_tok.tok {
//                        $x => {
//                            if !tok_cmp(&rustc_tok.tok, &antlr_tok.tok) {
//                                // FIXME #15677: needs more robust escaping in
//                                // antlr
//                                warn!("Different names for {:?} and {:?}", rustc_tok, antlr_tok);
//                            }
//                        }
//                        _ => panic!("{:?} is not {:?}", antlr_tok, rustc_tok)
//                    },)*
//                    ref c => assert!(c == &antlr_tok.tok, "{:?} is not {:?}", antlr_tok, rustc_tok)
//                }
//            )
//        }
