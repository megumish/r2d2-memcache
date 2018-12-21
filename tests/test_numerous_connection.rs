extern crate memcache;
extern crate r2d2;
extern crate r2d2_memcache;

use r2d2_memcache::MemcacheConnectionManager;
use std::thread;

#[test]
fn get_thousand_data_parallel() {
    let manager = MemcacheConnectionManager::new("memcache://localhost:11211");
    let pool = r2d2::Pool::builder().max_size(15).build(manager).unwrap();

    pool.get().unwrap().flush().unwrap();

    let mut threads = vec![];
    for i in 0..1000 {
        let pool = pool.clone();
        let t = thread::spawn(move || {
            let mut conn = pool.get().unwrap();
            let value = format! {"{}{}","bar_thousand_",i};
            conn.set(&format!("{}{}", "foo_thousand_", i), value, 0)
                .unwrap();
        });
        threads.push(t);
    }
    for thread in threads {
        thread.join().unwrap();
    }

    let mut threads = vec![];
    for i in 0..1000 {
        let pool = pool.clone();
        let t = thread::spawn(move || {
            let value = format! {"{}{}","bar_thousand_",i};
            let mut conn = pool.get().unwrap();
            let result: String = conn
                .get(&format!("{}{}", "foo_thousand_", i))
                .unwrap()
                .unwrap();
            assert!(result == value);
        });
        threads.push(t);
    }
    for thread in threads {
        thread.join().unwrap();
    }
}
