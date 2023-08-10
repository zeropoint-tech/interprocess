#![cfg(unix)]

#[path = "../util/mod.rs"]
#[macro_use]
mod util;
use util::*;

mod credentials;
mod datagram;
mod stream;

#[test]
fn udsocket_stream() {
    use stream::*;
    install_color_eyre();
    run_with_namegen(NameGen::new(make_id!(), false));
    if cfg!(target_os = "linux") {
        run_with_namegen(NameGen::new(make_id!(), true));
    }
}

#[cfg(uds_cont_credentials)]
#[test]
fn udsocket_continuous_credentials() {
    use credentials::*;
    install_color_eyre();
    run(NameGen::new(make_id!(), false), true);
    if cfg!(target_os = "linux") {
        run(NameGen::new(make_id!(), true), true);
    }
}

#[cfg(uds_cmsgcred)]
#[test]
fn udsocket_explicitly_sent_credentials() {
    use credentials::*;
    install_color_eyre();
    run(NameGen::new(make_id!(), false), false);
    if cfg!(target_os = "linux") {
        run(NameGen::new(make_id!(), true), false);
    }
}

#[test]
fn udsocket_datagram() -> TestResult {
    use datagram::*;
    install_color_eyre();
    run(NameGen::new(make_id!(), false))?;
    if cfg!(target_os = "linux") {
        run(NameGen::new(make_id!(), true))?;
    }
    Ok(())
}
