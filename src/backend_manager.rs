extern crate itertools;

#[derive(Debug)]
pub struct ServiceDeclaration {
    pub id: String,
    pub name: String,
    pub tags: Vec<String>,
    pub port: u32,
    pub address: String,
}

pub trait Backend {
    fn register_service(&self, service: &ServiceDeclaration) -> Result<(), String>;
    fn deregister_service(&self, service: &ServiceDeclaration) -> Result<(), String>;
}

pub struct BackendManager {
    pub backends: Vec<Box<Backend>>,
}

impl BackendManager {
    pub fn register(&self, service: ServiceDeclaration) -> Result<Vec<()>, String> {
        self.backends.iter()
            .map(|back| { back.register_service(&service) })
            .collect()
    }
    pub fn deregister(&self, service:ServiceDeclaration) -> Result<Vec<()>, String> {
        self.backends.iter()
            .map(|back| { back.deregister_service(&service) })
            .collect()
    }
}

pub struct FakeBackend {
    pub name: String,
    pub failing: bool,
}

impl Backend for FakeBackend {
    fn register_service(&self, service: &ServiceDeclaration) -> Result<(), String> {
        if self.failing {
            println!("Failing fake backend {} cannot register {:?}", self.name, service);
            Err(String::from("Cannot register"))
        } else {
            println!("Fake backend {} register {:?}", self.name, service);
            Ok(())
        }
    }

    fn deregister_service(&self, service: &ServiceDeclaration) -> Result<(), String> {
        println!("Fake backend {} deregister {:?}", self.name, service);
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_successful_service_registration() {
        let fake_service = ServiceDeclaration{
            id: String::from("fake_id"),
            name: String::from("fake_name"),
            tags: vec![],
            port: 1999,
            address: String::from("fake_address"),
        };
        let fake1 = FakeBackend{
            name: String::from("Fake1"),
            failing: false,
        };
        let fake2 = FakeBackend{
            name: String::from("Fake2"),
            failing: false,
        };
        let backends = BackendManager{
            backends: vec![Box::new(fake1), Box::new(fake2)],
        };
        let res = backends.register(fake_service);
        assert_eq!(res, Ok(vec![(), ()]));
    }

    #[test]
    fn test_failing_service_registration() {
        let fake_service = ServiceDeclaration{
            id: String::from("fake_id"),
            name: String::from("fake_name"),
            tags: vec![],
            port: 1999,
            address: String::from("fake_address"),
        };
        let fake3 = FakeBackend{
            name: String::from("Fake3"),
            failing: true,
        };
        let fake1 = FakeBackend{
            name: String::from("Fake1"),
            failing: false,
        };
        let backends = BackendManager{
            backends: vec![Box::new(fake1), Box::new(fake3)],
        };
        let res = backends.register(fake_service);
        assert_eq!(res, Err(String::from("Cannot register")));
    }

}
