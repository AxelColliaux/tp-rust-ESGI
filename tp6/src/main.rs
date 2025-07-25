// TP 6: Implémentation d'un Protocole Personnalisé

// Description:
// Les étudiants concevront et implémenteront un protocole réseau simple (par exemple, un protocole de messagerie ou de transfert de fichiers) au-dessus de TCP ou UDP. Ils devront définir le format des messages, les règles d'échange et implémenter un client et un serveur conformes à ce protocole.

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Serveur en écoute sur 127.0.0.1:7878");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 128];
        stream.read(&mut buffer).unwrap();
        let message = String::from_utf8_lossy(&buffer);
        println!("Message recu: {}", message);

        let response = b"Message bien recu !";
        stream.write(response).unwrap();
    }
}

fn run_client() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    let message = b"Bonjour serveur !";
    stream.write(message).unwrap();

    let mut buffer = [0; 128];
    stream.read(&mut buffer).unwrap();
    let response = String::from_utf8_lossy(&buffer);
    println!("Réponse du serveur: {}", response);
}

fn main() {
    thread::spawn(|| {
        run_server();
    });
    std::thread::sleep(std::time::Duration::from_millis(500));
    run_client();
}