use gtk::prelude::*;
//use gtk::{Application,
use gtk::ApplicationWindow;
use gtk::Button;

use std::fs;
use std::env;
use std::fmt;

const APPLICATION_ID: &str = "org.undisclosedlocation.author-notes";
const APPLICATION_TITLE: &str = "Author Notes";

const BASE_FOLDER: &str = ".author_notes/";


fn main() {
    setup_structure();
    
    let application = gtk::Application::builder().application_id(APPLICATION_ID).build();
  //      gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}


fn build_ui(application: &gtk::Application) {

    let button = Button::builder().label("Press me!").margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(move |button| {
        print();
    });

    
    
    
    let window = ApplicationWindow::builder().application(application).title(APPLICATION_TITLE).child(&button).build();
    window.present();
}

fn print() {
    println!("{}/{}", env::home_dir().expect("test").display(), BASE_FOLDER);
}

fn setup_structure() {

    fs::create_dir_all(format!("{}/{}", env::home_dir().expect("test").display(), BASE_FOLDER));
}
