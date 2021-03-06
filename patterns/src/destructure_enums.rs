#![allow(dead_code)]

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } =>
            println!("Move in the x direction {} and in the y direction {}", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) =>
            println!("Change the color to red {}, green {}, and blue {}", r, g, b),
    }

    let msg = MessageV2::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        MessageV2::ChangeColor(Color::Rgb(r, g, b)) =>
            println!("Change the color to red {}, green {}, and blue {}", r, g, b),
        MessageV2::ChangeColor(Color::Hsv(h, s, v)) =>
            println!("Change the color to hue {}, saturation {} and value {}",
                     h, s, v),
        x => println!("Got {:?}", x)
    }

    let mut setting_value = Some(5);
    let new_setting_value = Some(3);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't overwrite an existing customized value"),
        _ => setting_value = new_setting_value,
    }
    println!("setting is {:?}", setting_value);
}

#[derive(Debug)]
enum Color {
    Rgb(u8, u8, u8),
    Hsv(i32, i32, i32),
}

#[derive(Debug)]
enum MessageV2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
