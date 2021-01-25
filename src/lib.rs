
#[macro_use]
extern crate redis_module;

use redis_module::native_types::RedisType;
use redis_module::{raw, Context, NextArg, RedisResult, REDIS_OK};
use redis_module::logging::{log};
use redis_module::LogLevel;
use std::os::raw::c_void;
use std::rc::Rc;
use std::ptr;


fn inner_log(message: &str) {
    log(LogLevel::Warning, message)
}



#[derive(Debug)]
struct Inner {
    data: String,
}

impl Drop for Inner {
    fn drop(&mut self) { 
        inner_log("drop in inner")
    }
}

impl Drop for MyType {
    fn drop(&mut self) { 
        inner_log("drop in myType")
    }
}


#[derive(Debug)]
struct MyType {
    data: Box<Inner>,
}

static MY_REDIS_TYPE: RedisType = RedisType::new(
    "mytype123",
    0,
    raw::RedisModuleTypeMethods {
        version: raw::REDISMODULE_TYPE_METHOD_VERSION as u64,
        rdb_load: None,
        rdb_save: None,
        aof_rewrite: None,
        free: Some(free),

        // Currently unused by Redis
        mem_usage: None,
        digest: None,

        // Aux data
        aux_load: None,
        aux_save: None,
        aux_save_triggers: 0,
    },
);

unsafe extern "C" fn free(value: *mut c_void) {
    Box::from_raw(value as *mut MyType);
}

fn alloc_set(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;
    let size = args.next_i64()?;

    ctx.log_debug(format!("key: {}, size: {}", key, size).as_str());

    let key = ctx.open_key_writable(&key);

    inner_log("hello set");
    match key.get_value::<MyType>(&MY_REDIS_TYPE)? {
        Some(value) => {
        }
        None => {
            let value = MyType {
                data: Box::new(Inner{
                   data: "ABC".to_owned(), 
                })
            };

            inner_log(&format!("{:p}", value.data));

            key.set_value(&MY_REDIS_TYPE, value)?;
        }
    }

    Ok(size.into())
}

fn alloc_del(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;

    inner_log("hello del");
    let key = ctx.open_key_writable(&key);
    let value = match key.get_value::<MyType>(&MY_REDIS_TYPE)? {
        Some(_) => {
            key.delete()?;
            "ok".into()
        },
        None => "nil".into(),
    };    

    Ok(value)
}


fn alloc_get(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;
    let key = ctx.open_key(&key);

    inner_log("hello get");

    let value = match key.get_value::<MyType>(&MY_REDIS_TYPE)? {
        Some(value) => (&*value.data.data).into(),
        None => ().into(),
    };

    Ok(value)
}

//////////////////////////////////////////////////////

redis_module! {
    name: "alloc",
    version: 1,
    data_types: [
        MY_REDIS_TYPE,
    ],
    commands: [
        ["alloc.set", alloc_set, "write", 1, 1, 1],
        ["alloc.del", alloc_del, "write", 1, 1, 1],
        ["alloc.get", alloc_get, "readonly", 1, 1, 1],
    ],
}