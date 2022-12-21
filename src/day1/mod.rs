pub fn day_1(lines: Vec<String>) {
    let mut calories: Vec<i32> = Vec::new();
    let mut current_calories = 0;
    let mut current_largest_calories = 0;
    for line in lines {
        if line.is_empty() {
            calories.push(current_calories);
            if current_largest_calories == 0 || current_calories > current_largest_calories {
                current_largest_calories = current_calories;
            }
            current_calories = 0;
        } else {
            current_calories += line.parse::<i32>().unwrap();
        }
    }
    
    let top_3_calories;
    calories.sort_by(|a, b| b.cmp(a));
    top_3_calories = calories[0] + calories[1] + calories[2];

    println!("Largest Calories: {}", current_largest_calories);
    println!("Top 3 Calories: {}", top_3_calories);
}
