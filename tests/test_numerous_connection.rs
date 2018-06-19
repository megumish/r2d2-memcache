extern crate memcache;
extern crate r2d2;
extern crate r2d2_memcache;

use r2d2_memcache::MemcacheConnectionManager;
use std::thread;

#[test]
fn get_thousand_data_parallel() {
    let config = r2d2::Config::default();
    let manager = MemcacheConnectionManager::new("memcache://localhost:11211");

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
            let value = format!{"{}{}","bar_thousand",i};
            conn.set(&format!("{}{}", "foo_thousand", i), value, 0)
                .unwrap();
        });
    }
    for i in 0..1000 {
        let pool = pool.clone();
        thread::spawn(move || {
            let value = format!{"{}{}","bar_thousand",i};
            let mut conn = pool.get().unwrap();
            let result: String = conn.get(&format!("{}{}", "foo_thousand", i))
                .unwrap()
                .unwrap();
            assert!(result == value);
        });
    }
}
