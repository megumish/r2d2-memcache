extern crate memcache;
extern crate r2d2;
extern crate r2d2_memcache;

use r2d2_memcache::MemcacheConnectionManager;
use std::thread;

#[test]
fn get_thousand_data_parallel() {
    let config = r2d2::Config::default();
    let manager = MemcacheConnectionManager::new("localhost:11211").unwrap();

    let pool = r2d2::Pool::new(config, manager).unwrap();
    {
        let pool = pool.clone();
        let mut conn = pool.get().unwrap();
        conn.flush().unwrap();
    }
    for i in 0..1000 {
        let pool = pool.clone();
        thread::spawn(move || {
            let mut conn = pool.get().unwrap();
            conn.flush().unwrap();
            let bytes = format!{"{}{}","bar_thousand",i};
            let bytes = bytes.as_bytes();
            let value = &memcache::Raw {
                bytes: bytes.clone(),
                flags: 0,
            };
            conn.set(&format!("{}{}","foo_thousand", i), value).unwrap();
        });
    }
    for i in 0..1000 {
        let pool = pool.clone();
        thread::spawn(move || {
            let bytes = format!{"{}{}","bar_thousand",i};
            let bytes = bytes.as_bytes();
            let mut conn = pool.get().unwrap();
            let result: (Vec<u8>, u16) = conn.get(&format!("{}{}","foo_thousand", i)).unwrap();
            assert!(result.0 == bytes);
            assert!(result.1 == 0);
        });
    }
}

