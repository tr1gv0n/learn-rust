fn call(mut f: impl FnMut() + Copy) {
    f();
}
fn g(mut f: impl FnMut() + Copy) {
    f();
    call(f);
    // ↑call(f) 复制 f 作为 call 的参数。副本被执行，其 i 变为 2，但原始闭包仍为其捕获的 i 保留值 1。当闭包的副本超出 call 主体末尾的范围时，它会被删除。
    f();
    call(f);
}

fn main() {
    let mut i = 0i32;
    g(move || {
        i += 1;
        println!("{}", i);
    })
}
