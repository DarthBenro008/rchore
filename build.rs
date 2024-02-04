use dotenv::dotenv;
use std::env;
use std::io::Write;

fn write_link_info_type(file: &mut std::fs::File) -> Result<(), std::io::Error> {
    let cid = env::var("GOOGLE_CLIENT_ID").expect("env var GOOGLE_CLIENT_ID should be set but was");
    let cls = env::var("GOOGLE_CLIENT_SECRET")
        .expect("env var GOOGLE_CLIENT_SECRET should be set but was");
    let data = format!(
        "pub struct Secrets {{
    pub client_id: String,
    pub client_secret: String,
}}
impl Secrets {{
    pub fn new() -> Secrets {{
        Secrets {{
                client_id: \"{}\".to_string(),
                client_secret: \"{}\".to_string(),
        }}
    }}
}}",
        cid, cls
    );
    file.write_all(data.as_bytes())?;
    Ok(())
}

fn generate_module() -> Result<(), std::io::Error> {
    let mut module = std::fs::File::create(format!("src/{}.rs", "secrets"))?;
    write_link_info_type(&mut module)?;
    Ok(())
}

fn main() {
    dotenv().ok();
    if let Err(e) = generate_module() {
        eprintln!("Error: {}", e);
    }
}
