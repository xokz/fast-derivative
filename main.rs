use std::{time::Instant};

const DELTA: f64 = 0.000001;

pub struct Function {
    equation: fn(f64) -> f64,
    range: (f64, f64),
    written: &'static str,
}

fn main() {

    let functions = vec![
        Function {
            equation: |x| x.abs().sqrt(),
            range: (-100.0, 100.0),
            written: "√|x|",
        },
        Function {
            equation: |x| x * (x + 1.0) * (x - 1.0),
            range: (-100.0, 100.0),
            written: "x(x + 1)(x - 1)",
        },
        Function {
            equation: |x| x * (x + 1.0),
            range: (-100.0, 100.0),
            written: "x(x + 1)",
        },
        Function {
            equation: |x| x.cos(),
            range: (-100.0, 100.0),
            written: "Cos(x)",
        },
        Function {
            equation: |x| x.sin(),
            range: (-100.0, 100.0),
            written: "Sin(x)",
        },
        Function {
            equation: |x| x.sin() * x.cos(),
            range: (-100.0, 100.0),
            written: "Sin(x)Cos(x)",
        },
        Function {
            equation: |x| x,
            range: (-100.0, 100.0),
            written: "x",
        },
        Function {
            equation: |x| x * x,
            range: (-100.0, 100.0),
            written: "x^2",
        },
    ];

    println!("==================================================================================\nStep size: 1/1000000");
    for function in functions {
        // fixed
        let fixed_start = Instant::now();
        let fixed_ans = fixed_integral(function.equation, DELTA, function.range);
        let fixed_time = fixed_start.elapsed();

        // dynamic
        let dyn_start = Instant::now();
        let dyn_ans = dyn_integral(function.equation, function.range);
        let dyn_time = dyn_start.elapsed();

        // Display results
        println!(
            "==================================================================================\n"
        );
        println!(
            "Integral:\n    f(x) = {}\n    Range = {} to {}\n",
            function.written, function.range.0, function.range.1
        );
        println!(
            "Fixed integral:\n    ans: {}\n    time: {:?}\n",
            fixed_ans, fixed_time
        );
        println!(
            "Dynamic integral:\n    ans: {}\n    time: {:?}\n",
            dyn_ans, dyn_time
        );
    }
}

fn dyn_integral(f: fn(f64) -> f64, range: (f64, f64)) -> f64 {

    const DELTA: f64 = 0.000001;
    const MAX_STEP_SIZE: f64 = 0.001;

    // make sure range start is less than range end
    let range = {
        if range.0 > range.1 {
            (range.1, range.0)
        } else {
            range
        }
    };

    let total_area = {
        
        let mut step: f64 = DELTA;
        let mut sum: f64 = 0.0;
        let mut f_x: f64 = f(range.0);

        
        let mut prev_step: f64 = DELTA;
        let mut prev_velocity: f64;
        let mut velocity: f64 = 0.0;
        let mut acceleration: f64;

        let mut x: f64 = range.0;
        while x <= range.1 {
            prev_velocity = velocity;
            // derivative of f(x) at point x, aka the slope
            // f'(x)
            velocity = (f(x + DELTA) - f(x - DELTA)) * (0.5 * DELTA);
            if velocity.is_nan() {
                velocity = 0.0;
            };
            // how much the slope changed in respect to the previous slope
            // f''(x)
            acceleration = (prev_step - step) / (prev_velocity - velocity);
            if acceleration.is_nan() {
                acceleration = 0.0;
            };
            
            prev_step = step;
            // if only acceleration OR velocity is a large number, the step wont be large.
            step = (DELTA / (acceleration - velocity).clamp(DELTA, MAX_STEP_SIZE)).clamp(DELTA, MAX_STEP_SIZE);
            if step.is_nan() {
                step = DELTA;
            };
            
            // calculate current area as a trapezoid for accuracy
            let area = 0.5 * (f_x + f(x + step)) * step;

            f_x = f(x + step);
            sum += area;
            x += step;
        }

        sum
    };

    total_area
}

fn fixed_integral(f: fn(f64) -> f64, step: f64, range: (f64, f64)) -> f64 {
    // make sure step is positive and greater than 0.0
    if step <= 0.0 {
        panic!("Step must be greater than 0");
    }
    // make sure range.0 is less than range.1
    let range = {
        if range.0 > range.1 {
            (range.1, range.0)
        } else {
            range
        }
    };

    let middle_sum = {
        let start = range.0 + step;
        let end = range.1 - step;

        let mut x: f64 = start;
        let mut sum: f64 = 0.0;

        while x <= end {
            let area = f(x) * step;
            sum += area;
            x += step;
        }

        sum
    };

    let upper_sum = f(range.0) * step + middle_sum;
    let lower_sum = f(range.1) * step + middle_sum;

    (upper_sum + lower_sum) / 2.0
}
