use rand::{thread_rng, Rng};
use std::net::Ipv6Addr;

/// Generates a random IPv6 address.
pub(crate) fn gen_ipv6() -> String {
    let mut rng = thread_rng();
    let a = rng.r#gen::<u16>();
    let b = rng.r#gen::<u16>();
    let c = rng.r#gen::<u16>();
    let d = rng.r#gen::<u16>();
    let e = rng.r#gen::<u16>();
    let f = rng.r#gen::<u16>();
    let g = rng.r#gen::<u16>();
    let h = rng.r#gen::<u16>();
    Ipv6Addr::new(a, b, c, d, e, f, g, h).to_string()
}
