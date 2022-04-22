use pyo3::prelude::*;

#[pyfunction]
#[pyo3(text_signature = "(n: i64, g1: f64, g2:f64, p1:f64, p2:f64)")]
fn gen(n: i64, g1: f64, g2:f64, p1:f64, p2:f64) -> PyResult<(Vec<f64>, Vec<f64>)> 
{
    let mut a = Vec::new();
    let mut b = Vec::new();

    for i in 0..n
    {
        let i2 = i as f64;
        let n2 = n as f64;
        a.push((i2+p1/g1 * n2)/(n2/g1));
        b.push((i2+p2/g2 * n2)/(n2/g2));
    }
    Ok((a, b))
}

#[pyfunction]
fn mandmis(a: f64, b: f64) -> PyResult<String> 
{
    let c_re = a;
    let c_im = b;
    let mut zp_re: f64 = 0.0;
    let mut zp_im: f64 = 0.0;

    for _i in 0..400000
    {   
        
        let zn_re = zp_re.powf(2.0) - zp_im.powf(2.0) + c_re;
        let zn_im = 2.0 * zp_re * zp_im + c_im;
        if zn_re.powf(2.0) + zn_im.powf(2.0)  >= 4.0
        {
            return Ok("false".to_string());
        }
        zp_re = zn_re;
        zp_im = zn_im;
        
    }
    return Ok(format!("{} + {}j", c_re, c_im));
}

#[pyfunction]
fn mand(a: f64, b: f64) -> PyResult<i64> 
{
    let c_re = a;
    let c_im = b;
    let mut zp_re: f64 = 0.0;
    let mut zp_im: f64 = 0.0;

    for i in 0..4000
    {   
        
        let zn_re = zp_re.powf(2.0) - zp_im.powf(2.0) + c_re;
        let zn_im = 2.0 * zp_re * zp_im + c_im;
        if zn_re.powf(2.0) + zn_im.powf(2.0)  >= 4.0
        {
            return Ok(i%200 + 30);
        }
        zp_re = zn_re;
        zp_im = zn_im;
        
    }
    return Ok(0);
}

/// A Python module implemented in Rust.
#[pymodule]
fn zupa1(_py: Python<'_>, m: &PyModule) -> PyResult<()> 
{
    m.add_function(wrap_pyfunction!(mand, m)?)?;
    m.add_function(wrap_pyfunction!(gen, m)?)?;
    m.add_function(wrap_pyfunction!(mandmis, m)?)?;
    Ok(())
}