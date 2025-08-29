use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper::body::Incoming as HyperIncomingBody;
use hyper::body::Bytes as HyperBytes;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::sync::oneshot;
use url::Url;
use http_body_util::{Full};
use hyper_util::rt::TokioIo;

pub async fn start_oauth_server(
    tx: oneshot::Sender<(String, String)>,
    port: u16,
) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("OAuth server listening on {}", addr);

    // Single connection only
    let (stream, _remote_addr) = listener.accept().await?;
    let io = TokioIo::new(stream);

    use std::sync::{Arc, Mutex};
    let tx = Arc::new(Mutex::new(Some(tx)));
    let service = service_fn({
        let tx = tx.clone();
        move |req| {
            let tx = tx.clone();
            async move {
                let tx_opt = tx.lock().unwrap().take();
                handle_request(req, tx_opt).await
            }
        }
    });

    http1::Builder::new()
        .serve_connection(io, service)
        .await?;

    Ok(())
}

async fn handle_request(
    req: Request<HyperIncomingBody>,
    tx: Option<oneshot::Sender<(String, String)>>,
) -> Result<Response<Full<HyperBytes>>, Infallible> {
    let path = req.uri().path();
    let query = req.uri().query();

    if path == "/oauth/callback" {
        if let Some(query_str) = query {
            let url = Url::parse(&format!("http://localhost/?{}", query_str)).unwrap();
            let code = url.query_pairs().find(|(key, _)| key == "code").map(|(_, value)| value.to_string());
            let state = url.query_pairs().find(|(key, _)| key == "state").map(|(_, value)| value.to_string());

            if let (Some(code), Some(state)) = (code, state) {
                println!("Received OAuth callback. Code: {}, State: {}", code, state);
                if let Some(tx) = tx {
                    let _ = tx.send((code, state)); // Send the code and state back to the main app
                }
                let response_html = r#"
                    <!DOCTYPE html>
                    <html lang="ja">
                    <head>
                        <meta charset="UTF-8">
                        <meta name="viewport" content="width=device-width, initial-scale=1.0">
                        <title>認証完了</title>
                        <style>
                            body { 
                                font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif; 
                                text-align: center; 
                                margin-top: 50px; 
                                background-color: #f5f5f5;
                                padding: 20px;
                            }
                            .container {
                                background-color: white;
                                border-radius: 8px;
                                padding: 40px;
                                box-shadow: 0 2px 10px rgba(0,0,0,0.1);
                                max-width: 500px;
                                margin: 0 auto;
                            }
                            h1 { 
                                color: #4CAF50; 
                                margin-bottom: 20px;
                            }
                            p { 
                                color: #666; 
                                margin-bottom: 20px;
                                line-height: 1.6;
                            }
                            .countdown {
                                font-size: 18px;
                                color: #2196F3;
                                font-weight: bold;
                            }
                        </style>
                        <script>
                            let countdown = 5;
                            function updateCountdown() {
                                const el = document.getElementById('countdown');
                                if (el) el.textContent = countdown;
                                if (countdown <= 0) {
                                    window.close();
                                } else {
                                    countdown--;
                                    setTimeout(updateCountdown, 1000);
                                }
                            }
                            window.onload = updateCountdown;
                        </script>
                    </head>
                    <body>
                        <div class="container">
                            <h1>✅ 認証が完了しました</h1>
                            <p>認証が正常に完了しました。<br>アプリケーションに戻ってご利用ください。</p>
                            <p class="countdown">このタブは <span id="countdown">5</span> 秒後に自動で閉じます</p>
                        </div>
                    </body>
                    </html>
                "#;

                return Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "text/html")
                    .body(Full::new(HyperBytes::from(response_html)))
                    .unwrap());
            }
        }
    }

    // For any other path or missing parameters, return a 404
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new(HyperBytes::from("Not Found")))
        .unwrap())
}
