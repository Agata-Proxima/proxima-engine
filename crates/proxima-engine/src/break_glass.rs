use std::io;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
pub fn default_outbox_dir() -> PathBuf {
    std::env::var_os("PROXIMA_BREAK_GLASS_OUTBOX_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("data/proxima-break-glass-outbox"))
}
pub async fn record_event(dir: &Path, node_id: &str, nonce_hash: &str) -> io::Result<()> {
    if node_id.is_empty()
        || node_id.len() > 128
        || !node_id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
        || nonce_hash.len() != 64
        || !nonce_hash.bytes().all(|b| b.is_ascii_hexdigit())
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "invalid break-glass event fields",
        ));
    }
    tokio::fs::create_dir_all(dir).await?;
    let event_id = Uuid::new_v4().to_string();
    let observed_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| io::Error::other("system clock before UNIX epoch"))?
        .as_secs() as i64;
    let event = serde_json::json!({"event_id":event_id,"node_id":node_id,"event_type":"security.break_glass.used","severity":"warning","peer_fingerprint":nonce_hash,"reason":"A short-lived, one-time emergency tenant token was used; tenant isolation remains enforced.","observed_at":observed_at});
    let path = dir.join(format!("event-{event_id}.json"));
    let mut file = tokio::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .await?;
    file.write_all(format!("{event}\n").as_bytes()).await?;
    file.sync_all().await
}
pub async fn pending_events(dir: &Path) -> io::Result<Vec<(PathBuf, serde_json::Value)>> {
    let mut entries = match tokio::fs::read_dir(dir).await {
        Ok(v) => v,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e),
    };
    let mut paths = Vec::new();
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.starts_with("event-") && n.ends_with(".json"))
        {
            paths.push(path);
        }
    }
    paths.sort();
    let mut events = Vec::new();
    for path in paths {
        let raw = tokio::fs::read_to_string(&path).await?;
        let value = serde_json::from_str(&raw)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        events.push((path, value));
    }
    Ok(events)
}
pub async fn acknowledge_event(path: &Path) -> io::Result<()> {
    tokio::fs::remove_file(path).await
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn event_outbox_persists_until_acknowledged() {
        let dir = tempfile::tempdir().unwrap();
        record_event(dir.path(), "node-test", &"a".repeat(64))
            .await
            .unwrap();
        let pending = pending_events(dir.path()).await.unwrap();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].1["event_type"], "security.break_glass.used");
        assert_eq!(pending[0].1["severity"], "warning");
        acknowledge_event(&pending[0].0).await.unwrap();
        assert!(pending_events(dir.path()).await.unwrap().is_empty());
    }
}
