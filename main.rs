#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(deprecated)]
#![allow(unused_mut)]

use fltk::{
    app, button::*, frame::Frame, group::Pack, output::Output, prelude::*, tree::*, window::Window,
};
use fltk::{prelude::*, *};
extern crate chrono;
use crate::fs::*;
use chrono::prelude::*;
use filetime::FileTime;
use fltk::enums::{Color, FrameType};
use fltk_theme::{color_themes, ColorTheme};
use fs_extra::dir::get_size;
use std::collections::HashMap;
use std::{env, string};
use std::fs;
use std::io;
use std::path::Path;
use walkdir::WalkDir;
//use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Tree_view,
    size,
    time,
    duplicate,
    delete_duplicate,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(1200, 800)
        .with_label("Disk Analyzer");
    wind.set_color(Color::from_rgb(160, 160, 160));
    let mut pack = Pack::default().with_size(400, 400).with_pos(40, 40);
    pack.set_spacing(10);
    let mut frame1 = Frame::default()
        .with_size(40, 40)
        .with_label("Enter Directory Name");
    let mut inp1 = input::Input::default().with_size(0, 40);
    //inp1.set_value("/home/maram/OSProject");
    let mut but_inc = ReturnButton::default()
        .with_size(0, 40)
        .with_label("Sort by size");
    let mut button_time = Button::default()
        .with_size(0, 40)
        .with_label("Sort by time of modification ");
    let mut button_duplicate = Button::default()
        .with_size(0, 40)
        .with_label("Find duplicates");
    let mut button_tree_view = Button::default()
        .with_size(0, 40)
        .with_label("View as a Tree");
    let mut button_delete_duplicate = Button::default()
        .with_size(0, 40)
        .with_label("Delete duplicates"); 
    //button_delete_duplicate.hide();
    let mut frame = Frame::default()
        .with_size(0, 40)
        .with_label("")
        .with_pos(450, 450);
    let mut chart = misc::Chart::default().with_size(300, 300);
    but_inc.set_color(Color::from_rgb(127, 129, 131));
    button_time.set_color(Color::from_rgb(127, 129, 131));
    button_duplicate.set_color(Color::from_rgb(127, 129, 131));
    button_tree_view.set_color(Color::from_rgb(127, 129, 131));
    button_delete_duplicate.set_color(Color::from_rgb(127, 129, 131));
    chart.set_type(misc::ChartType::Pie);
    pack.end();
    let mut path_tree = tree::Tree::default().with_size(600, 800).with_pos(550, 0);
    let mut frame1 = Frame::default().with_size(800, 1000);
    frame1.set_label("");
    frame1.set_pos(450, 10);
    wind.end();
    wind.show();


    let mut total = 0.;
    let mut sorted_by_size;
    let mut sorted_by_time;
    let mut find_duplicate_vec;
    let (s, r) = app::channel::<Message>();

    but_inc.emit(s, Message::size);
    button_time.emit(s, Message::time);
    button_duplicate.emit(s, Message::duplicate);
    button_tree_view.emit(s, Message::Tree_view);
    button_delete_duplicate.emit(s,Message::delete_duplicate);
    while app.wait() {
        let label: std::string::String = inp1.value().to_string();
        let label_uw = label.clone();
        let mut label_uw1 = label.clone();
        let label_uw2 = label.clone();
        let label_uw3 = label.clone();
        if let Some(msg) = r.recv() {
            match msg {
                Message::Tree_view => {
                    path_tree.clear();
                    if label_uw1 == "/home/"
                     { label_uw1= "/home/maram/".to_string();
                     }
                    
                    for entry in WalkDir::new(label_uw1.to_string()) {
                        

                        let entry = entry.unwrap();
                        let path = Path::new(entry.path());
                        
                        let name = path
                            .file_stem()
                            .unwrap()
                            .to_os_string()
                            .into_string()
                            .unwrap();
                       
                        if (path.has_root()) {
                            let parent2 = path.parent().unwrap().display().to_string();
                            let entry_data = parent2 + "/" + &name;
                            path_tree.add(&entry_data);
                           
                        }
                    }
                }
                Message::size => {
                    chart.clear();

                    sorted_by_size = sort_by_size(label_uw);
                    total = 0.;
                    for item in &sorted_by_size {
                        total += item.0;
                    }

                    chart.set_bounds(0.0, total);
                    chart.set_text_size(18);
                    let mut i = 0;
                    for item in &sorted_by_size {
                        i += 100000;
                        chart.add(
                            item.0,
                            &item.1.to_string(),
                            enums::Color::from_u32(0xcc9c59 + i),
                        );
                    }

                    chart.redraw();
                }

                Message::time => {
                    path_tree.clear();
                    frame1.set_color(Color::Blue);
                    sorted_by_time = sort_by_time(label_uw1);

                    let join_vector_modification_time = sorted_by_time.concat();

                    let message = if label_uw.as_str().is_empty() {
                        &"No directory entered"
                    } else {
                        join_vector_modification_time.as_str()
                    };
                    for i in &sorted_by_time {
                        path_tree.add(i);
                    }
                }

                Message::duplicate => {
                    path_tree.clear();
                    find_duplicate_vec = find_duplicate(label_uw2);
                    let join_vector_duplicate = find_duplicate_vec.concat();
                    let message1 = if label_uw.as_str().is_empty() {
                        &"No directory entered"
                    } else {
                        join_vector_duplicate.as_str()
                    };
                    for i in &find_duplicate_vec {
                        path_tree.add(i);
                    }
                }
                Message::delete_duplicate =>{
                    let selected_stuff = path_tree.get_selected_items().unwrap();
                    for item in selected_stuff{
                        let mut item2= item.label().unwrap();
                       let mut path_to_delete = find_duplicate_of_item(label_uw3.to_string(), item2);
                       for item3 in path_to_delete{
                        remove_file(item3);
                       }

                    }
                    

                }
            }
        }
    }

    Ok(())
}

