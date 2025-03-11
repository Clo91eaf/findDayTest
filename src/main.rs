use anyhow;
use chrono::{Datelike, Duration, NaiveDate, Weekday};
use plotters::prelude::*;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;

fn test_find_day(day: u32, month: u32, year: i32) -> u32 {
    let output = Command::new("./resources/test.out")
        .arg(&format!("{}", day))
        .arg(&format!("{}", month))
        .arg(&format!("{}", year))
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stderr);

        let re = Regex::new(r"The Day is (\w+)\.").unwrap();
        match re.captures(&output_str) {
            Some(caps) => {
                let day_of_week = caps.get(1).map_or("", |m| m.as_str()).to_string();

                match day_of_week.as_str() {
                    "Monday" => return 1,
                    "Tuesday" => return 2,
                    "Wednesday" => return 3,
                    "Thursday" => return 4,
                    "Friday" => return 5,
                    "Saturday" => return 6,
                    "Sunday" => return 7,
                    _ => return 0,
                }
            }
            None => return 0,
        }
    } else {
        return 0;
    }
}

fn test_chrono(day: u32, month: u32, year: i32) -> u32 {
    let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
    let weekday = date.weekday();

    match weekday {
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
        Weekday::Sun => 7,
    }
}

fn test(current_date: NaiveDate) -> anyhow::Result<()> {
    let day = current_date.day();
    let month = current_date.month();
    let year = current_date.year();

    let find_day_result = test_find_day(day, month, year);
    let chrono_result = test_chrono(day, month, year);

    if find_day_result != chrono_result {
        return Err(anyhow::anyhow!(
            "{} {} {} {} {}",
            day,
            month,
            year,
            find_day_result,
            chrono_result
        ));
    }

    Ok(())
}

fn parse_diff_log() -> [[usize; 12]; 748] {
    let file = File::open(&Path::new("output/diff_log.txt")).unwrap();
    let reader = BufReader::new(file);

    let mut array: [[usize; 12]; 748] = [[0; 12]; 748];

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(parts.len(), 5);

        // 解析每行的数据
        let (_, month, year, _, _): (i32, usize, usize, i32, i32) = (
            parts[0].parse().unwrap_or(0),
            parts[1].parse().unwrap_or(0),
            parts[2].parse().unwrap_or(0),
            parts[3].parse().unwrap_or(0),
            parts[4].parse().unwrap_or(0),
        );

        assert!(year >= 1753 && year <= 2500);
        assert!(month >= 1 && month <= 12);

        array[(year - 1753) as usize][(month - 1) as usize] += 1;
    }

    array
}

fn generate_heatmap() {
    let array = parse_diff_log();

    let root = BitMapBackend::new("doc/heatmap.png", (1920, 1080)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let max_value = array
        .iter()
        .flat_map(|row| row.iter())
        .cloned()
        .fold(usize::MIN, |a, b| a.max(b));
    let min_value = array
        .iter()
        .flat_map(|row| row.iter())
        .cloned()
        .fold(usize::MAX, |a, b| a.min(b));

    let mut chart = ChartBuilder::on(&root)
        .caption("Heatmap", ("sans-serif", 40))
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(1753..2500, 0..12)
        .unwrap();

    chart
        .configure_mesh()
        .x_labels(20)
        .y_labels(12)
        .x_label_formatter(&|x: &i32| format!("{}", x))
        .y_label_formatter(&|y| format!("{}", y + 1))
        .x_desc("Year")
        .y_desc("Month")
        .draw()
        .unwrap();

    for (y, row) in array.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            let normalized_value = if max_value == min_value {
                0.0
            } else {
                ((value - min_value) as f64) / ((max_value - min_value) as f64)
            };
            let color = RGBColor(
                (255.0 * (1.0 - normalized_value)) as u8,
                255,
                (255.0 * (1.0 - normalized_value)) as u8,
            );
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [
                        (1753 + y as i32, x as i32),
                        (1753 + y as i32 + 1, x as i32 + 1),
                    ],
                    color.filled(),
                )))
                .unwrap();
        }
    }

    root.present().unwrap();
}

fn generate_self_diff_log() {
    let start = NaiveDate::from_ymd_opt(1753, 1, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2500, 12, 31).unwrap();

    let mut current_date = start;

    let mut file = File::create("output/self_diff_log.txt").expect("Unable to create file");

    let mut yesterday = 0;
    while current_date <= end {
        let today = test_find_day(
            current_date.day(),
            current_date.month(),
            current_date.year(),
        );
        if yesterday != 0 && today != yesterday % 7 + 1 {
            println!(
                "{} {} {} yesterday = {yesterday} today = {today}, day mismatch",
                current_date.day(),
                current_date.month(),
                current_date.year()
            );
            writeln!(
                file,
                "{} {} {} yesterday = {yesterday} today = {today}, day mismatch",
                current_date.day(),
                current_date.month(),
                current_date.year()
            )
            .expect("Unable to write data");
        }
        current_date = current_date + Duration::days(1);
        yesterday = today;
    }
}

fn generate_diff_log() {
    let start = NaiveDate::from_ymd_opt(1753, 1, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2500, 12, 31).unwrap();

    let mut current_date = start;

    let mut count = 0;

    let mut file = File::create("output/diff_log.txt").expect("Unable to create file");

    while current_date <= end {
        if count % 20 == 0 {
            println!(
                "{} {} {} {}",
                current_date.day(),
                current_date.month(),
                current_date.year(),
                current_date.weekday(),
            );
        }

        match test(current_date) {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
                writeln!(file, "{}", e).expect("Unable to write data");
            }
        }
        current_date = current_date + Duration::days(1);
        count += 1;
    }
}

fn print_help() {
    println!("Usage: program [option]");
    println!("Options:");
    println!("  h    Generate heatmap");
    println!("  e    Generate error log");
    println!("  s    Generate self diff log");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        match args[1].as_str() {
            "h" => generate_heatmap(),
            "e" => generate_diff_log(),
            "s" => generate_self_diff_log(),
            _ => print_help(),
        }
    } else {
        print_help();
    }
}
