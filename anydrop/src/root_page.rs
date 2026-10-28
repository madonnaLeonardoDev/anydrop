use axum::{response::Html};

const INDEX_HTML: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>anydrop 🚀</title>
    <style>
        * { box-sizing: border-box; margin: 0; padding: 0; }
        body { 
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; 
            max-width: 600px; 
            margin: 40px auto; 
            padding: 0 20px; 
            background: #0d1117; 
            color: #c9d1d9; 
        }
        h1 { font-size: 1.8rem; color: #f0f6fc; margin-bottom: 20px; }
        .card { 
            background: #161b22; 
            padding: 24px; 
            border-radius: 8px; 
            border: 1px solid #30363d; 
            margin-bottom: 20px; 
        }
        input[type="file"] { margin-bottom: 16px; color: #8b949e; display: block; }
        button { 
            background: #238636; 
            color: #ffffff; 
            border: none; 
            padding: 10px 18px; 
            border-radius: 6px; 
            cursor: pointer; 
            font-weight: 600; 
            font-size: 0.95rem; 
        }
        button:hover { background: #2ea043; }
    </style>
</head>
<body>
    <h1>anydrop ⚡</h1>
    <div class="card">
        <h3 style="margin-bottom: 12px;">Upload File to PC</h3>
        <form action="/upload" method="POST" enctype="multipart/form-data">
            <input type="file" name="file" required />
            <button type="submit">Upload</button>
        </form>
    </div>
</body>
</html>
"#;

pub async fn get_index_page() -> Html<&'static str> {
    Html(INDEX_HTML)
}