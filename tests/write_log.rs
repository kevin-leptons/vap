use std::env::consts::OS;

#[test]
fn write_log() {
    vap::info!("A process runs on {}", OS);
    vap::warn!("A process runs on {}", OS);
    vap::debug!("A process runs on {}", OS);
    vap::error!("A process runs on {}", OS);
}
