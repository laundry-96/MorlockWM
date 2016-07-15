#[macro_use]
extern crate wayland_client;

wayland_env!(WaylandEnv,
             );

fn main() {
    use wayland_client::wayland::get_display;

    let display = match get_display() {
        Some(d) => d,
        None    => panic!("Unable to connect to a wayland server!")
    };

    let (env, events) = WaylandEnv::init(display);

    for &(id, ref interface, version) in &env.globals {
        println!("{: >4} {} {}", id, interface, version);
    }
}
