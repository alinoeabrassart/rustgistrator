extern crate shiplift;

// use std::sync::mpsc::Sender;
// use std::thread;
// use self::shiplift::builder::EventFilter;
use self::shiplift::rep::Event;

use backend_manager::ServiceDeclaration;

// listen to docker daemon and convert daemon
// add callbacks on event

#[derive(Debug, PartialEq)]
enum EventType {
    Start,
    Stop,
    Unknown
}

#[derive(Debug, PartialEq)]
struct ServiceEvent {
    event: EventType,
    service: Option<ServiceDeclaration>
}

fn event_to_declaration(event: &Event) -> Option<ServiceDeclaration> {
    Some(ServiceDeclaration {
        id: event.clone().id?,
        name: String::from("???"),
        tags: vec![String::from("???")],
        port: 0,
        address: String::from("???")
    })
}

fn event_to_service(event: &Event) -> ServiceEvent {
    let mut e_t = EventType::Unknown;
    match event.status.as_ref().map(|s| s.as_str()) {
        Some("start") => e_t = EventType::Start,
        Some("stop") => e_t = EventType::Stop,
        _ => e_t = EventType::Unknown
    }
    if e_t != EventType::Unknown {
        ServiceEvent {
            event: e_t,
            service: event_to_declaration(event)
        }
    } else {
        ServiceEvent {
            event: e_t,
            service: None
        }
    }
}

// pub fn start_listener(back: Sender<ServiceEvent>) -> thread::JoinHandle<()> {
//     thread::spawn(|| {
//         let docker = shiplift::Docker::new();
//         let filter_options = vec![
//             EventFilter::Event(String::from("start"))
//         ];
//         let events_options = shiplift::builder::EventsOptionsBuilder::new()
//             .filter(filter_options)
//             .build();
//         if let Ok(docker_events) = docker.events(&events_options) {
//             for event in docker_events {
//                 back.register(
//                     event_to_service(&event)
//                 );
//             }
//         }
//     })
// }
// 
// pub fn stop_listener(back: &BackendManager) -> thread::JoinHandle<()> {
//     thread::spawn(|| {
//         let docker = shiplift::Docker::new();
//         let filter_options = vec![
//             EventFilter::Event(String::from("die"))
//         ];
//         let events_options = shiplift::builder::EventsOptionsBuilder::new()
//             .filter(filter_options)
//             .build();
//         if let Ok(docker_events) = docker.events(&events_options) {
//             for event in docker_events {
//                 back.register(
//                     event_to_service(&event)
//                 );
//             }
//         }
//     })
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_event_conversion() {
        let docker = shiplift::Docker::new();
        let container_id = "fdd9ade7b2b6fc585e1d7511f0edd45933069038bea76240ae03bb5fd1211bc5";
        let docker_event = Event {
            status: Some(String::from("start")),
            id: Some(String::from(container_id)),
            from: Some(String::from("ubuntu")),
            time: 1525205611,
            timeNano: 1525205611444893878
        };
        let service_res = ServiceDeclaration {
            id: String::from(container_id),
            name: String::from("???"),
            tags: vec![String::from("???")],
            port: 0,
            address: String::from("???"),
        };
        let res = ServiceEvent {
            event: EventType::Start,
            service: Some(service_res)
        };
        assert_eq!(res, event_to_service(&docker_event));
    }

}
