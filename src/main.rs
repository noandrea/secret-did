mod model;

use crate::model::{Did, DidDocument};

fn main() {
    let did = Did::new("cosmos", "net:cash:stuff");
    let mut doc = DidDocument::new(did);
    println!("new did is {}", doc);

    let sid = doc.add_didcomm_service("ws://something:3103");
    println!("added service      {}", sid);

    let vid = doc.add_verification_method(
        "6uxEqXYCV7zVykLaJ_r16ZRDZ91YnsT03WC2sW8zLpk",
        "X25519KeyAgreementKey2019",
        "F1609a268282e7fe247b9b6153a51196a90abcfc39611cef7f18f5511be1ba641",
        vec![],
    );
    println!("added verification {}", vid);

    let ctrl = Did::new_cosmos("net:cash:ctrl");
    doc.add_controller(&ctrl);
    println!("add controller {}", ctrl);
}
