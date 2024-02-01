use std::path::Path;
use std::fmt::Display;
use std::{error::Error, io, process};

use chrono::{DateTime, NaiveDate, Duration, Utc, TimeZone, Local, Datelike};

pub struct Event
{
    pub date: NaiveDate,
    pub category: String,
    pub description: String
}


impl Event
{
    pub fn new(date: &str, category: &str, description: &str ) -> Self 
    {
        let the_date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
        Event { date: the_date, category: category.to_string(), description: description.to_string() }
    }

    pub fn get_difference_string(&self, p: NaiveDate) -> String
    {
        let date1 = Utc.ymd(p.year(), p.month(), p.day()).and_hms(0, 0, 0);
        let date2 =  Utc.ymd(self.date.year(), self.date.month(), self.date.day()).and_hms(0, 0, 0);

        let date_interval = date_component::date_component::calculate(&date1, &date2);

        let mut returner = String::from("");

        if(date_interval.year == 0 && date_interval.month == 0 && date_interval.day == 0)
        {
            returner.push_str("today");
            return returner;
        }

        if(!date_interval.invert)
        {
            returner.push_str("in ");
        }

        if(date_interval.year != 0)
        {
            returner.push_str(&(date_interval.year).to_string());
            returner.push_str(" years ");
        }

        if(date_interval.month != 0)
        {
            returner.push_str(&(date_interval.month).to_string());
            returner.push_str(" months ");
        }

        if(date_interval.day != 0)
        {
            returner.push_str(&(date_interval.day).to_string());
            returner.push_str(" days");
        }

        if(date_interval.invert)
        {
            returner.push_str(" ago");
        }

        returner
    }
}


pub struct EventManager
{
    pub events: Vec<Event>
}

impl EventManager
{
    pub fn new() -> Self
    {
        let events = Vec::new();
        EventManager { events }
    }
    
    pub fn get_events_path() -> String
    {
        let home = dirs::home_dir().unwrap();

        let home_str = home.to_str();
    
        let mut s = String::from("");
    
        s.push_str(home_str.unwrap());
        if(!Path::new(&s).exists())
        {
            eprintln!("Unable to determine user home directory!");
            process::exit(1);
        }
        
        s.push_str("/.days");
        if(!Path::new(&s).exists())
        {
            eprintln!("{} directory does not exist, please create it!", s);
            process::exit(1);
        }

        s.push_str("/events.csv");
        if(!Path::new(&s).exists())
        {
            eprintln!("{} file not found!", s);
            process::exit(1);
        }


        s
    }

    pub fn load_events(&mut self, path: &String) -> Result<(), Box<dyn Error>>
    {
        // Build the CSV reader and iterate over each record.
        let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;
        
        
        for result in rdr.records() 
        {
            // The iterator yields Result<StringRecord, Error>, so we check the
            // error here.
            let record = result?;
        
            let record0 = &record[0];
            let record1 = &record[1];
            let record2 = &record[2];
        
            let event = Event::new(record0, record1, record2);
        
            self.events.push(event);
        }

        Ok(())
    }

    pub fn save_events(&mut self, path: &String) -> Result<(), Box<dyn Error>>
    {
        let mut wtr = csv::WriterBuilder::new()
        .from_path(path)?;
    
        //Add the headers first
        wtr.write_record(&[&"date", &"category", &"description"])?;
    
        //Write the events
        for Event in &self.events
        {
            wtr.write_record(&[&Event.date.format("%Y-%m-%d").to_string(), &Event.category, &Event.description])?;
        }

        Ok(())
    }

    pub fn sort_events(&mut self) 
    {
        //Basic selection sort for sorting events
        let len = self.events.len();

        for i in 0..len 
        {
            let mut min_index = i;
            for j in i + 1..len 
            {
                if (self.events[j].date < self.events[min_index].date)
                {
                    min_index = j;
                }
            }
            if min_index != i 
            {
                self.events.swap(i, min_index);
            }
        }
    }

    pub fn add_event(&mut self, event: Event)
    {
        self.events.push(event);
    }

    pub fn print_and_remove_event(&mut self, index: usize)
    {
        //Get today's date
        let mut today = chrono::offset::Local::now().date_naive();

        print!("{}: {} ({})", self.events[index].date.format("%Y-%m-%d").to_string(), self.events[index].description, self.events[index].category);
        println!(" -- {}", self.events[index].get_difference_string(today));

        self.events.remove(index);
    }

}