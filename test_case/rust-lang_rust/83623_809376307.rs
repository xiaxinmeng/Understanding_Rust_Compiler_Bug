rust
pub fn is_leap_year1(year: i32) -> bool {
    let div_4 = year % 4 == 0;
    let div_100 = year % 100 == 0;
    let div_400 = year % 400 == 0;
    div_4 && !(div_100 && !div_400)
}
