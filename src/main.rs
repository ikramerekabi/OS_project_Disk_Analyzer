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
//let window = ApplicationWindow::new(app);
//window.set_title("disk");
//window.set_default_size(350,70);
//let layout_box = Box
window.show_all();

});
app.run(&env::args().collect::<Vec<_>>());


//sort_by_size();

//sort_by_time();
//find_duplicate();
//Ok() ;
}
fn sort_by_size(){

let mut file_and_size_vec = vec![];
  for entry in WalkDir::new("/home/maram/disk") {
    
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
for item in file_and_size_vec
{
println!("{}\n{} bytes\n", item.1, item.0);
}

}
fn sort_by_time(){

let mut file_and_time_vec = vec![];
  for entry in WalkDir::new("/home/maram/disk") {
    
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

}
}

fn find_duplicate(){

let mut filenames = HashMap::new();

for entry in WalkDir::new(".")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir()) {
        let f_name = String::from(entry.file_name().to_string_lossy());
        let counter = filenames.entry(f_name.clone()).or_insert(0);
        *counter += 1;

        if *counter == 2 {
            println!("{}", f_name);
        }
    }
}
