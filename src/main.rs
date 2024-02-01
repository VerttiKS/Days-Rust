mod args;
mod events;

use args::{DaysArgs, EntityType, ListArgs, AddArgs};
use events::{Event, EventManager};
use clap::Parser;
use std::path::Path;
use std::env;


use std::{error::Error, io, process};
use chrono::{DateTime, NaiveDate, Duration, Utc, TimeZone, Local, Datelike};


fn main() {
    let args = DaysArgs::parse().entity_type;

    match args 
    {
        EntityType::List(value) => print_events(value),
        EntityType::Add(value) => add_event(value),
        EntityType::Delete(value) => delete_events(value),
        _ => println!("Error no commands!")
    }
}

fn check_event(args: &ListArgs, event: &Event) -> bool
{
    let mut dateChecked = false;    //This becomes true once we check if there's multiple dates
    let mut datesBetween = false;   //Checks if we dates are between "before" and "after" or outside of them. When true they're between
    let mut dateCorrect = false;    //Used when dates are outside before and after. Tells the system if the even is the one we're looking for
    let empty = String::from(""); //The default value of empty args


    if(args.today)
    {
        let mut today = chrono::offset::Local::now().date_naive();

        if(today != event.date)
        {
            return false;
        }
    }

    if(args.before_date != empty)
    {
        //Turns before-date into a NaiveDate
        let the_before = NaiveDate::parse_from_str(&args.before_date, "%Y-%m-%d").unwrap();

        //Check if multiple dates in avgs
        if(dateChecked == false)
        {
            if(args.after_date != empty)
            {
                //Turns after-date into a NaiveDate
                let the_after = NaiveDate::parse_from_str(&args.after_date, "%Y-%m-%d").unwrap();

                //If "before-date" is a bigger date than "after-date"
                if(the_before > the_after)
                {
                    datesBetween = true;
                }
            }
            
            dateChecked = true;
        }
        
        if(datesBetween)
        {
            if((event.date < the_before) == false)
            {
                return false;
            }
        }
        else if(event.date < the_before)
        {
            dateCorrect = true;
        }
    }

    if(args.after_date != empty)
    {
        //Turns after-date into a NaiveDate
        let the_after = NaiveDate::parse_from_str(&args.after_date, "%Y-%m-%d").unwrap();

        //Check if multiple dates in avgs
        if(dateChecked == false)
        {
            if(args.before_date != empty)
            {
                //Turns before-date into a NaiveDate
                let the_before = NaiveDate::parse_from_str(&args.before_date, "%Y-%m-%d").unwrap();

                //If "after-date" is a smaller date than "before-date"
                if(the_after < the_before)
                {
                    datesBetween = true;
                }
            }

            dateChecked = true;
        }

        if(datesBetween)
        {
            if((event.date > the_after) == false)
            {
                if((event.date == the_after) == false)
                {
                    return false;
                }
            }
        }
        else if(event.date > the_after)
        {
            dateCorrect = true;
        }
    }

    if(args.date != empty)
    {
        //Turns date into a NaiveDate
        let the_date = NaiveDate::parse_from_str(&args.date, "%Y-%m-%d").unwrap();

        if(event.date != the_date)
        {
            return false;
        }
    }

    if(args.categories != empty)
    {
        let mut commaLoc: isize = -1;
        let mut matched = false;
        let arg_length = args.categories.chars().count();

        //We go through the chars of the string
        for (i, c) in args.categories.chars().enumerate()
        {
            //If the char is a comma
            if(c == ',')
            {
                let substring: String = args.categories.chars().skip((commaLoc+1) as usize).take(i-((commaLoc+1) as usize)).collect();
                
                //We use substring to check if it matched the category of the event
                if(event.category == substring)
                {
                    matched = true;
                }

                commaLoc = i as isize;
            }

            //If this is the last loop
            if(i+1 == arg_length)
            {
                let substring: String = args.categories.chars().skip((commaLoc+1) as usize).take(i+1-((commaLoc+1) as usize)).collect();
                
                //We use substring to check if it matched the category of the event
                if(event.category == substring)
                {
                    matched = true;
                }
            }
        }
        
        if(matched == false)
        {
            return false;
        }
    }

    if(args.category != empty)
    {
        let mut commaLoc: isize = -1;
        let mut matched = false;
        let arg_length = args.category.chars().count();

        //We go through the chars of the string
        for (i, c) in args.category.chars().enumerate()
        {
            //If the char is a comma
            if(c == ',')
            {
                let substring: String = args.category.chars().skip((commaLoc+1) as usize).take(i-((commaLoc+1) as usize)).collect();
                
                //We use substring to check if it matched the category of the event
                if(event.category == substring)
                {
                    matched = true;
                }

                commaLoc = i as isize;
            }

            //If this is the last loop
            if(i+1 == arg_length)
            {
                let substring: String = args.category.chars().skip((commaLoc+1) as usize).take(i+1-((commaLoc+1) as usize)).collect();
                
                //We use substring to check if it matched the category of the event
                if(event.category == substring)
                {
                    matched = true;
                }
            }
        }
        
        if(matched == false)
        {
            return false;
        }
    }

    if(args.description != empty)
    {
        let mut commaLoc: isize = -1;
        let mut matched = false;
        let arg_length = args.description.chars().count();

        //While it should be impossible to have empty descriptions, this prevents bad things from happening if them exist.
        //(I had this issue, so I'll make sure no one else has it either)
        if(event.description == empty)
        {
            return false;
        }

        //We go through the chars of the string
        for (i, c) in args.description.chars().enumerate()
        {
            //If the char is a comma
            if(c == ',')
            {
                let substring: String = args.description.chars().skip((commaLoc+1) as usize).take(i-((commaLoc+1) as usize)).collect();

                //Makes sure we don't get a StringIndexOutOfBoundsException
                if(substring.chars().count() <= event.description.chars().count())
                {
                    //We check if the string matches char by char
                    for (j, ch) in substring.chars().enumerate()
                    {
                        //Get char from event
                        let event_char: char = event.description.chars().nth(j+((commaLoc+1) as usize)).unwrap();

                        if(event_char == ch)
                        {
                            matched = true;
                        }
                        else
                        {
                            matched = false;
                            break;
                        }
                    }
                }

                
                //If the matched is true, we can just exit here
                if(matched == true)
                {
                    break;
                }
                
                //We use substring to check if it matched the description of the event
                if(event.description == substring)
                {
                    matched = true;
                }

                commaLoc = i as isize;
            }

            //If this is the last loop
            if(i+1 == arg_length)
            {
                let substring: String = args.description.chars().skip((commaLoc+1) as usize).take(i+1-((commaLoc+1) as usize)).collect();

                //Makes sure we don't get a String out of bounds
                if(substring.chars().count() <= event.description.chars().count())
                {
                    //We check if the string matches char by char
                    for (j, ch) in substring.chars().enumerate()
                    {
                        //Get char from event
                        let event_char: char = event.description.chars().nth(j).unwrap();

                        if(event_char == ch)
                        {
                            matched = true;
                        }
                        else
                        {
                            matched = false;
                            break;
                        }
                    }
                }
            }
        }
        
        if(matched == false)
        {
            return false;
        }
    }

    if(args.no_category)
    {
        if(event.category != empty)
        {
            return false;
        }
    }

    if(args.no_description)
    {
        //This is technically useless, because there shouldn't be events without description. It is mostly there for eventList error fixes (example: delete ones without description)
        if(event.description != empty)
        {
            return false;
        }
    }

    if(dateChecked && !datesBetween)
    {
        if(dateCorrect)
        {
            return true;
        }
        else
        {
            return false;
        }
    }

    true
}

