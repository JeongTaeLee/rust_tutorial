use std::i32;

struct QuitMessage {
    reason: i8,
}

struct MoveMessage {
    x : i32,
    y : i32,
}

struct WriteMessage {
    message: String,
}

struct ChangeColorMessage {
    r: i8,
    g: i8,
    b: i8,
}


enum Message {
    Quit(QuitMessage),
    Move { x: i32, y: i32 },
    Write(WriteMessage),
    ChangeColor(ChangeColorMessage)
}

impl Message {
    fn print(&self) {
        match self {
            Message::Quit(value) => {
                println!("QuitMessage.reason : {}", value.reason);
            },
            Message::Move { x, y } => {
                println!("Move.x : {}\nMove.y : {}", x, y);
            },
            Message::Write(value) => {
                println!("Write.message : {}", value.message);
            },

            _ => {
                println!("Message type not found!")
            }
        };
    }

    fn let_print(&self) {
        if let Message::Quit(value) = self {
            println!("QuitMessage.reason : {}", value.reason);
        } else if let Message::Move{x,y} = self {
            println!("Move.x : {}\nMove.y : {}", x, y);
        } else if let Message::Write(value) = self {
            println!("Write.message : {}", value.message);
        } else {
            println!("Message type not found!");
        }
    }
}

fn main() {
    Message::Quit(QuitMessage {
        reason: 1
    }).print();

    Message::ChangeColor(ChangeColorMessage {
        r: 1,
        g: 2,
        b: 3,
    }).let_print();
}

