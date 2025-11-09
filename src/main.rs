extern crate eframe;
use eframe::egui;

/*
TODO:
  When a button is pressed, add that value/operator to a dataset(array/list)
  make a function to parse this dataset into instructions
  make a function to interpret and execute the instructions

*/


struct App<'a> {
  output: i32,
  mode: &'a str
}

impl Default for App<'_> {
  fn default() -> Self {
    Self {
      output: 0,
      mode: "add"
    }
  }
}

impl eframe::App for App<'_> {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    let Self {
      output,
      mode,
    } = self;
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.label(output.to_string());
      ui.horizontal(|ui| {
        if ui.button("1").clicked() {
          match *mode {
            "add" => *output += 1,
            "subtract" => *output -= 1,
            &_ => println!("Not a mode"),
          }
        }
        if ui.button("2").clicked() {
          match *mode {
            "add" => *output += 2,
            "subtract" => *output -= 2,
            &_ => println!("Not a mode"),
          }
        }
        if ui.button("3").clicked() {
          match *mode {
            "add" => *output += 3,
            "subtract" => *output -= 3,
            &_ => println!("Not a mode"),
          }
        }
        if ui.button("+").clicked(){
          *mode = "add";
        }
      });
      ui.horizontal(|ui| {
        if ui.button("4").clicked() {
          match *mode {
            "add" => *output += 4,
            "subtract" => *output -= 4,
            &_ => println!("Not a mode"),
          }
        }
        if ui.button("5").clicked() {
          match *mode {
            "add" => *output += 5,
            "subtract" => *output -= 5,
            &_ => println!("Not a mode"),
          }
        }
        if ui.button("6").clicked() {
          match *mode {
            "add" => *output += 6,
            "subtract" => *output -= 6,
            &_ => println!("Not a mode"),
          }
        }
        if ui.button("-").clicked(){
          *mode = "subtract";
        }
      });
      ui.horizontal(|ui| {
        if ui.button("7").clicked() {
          match *mode {
            "add" => *output += 7,
            "subtract" => *output -= 7,
            &_ => println!("Not a mode"),
          }
        }
        if ui.button("8").clicked() {
          match *mode {
            "add" => *output += 8,
            "subtract" => *output -= 8,
            &_ => println!("Not a mode"),
          }
        }
        if ui.button("9").clicked() {
          match *mode {
            "add" => *output += 9,
            "subtract" => *output -= 9,
            &_ => println!("Not a mode"),
          }
        }
      });
    });
  }
}

fn main() -> Result<(), eframe::Error> {
  let options = eframe::NativeOptions {
    vsync: false,
    ..Default::default()
  };
  eframe::run_native(
    "Calculator",
    options,
    Box::new(|_cc| Ok(Box::new(App::default())))
  )
}
