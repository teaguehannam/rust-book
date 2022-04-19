fn main() {
    // ---- Panic backtrace ----
    let v = vec![1, 2, 3];
    // Attempt to access non-existant index, running gives error:
    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99'
    v[99];
}
