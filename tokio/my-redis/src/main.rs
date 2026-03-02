use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
#[tokio::main]
async fn main() {

    let handle = tokio::spawn(async {
        // Do some async work
        "return value"
    });
    let out = handle.await.unwrap();
    println!("GOT {}", out);
    // let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    // loop {
    //     let (socket, _) = listener.accept().await.unwrap();
    //     println!("accepted connection");
    //     tokio::spawn(async move {
    //         process(socket).await;
    //     });
    // }
}


async fn process(socket: TcpStream) {
    // let mut buf = [0; 1024];
    // loop {
    //     let n = socket.read(&mut buf).await.unwrap();
    //     if n == 0 {
    //         return;
    //     }
    //     socket.write_all(&buf[0..n]).await.unwrap();
    // }

    let mut connection = Connection::new(socket);
    if let Some(frame) =  connection.read_frame().await.unwrap() {
        println!("got frame: {:?}", frame);
        //connection.write_frame(frame).await.unwrap();   
        let response = Frame::Error("unimplemented ".to_string());
        connection.write_frame(&response).await.unwrap();
    }
    // while let Some(frame) = connection.read_frame().await.unwrap() {
    //     println!("got frame: {:?}", frame);
    //     connection.write_frame(frame).await.unwrap();   

}