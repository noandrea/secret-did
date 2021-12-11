use std::collections::{HashMap, HashSet};
use std::fmt;

fn hash(data: &str) -> String {
    let h = blake3::hash(data.as_bytes());
    h.to_hex().to_string()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Did {
    method: String,
    id: String,
}

impl Did {
    pub fn new(method: &str, id: &str) -> Self {
        Did {
            method: String::from(method),
            id: String::from(id),
        }
    }

    pub fn new_cosmos(id: &str) -> Self {
        Did::new("cosmos", id)
    }
}

impl fmt::Display for Did {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "did:{}:{}", self.method, self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct VerificationMethod {
    id: String,
    typ: String,
    controller: String,
    material: String,
}

impl VerificationMethod {
    pub fn new(id: &str, typ: &str, controller: &str, material: &str) -> Self {
        VerificationMethod {
            id: String::from(id),
            typ: String::from(typ),
            controller: String::from(controller),
            material: String::from(material),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Service {
    id: String,
    endpoint: String,
    typ: String,
}

impl Service {
    pub fn new(id: &str, endpoint: &str, typ: &str) -> Self {
        Service {
            id: String::from(id),
            endpoint: String::from(endpoint),
            typ: String::from(typ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DidDocument {
    did: Did,
    context: Vec<String>,
    controllers: Vec<Did>,
    verification_methods: HashMap<String, VerificationMethod>,
    verification_relationships: HashMap<String, HashSet<String>>,
    services: HashMap<String, Service>,
}

impl DidDocument {
    pub fn new(did: Did) -> Self {
        DidDocument {
            did,
            context: vec![],
            controllers: vec![],
            verification_methods: HashMap::new(),
            verification_relationships: HashMap::new(),
            services: HashMap::new(),
        }
    }

    pub fn did(&self) -> String {
        self.did.to_string()
    }

    /**
     * Add Did comm service
     */
    pub fn add_didcomm_service(&mut self, endpoint: &str) -> String {
        let sid = format!("{}/service#{}", self.did(), hash(endpoint));
        self.services.insert(
            String::from(&sid),
            Service::new(&sid, endpoint, "DIDCommMessaging"),
        );
        sid
    }

    /**
     * Add verification method
     */
    pub fn add_verification_method(
        &mut self,
        id: &str,
        typ: &str,
        material: &str,
        relationships: Vec<String>,
    ) -> String {
        let vid = format!("{}#{}", self.did(), id);
        self.verification_methods.insert(
            String::from(&vid),
            VerificationMethod::new(&vid, typ, &self.did(), material),
        );
        relationships
            .iter()
            .for_each(|name| match self.verification_relationships.get_mut(name) {
                Some(hs) => {
                    hs.insert(String::from(&vid));
                }
                None => {
                    let mut hs = HashSet::new();
                    hs.insert(String::from(&vid));
                    self.verification_relationships
                        .insert(String::from(name), hs);
                }
            });
        vid
    }

    /**
     * Add a controller for the
     */
    pub fn add_controller(&mut self, did: &Did) {
        self.controllers.push(did.clone())
    }
}

impl fmt::Display for DidDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.did())
    }
}
