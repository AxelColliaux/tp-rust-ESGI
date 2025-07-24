// TP : implémenter un serveur de journalisation asynchrone. ce serveur ecoute sur un port TCP, recoit des messages textes clients 
// puis les enregistres dans un fichier logs, avec un horodatage
// plusieurs clients peuvent se connecter simultanement 

// consignes :    tokio,  TCP ( TcpListener et TcpStream)
// strcuture projets : 
//     journalisation_server/
//     |---src/
//     |__main.rs
//     |--logs/
//     |__server.log  ( généré automatiquement )
//     |-- Cargo.toml 
// [2025-07-24T14:20:00Z]  log de A                                
// [2025-07-24T14:20:01Z]  log de B
// [2025-07-24T14:20:02Z]  log de C
// Gestion des tâches avec spawn : 

// lancer plusieurs tâches en parallèle indépendante
//     let t1 = tokio::spawn(task("Task1", 2 )); 
//     let t2 = tokio::spawn(task("Task1", 2 )); 
//     let t3 = tokio::spawn(task("Task1", 2 )); 
// Attendre que toutes les tâches se terminent 
// let _= tokio::join!(t1,t2,t3);   
// println!("Toutes les tâches terminées avec succes en {:?}", debut.elapsed());

use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, BufReader, AsyncWriteExt};
use tokio::fs::OpenOptions;
use tokio::sync::Mutex;
use std::sync::{Arc};
use chrono::{Utc};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tokio::fs::create_dir("logs").await.ok();

    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/server.log")
        .await?;
    let log_file = Arc::new(Mutex::new(log_file));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Serveur en écoute sur 127.0.0.1:8080");

    loop {
        let (stream, adresse) = listener.accept().await?;
        let log_file = log_file.clone();

        tokio::spawn(async move {
            let reader = BufReader::new(stream);
            let mut lines = reader.lines();

            loop {
                match lines.next_line().await {
                    Ok(Some(line)) => {
                        let now = Utc::now().format("%d/%m/%Y %H:%M:%S");
                        let log_entry = format!(
                            "[{}] ({}): {}\n",
                            now, adresse, line
                        );
                        let mut file = log_file.lock().await;
                        let _ = file.write_all(log_entry.as_bytes()).await;
                        println!("({}): {}", adresse, line);
                    }
                    Ok(None) => break,
                    Err(_) => break,
                }
            }
        });
    }
}
