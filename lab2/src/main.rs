use eframe::egui;
use meval::eval_str;

#[derive(Default)]
struct CalculatorApp {
    input: String,
    result: String,
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Калькулятор");

            // Поле для введення виразу
            ui.horizontal(|ui| {
                ui.label("Введіть вираз:");
                ui.text_edit_singleline(&mut self.input);
            });

            // Віртуальна клавіатура
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

            // Кнопка "Розрахувати"
            if ui.button("Розрахувати").clicked() {
                self.result = match eval_str(&self.input) {
                    Ok(res) => {
                        if res.is_infinite() {
                            "Помилка: Ділення на нуль!".to_string()
                        } else {
                            res.to_string()
                        }
                    }
                    Err(_) => "Помилка в виразі!".to_string(),
                };
            }

            // Кнопка "Очистити"
            if ui.button("Очистити").clicked() {
                self.input.clear();
                self.result.clear();
            }

            // Виведення результату
            ui.label(format!("Результат: {}", self.result));
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Калькулятор",
        options,
        Box::new(|_cc| Ok(Box::new(CalculatorApp::default()) as Box<dyn eframe::App>))
    )
}
