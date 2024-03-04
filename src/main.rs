use std::io;
use rand::Rng;

// Вывод карты
fn show_map(map: &Vec<i32>) {
    println!("\n=============");
    for i in 0..map.len() {
        match map[i] {
            0 => print!("| o "),
            11 => print!("| x "),
            _ => print!("| {} ", map[i])
        }
        if (i+1) % 3 == 0 && i != 0 {
            print!("|");
            println!();
            println!("=============");
        }
    }
}

// Ход бота
fn get_bot_step() -> usize {
    rand::thread_rng().gen_range(1..10)
}

// Ход игрока
fn get_user_step() -> usize {
    let mut str_user_step = String::new();
            
    println!("Ваш ход: ");
    let _ = io::stdin().read_line(&mut str_user_step);
    let user_step: usize = str_user_step.trim().parse().unwrap();

    if user_step > 9 || user_step < 1 {
        10
    } else {
        user_step
    }
}

// Поиск победителя или проверка на ничью
fn end_of_game(map: &Vec<i32>) -> i32 {
    // Проверка по вертикали и горизонтали
    let mut n = 0;
    for i in 0..3 {
        if map[i] == map[i+3] && map[i+3] == map[i+6] {
            return map[i];
        } else if map[i+n] == map[i+1+n] && map[i+1+n] == map[i+2+n] {
            return map[i];
        } else if (map[0] == map[4] && map[4] == map[8]) || (map[2] == map[4] && map[4] == map[6]) {
            return map[4];
        } 
        n += 2;
    }
    return -1;
}

fn main() {
    loop { 
        let mut map = vec![
            1, 2, 3,
            4, 5, 6,
            7, 8, 9
        ]; 
        
        println!("Игра началась!\n");

        loop {
            show_map(&map);
            let u_step = get_user_step();  // Получаем ход игрока
            
            if u_step == 10 {
                break;
            }
            
            // Сохраняем ход игрока на карте (в векторе map)
            if map[u_step-1] != 0 && map[u_step-1] != 11 {
                map[u_step-1] = 0;
            } else {
                println!("Эта ячейка уже занята!");
                continue;
            }

            // Ход бота
            loop {
                let b_step = get_bot_step();  // Получаем ход бота
                if map[b_step-1] != 11 && map[b_step-1] != 0 {
                    map[b_step-1] = 11;
                    break;
                } else {
                    continue;
                }       
            }

            // Определение победителя если таковой имеется
            let end_of_the_game = end_of_game(&map);

            match end_of_the_game {
                0 => { 
                    println!("Ты победил");
                    break;
                },
                11 => {
                    println!("Ты проиграл!");
                    break;
                },
                _ => {}
            }
        }
        println!("Игра закончилась!");
        println!("Хотите продолжить?: \n[1] - Да, [2] - Нет\n");
        
        let mut str_mes = String::new();
        let _ = io::stdin().read_line(&mut str_mes);
        let mes: i32 = str_mes.trim().parse().unwrap();

        match mes {
            1 => continue,
            _ => break
        }
    }
}
