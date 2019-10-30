use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::collections::HashSet;


#[pyfunction]
/// Formats the sum of two numbers as string
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn last_ticker_accuracy(predict: &str, result: &str) -> PyResult<bool> {
    let p_tickers: Vec<&str> = predict.split(',').collect();
    let r_tickers: Vec<&str> = result.split(',').collect();
    Ok(p_tickers.last().unwrap().clone() == r_tickers.last().unwrap().clone())
}

#[pyfunction]
fn pairs_with_last(predicts: Vec<&str>, results: Vec<&str>) -> PyResult<f32> {
    let mut accs = Vec::with_capacity(predicts.len());
    for (result_index, result) in results.iter().enumerate() {
	let predict = predicts[result_index];
        let r_tickers: Vec<&str> = result.split(',').collect();
        let len_r_tickers = r_tickers.len();
        let last_result_ticker = r_tickers[len_r_tickers - 1];
        let result_tickers_without_last: HashSet<&str> = r_tickers.iter().take(len_r_tickers - 1).cloned().collect();
        let first_predict_tickers: HashSet<&str> = predict.split(',').take_while(|x| *x != last_result_ticker).collect();
	accs.push(result_tickers_without_last.intersection(&first_predict_tickers).count() as f32 / len_r_tickers as f32);
    }
    let acc_sum: f32 = accs.iter().sum();
    Ok(acc_sum / accs.len() as f32)
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn libnumex(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    m.add_wrapped(wrap_pyfunction!(last_ticker_accuracy))?;
    m.add_wrapped(wrap_pyfunction!(pairs_with_last))?;
    Ok(())
}
