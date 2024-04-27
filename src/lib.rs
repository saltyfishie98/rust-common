use std::process::exit;

pub trait ResultExt<T, E> {
    fn error_out_with(self, msg: impl FnOnce(&E)) -> T;
    fn expect_with(self, msg: impl FnOnce(&E)) -> T;
}

impl<T: Default, E> ResultExt<T, E> for Result<T, E> {
    fn error_out_with(self, msg: impl FnOnce(&E)) -> T {
        match self {
            Ok(o) => o,
            Err(e) => {
                msg(&e);
                exit(1);
            }
        }
    }

    #[track_caller]
    fn expect_with(self, msg: impl FnOnce(&E)) -> T {
        match self {
            Ok(o) => o,
            Err(e) => {
                msg(&e);
                panic!();
            }
        }
    }
}
