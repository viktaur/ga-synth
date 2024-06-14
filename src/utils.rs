use std::f32::consts::PI;
use itertools::Itertools;
use rand::{thread_rng, Rng};

/// Performs a weighted average with randomly generated weights between two values. However, if a mutation is triggered,
/// the value returned will be completely random, specified by the calling code as the ranges may vary.
pub fn random_weighted_average(v_self: f32, v_other: f32, r: f32, random_val: f32) -> f32 {
    let mut prob = thread_rng();

    let beta: f32 = prob.gen();
    let mutation: f32 = prob.gen();

    if mutation < r {
        random_val
    } else {
        beta * v_self + (1.0 - beta) * v_other
    }
}

/// Sigmoid function.
pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

/// Calculates the mean of a set of elements.
pub fn mean(values: &[f32]) -> f32 {
    values.iter().sum::<f32>() / values.len() as f32
}

/// Calculates the standard deviation of a set of elements.
pub fn std(values: &[f32]) -> f32 {
    mean(&values.iter().map(|f| f.powi(2)).collect_vec())
    - mean(values).powi(2)
}

/// Performs a convolution between a given filter and an input signal.
pub fn convolve(filter: &[f32], input: &[f32]) -> Vec<f32> {
    let mut output: Vec<f32> = Vec::new();
    let h_len = (filter.len() / 2) as isize;

    for i in -(filter.len() as isize / 2)..(input.len() as isize - 1) {
        output.push(0.0);
        for j in 0isize..filter.len() as isize {
            let input_idx = i + j;
            let output_idx = i + h_len;
            if input_idx < 0 || input_idx >= input.len() as isize {
                continue;
            }
            output[output_idx as usize] += input[input_idx as usize] * filter[j as usize]
        }
    }

    output
}


/// Creates a blackman window filter of a given size.
pub fn blackman_window(size: usize) -> Vec<f32> {
    (0..size)
        .map(|i| {
            0.42 - 0.5 * (2.0 * PI * i as f32 / (size as f32 - 1.0)).cos()
                + 0.08 * (4.0 * PI * i as f32 / (size as f32 - 1.0)).cos()
        })
        .collect()
}

/// Inverts the frequencies of a filter. For example, inverting a low-pass filter will result in a
/// high-pass filter.
pub fn spectral_invert(filter: &[f32]) -> Vec<f32> {
    assert_eq!(filter.len() % 2, 0);
    let mut count = 0;

    filter
        .iter()
        .map(|&el| {
            let add = if count == filter.len() / 2 { 1.0 } else { 0.0 };
            count += 1;
            -el + add
        })
        .collect()
}

/// Performs addition over the elements of two slices.
pub fn add(a: &[f32], b: &[f32]) -> Vec<f32> {
    a.iter().zip(b.iter()).map(|(i, j)| i + j).collect()
}