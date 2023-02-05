/// Retruns a vec of length 'n_steps' evenly spaced between start and end.
/// If 'include_end' = true, then the final value will equal end.
/// If 'include_end' = false, then the number of steps will remain the same,
/// but the final value will be one increment short of end.
/// This is useful behaviour if a periodic linspace is desired.
pub fn linspace(start: f64, end: f64, n_steps: i32, include_end: bool) -> Vec<f64> {
    // Get step size based on 'include_end'
    let interval: f64 = end - start;

    let step_size = if include_end {
        interval / (n_steps - 1) as f64
    } else {
        interval / n_steps as f64
    };

    let mut line: Vec<f64> = Vec::new();
    for n in 0..n_steps {
        line.push(start + step_size * n as f64);
    }

    return line;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linspace_include_end() {
        let result = linspace(1.0, 10.0, 10, true);
        assert_eq!(
            result,
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]
        );
    }

    #[test]
    fn test_linspace_exclude_end() {
        let result = linspace(1.0, 11.0, 10, false);
        assert_eq!(
            result,
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]
        );
    }
}
