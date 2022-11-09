use gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};


use gtk::Button;
 use gtk::Label;

use std::time::Duration;
use std::thread;
use std::process::Command;
use std::str::from_utf8;


fn main() {


    let app = Application::builder()
        .application_id("xyz.toastxc.jan-gae")
        .build();


    app.connect_activate(gui);

    app.run();
}


fn gui(application: &Application) {

    let b_template = Button::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12);



    let internet_button = b_template.clone()
        .label("status of jan's torrents")
        .build();



    //let label = Label::new(Some("t"));

    internet_button.connect_clicked(move |internet_button| {
        let cmd_res = Command::new("sh")
            .arg("payload.sh")
            .output();


        let cmd = match cmd_res {

            Ok(o) => from_utf8(&o.stdout).unwrap().to_string(),
            Err(e) => e.to_string()
        };
 
        println!("{cmd}");
        internet_button.set_label(&cmd);


    
    });



     let gtk_box = gtk::Box::builder()
         .build();

     gtk_box.append(&internet_button);



     let window = ApplicationWindow::builder()
        .application(application)
        .title("Torrent Stat")
        .child(&gtk_box)
        .build();

       window.present();

}
