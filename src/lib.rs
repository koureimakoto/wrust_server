

pub mod process_time;

#[cfg(test)]
mod test_config {
    use std::panic;

    fn no_panic_backtrace() {
        panic::set_hook(Box::new(|_|{}));
    }

    #[test]
    fn configure() {
        no_panic_backtrace();
    }
}