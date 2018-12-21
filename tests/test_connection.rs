extern crate memcache;
extern crate r2d2;
extern crate r2d2_memcache;

use r2d2_memcache::MemcacheConnectionManager;

#[test]
fn connect() {
    let manager = MemcacheConnectionManager::new("memcache://localhost:11211");
    r2d2::Pool::builder().max_size(15).build(manager).unwrap();
}

#[test]
fn flush() {
    let manager = MemcacheConnectionManager::new("memcache://localhost:11211");
    let pool = r2d2::Pool::builder().max_size(15).build(manager).unwrap();

    let mut conn = pool.get().unwrap();
    conn.flush().unwrap();
}

#[test]
fn version() {
    let manager = MemcacheConnectionManager::new("memcache://localhost:11211");
    let pool = r2d2::Pool::builder().max_size(15).build(manager).unwrap();

    let mut conn = pool.get().unwrap();
    conn.version().unwrap();
}

#[test]
fn store() {
    let manager = MemcacheConnectionManager::new("memcache://localhost:11211");
    let pool = r2d2::Pool::builder().max_size(15).build(manager).unwrap();

    let mut conn = pool.get().unwrap();
    conn.flush().unwrap();
    let value = "bar";
    conn.add("foo", value, 10).unwrap();
    conn.set("foo", value, 10).unwrap();
    conn.replace("foo", value, 10).unwrap();
    conn.append("foo", value).unwrap();
    conn.prepend("foo", value).unwrap();
}

#[test]
fn get() {
    let manager = MemcacheConnectionManager::new("memcache://localhost:11211");
    let pool = r2d2::Pool::builder().max_size(15).build(manager).unwrap();

    let mut conn = pool.get().unwrap();
    conn.flush().unwrap();
    let value = "bar";
    conn.set("foo_get", value, 10).unwrap();
    let result: String = conn.get("foo_get").unwrap().unwrap();
    assert!(result == "bar");
}

#[test]
fn delete() {
    let manager = MemcacheConnectionManager::new("memcache://localhost:11211");
    let pool = r2d2::Pool::builder().max_size(15).build(manager).unwrap();

    let mut conn = pool.get().unwrap();
    conn.delete("foo").unwrap();
}

#[test]
fn increment() {
    let manager = MemcacheConnectionManager::new("memcache://localhost:11211");
    let pool = r2d2::Pool::builder().max_size(15).build(manager).unwrap();

    let mut conn = pool.get().unwrap();
    conn.flush().unwrap();
    let value = 100;
    conn.set("foo_increment", value, 0).unwrap();
    assert_eq!(conn.increment("foo_increment", 1).unwrap(), 101);
}

#[test]
fn decrement() {
    let manager = MemcacheConnectionManager::new("memcache://localhost:11211");

    let pool = r2d2::Pool::builder().max_size(15).build(manager).unwrap();
    let mut conn = pool.get().unwrap();
    conn.flush().unwrap();
    let value = 100;
    conn.set("foo_decrement", value, 0).unwrap();
    assert_eq!(conn.decrement("foo_decrement", 1).unwrap(), 99);
}
