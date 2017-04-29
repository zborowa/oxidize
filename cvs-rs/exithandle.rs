extern {
    fn SIG_beginCrSect();
    fn SIG_register(
        sig : i32, sigcleanup : unsafe extern fn(i32)
    ) -> i32;
    fn atexit(__func : unsafe extern fn()) -> i32;
}

#[no_mangle]
pub unsafe extern fn signals_register(
    mut handler : unsafe extern fn(i32)
) {
    SIG_register(6i32,handler);
    SIG_register(1i32,handler);
    SIG_register(2i32,handler);
    SIG_register(3i32,handler);
    SIG_register(13i32,handler);
    SIG_register(15i32,handler);
}

#[no_mangle]
pub unsafe extern fn cleanup_register(
    mut handler : unsafe extern fn()
) {
    atexit(handler);
    atexit(SIG_beginCrSect);
}
