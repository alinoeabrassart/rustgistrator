extern crate shiplift;

use std::sync::mpsc::Sender;
use std::thread;
use self::shiplift::builder::EventFilter;
use self::shiplift::rep::Event;
use backend_manager::Event;

fn event_to_service(event: &Event) -> ServiceDeclaration {
    println!("{:?}", event);
    let event_type = match event.status {
        Ok(status) => match status {
            "start" => StartEvent,
            "stop" => StopEvent,
            _ => UnknownEvent,
        },
        None => UnknownEvent,
    }
    ServiceDeclaration {
        event_type: 
        id: String::from("???"),
        name: String::from("???"),
        tags: vec![String::from("???")],
        port: 0,
        address: String::from("???"),
    }
}

pub fn start_listener(back: Sender<ServiceEvent>) -> thread::JoinHandle<()> {
    thread::spawn(|| {
        let docker = shiplift::Docker::new();
        let filter_options = vec![
            EventFilter::Event(String::from("start"))
        ];
        let events_options = shiplift::builder::EventsOptionsBuilder::new()
            .filter(filter_options)
            .build();
        if let Ok(docker_events) = docker.events(&events_options) {
            for event in docker_events {
                back.register(
                    event_to_service(&event)
                );
            }
        }
    })
}

pub fn stop_listener(back: &BackendManager) -> thread::JoinHandle<()> {
    thread::spawn(|| {
        let docker = shiplift::Docker::new();
        let filter_options = vec![
            EventFilter::Event(String::from("die"))
        ];
        let events_options = shiplift::builder::EventsOptionsBuilder::new()
            .filter(filter_options)
            .build();
        if let Ok(docker_events) = docker.events(&events_options) {
            for event in docker_events {
                back.register(
                    event_to_service(&event)
                );
            }
        }
    })
}
