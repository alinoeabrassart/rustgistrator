/// convert a container description to a service
/// for consul
extern crate consul;

use docker_iface;
use backend_manager::FakeBackend;
use backend_manager::BackendManager;

pub fn start_to_register() {
    let fake1 = FakeBackend {
        name: String::from("Fake backend"),
        failing: false,
    };
    let backends = BackendManager{
        backends: vec![Box::new(fake1)],
    };
    let start_thread = docker_iface::start_listener(&backends);
    docker_iface::stop_listener(&backends);
    start_thread.join().unwrap();
}
