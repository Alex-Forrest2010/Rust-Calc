extern crate eframe;
use eframe::egui;

/*
TODO:
  When a button is pressed, add that value/operator to a dataset(array/list)
  make a function to parse this dataset into instructions
  make a function to interpret and execute the instructions
*/


struct App<> {
  output: String,
  rawInput: Vec<i32>,
  inputCounter: u32
}

impl Default for App<> {
  fn default() -> Self {
    Self {
      output: String::from(""),
      rawInput: [].to_vec(),
      inputCounter: 0
    }
  }
}

fn parse(arr: Vec<String>) {

}

impl eframe::App for App<> {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    let Self {
      output,
      rawInput,
      inputCounter,
    } = self;
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.label(output.to_string());
      ui.horizontal(|ui| {
        if ui.button("1").clicked() {
          *output = output.to_owned() + "1";
          rawInput.push(1)
        }
        if ui.button("2").clicked() {
          *output = output.to_owned() + "2";
          rawInput.push(2);
        }
        if ui.button("3").clicked() {
          *output = output.to_owned() + "3";
          rawInput.push(3)
        }
        if ui.button("+").clicked(){
        }
      });
      ui.horizontal(|ui| {
        if ui.button("4").clicked() {
          *output = output.to_owned() + "4";
          rawInput.push(4)
        }
        if ui.button("5").clicked() {
          *output = output.to_owned() + "5";
          rawInput.push(5);
        }
        if ui.button("6").clicked() {
          *output = output.to_owned() + "6";
          rawInput.push(6);
        }
        if ui.button("-").clicked(){
        }
      });
      ui.horizontal(|ui| {
        if ui.button("7").clicked() {
          *output = output.to_owned() + "7";
          rawInput.push(7);
        }
        if ui.button("8").clicked() {
          *output = output.to_owned() + "8";
          rawInput.push(8);
        }
        if ui.button("9").clicked() {
          *output = output.to_owned() + "9";
          rawInput.push(9);
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
