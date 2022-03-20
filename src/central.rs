use super::Question;
use tokio::time::timeout;
use tokio::sync::mpsc;
use tokio::select;

pub enum CentralToPort {
    NameValidity(bool),
    TimeEnd(usize),
    GameEnd(usize),
    NewQuestion(Question)
}

pub enum PortToCentral {
    SetName(usize, String),
    AnswerGiven((usize, Option<usize>)),
    Disconnection(usize)
}

struct Player {
    name: Option<String>,
    sender: mpsc::Sender<CentralToPort>,
}

pub async fn run_game(questions: Vec<Question>){
    // let players = timeout(std::time::Duration::from_secs(10), wait_players(&mut players));

    let (port_tx, central_rx) = mpsc::channel::<PortToCentral>(5);

    let player_list = Vec::new();

    tokio::task::spawn(async {
        select!{
            _ = connect_players(port_tx) => {},
            _ = tokio::time::sleep(std::time::Duration::from_secs(20));
    });

    loop {



    }

}

async fn connect_players(port_tx: mpsc::channel<PortToCentral>){

    let server = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();

    loop {
        let new_stream = server.accept().await.unwrap().0;

        let port_tx_clone = port_tx.clone();

        tokio::task::spawn(async move {
            handle_player(new_stream, port_tx_clone).await;
        }
    }
}


async fn handle_player( stream: tokio::net::TcpStream,
                        tx: mpsc::Sender<PortToCentral>,
                        ) {

    stream.writable().await.unwrap();
    stream.try_write("Welcome to the game:\nPlease introduce name:\n".as_bytes()).unwrap();

    let mut name: &str;

    loop {

        let mut buffer = [0u8; 2048];

        let status = stream.readable().await;
        match stream.try_read(&mut buffer) {
            Ok(n_bytes) => {
                name = std::str::from_utf8(&buffer[0..n_bytes-2]).unwrap();
                stream.writable().await.unwrap();
                stream.try_write("Thank you\n".as_bytes()).unwrap();
            },
            _ => {}
        }
    }
        
}
