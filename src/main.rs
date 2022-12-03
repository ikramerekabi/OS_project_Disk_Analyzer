
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(deprecated)]
use fltk::{prelude::*, *};
extern crate chrono; 
use walkdir::WalkDir;
use std::path::Path;
use std::fs;
use filetime::FileTime;
use fs_extra::dir::get_size;
use std::collections::HashMap;
use chrono::prelude::*;
use std::env;
fn main() {


    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::new(100, 100, 1000, 800, "Charts");
    let mut chart = misc::Chart::default().size_of_parent();
    chart.set_type(misc::ChartType::Pie);
    let sorted_by_size = sort_by_size();
    let mut total=0.0;
    
     for item in &sorted_by_size
{
total+=item.0;
}
    
    chart.set_bounds(0.0, total);
    chart.set_text_size(18);
    for item in &sorted_by_size
{
chart.add(item.0, &item.1.to_string(), enums::Color::Red);
}
    chart.set_color(enums::Color::White);
    let mut choice = menu::Choice::new(300, 5, 200, 40, "Chart type");
    choice.add_choice("Bar | HorzBar | Line | Fill | Spike | Pie | SpecialPie");
    choice.set_value(5);
    choice.set_color(enums::Color::White);
    win.end();
    win.show();

    choice.set_callback(move |c| {
        chart.set_type(misc::ChartType::from_i32(c.value()));
        chart.redraw();
    });

    app.run().unwrap();
}




fn sort_by_size()->Vec<(f64, std::string::String)>{

let mut file_and_size_vec = vec![];
  for entry in WalkDir::new("/home/mohamadalzarif/OS") {
    
    let entry = entry.unwrap();
    let path = Path::new(entry.path());
    //let file_data_size = fs::metadata(entry.path())?.len();
    let folder_data_size = get_size(path).unwrap();
    let name = path.file_stem().unwrap().to_os_string().into_string().unwrap();
    let metadata_d = fs::metadata(path).unwrap();
    
   if metadata_d.is_dir()
   {
   	file_and_size_vec.push((folder_data_size as f64,name ));
   }
}
file_and_size_vec
}
