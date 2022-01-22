use std::fs::{ OpenOptions };
use std::process::Command;
use serde::{ Deserialize, Deserializer };
//use serde_json::Result;
use gtk::prelude::*;
use gtk::{Align, Application, ApplicationWindow, Button, Label, Orientation};
use webbrowser;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(remote = "EntryAction")]
enum EntryAction {
    OpenUrl,
    LaunchScript,
}

impl<'de> Deserialize<'de> for EntryAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        if s == "openurl" {
            Ok(EntryAction::OpenUrl)
        } else {
            //State::deserialize(s.into_deserializer())
            Ok(EntryAction::LaunchScript)
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Entry {
    title: String,
    action: EntryAction,
    description: String,
    path: String,
}

 fn read_config_json() -> Vec<Entry> {
     let file = OpenOptions::new().read(true).open("entries.json").unwrap();
     let entries:Vec<Entry> = serde_json::from_reader(&file)
         .expect("error while reading entries");
     return entries;
 }

fn main() {
    let application = Application::new(
        Some("de.gibbs.launcher"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &Application) {
    let entries = read_config_json();
    let window = ApplicationWindow::builder()
        .application(application)
        .title("Launcher")
        .default_width(350)
        .default_height((70 * entries.len()).try_into().unwrap())
        .build();

    let grid = gtk::Grid::builder()
        .margin_start(6)
        .margin_end(6)
        .margin_top(6)
        .margin_bottom(6)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .row_spacing(6)
        .column_spacing(6)
        .build();

    window.set_child(Some(&grid));

    let mut row = 0;
    for e in entries.iter() {
        let label = Label::builder()
            .label(&e.description)
            .build();
        let gtk_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .spacing(12)
            .halign(Align::Center)
            .build();
        let button = Button::builder().label(&e.title).build();
        let entry = e.clone();
        button.connect_clicked(move |_button| {
            match entry.action {
                EntryAction::OpenUrl => {
                    println!("Open: {}", &entry.path);
                    if webbrowser::open(&entry.path).is_err() {
                        println!("Cannot open browser");
                    }
                },
                EntryAction::LaunchScript => {
                    println!("Launch: {}", &entry.path);
                    Command::new("cmd")
                        .args(&["/C", &entry.path]);
                        //.expect("failed to execute process");
                }
            }
        });
        gtk_box.append(&button);
        gtk_box.append(&label);
        grid.attach(&gtk_box, 0, row, 1, 1);
        row = row + 1;
    }

    window.show();
}
