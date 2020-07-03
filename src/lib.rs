/*
Introduction
https://developers.redhat.com/blog/2017/11/16/speed-python-using-rust/

cpython github
https://github.com/dgrunwald/rust-cpython
*/

/*
runtime error when 
    import rs2py
ImportError: undefined symbol: _Py_Dealloc

reference: https://github.com/dgrunwald/rust-cpython/issues/122
*/

use cpython::{
    Python, PyResult, py_module_initializer, py_fn,
};

// add bindings to the generated python module
// N.B: names: "rust2py" must be the name of the `.so` or `.pyd` file
py_module_initializer!(rust2py, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "sum_as_string", py_fn!(py, sum_as_string_py(a: i64, b:i64)))?;
    m.add(py, "count_double_letters", py_fn!(py, count_double_letters_py(val:&str)))?;
    Ok(())
});

// logic implemented as a normal rust function
fn sum_as_string(a:i64, b:i64) -> String {
    format!("{}", a + b).to_string()
}

fn count_double_letters(val: &str) -> u64 {
    let mut count: u64 = 0;
    
    let mut bytes = val.bytes();
    
    if let Some(mut last) = bytes.next() {
        for byte in bytes {
            if last == byte {
                count += 1;
            }
            last = byte;
        }
    };

    count
}

// python interface
fn count_double_letters_py(_: Python, val: &str) -> PyResult<u64> {
    Ok(count_double_letters(val))
}

// rust-cpython aware function. All of our python interface could be
// declared in a separate module.
// Note that the py_fn!() macro automatically converts the arguments from
// Python objects to Rust values; and the Rust return value back into a Python object.
fn sum_as_string_py(_: Python, a:i64, b:i64) -> PyResult<String> {
    let out = sum_as_string(a, b);
    Ok(out)
}
