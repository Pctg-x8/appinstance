//! Lazy static, dropped value when application is exiting normally

#[macro_export]
macro_rules! AppInstance
{
    (__FnBody $t: ty = $e: expr) =>
    {{
        static mut OBJ: *mut $t = 0 as *mut _;
        static INIT: ::std::sync::Once = ::std::sync::Once::new();
        extern "C" fn dropper() { unsafe { ::std::mem::drop(Box::from_raw(OBJ)); } }

        INIT.call_once(|| unsafe { OBJ = Box::into_raw(Box::new($e)); ::libc::atexit(dropper); });
        unsafe { &*OBJ }
    }};
    (static $n: ident: $t: ty = $e: expr) =>
    {
        fn $n<'a>() -> &'a $t { AppInstance!(__FnBody $t = $e) }
    };
    (pub static $n: ident: $t: ty = $e: expr) =>
    {
        pub fn $n<'a>() -> &'a $t { AppInstance!(__FnBody $t = $e) }
    }
}