fn print_events(args: ListArgs)
{
    //If the exclude arg is used, this flips the print
    let mut checker = true;
    if(args.exclude)
    {
        checker = false;
    }

    //Get today's date
    let mut today = chrono::offset::Local::now().date_naive();

    //Start event manager
    let mut events_manager = EventManager::new();

    //Get homepath
    let homePath = EventManager::get_events_path();

    //Get events from the csv
    if let Err(err) = events_manager.load_events(&homePath) 
    {
        eprintln!("Error loading events: {}", err);
        process::exit(1);
    }

    //Sort events
    events_manager.sort_events();

    //Print events
    if(args.all)
    {
        for Event in &events_manager.events
        {
            print!("{}: {} ({})", Event.date.format("%Y-%m-%d").to_string(), Event.description, Event.category);
            println!(" -- {}", Event.get_difference_string(today));
        }
    }
    else 
    {
        for Event in &events_manager.events
        {
            if(check_event(&args, &Event) == checker)
            {
                print!("{}: {} ({})", Event.date.format("%Y-%m-%d").to_string(), Event.description, Event.category);
                println!(" -- {}", Event.get_difference_string(today));
            }
        }
    }
}


fn add_event(args: AddArgs)
{
    let empty = String::from(""); //The default value of empty args

    //Check if description is empty
    if(args.description == empty)
    {
        eprintln!("Error cannot save an event without a description!");
        return;
    }

    //Get today's date
    let mut today = chrono::offset::Local::now().date_naive();

    //Make a string for new event date
    let mut the_date = String::new();

    //Check if date is empty. If it is, make the new_event date into today. Else make the new_event date into args.date
    if(args.date == empty)
    {
        the_date = today.format("%Y-%m-%d").to_string();
    }
    else 
    {
        the_date = args.date;
    }

    //Start event manager
    let mut events_manager = EventManager::new();

    //Get homepath
    let homePath = EventManager::get_events_path();

    //Get events from the csv
    if let Err(err) = events_manager.load_events(&homePath) 
    {
        eprintln!("Error loading events: {}", err);
        process::exit(1);
    }

    //Creating new event
    let new_event = Event::new(&the_date, &args.category, &args.description);

    //true = don't add events. false = add events
    if(args.dry_run)
    {
        print!("Would have added event: [{}: {} ({})", new_event.date.format("%Y-%m-%d").to_string(), new_event.description, new_event.category);
        println!(" -- {}]", new_event.get_difference_string(today));
    }
    else 
    {
        print!("Adding event: [{}: {} ({})", new_event.date.format("%Y-%m-%d").to_string(), new_event.description, new_event.category);
        println!(" -- {}]", new_event.get_difference_string(today));

        //Add new event to the manager
        events_manager.add_event(new_event);

        //Sort events
        events_manager.sort_events();

        //Save event to the file
        if let Err(err) = events_manager.save_events(&homePath) 
        {
        eprintln!("Error while saving an event: {}", err);
        process::exit(1);
        }

        println!("Event saved successfully!");
    }

}

