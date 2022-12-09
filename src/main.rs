#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(deprecated)]
#![allow(unused_mut)]

use fltk::{
    app, button::*, dialog::*, frame::*, group::*, input::*, output::*, prelude::*, tree::*,
    window::Window,
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
use std::*;
use std::io;
use std::path::*;
use walkdir::WalkDir;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    CreateDir,
    TreeView,
    Size,
    Time,
    Duplicate,
    DeleteDuplicate,
    RenameFile,
}

pub fn show_dialog() -> MyDialog {
    MyDialog::default()
}

pub fn show_dialog_addition(window_text: string::String,
    screen_message: string::String,
    enter_message: string::String,
) -> MyDialog {
    MyDialog::custom(window_text,screen_message,enter_message)
}
pub struct MyDialog {
    inp: input::Input,
}

impl MyDialog {
    pub fn default() -> Self {
        let mut win = window::Window::default()
            .with_size(400, 100)
            .with_label("Create File");
        win.set_color(Color::from_rgb(240, 240, 240));
        let mut pack1 = group::Pack::default()
            .with_size(300, 30)
            .center_of_parent()
            .with_type(group::PackType::Vertical);
            pack1.set_spacing(20);
            frame::Frame::default().with_size(80, 0).with_label("Directory will be created at\n current traversed directory location.");
        let mut pack = group::Pack::default()
            .with_size(300, 30)
            .center_of_parent()
            .with_type(group::PackType::Horizontal);
        
        pack.set_spacing(20);

        frame::Frame::default()
            .with_size(80, 0)
            .with_label("Enter name:");

        let mut inp = input::Input::default().with_size(100, 0);
        inp.set_frame(FrameType::FlatBox);
        let mut ok = button::Button::default().with_size(80, 0).with_label("Ok");

        pack.end();
        pack1.end();
        win.end();
        win.make_modal(true);
        win.show();
        ok.set_callback({
            let mut win = win.clone();
            move |_| {
                win.hide();
            }
        });
        while win.shown() {
            app::wait();
        }
        Self { inp }
    }
    
    pub fn custom(window_text: string::String,
         screen_message: string::String,
        enter_message: string::String,) -> Self {
        let mut win = window::Window::default()
            .with_size(500, 200)
            .with_label(&window_text);
        win.set_color(Color::from_rgb(240, 240, 240));
        let mut pack1 = group::Pack::default()
            .with_size(300, 100)
            .center_of_parent()
            .with_type(group::PackType::Vertical);
            pack1.set_spacing(20);
            frame::Frame::default().with_size(80, 0).with_label(&screen_message);
        let mut pack = group::Pack::default()
            .with_size(300, 30)
            .with_type(group::PackType::Horizontal);
        
        pack.set_spacing(100);

        frame::Frame::default()
            .with_size(80, 0)
            .with_label(&enter_message)
            .with_pos(770, 50);
        let mut inp = input::Input::default().with_size(100, 0).with_pos(770, 50);
        inp.set_frame(FrameType::FlatBox);
        

        pack.end();
        let mut ok = button::Button::default().with_size(80, 40).with_label("Ok");
        pack1.end();
        win.end();
        win.make_modal(true);
        win.show();
        ok.set_callback({
            let mut win = win.clone();
            move |_| {
                win.hide();
            }
        });
        while win.shown() {
            app::wait();
        }
        Self { inp }
    }

    pub fn value(&self) -> String {
        self.inp.value()
    }
}

