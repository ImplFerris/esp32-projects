//! Demo test suite using embedded-test
//!
//! You can run this using `cargo test` as usual.

#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests]
mod tests {
    use defmt::assert_eq;
    use esp_hal as _;

    #[init]
    fn init() {
        let _ = esp_hal::init(esp_hal::Config::default());

        rtt_target::rtt_init_defmt!();
    }

    #[test]
    fn hello_test() {
        defmt::info!("Running test!");

        assert_eq!(1 + 1, 2);
    }
}
