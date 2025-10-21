use std::fmt::Debug;

use polars::prelude::*;
use rusqlite::{Connection, Result};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct RunningData {
    pub date: String,
    pub start_time: String,
    pub distance: f64,
    pub day: String,
    pub month: String,
}

pub fn get_running_data() -> Result<Vec<RunningData>, Box<dyn std::error::Error>> {
    let conn = Connection::open("/home/computer/HealthData/DBs/garmin_activities.db")?;

    let mut date: Vec<String> = Vec::new();
    let mut start_time: Vec<String> = Vec::new();
    let mut distance: Vec<f64> = Vec::new();

    {
        let mut stmt = conn.prepare("SELECT start_time, distance FROM running_activities_view LIMIT 10")?;
        let mut columns = stmt.query([])?;
        
        while let Some(row) = columns.next()? {
            let id1: String = row.get(0)?;
            let split: Vec<&str> = id1.split(' ').collect();
            let _date = split[0];
            let time = split[1];
            let split1: Vec<&str> = time.split('.').collect();
            let _time = split1[0];
            
            let id2: f64 = row.get(1)?;
            
            date.push(_date.to_string());
            start_time.push(_time.to_string());
            distance.push(id2);
        }
    }

    let groups = vec![1; date.len()];
    let mut df = df!(
        "date" => date,
        "start_time" => start_time,
        "distance" => distance,
        "groups" => groups
    )?;
    
    // Process dates and times
    let dates = df["date"].str()?;
    let processed_dates = dates.as_date(Some("%Y-%m-%d".into()), false)?;
    let df = df.with_column(processed_dates.into_column())?;
    
    let times = df["start_time"].str()?;
    let processed_times = times.as_time(Some("%H:%M:%S".into()), false)?;
    let df = df.with_column(processed_times.into_series().into_column())?;
    
    // Upsample and fill missing dates
    let mut df = df.sort(["date"], Default::default())?
        .upsample_stable(["groups"], "date", Duration::parse("1d"))?
        .fill_null(FillNullStrategy::Zero)?;
    
    // Add weekday and month columns
    let week_day = df["date"].as_series().unwrap().strftime("%A")?.rename(PlSmallStr::from_str("day")).clone();
    let month = df["date"].as_series().unwrap().strftime("%B")?.rename(PlSmallStr::from_str("month")).clone();
    
    let df = df.with_column(week_day.into_series())?.with_column(month.into_series())?;

    // Convert to Vec<RunningData> for serialization
    let mut result = Vec::new();

    for i in 0..df.height() {
        let date = df["date"].get(i)?.to_string();
        let start_time = df["start_time"].get(i).unwrap().to_string();
        let distance = df["distance"].get(i)?.try_extract::<f64>().unwrap();
        let day = df["day"].get(i)?.to_string();
        let month = df["month"].get(i)?.to_string();
        result.push(RunningData {
            date: date,
            start_time: start_time,
            distance: distance,
            day: day,
            month: month,
        });
    }
    //
    // let date_series = df["date"].as_series().iter().map(|x| x.iter()).collect::<Vec<String>>();
    // let time_series = df.column("start_time")?.str()?.iter();
    // let distance_series = df.column("distance")?.f64()?;
    // let day_series = df.column("day")?.str()?;
    // let month_series = df.column("month")?.str()?;

    // let mut result = Vec::new();
    // for i in 0..df.height() {
    //     println!("{}", i);
    //     let val = date_series.get(i).unwrap().to_string();
    //
    //     println!("{:?}", val);

    //     result.push(RunningData {
    //         date: date_series.get(i).unwrap().to_string(),
    //         start_time: time_series.get(i).unwrap().to_string(),
    //         distance: distance_series.get(i).unwrap_or(0.0),
    //         day: day_series.get(i).unwrap().to_string(),
    //         month: month_series.get(i).unwrap().to_string(),
    //     });
    // }
    
    Ok(result)
}
