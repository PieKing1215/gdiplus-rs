use winapi::um::gdiplustypes::Status;

#[macro_export]
macro_rules! return_iferror {
    ( $code:expr ) => {{
        let res = unsafe { $code };
        if res != 0 {
            return Err(crate::Error::from(res));
        }
    }};
}

#[macro_export]
macro_rules! panic_iferror {
    ( $code:expr ) => {{
        let res = unsafe { $code };
        if res != 0 {
            panic!("{:?}", crate::Error::from(res));
        }
    }};
}
