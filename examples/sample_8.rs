use core::panic;
use std::collections::HashSet;
use std::error::Error;
use std::sync::{Arc, RwLock};

fn main() -> Result<(), Box<dyn Error>> {
    let dogs: HashSet<_> = ["柴", "トイプードル"].iter().cloned().collect();
    let dogs = Arc::new(RwLock::new(dogs));

    // PoisonErrorを変換する便利関数
    fn stringify(x: impl ToString) -> String { x.to_string() }

    {
        let ds = dogs.read().map_err(stringify)?;
        assert!(ds.contains("柴"));
        assert!(ds.contains("トイプードル"));
    }

    dogs.write().map_err(stringify)?.insert("ブル・テリア");

    let dogs1 = Arc::clone(&dogs);
    
    // Poison Panic Pattern
    std::thread::spawn(move || {
        let _guard = dogs1.write();
        panic!();
    }).join().expect_err("");

    // Normal Pattern
    // std::thread::spawn(move ||
    //     dogs1.write().map(|mut ds| ds.insert("コーギー")).map_err(stringify)
    // ).join().expect("Thread error")?;

    assert!(dogs.read().map_err(stringify)?.contains("ブル・テリア"));
    assert!(dogs.read().map_err(stringify)?.contains("コーギー"));
    Ok(())
}
