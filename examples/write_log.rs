use std::env::consts::OS;

fn main() {
    vap::info!("A process runs on {}", OS);
    vap::warn!("A process runs on {}", OS);
    vap::debug!("A process runs on {}", OS);
    vap::error!("A process runs on {}", OS);
}
