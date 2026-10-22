use proxima_engine::tenant::issue_break_glass_token;
use std::env;
fn main() {
    let mut args = env::args().skip(1);
    let tenant = args.next().unwrap_or_default();
    let ttl = args
        .next()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(300);
    if tenant.is_empty() || args.next().is_some() {
        eprintln!("Usage: cargo run -p proxima-engine --bin proxima-break-glass-token -- <tenant_id> [ttl_seconds]");
        std::process::exit(2);
    }
    let secret = match env::var("PROXIMA_TENANT_SIGNING_KEY") {
        Ok(v) if v.len() >= 32 => v,
        _ => {
            eprintln!("PROXIMA_TENANT_SIGNING_KEY must contain at least 32 bytes.");
            std::process::exit(2)
        }
    };
    match issue_break_glass_token(secret.as_bytes(), &tenant, ttl) {
        Ok(token) => println!("{token}"),
        Err(error) => {
            eprintln!("Could not issue emergency token: {error}");
            std::process::exit(2)
        }
    }
}
