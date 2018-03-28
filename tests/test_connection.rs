extern crate memcache;
extern crate r2d2;
extern crate r2d2_memcache;

use r2d2_memcache::MemcacheConnectionManager;

#[test]
fn connect() {
    let config = r2d2::Config::default();
    let manager = MemcacheConnectionManager::new("memcache://localhost:11211").unwrap();

    let pool = r2d2::Pool::new(config, manager).unwrap();
}

#[test]
fn flush() {
    let config = r2d2::Config::default();
    let manager = MemcacheConnectionManager::new("memcache://localhost:11211").unwrap();

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
    let manager = MemcacheConnectionManager::new("memcache://localhost:11211").unwrap();

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
    let manager = MemcacheConnectionManager::new("memcache://localhost:11211").unwrap();

    let pool = r2d2::Pool::new(config, manager).unwrap();

    {
        let pool = pool.clone();
        let mut conn = pool.get().unwrap();
        conn.flush().unwrap();
        let value = "bar";
        conn.add("foo", value, 10).unwrap();
        conn.set("foo", value, 10).unwrap();
        conn.replace("foo", value, 10).unwrap();
        conn.append("foo", value).unwrap();
        conn.prepend("foo", value).unwrap();
    }
}

#[test]
fn get() {
    let config = r2d2::Config::default();
    let manager = MemcacheConnectionManager::new("memcache://localhost:11211").unwrap();

    let pool = r2d2::Pool::new(config, manager).unwrap();
    {
        let pool = pool.clone();
        let mut conn = pool.get().unwrap();
        conn.flush().unwrap();
        let value = "bar";
        conn.set("foo_get", value, 10).unwrap();
        let result: String = conn.get("foo_get").unwrap().unwrap();
        assert!(result == "bar");
    }
}

#[test]
fn delete() {
    let config = r2d2::Config::default();
    let manager = MemcacheConnectionManager::new("memcache://localhost:11211").unwrap();

    let pool = r2d2::Pool::new(config, manager).unwrap();
    {
        let pool = pool.clone();
        let mut conn = pool.get().unwrap();
        conn.delete("foo").unwrap();
    }
}

#[test]
fn increment() {
    let config = r2d2::Config::default();
    let manager = MemcacheConnectionManager::new("memcache://localhost:11211").unwrap();

    let pool = r2d2::Pool::new(config, manager).unwrap();
    {
        let pool = pool.clone();
        let mut conn = pool.get().unwrap();
        conn.flush().unwrap();
        let value = 100;
        conn.set("foo_increment", value, 0).unwrap();
        assert_eq!(conn.increment("foo_increment", 1).unwrap(), 101);
    }
}

#[test]
fn decrement() {
    let config = r2d2::Config::default();
    let manager = MemcacheConnectionManager::new("memcache://localhost:11211").unwrap();

    let pool = r2d2::Pool::new(config, manager).unwrap();
    {
        let pool = pool.clone();
        let mut conn = pool.get().unwrap();
        conn.flush().unwrap();
        let value = 100;
        conn.set("foo_decrement", value, 0).unwrap();
        assert_eq!(conn.decrement("foo_decrement", 1).unwrap(), 99);
    }
}
