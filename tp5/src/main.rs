// TP 5: Client et Serveur DNS Simples

// Description:
// Ce TP propose d'implémenter un client DNS basique capable de résoudre des noms de domaine en adresses IP, et un serveur DNS simple qui répond à des requêtes pour quelques domaines prédéfinis. Ce TP explorera la programmation UDP et le format des messages DNS.

use std::net::{UdpSocket};
use std::str;
use std::thread;

fn get_ip_for_domain(domain: &str) -> Option<&'static str> {
    match domain {
        "example.com" => Some("93.184.216.34"),
        "google.com" => Some("142.250.190.78"),
        "localhost" => Some("127.0.0.1"),
        _ => None,
    }
}

fn run_server() {
    let socket = UdpSocket::bind("127.0.0.1:8053").expect("Erreur de bind");
    println!("Serveur DNS en écoute sur 127.0.0.1:8053");

    loop {
        let mut buf = [0u8; 256];
        let (len, src) = socket.recv_from(&mut buf).expect("Erreur de réception");
        let domain = str::from_utf8(&buf[..len]).unwrap_or("");
        println!("Requête reçue pour: {}", domain);

        let response = get_ip_for_domain(domain).unwrap_or("Domaine inconnu");
        socket.send_to(response.as_bytes(), src).expect("Erreur d'envoi");
    }
}

fn run_client(domain: &str) {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Erreur de bind client");
    socket.send_to(domain.as_bytes(), "127.0.0.1:8053").expect("Erreur d'envoi");

    let mut buf = [0u8; 256];
    let (len, _) = socket.recv_from(&mut buf).expect("Erreur de réception");
    let ip = str::from_utf8(&buf[..len]).unwrap_or("");
    println!("IP pour {}: {}", domain, ip);
}

fn main() {
    thread::spawn(|| run_server());
    std::thread::sleep(std::time::Duration::from_millis(500));
    run_client("example.com");
    run_client("google.com");
    run_client("unknown.com");
}