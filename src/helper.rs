
pub fn is_debug_mode() -> bool {
    return cfg!(debug_assertions);
}

pub fn debug_run<F>(f: F)
where
    F: Fn() -> (),
{
    if cfg!(debug_assertions) {
        f()
    }
}
