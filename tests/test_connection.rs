extern crate memcache;
extern crate r2d2;
extern crate r2d2_memcache;

use r2d2_memcache::MemcacheConnectionManager;

#[test]
fn connect() {
    let config = r2d2::Config::default();
    let manager = MemcacheConnectionManager::new("localhost:11211").unwrap();

    let pool = r2d2::Pool::new(config, manager).unwrap();
}

#[test]
fn flush() {
    let config = r2d2::Config::default();
    let manager = MemcacheConnectionManager::new("localhost:11211").unwrap();

    let pool = r2d2::Pool::new(config, manager).unwrap();

    {
        let pool = pool.clone();
        let mut conn = pool.get().unwrap();
        conn.flush().unwrap();
    }
}

#[test]
fn version() {
    let config = r2d2::Config::default();
    let manager = MemcacheConnectionManager::new("localhost:11211").unwrap();

    let pool = r2d2::Pool::new(config, manager).unwrap();

    {
        let pool = pool.clone();
        let mut conn = pool.get().unwrap();
        conn.version().unwrap();
    }
}

#[test]
fn store() {
    let config = r2d2::Config::default();
    let manager = MemcacheConnectionManager::new("localhost:11211").unwrap();

    let pool = r2d2::Pool::new(config, manager).unwrap();

    {
        let pool = pool.clone();
        let mut conn = pool.get().unwrap();
        let value = &memcache::Raw {
            bytes: b"bar",
            flags: 0,
        };
        conn.set("foo", value).unwrap();
        conn.replace("foo", value).unwrap();
        conn.add("foo", value).unwrap();
        conn.append("foo", value).unwrap();
        conn.prepend("foo", value).unwrap();
    }
}

#[test]
fn get() {
    let config = r2d2::Config::default();
    let manager = MemcacheConnectionManager::new("localhost:11211").unwrap();

    let pool = r2d2::Pool::new(config, manager).unwrap();
    {
        let pool = pool.clone();
        let mut conn = pool.get().unwrap();
        conn.flush().unwrap();
        let value = &memcache::Raw {
            bytes: b"bar",
            flags: 0,
        };
        conn.set("foo_get", value).unwrap();
        let result: (Vec<u8>, u16) = conn.get("foo_get").unwrap();
        assert!(result.0 == b"bar");
        assert!(result.1 == 0);
    }
}

#[test]
fn delete() {
    let config = r2d2::Config::default();
    let manager = MemcacheConnectionManager::new("localhost:11211").unwrap();

    let pool = r2d2::Pool::new(config, manager).unwrap();
    {
        let pool = pool.clone();
        let mut conn = pool.get().unwrap();
        conn.delete("foo").unwrap();
    }
}

#[test]
fn incr() {
    let config = r2d2::Config::default();
    let manager = MemcacheConnectionManager::new("localhost:11211").unwrap();

    let pool = r2d2::Pool::new(config, manager).unwrap();
    {
        let pool = pool.clone();
        let mut conn = pool.get().unwrap();
        let value = &memcache::Raw {
            bytes: b"100",
            flags: 0
        };
        conn.set("foo_incr", value).unwrap();
        assert!(conn.incr("foo_incr", 1).unwrap() == Some(101));
    }
}

#[test]
fn decr() {
    let config = r2d2::Config::default();
    let manager = MemcacheConnectionManager::new("localhost:11211").unwrap();

    let pool = r2d2::Pool::new(config, manager).unwrap();
    {
        let pool = pool.clone();
        let mut conn = pool.get().unwrap();
        let value = &memcache::Raw {
            bytes: b"100",
            flags: 0
        };
        conn.set("foo_decr", value).unwrap();
        assert!(conn.decr("foo_decr", 1).unwrap() == Some(99));
    }
}
