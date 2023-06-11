fn calculate_times(h: f64, bounce: f64, window: f64, times: i32) -> i32 {
    if h > window {
        if (h * bounce) > window {
            calculate_times(h * bounce, bounce, window, times + 2)
        } else { times + 1 }
    } else { times }
}

fn bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
    if h > 0.0 && bounce > 0.0 && bounce < 1.0 && window < h {
        calculate_times(h, bounce, window, 0)
    } else { -1 }
}

