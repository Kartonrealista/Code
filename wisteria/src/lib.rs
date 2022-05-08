use pyo3::prelude::*;

#[pyfunction]
fn f1(a: usize) -> PyResult<(Vec<i64>, Vec<usize>, Vec<usize>)> {
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut s = Vec::new();

    for i in 1..a
    {
        let i_list = format!("{}", i as i64);
        let mut prod = 1 as i64;
        for j in (&i_list).chars()
        {
            if (j as i64) != 0
            {
                prod *= format!("{}", j).parse::<i64>().unwrap();
            }
            
        }
        
        let xval = i as i64 - prod;
        y.push(i);
        x.push(xval);
        s.push(2);
    }
    Ok((x, y, s))
}

/// A Python module implemented in Rust.
#[pymodule]
fn wisteria(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(f1, m)?)?;
    Ok(())
}