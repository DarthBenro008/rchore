use crate::service::database_api::TasksDatabase;
use anyhow;
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use url::Url;

pub fn oauth_login(tasks_database: &TasksDatabase) -> anyhow::Result<()> {
    let client = create_oauth_client()?;
    let pkce_code_verification = initiate_oauth(&client)?;
    let token = get_token(&client, pkce_code_verification)?;
    tasks_database.insert_token(token)?;
    return Ok(());
}

fn create_oauth_client() -> anyhow::Result<BasicClient> {
    let client_id = env::var("GOOGLE_CLIENT_ID").unwrap();
    let client_secret = env::var("GOOGLE_CLIENT_SECRET").unwrap();
    let client = BasicClient::new(
        ClientId::new(String::from(client_id)),
        Some(ClientSecret::new(String::from(client_secret))),
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?,
        Some(TokenUrl::new(
            "https://oauth2.googleapis.com/token".to_string(),
        )?),
    )
    .set_redirect_uri(RedirectUrl::new("http://localhost:6555/".to_string())?);
    return Ok(client);
}

fn initiate_oauth(basic_client: &BasicClient) -> anyhow::Result<PkceCodeVerifier> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, _) = basic_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/tasks".to_string(),
        ))
        .set_pkce_challenge(pkce_challenge)
        .url();

    println!(
        "Open the following link in your browser to authenticate yourself: \n\n {}",
        auth_url
    );

    Ok(pkce_verifier)
}

fn get_token(
    basic_client: &BasicClient,
    pkce_verifier: PkceCodeVerifier,
) -> anyhow::Result<String> {
    let listener = TcpListener::bind("127.0.0.1:6555").unwrap();
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let code;
            {
                let mut reader = BufReader::new(&stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line).unwrap();

                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                let code_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();

                let (_, value) = code_pair;
                code = AuthorizationCode::new(value.into_owned());
            }

            let message =
                "Awesome! RChore has been authenticated! You can close this window now :D";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).unwrap();

            let token_response = basic_client
                .exchange_code(code)
                .set_pkce_verifier(pkce_verifier)
                .request(http_client);

            let result = token_response.unwrap();
            return Ok(result.access_token().secret().clone());
        }
    }
    return Ok(String::from(""));
}
