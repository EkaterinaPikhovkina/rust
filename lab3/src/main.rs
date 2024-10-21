use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read}; // Видаляємо Write

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    name: String,
    done: bool,
}

#[derive(Default)]
struct TodoApp {
    tasks: Vec<Task>,
    new_task: String,
    edited_task: Option<usize>, // Зберігає індекс редагованого завдання
}

impl TodoApp {
    // Зберегти список завдань у файл
    fn save_tasks(&self) {
        if let Ok(file) = OpenOptions::new().create(true).write(true).truncate(true).open("tasks.json") {
            serde_json::to_writer(file, &self.tasks).expect("Не вдалось записати у файл");
        }
    }

    // Завантажити список завдань із файлу
    fn load_tasks(&mut self) {
        if let Ok(mut file) = File::open("tasks.json") {
            let mut data = String::new();
            file.read_to_string(&mut data).expect("Не вдалось прочитати файл");
            self.tasks = serde_json::from_str(&data).unwrap_or_default();
        }
    }

    // Додати нове завдання
    fn add_task(&mut self) {
        if !self.new_task.is_empty() {
            self.tasks.push(Task {
                name: self.new_task.clone(),
                done: false,
            });
            self.new_task.clear();
            self.save_tasks();
        }
    }

    // Видалити завдання
    fn delete_task(&mut self, index: usize) {
        self.tasks.remove(index);
        self.save_tasks();
    }

    // Позначити завдання як виконане/невиконане
    fn toggle_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.done = !task.done;
            self.save_tasks();
        }
    }

    // Почати редагування завдання
    fn start_editing(&mut self, index: usize) {
        self.edited_task = Some(index);
        self.new_task = self.tasks[index].name.clone();
    }

    // Завершити редагування завдання
    fn finish_editing(&mut self) {
        if let Some(index) = self.edited_task {
            if let Some(task) = self.tasks.get_mut(index) {
                task.name = self.new_task.clone();
            }
            self.edited_task = None;
            self.new_task.clear();
            self.save_tasks();
        }
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Список завдань");

            // Копіюємо завдання для уникнення подвійного позичення self
            let tasks: Vec<(usize, Task)> = self.tasks.iter().enumerate().map(|(i, t)| (i, t.clone())).collect();

            // Виведення існуючих завдань
            for (index, task) in tasks.iter() {
                ui.horizontal(|ui| {
                    let mut done = task.done;
                    let check = ui.checkbox(&mut done, "");
                    if check.clicked() {
                        self.toggle_task(*index); // Використовуємо toggle_task з індексом
                    }

                    if self.edited_task == Some(*index) {
                        ui.text_edit_singleline(&mut self.new_task);
                        if ui.button("Зберегти").clicked() {
                            self.finish_editing();
                        }
                    } else {
                        ui.label(if task.done {
                            egui::RichText::new(&task.name).strikethrough()
                        } else {
                            egui::RichText::new(&task.name)
                        });
                        if ui.button("Редагувати").clicked() {
                            self.start_editing(*index);
                        }
                    }

                    if ui.button("Видалити").clicked() {
                        self.delete_task(*index);
                    }
                });
            }

            ui.separator();

            // Введення нового завдання або редагування
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.new_task);
                if self.edited_task.is_none() && ui.button("Додати").clicked() {
                    self.add_task();
                }
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let mut app = TodoApp::default();
    app.load_tasks(); // Завантаження завдань із файлу під час запуску
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Список завдань",
        options,
        Box::new(|_cc| Ok(Box::new(app)))
    )
}
