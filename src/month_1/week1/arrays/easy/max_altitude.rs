fn max_altitude(nums: Vec<i32>) -> i32 {
    let mut current_altitude = 0;
    let mut max_altitude = 0;

    for al_gain in nums {
        current_altitude += al_gain;
        max_altitude = max_altitude.max(current_altitude);
    }
    max_altitude
}
