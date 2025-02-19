// Jackson Coxson

use log::{error, info};
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::mpsc::UnboundedSender,
    time::sleep_until,
};
use std::fs::OpenOptions;
use std::io::Write;
use std::fs::File;

use crate::church::ChurchClient;

pub mod config;
mod send_time;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Message {
    sender: String,
    content: String,
    chat_id: String,
}

impl Message {
    fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}

pub async fn main(church_client: &mut ChurchClient) -> anyhow::Result<()> {
    pub fn clear_log() {
        File::create("holly.log").expect("Failed to clear log file"); // Truncates the file
    }
    clear_log();
    info!("Connecting to Holly...");
    let holly_config = church_client
        .holly_config
        .clone()
        .unwrap_or(config::Config::force_load(church_client).await?);

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    tokio::task::spawn_blocking(move || user_input_loop(tx));

    // Connect to Holly only once.
    let mut stream = tokio::net::TcpStream::connect(&holly_config.holly_socket).await?;
    let next_time_check = tokio::time::Instant::now() + tokio::time::Duration::from_secs(1);
    let mut buf = [0u8; 1024 * 8];

    tokio::select! {
        // Option 1: Data received from Holly.
        written = stream.read(&mut buf) => {
            let written = match written {
                Ok(w) => w,
                Err(e) => {
                    error!("Error receiving data from Holly! {e:?}");
                    return Ok(());
                },
            };
            if written == 0 {
                error!("Holly stopped sending data!");
                return Ok(());
            }

            if let Ok(payload) = String::from_utf8(buf[0..written].to_vec()) {
                if let Ok(payload) = serde_json::from_str::<Message>(&payload) {
                    info!("Received message from Holly: {payload:?}");
                }
            } else {
                error!("Received a non-utf8 vector from Holly");
                return Ok(());
            }
        }
        // Option 2: Time to check and possibly send Holly's list.
        _ = sleep_until(next_time_check) => {
            info!("Checking if it's time to send Holly's list");
            let mut st = send_time::SendTime::load(&church_client.env).await?;
            println!("about to check if go time");
            println!("{}", st.has_gone);
            if !st.has_gone {
                info!("Sending Holly's list!");
                println!("not true, sending");
                st.gone().await?;
                let report = if let Some(report) = crate::report::Report::read_report(&church_client.env)? {
                    report
                } else {
                    crate::generate_report(church_client).await?
                };

                let contacts = crate::get_average(church_client).await?;
                let mut contacts = contacts.into_iter().collect::<Vec<(String, usize)>>();
                contacts.sort_unstable_by(|a, b| a.1.cmp(&b.1));

                // the following is my baby and I wrote it in Rust
                fn format_contact_time(total_minutes: usize) -> String {
                    if total_minutes < 60 {
                        format!("{}min", total_minutes)
                    } else {
                        let hours = total_minutes / 60;
                        let minutes = total_minutes % 60;
                        format!("{}hr {}min", hours, minutes)
                    }
                }
                

                let mut avg_report = String::new();
                for (k, v) in contacts {
                    if let Some(bl) = &holly_config.blacklist {
                        if bl.contains(&k) {
                            continue;
                        }
                    }
                    let contact_time = format_contact_time(v);
                    let mut mut_k = k.clone();

                    if !k.starts_with(" ") {
                        mut_k = format!(" {}", k);
                    }

                    avg_report = format!("{avg_report}\n-{mut_k}: {}", contact_time);
                }
                // my baby ends here

               
                for (zone_id, chat_id) in &holly_config.zone_chats {
                    let msg = if let Some(p) = report.get_pretty_zone(zone_id) {
                        format!("Good Morning, y'all! I hope everyone has a great day! Let's BELIEVE, and GO contact those referrals! \n ._.)/\\(._. \n\n\n-->Average Contact Time:<--\n{avg_report}\n\n\n-->Uncontacted Referrals<--\n{p}\n\n\nClarification: when I name something as an 'uncontacted referral' that means it's a referral you've received in the past 48hrs - 10 days that has not been successfully contacted <3. ")
                    } else {
                        info!("BOOYAH! No uncontacted referrals in {zone_id}");
                        format!("Good Morning, y'all! I hope everyone has a great day!\n\n\nAverage Contact Time:{avg_report}\n\n BOOYAH, No uncontacted referrals! Great work!\n\n(•_•)\n( •_•)>⌐■-■\n(⌐■_■)")
                    };
                    info!("Sending {msg} to {chat_id}");
                    stream.write_all(&Message {
                        content: msg,
                        chat_id: chat_id.to_string(),
                        ..Default::default()
                    }.to_bytes()).await?;
                }
                if let Some(chat_id) = &holly_config.unassigned_chat {
                    let msg = report.unassigned.join("\n");
                    info!("Sending {msg} to {chat_id}");
                    stream.write_all(&Message {
                        content: msg,
                        chat_id: chat_id.to_string(),
                        ..Default::default()
                    }.to_bytes()).await?;
                }
                pub fn log_message(message: &str) {
                    let mut file = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open("holly.log")
                        .expect("Cannot open log file");
                    writeln!(file, "{}", message).expect("Failed to write to log file");
                }
                
                log_message("DONE");
            }
           
        }
        // Option 3: User requested disconnect.
        _ = rx.recv() => {
            info!("Disconnecting from Holly...");
        }
    }

    Ok(())
}

fn user_input_loop(sender: UnboundedSender<()>) {
    println!("Press 'q' and then enter to disconnect from Holly gracefully.");
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf).is_ok();
    let _ = sender.send(());
}