use eframe::egui;
use meval::eval_str;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Default)]
struct CalculatorApp {
    input: String,
    result: String,
    history: Vec<String>,
}

impl CalculatorApp {
    fn load_history(&mut self) {
        if let Ok(data) = fs::read_to_string("history.json") {
            if let Ok(history) = serde_json::from_str::<Vec<String>>(&data) {
                self.history = history;
            }
        }
    }

    fn save_history(&self) {
        if let Ok(data) = serde_json::to_string(&self.history) {
            let _ = fs::write("history.json", data);
        }
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Калькулятор");

            ui.horizontal(|ui| {
                ui.label("Введіть вираз:");
                ui.text_edit_singleline(&mut self.input);
            });

            ui.horizontal(|ui| {
                for num in 0..=9 {
                    if ui.button(num.to_string()).clicked() {
                        self.input.push_str(&num.to_string());
                    }
                }
            });

            ui.horizontal(|ui| {
                if ui.button("+").clicked() {
                    self.input.push('+');
                }
                if ui.button("-").clicked() {
                    self.input.push('-');
                }
                if ui.button("*").clicked() {
                    self.input.push('*');
                }
                if ui.button("/").clicked() {
                    self.input.push('/');
                }
                if ui.button("(").clicked() {
                    self.input.push('(');
                }
                if ui.button(")").clicked() {
                    self.input.push(')');
                }
            });

            if ui.button("Розрахувати").clicked() {
                self.result = match eval_str(&self.input) {
                    Ok(res) => {
                        if res.is_infinite() {
                            "Помилка: Ділення на нуль!".to_string()
                        } else {
                            res.to_string()
                        }
                    }
                    Err(_) => "Помилка у виразі!".to_string(),
                };

                if !self.input.is_empty() && !self.result.starts_with("Помилка") {
                    let entry = format!("{} = {}", self.input, self.result);
                    self.history.push(entry);
                    self.save_history();
                }
            }

            if ui.button("Очистити").clicked() {
                self.input.clear();
                self.result.clear();
            }

            ui.label(format!("Результат: {}", self.result));

            ui.label("Історія:");
            for entry in self.history.iter().rev() {
                ui.label(entry);
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Калькулятор",
        options,
        Box::new(|_cc| {
            let mut app = CalculatorApp::default();
            app.load_history();
            Ok(Box::new(app) as Box<dyn eframe::App>)
        }),
    )
}
