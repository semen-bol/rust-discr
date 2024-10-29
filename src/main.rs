extern crate crossterm;

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io;
use console;

fn main() {
    let term = console::Term::stdout();

    loop { // infinity

        let mut a_str = String::new();
        let mut b_str = String::new();
        let mut c_str = String::new();

        term.clear_screen().expect("Не удалось очистить консоль"); // clean console if it use after first usage

        println!("Решить ax^2 + bx + c = 0 уравнение");

        // Get a, b, c numbers
        println!("Введите a (ax^2 + bx + c = 0): ");
        match io::stdin().read_line(&mut a_str) {
            Ok(_) => {}
            Err(e) => println!("Ошибка ввода - {}", e)
        }
        println!("Введите b (ax^2 + bx + c = 0): ");
        match io::stdin().read_line(&mut b_str) {
            Ok(_) => {}
            Err(e) => println!("Ошибка ввода - {}", e)
        }
        println!("Введите c (ax^2 + bx + c = 0): ");
        match io::stdin().read_line(&mut c_str) {
            Ok(_) => {}
            Err(e) => println!("Ошибка ввода - {}", e)
        }

        // delete spaces in a, b, c numbers
        let a: f64 = a_str.trim().parse().unwrap();
        let b: f64 = b_str.trim().parse().unwrap();
        let c: f64 = c_str.trim().parse().unwrap();

        // D = b^2 - 4*(a*c)
        let d: f64 = (b*b) - 4.0 * (a * c);
        
        // clean console to see result better
        term.clear_screen().expect("Не удалось очистить консоль");

        // x1, x2 find or empty
        println!("a: {}, b: {}, c: {}", a, b, c);
        if d > 0.0{
            let x1 = ((-b) + d.sqrt()) / (2.0 * a);
            let x2 = ((-b) - d.sqrt()) / (2.0 * a);
            println!("Решено:\nЕсть 2 корня\nD = {}\nКорень 1 = {}\nКорень 2 = {}\n", d, x1, x2)
        } else if d == 0.0{
            let x1 = (-b) / (2.0 * a);
            println!("Решено:\nЕсть 2 корня\nD = {}\nКорень 1 = {}\n", d, x1)
        } else {
            println!("Решено:\nКорней нет!\nD = {}\n", d)
        }

        // continue or quit ?
        println!("Нажмите комбинацию CTRL+C чтобы выйти, если нужно решить еще один пример нажмите ENTER!");
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }) => break, // break (ctrl+c)
            _ => (),
        }
    }
}