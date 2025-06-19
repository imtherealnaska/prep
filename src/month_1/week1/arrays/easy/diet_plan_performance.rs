fn diet_plan_performance(calories: Vec<i32>, k: i32, lower: i32, upper: i32) -> i32 {
    if calories.len() < k as usize {
        return 0;
    }

    let k = k as usize;

    let mut points = 0;

    let mut window: i32 = calories[0..k].iter().sum();

    points += get_points(window, lower, upper);

    for i in k..calories.len() {
        window = window - calories[i - k] + calories[i];
        points += get_points(window, lower, upper);
    }
    points
}

fn get_points(window: i32, lower: i32, upper: i32) -> i32 {
    if window < lower {
        -1
    } else if window > upper {
        1
    } else {
        0
    }
}
