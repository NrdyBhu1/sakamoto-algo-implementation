use std::io;

fn input() -> String 
{
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    return guess.trim().to_string();
}

fn to_int(num: String) -> i32
{
    let intnum: i32 = num.parse::<i32>().unwrap();
    return intnum;
}


fn day_of_week(mut y: i32, m: i32, d: i32) -> String {
	let DayTypes = 
	[
		"Sunday",
		"Monday",
		"Tuesday",
		"Wednesday",
		"Thursday",
		"Friday",
		"Saturday"
	];
	
	let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
	if m < 3
	{
		y -= 1;
	}
	
	let day  = ((y + y/4 - y/100 + y/400 + t[(m-1) as usize] + d) % 7) as usize;
	return DayTypes[day].to_string();
}

fn check_friday_13(year: i32, month: i32)
{
	let day = day_of_week(year, month, 13);
	if day == "Friday"
	{
		println!("It has Friday the 13th");
	} else
	{
		println!("Sorry it doesn't have friday the 13th");
	} 
}

fn main() 
{
	let days = [ "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday" ];
	println!("Year: ");
	let year = to_int(input());
	println!("Month: ");
	let month = to_int(input());
	check_friday_13(year, month);
}