//////
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(1300, 800)
        .with_label("Disk Analyzer");
    wind.set_color(Color::from_rgb(160, 160, 160));
    let mut inp1 = input::Input::default().with_size(250, 40).with_pos(20, 50);
    let mut exclude_filetype_input = input::Input::default().with_size(250, 40).with_pos(20, 100);
    
    let mut but_inc: Button = Button::default()
        .with_size(150, 40)
        .with_label("Sort by size")
        .with_pos(380, 50);
    let mut button_time: Button = Button::default()
        .with_size(220, 40)
        .with_label("Sort by time of modification ")
        .with_pos(540, 50);
    let mut button_tree_view: Button = Button::default()
        .with_size(150, 40)
        .with_label("View as a Tree")
        .with_pos(770, 50);
    let mut button_duplicate: Button = Button::default()
        .with_size(150, 40)
        .with_label("Find duplicates")
        .with_pos(930, 50);

    let mut button_delete_duplicate: Button = Button::default()
        .with_size(150, 40)
        .with_label("Delete duplicates")
        .with_pos(1090, 50);
    let mut create_dir: Button = Button::default()
        .with_size(150, 40)
        .with_label("Create Directory")
        .with_pos(380, 100);
    let mut RenameFile: Button = Button::default()
        .with_size(220, 40)
        .with_label("Rename File")
        .with_pos(540, 100);
    let mut frame = Frame::default()
        .with_size(0, 40)
        .with_label("")
        .with_pos(800, 200);

    let mut chart = misc::Chart::default()
        .with_size(600, 550)
        .with_pos(660, 160);
    but_inc.set_color(Color::from_rgb(127, 129, 131));
    button_time.set_color(Color::from_rgb(127, 129, 131));
    button_duplicate.set_color(Color::from_rgb(127, 129, 131));
    button_tree_view.set_color(Color::from_rgb(127, 129, 131));
    button_delete_duplicate.set_color(Color::from_rgb(127, 129, 131));
    create_dir.set_color(Color::from_rgb(127, 129, 131));
    RenameFile.set_color(Color::from_rgb(127, 129, 131));
    chart.set_type(misc::ChartType::Pie);
    let mut path_tree = tree::Tree::default().with_size(600, 550).with_pos(30, 160);
    let mut last_modified_date = output::Output::default().with_size(250, 40).with_pos(180, 730);
    let mut last_modified_date2 = last_modified_date.clone();
    //last_modified_date2.hide();
    last_modified_date2.set_label("Last Modified Date: ");
    
    let mut path_tree2 = path_tree.clone();
    let mut path_tree3 = path_tree.clone();
    let mut path_tree4 = path_tree.clone();
    let mut path_tree6 = path_tree.clone();
    let mut frame1 = Frame::default()
        .with_size(40, 40)
        .with_label("Enter Directory Name")
        .with_pos(120, 20);

    wind.end();
    wind.show();

    let mut total = 0.;
    let mut sorted_by_size;
    let mut sorted_by_time;
    let mut find_duplicate_vec;
    let (s, r) = app::channel::<Message>();

    but_inc.emit(s, Message::Size);
    button_time.emit(s, Message::Time);
    button_duplicate.emit(s, Message::Duplicate);
    button_tree_view.emit(s, Message::TreeView);
    button_delete_duplicate.emit(s, Message::DeleteDuplicate);
    create_dir.emit(s, Message::CreateDir);
    RenameFile.emit(s, Message::RenameFile);
    while app.wait() {
        let label: string::String = inp1.value().to_string();
        let exclude_filetype_label: string::String = exclude_filetype_input.value().to_string();
        let exclude_filetype_label2 = exclude_filetype_label.clone();
        let label_uw = label.clone();
        let mut label_uw1 = label.clone();
        let mut label_uw2 = label.clone();
        let mut label_uw3 = label.clone();
        let mut label_uw4 = label.clone();
        let mut label_uw5 = label.clone();
        let mut path_tree7 = path_tree6.clone();
        let mut path_tree8 = path_tree6.clone();
        let mut path_tree9 = path_tree6.clone();
        let mut last_modified_date3 = last_modified_date2.clone();
        last_modified_date2.set_value(&show_last_modified_time(last_modified_date3,path_tree9,label_uw5,exclude_filetype_label2));
        if let Some(msg) = r.recv() {
            match msg {
                
                Message::TreeView => {
                    let mut path_tree5 = path_tree.clone();
                    path_tree5.clear();
                    tree_view(path_tree5, label_uw1,exclude_filetype_label);
                }
                Message::Size => {
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

                Message::Time => {
                    path_tree2.clear();
                    frame.set_color(Color::Blue);
                    sorted_by_time = sort_by_time(label_uw1);

                    let join_vector_modification_time = sorted_by_time.concat();

                    let message = if label_uw.as_str().is_empty() {
                        &"No directory entered"
                    } else {
                        join_vector_modification_time.as_str()
                    };
                    for i in &sorted_by_time {
                        path_tree2.add(i);
                    }
                }

                Message::Duplicate => {
                    path_tree4.clear();
                    find_duplicate_vec = find_duplicate(label_uw2);
                    let join_vector_duplicate = find_duplicate_vec.concat();
                    let message1 = if label_uw.as_str().is_empty() {
                        &"No directory entered"
                    } else {
                        join_vector_duplicate.as_str()
                    };
                    for i in &find_duplicate_vec {
                        path_tree4.add(i);
                    }
                }
                Message::DeleteDuplicate => {
                    delete_duplicates(path_tree7,label_uw3);
                }
                Message::RenameFile => {
                    let mut frame3 = frame1.clone();
                    let b = show_dialog_addition("Rename".to_string(),
                    "File name will be changed to newly inputed one.".to_string()
                    ,"Enter New File Name".to_string());
                    let selected_stuff = path_tree4.get_selected_items();
                    if !selected_stuff.is_none() {
                        let selected_stuff2 = selected_stuff.unwrap();
                        for item in selected_stuff2 {
                            let mut item2 = item.label().unwrap();
                            let item7 = item2.clone();
                            let mut path_to_rename = item_location(label_uw4.to_string(), item2);
                            for item3 in path_to_rename {
                                let mut original_address = Path::new(&item7);
                                let file_extension = original_address.extension();
                            match file_extension{
                                Some(_) => {
                                    let name_change_new_file = label_uw4.to_string() + "/" + &b.value() + "." + &file_extension.unwrap().to_str().unwrap();
                                    
                                    change_file_name(&item3, &name_change_new_file);
                    }
                    None => {
                    }

                            }
                               
                            }
                        }
                        let mut path_tree10 = path_tree8.clone();
                        path_tree10.clear();
                        tree_view(path_tree10, label_uw4,exclude_filetype_label);
                    } else {
                    }
                }
                Message::CreateDir => {
                    let mut frame2 = frame1.clone();
                    let d = show_dialog_addition("Create File".to_string(),
                    "Directory will be created at\n current traversed directory location.".to_string()
                    ,"Enter New Directory Name".to_string());
                    create_directory(label_uw1, d.value());
                }
            }
        }
    }

    Ok(())
}