fn delete_events(args: ListArgs)
{
    //Failsafe for the command "days delete"
    if(!args.all)
    {
        let empty = String::from(""); //The default value of empty args

        if
        ( 
            args.today == false &&                 
            args.before_date == empty &&  
            args.after_date == empty &&  
            args.date == empty &&  
            args.category == empty &&  
            args.categories == empty &&  
            args.description == empty &&  
            args.no_category == false &&   
            args.no_description == false
        )
        {
            eprintln!("Oops almost deleted all! Please use 'days delete --help' for help.");
            process::exit(1);
        }
    }


    //If the exclude arg is used, this flips the print
    let mut checker = true;
    if(args.exclude)
    {
        checker = false;
    }

    //Get today's date
    let mut today = chrono::offset::Local::now().date_naive();

    //Start event manager
    let mut events_manager = EventManager::new();

    //Get homepath
    let homePath = EventManager::get_events_path();

    //Get events from the csv
    if let Err(err) = events_manager.load_events(&homePath) 
    {
        eprintln!("Error loading events: {}", err);
        process::exit(1);
    }

    //Sort events
    events_manager.sort_events();


    let mut delete_from_index = Vec::new(); //Mark locations of the events that will be deleted
    let mut event_index: usize = 0;                     //Keeps track of the event positions

    //Mark events for deletion
    if(args.all)
    {
        for Event in &events_manager.events
        {
            delete_from_index.push(event_index);
            event_index += 1;
        }
    }
    else 
    {
        for Event in &events_manager.events
        {
            if(check_event(&args, &Event) == checker)
            {
                delete_from_index.push(event_index);
            }
            event_index += 1;
        }
    }

    //Flip the order of the removal index
    delete_from_index.reverse();

    //Remove the events according to the "delete_from_index"-variable
    println!("Deleting following events:");
    for j in delete_from_index
    {
       events_manager.print_and_remove_event(j);
    }

    //true = don't delete events. false = delete events
    if(args.dry_run)
    {
        println!("Didn't delete, because this is a dry-run!");
    }
    else
    {
        //Sort events
        events_manager.sort_events();

        //Save current events to the file (the removed events are removed)
        if let Err(err) = events_manager.save_events(&homePath) 
        {
        eprintln!("Error removing events: {}", err);
        process::exit(1);
        }

        println!("Event removed successfully!");
    }
}

