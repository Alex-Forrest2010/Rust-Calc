extern crate eframe;
use eframe::egui;
struct App {
  output: i32,
}

impl Default for App {
  fn default() -> Self {
    Self {
      output: 0,
    }
  }
}

impl eframe::App for App {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    let Self { output } = self;
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.label(output.to_string());
      ui.horizontal(|ui| {
        if ui.button("1").clicked() {
          *output += 1;
        }
        if ui.button("2").clicked() {
          *output += 2;
        }
        if ui.button("3").clicked() {
          *output += 3;
        }
      });
      ui.horizontal(|ui| {
        if ui.button("4").clicked() {
          *output += 4;
        }
        if ui.button("5").clicked() {
          *output += 5;
        }
        if ui.button("6").clicked() {
          *output += 6;
        }
      });
      ui.horizontal(|ui| {
        if ui.button("7").clicked() {
          *output += 7;
        }
        if ui.button("8").clicked() {
          *output += 8;
        }
        if ui.button("9").clicked() {
          *output += 9;
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