fn sort_by_size(label: string::String) -> Vec<(f64, string::String)> {
    let mut file_and_size_vec = vec![];
    for entry in WalkDir::new(label) {
        match entry {
            Ok(entry) => {
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
                    Err(err) => {}
                }
            }
            Err(err) => {
                let path = err.path().unwrap_or(Path::new("")).display();
                if let Some(inner) = err.io_error() {
                    match inner.kind() {
                        io::ErrorKind::InvalidData => {}
                        io::ErrorKind::PermissionDenied => {}
                        io::ErrorKind::NotFound => {}
                        _ => {}
                    }
                }
            }
        }
    }
    return file_and_size_vec;
}
fn find_duplicate(message_input_duplicate: string::String) -> Vec<string::String> {
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

fn sort_by_time(message_input_fn: string::String) -> Vec<string::String> {
    let mut file_and_time_vec = vec![];
    let mut file_and_time_vec_return = vec![];
    for entry in WalkDir::new(message_input_fn) {
        match entry {
            Ok(entry) => {
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
                    Err(err) => {}
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

fn find_duplicate_of_item( path_dir: string::String,   item_duplicate: string::String,) -> Vec<string::String> {
    let mut find_duplicate_of_item = vec![];
    for entry in WalkDir::new(path_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let f_name = String::from(entry.file_name().to_string_lossy());
        println!("{} and {}",f_name,item_duplicate);
        if f_name == item_duplicate {
            find_duplicate_of_item.push(entry.path().as_os_str().to_str().unwrap().to_string());
        }
    }
    find_duplicate_of_item
}

fn tree_view(mut path_tree: Tree, directory_entry: string::String,exclude_extension: string::String,) {
    path_tree.clear();

    for entry in WalkDir::new(directory_entry.to_string()) {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                let metadata_d = fs::metadata(path);
                match metadata_d {
                    Ok(metadata_d) => {
                        let path = Path::new(entry.path());

                        let name = path
                            .file_stem()
                            .unwrap()
                            .to_os_string()
                            .into_string()
                            .unwrap();
                            
                            let f_name = entry.file_name().to_string_lossy();
                            let file_extension = path.extension();
                            match file_extension{
                                Some(_) => {
                                    let file_extensiondot: string::String = ".".to_string() + file_extension.unwrap().to_str().unwrap();
                                    if  f_name.ends_with(&exclude_extension) && !exclude_extension.is_empty() {

                                    }
                                    else{
                        if path.has_root() {
                            let parent2 = path.parent().unwrap().display().to_string();
                            let entry_data = parent2 + "/" + &name + "." + file_extension.unwrap().to_str().unwrap();
                            path_tree.add(&entry_data);
                        }
                        else{
                            println!("{}",path.display());
                        }
                    }
                }
                    None => {
                    }

                            }
                    }
                    Err(err) => {}
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
}

fn create_directory(create_dir_location: string::String, dir_name: string::String) {
    let final_dir_loc = create_dir_location + "/" + &dir_name;
    let copy_of_final_dir = final_dir_loc.clone();
    let create_direction = fs::create_dir(final_dir_loc);
    match create_direction {
        Ok(create_direction) => {
        }

        Err(create_direction) => {
        }
    }
}

fn item_location( path_dir: string::String,   item_name: string::String,) -> Vec<string::String> {
    let mut item_location_vec = vec![];
    for entry in WalkDir::new(path_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let f_name = String::from(entry.file_name().to_string_lossy());
        if f_name == item_name {
            item_location_vec.push(entry.path().as_os_str().to_str().unwrap().to_string());
        }
    }
    item_location_vec
}

fn change_file_name(path2: &str, name: &str) {

    rename(path2, name);
}

fn show_last_modified_time(mut output_button: output::Output,mut path_tree: Tree, mut label_uw4: std::string::String, exclude_filetype_label: std::string::String,) -> string::String{

    
    let mut newdate: std::string::String="".to_string();
    let selected_stuff = path_tree.get_selected_items();
    if !selected_stuff.is_none() {
        let selected_stuff2 = selected_stuff.unwrap();
        for item in selected_stuff2 {
            let mut item2 = item.label().unwrap();
            let item7 = item2.clone();
            let mut path_to_display_modified = item_location(label_uw4.to_string(), item2);
            for item3 in path_to_display_modified {
                let metadata_d = fs::metadata(item3);
                match metadata_d {
                    Ok(metadata_d) => {
                        let file_time = FileTime::from_last_modification_time(&metadata_d);
                        let timestamp = file_time.seconds();
                        let naive = NaiveDateTime::from_timestamp(timestamp, 0);
                        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

                        newdate = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
                        output_button.show();
                    }
                    Err(err) => {}
                }
               
            }
        }
    } else {
        output_button.hide();
        newdate= "".to_string();
    }

 return newdate;
}
fn delete_duplicates(mut path_tree3: Tree, label_uw3: string::String){

    let selected_stuff = path_tree3.get_selected_items();
                    if !selected_stuff.is_none() {
                        let selected_stuff2 = selected_stuff.unwrap();
                        for item in selected_stuff2 {
                            let mut item2 = item.label().unwrap();
                            let mut path_to_delete = find_duplicate_of_item(label_uw3.to_string(), item2);
                            for item3 in path_to_delete {
                                remove_file(item3);
                            }
                        }
                        path_tree3.clear();
                        let find_duplicate_vec2 = find_duplicate(label_uw3);
                        let join_vector_duplicate2 = find_duplicate_vec2.concat();
                        for i in &find_duplicate_vec2 {
                            path_tree3.add(i);
                        }
                    } else {
                    }
}