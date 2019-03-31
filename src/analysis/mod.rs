use redis::*;


pub fn analysis_loop(node: Box<Connection>, cursor: u64, count: usize) {
    let result: RedisResult<Iter<String>> = redis::cmd("SCAN").cursor_arg(cursor)
        .arg("COUNT").arg(count).iter(&*node);

    result.map(|keys| {
        for it in keys {
            it.split('_');
        }
    });
}