fn sort_by_size(label: std::string::String) -> Vec<(f64, std::string::String)> {
    let mut file_and_size_vec = vec![];
    for entry in WalkDir::new(label) {
        match entry {
            Ok(entry) => {
                {
                    let path = entry.path();
                    let folder_data_size = get_size(path);
                    match folder_data_size {
                        Ok(folder_data_size) => {
                            let name = path
                                .file_stem()
                                .unwrap()
                                .to_os_string()
                                .into_string()
                                .unwrap();
                            let metadata_d = fs::metadata(path).unwrap();
                            if metadata_d.is_dir() {
                                if folder_data_size > 800000000 {
                                    file_and_size_vec.push((folder_data_size as f64, name));
                                }
                            }
                        }
                        Err(err) => {
                        }
                    }
                }
            }
            Err(err) => {
                let path = err.path().unwrap_or(Path::new("")).display();
                if let Some(inner) = err.io_error() {
                    match inner.kind() {
                        io::ErrorKind::InvalidData => {
                        }
                        io::ErrorKind::PermissionDenied => {
                        }
                        io::ErrorKind::NotFound => {
                        }
                        _ => {
                        }
                    }
                }
            }
        }
    }
    return file_and_size_vec;
}
fn find_duplicate(message_input_duplicate: std::string::String) -> Vec<std::string::String> {
    let mut filenames = HashMap::new();
    let mut files_duplicates = vec![];
    for entry in WalkDir::new(message_input_duplicate)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let f_name = String::from(entry.file_name().to_string_lossy());
        let counter = filenames.entry(f_name.clone()).or_insert(0);
        *counter += 1;

        if *counter == 2 {
            files_duplicates.push(f_name);
            files_duplicates.push("\n".to_string());
        }
    }
    files_duplicates
}

fn sort_by_time(message_input_fn: std::string::String) -> Vec<std::string::String> {
    let mut file_and_time_vec = vec![];
    let mut file_and_time_vec_return = vec![];
    for entry in WalkDir::new(message_input_fn) {
        match entry {
            Ok(entry) => {
                {
                    let path = entry.path();
                    let metadata_d = fs::metadata(path);
                    match metadata_d {
                        Ok(metadata_d) => {
                            let name = path
                                .file_stem()
                                .unwrap()
                                .to_os_string()
                                .into_string()
                                .unwrap();
                            let file_time = FileTime::from_last_modification_time(&metadata_d);
                            let timestamp = file_time.seconds();
                            let naive = NaiveDateTime::from_timestamp(timestamp, 0);
                            let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

                            let newdate = datetime.format("%Y-%m-%d %H:%M:%S");

                            file_and_time_vec.push((timestamp, newdate, name));
                        }
                        Err(err) => {
                        }
                    }
                }
            }
            Err(err) => {
                let path = err.path().unwrap_or(Path::new("")).display();
                if let Some(inner) = err.io_error() {
                    match inner.kind() {
                        io::ErrorKind::InvalidData => {
                            //println!("entry contains invalid data: {}",inner)
                        }
                        io::ErrorKind::PermissionDenied => {
                            //println!( "Missing permission to read entry: {}",inner)
                        }
                        io::ErrorKind::NotFound => {
                            //println!( "NotFound: {}",inner)
                        }
                        _ => {
                            //println!("Unexpected error occurred: {}",inner)
                        }
                    }
                }
            }
        }
    }

    file_and_time_vec.sort_by(|b, a| b.0.cmp(&a.0));
    for item in file_and_time_vec {
        let message_to_tree = item.2.to_string() + "\t" + &item.1.to_string() + "+2 GMT" + "\n";
        file_and_time_vec_return.push(message_to_tree);
    }
    file_and_time_vec_return
}

fn find_duplicate_of_item(path_dir: std::string::String, item_duplicate: std::string::String) -> Vec<std::string::String> {
    let mut find_duplicate_of_item = vec![];
    for entry in WalkDir::new(path_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir()) {
        let f_name = String::from(entry.file_name().to_string_lossy());
        if f_name==item_duplicate {
            find_duplicate_of_item.push(entry.path().as_os_str().to_str().unwrap().to_string());
        }
    }
    find_duplicate_of_item
}
