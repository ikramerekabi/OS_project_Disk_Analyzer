#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(deprecated)]
extern crate chrono; 
use walkdir::WalkDir;
//use std::error::Error;
use std::path::Path;
//use filesize::PathExt;
use std::fs;
use filetime::FileTime;
use fs_extra::dir::get_size;
use std::collections::HashMap;
use chrono::prelude::*;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Label, Image, Orientation};
use std::env;
//->Result <(), Box<dyn Error>>

fn main()  {
let app = Application::new(
Some ("com.danlogs.disk"),
gio::ApplicationFlags::FLAGS_NONE,
).expect("Failed to initialize GTK.");

app.connect_activate(|app|  {
let glade_src = include_str!("../layout.glade");
let builder = gtk::Builder::from_string(glade_src);
let window :gtk::Window = builder.get_object("application_window1").unwrap();
window.set_application(Some(app));
let message_input : gtk::Entry = builder.get_object("message_input_entry").unwrap();
let message_input_clone1= message_input.clone();
let message_input_clone2= message_input.clone();
let by_size_button: gtk::Button = builder.get_object("sort_by_size").unwrap();
let time_of_modification_button: gtk::Button = builder.get_object("sort_by_time").unwrap();
let find_duplicate_button: gtk::Button = builder.get_object("find_duplicate").unwrap();
let message_output: gtk::Label = builder.get_object("message_output").unwrap();
let message_output_clone1= message_output.clone();
let message_output_clone2= message_output.clone();


//Button By Size
by_size_button.connect_clicked(move |_| {
let message_gstring = message_input.get_text();
let sorted_by_size = sort_by_size();
let mut join_vector = vec![];
for item in &sorted_by_size
{
join_vector.push(item.1.to_string());
join_vector.push("\t".to_string());
join_vector.push(item.0.to_string());
join_vector.push(" Bytes".to_string());
join_vector.push("\n".to_string());
}
let join_vector_bysize = join_vector.concat();
let message = if message_gstring.as_str().is_empty() { &"No directory entered" } else { join_vector_bysize.as_str() };
message_output.set_text(&format!("{}",message));


});
//Button By Time of Modification
time_of_modification_button.connect_clicked(move |_| {
let message_gstring = message_input_clone1.get_text();
let sorted_by_modification_time = sort_by_time();
let join_vector_modification_time = sorted_by_modification_time.concat();
let message = if message_gstring.as_str().is_empty() { &"No directory entered" } else { join_vector_modification_time.as_str() };
message_output_clone1.set_text(&format!("{}",message));


});

find_duplicate_button.connect_clicked(move |_| {
let message_gstring = message_input_clone2.get_text();
let duplicates = find_duplicate();
let join_vector_duplicates= duplicates.concat();
let message = if message_gstring.as_str().is_empty() { &"No directory entered" } else { join_vector_duplicates.as_str() };
message_output_clone2.set_text(&format!("{}",message));


});

window.set_title("Disk Analyzer");
window.set_default_size(350,70);
window.show_all();

});
app.run(&env::args().collect::<Vec<_>>());


//sort_by_size();

//sort_by_time();
//find_duplicate();
//Ok() ;
}



fn sort_by_size()->Vec<(u64, std::string::String)>{

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
   	file_and_size_vec.push((folder_data_size,name ));
   }
}

file_and_size_vec.sort_by(|a,b| b.0.cmp(&a.0));

//for item in file_and_size_vec
//{
//println!("{}\n{} bytes\n", item.1, item.0);
//}
file_and_size_vec
}



fn sort_by_time()->Vec<std::string::String>{

let mut file_and_time_vec = vec![];
let mut file_and_time_vec_return = vec![];
  for entry in WalkDir::new("/home/mohamadalzarif/OS") {
    
    let entry = entry.unwrap();
    let path = Path::new(entry.path());
    let name = path.file_stem().unwrap().to_os_string().into_string().unwrap();
    let metadata_d = fs::metadata(path).unwrap();
    
    let file_time= FileTime::from_last_modification_time(&metadata_d); 
    let timestamp = file_time.seconds();
   let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    //let naive = from_timestamp_opt(timestamp, 0);
    // Create a normal DateTime from the NaiveDateTime
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    
    // Format the datetime how you want
    let newdate = datetime.format("%Y-%m-%d %H:%M:%S");
    
  // if metadata_d.is_dir()
   //{
   	file_and_time_vec.push((timestamp,newdate,name));
   //}
}

file_and_time_vec.sort_by(|b,a| b.0.cmp(&a.0));
for item in file_and_time_vec
{
println!("{}\n{} +2 GMT\n", item.2, item.1);
file_and_time_vec_return.push(item.2.to_string());
file_and_time_vec_return.push("\t".to_string());
file_and_time_vec_return.push(item.1.to_string());
file_and_time_vec_return.push("+2 GMT".to_string());
file_and_time_vec_return.push("\n".to_string());
}
file_and_time_vec_return
}

fn find_duplicate()->Vec<std::string::String>{

let mut filenames = HashMap::new();
let mut files_duplicates = vec![];
for entry in WalkDir::new(".")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir()) {
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
