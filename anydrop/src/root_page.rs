use axum::{extract::State, response::Html};
use crate::server::AppState;

pub async fn send_page(State(state): State<AppState>) -> Html<String> {
let page_str: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>anydrop ⚡</title>
    <style>
        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }

        :root {
            /* Default: Dark Theme */
            --bg-color: #0d1117;
            --card-bg: #161b22;
            --border-color: #30363d;
            --text-primary: #f0f6fc;
            --text-secondary: #8b949e;
            --accent-color: #238636;
            --accent-hover: #2ea043;
            --disabled-color: #21262d;
        }

        @media (prefers-color-scheme: light) {
            :root {
                /* Automatic Light Theme if user's OS is set to light */
                --bg-color: #ffffff;
                --card-bg: #f6f8fa;
                --border-color: #e1e4e8;
                --text-primary: #24292e;
                --text-secondary: #586069;
                --accent-color: #2ea44f;
                --accent-hover: #2c974b;
                --disabled-color: #d1d5da;
            }
        }
        
        body {
            background-color: var(--bg-color);
            display: flex;
            align-items: center;
            justify-content: center;
            height: 100dvh;
            width: 100dvw;
            overflow: hidden;
            font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        }

        .main-card {
            display: flex;
            flex-direction: column;
            align-items: center;
            width: 100%;
            max-width: 420px;
            margin: 0 16px;
            padding: 32px 24px;
            gap: 24px;
            border-radius: 16px;
            background-color: var(--card-bg);
            border: 1px solid var(--border-color);
            box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
        }

        .file-info {
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 8px;
            width: 100%;
        }

        .file-name {
            color: var(--text-primary);
            font-size: 1.1rem;
            font-weight: 600;
            word-break: break-all;
            text-align: center;
        }

        .status-text {
            color: var(--text-secondary);
            font-size: 0.85rem;
            text-align: center;
            min-height: 1.2em;
        }

        .progress-track {
            width: 100%;
            height: 8px;
            background-color: var(--bg-color);
            border: 1px solid var(--border-color);
            border-radius: 4px;
            overflow: hidden;
        }

        .progress-fill {
            width: 0%;
            height: 100%;
            background-color: var(--accent-color);
            transition: width 0.15s ease-out;
        }

        .download-btn {
            width: 100%;
            padding: 12px 16px;
            cursor: pointer;
            background-color: var(--accent-color);
            color: #ffffff;
            border-radius: 8px;
            border: none;
            transition: background-color 0.15s ease, transform 0.1s ease, opacity 0.15s ease;
        }

        .download-btn span {
            font-size: 0.95rem;
            font-weight: 600;
            display: block;
            text-align: center;
        }

        .download-btn:hover:not(:disabled) {
            background-color: var(--accent-hover);
        }

        .download-btn:active:not(:disabled) {
            background-color: var(--accent-hover);
            transform: scale(0.98);
        }

        .download-btn:disabled {
            background-color: var(--disabled-color);
            cursor: not-allowed;
            opacity: 0.6;
        }
    </style>
</head>
<body>
    <div class="main-card">
        <div class="file-info">
            <!-- Dynamically populated or injected filename -->
            <span class="file-name" id="fileName">{_}</span>
            <span class="status-text" id="statusText">Ready to download</span>
        </div>

        <div class="progress-track" id="progressTrack">
            <div class="progress-fill" id="progressFill"></div>
        </div>

        <button class="download-btn" id="downloadBtn" onclick="downloadFile()">
            <span id="btnText">Download</span>
        </button>
    </div>

    <script>
        function downloadFile() {
            const statusText = document.getElementById('statusText');
            const progressFill = document.getElementById('progressFill');
            const downloadBtn = document.getElementById('downloadBtn');
            const btnText = document.getElementById('btnText');
            const fileUrl = `/download`;

            downloadBtn.disabled = true;
            btnText.innerText = 'Downloading...';
            statusText.innerText = 'Starting download...';
            progressFill.style.width = '0%';

            const xhr = new XMLHttpRequest();
            xhr.open('GET', fileUrl, true);
            xhr.responseType = 'blob';

            xhr.onprogress = function (event) {
                if (event.lengthComputable) {
                    const percentComplete = Math.round((event.loaded / event.total) * 100);
                    progressFill.style.width = percentComplete + '%';
                    
                    const loadedMB = (event.loaded / (1024 * 1024)).toFixed(1);
                    const totalMB = (event.total / (1024 * 1024)).toFixed(1);
                    statusText.innerText = `${percentComplete}% (${loadedMB} MB / ${totalMB} MB)`;
                } else {
                    statusText.innerText = 'Downloading...';
                }
            };

            xhr.onload = function () {
                if (xhr.status === 200) {
                    statusText.innerText = 'Download complete!';
                    btnText.innerText = 'Completed';
                    progressFill.style.width = '100%';

                    const blob = xhr.response;
                    const blobUrl = window.URL.createObjectURL(blob);
                    
                    const anchor = document.createElement('a');
                    anchor.style.display = 'none';
                    anchor.href = blobUrl;
                    anchor.download = fileName;
                    document.body.appendChild(anchor);
                    
                    anchor.click();

                    setTimeout(() => {
                        document.body.removeChild(anchor);
                        window.URL.revokeObjectURL(blobUrl);
                        downloadBtn.disabled = false;
                        btnText.innerText = 'Download Again';
                    }, 1000);
                } else {
                    statusText.innerText = `Download failed (Server status: ${xhr.status})`;
                    btnText.innerText = 'Retry Download';
                    downloadBtn.disabled = false;
                }
            };

            xhr.onerror = function () {
                statusText.innerText = 'Network error occurred during download.';
                btnText.innerText = 'Retry Download';
                downloadBtn.disabled = false;
            };

            xhr.send();
        }
    </script>
</body>
</html>
"#;
let file_name = state.dir.file_name().and_then(|s| s.to_str()).unwrap_or("Unkown FileName");
let page = page_str.replace("{_}", file_name);
    Html(page)
}

pub async fn recieve_page() -> Html<&'static str> {
    Html(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>anydrop ⚡</title>
    <style>
        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }

        :root {
            /* Default: Dark Theme */
            --bg-color: #0d1117;
            --card-bg: #161b22;
            --border-color: #30363d;
            --border-dashed: #484f58;
            --text-primary: #f0f6fc;
            --text-secondary: #8b949e;
            --accent-color: #238636;
            --accent-hover: #2ea043;
            --disabled-color: #21262d;
            --dropzone-hover: #1c2129;
        }

        @media (prefers-color-scheme: light) {
            :root {
                /* Automatic Light Theme if user's OS is set to light */
                --bg-color: #ffffff;
                --card-bg: #f6f8fa;
                --border-color: #e1e4e8;
                --border-dashed: #d1d5da;
                --text-primary: #24292e;
                --text-secondary: #586069;
                --accent-color: #2ea44f;
                --accent-hover: #2c974b;
                --disabled-color: #d1d5da;
                --dropzone-hover: #eff1f3;
            }
        }
        
        body {
            background-color: var(--bg-color);
            display: flex;
            align-items: center;
            justify-content: center;
            height: 100dvh;
            width: 100dvw;
            overflow: hidden;
            font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        }

        .main-card {
            display: flex;
            flex-direction: column;
            align-items: center;
            width: 100%;
            max-width: 420px;
            margin: 0 16px;
            padding: 32px 24px;
            gap: 20px;
            border-radius: 16px;
            background-color: var(--card-bg);
            border: 1px solid var(--border-color);
            box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
        }

        .drop-zone {
            width: 100%;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            padding: 24px 16px;
            border: 2px dashed var(--border-dashed);
            border-radius: 12px;
            background-color: transparent;
            cursor: pointer;
            transition: background-color 0.15s ease, border-color 0.15s ease;
            text-align: center;
        }

        .drop-zone:hover, .drop-zone.dragover {
            background-color: var(--dropzone-hover);
            border-color: var(--accent-color);
        }

        .drop-zone-icon {
            font-size: 2rem;
            margin-bottom: 8px;
            user-select: none;
        }

        .drop-zone-text {
            color: var(--text-primary);
            font-size: 0.95rem;
            font-weight: 500;
        }

        .drop-zone-hint {
            color: var(--text-secondary);
            font-size: 0.8rem;
            margin-top: 4px;
        }

        /* Hidden file input element */
        #fileInput {
            display: none;
        }

        .file-info {
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 6px;
            width: 100%;
        }

        .file-name {
            color: var(--text-primary);
            font-size: 1rem;
            font-weight: 600;
            word-break: break-all;
            text-align: center;
        }

        .status-text {
            color: var(--text-secondary);
            font-size: 0.85rem;
            text-align: center;
            min-height: 1.2em;
        }

        .progress-track {
            width: 100%;
            height: 8px;
            background-color: var(--bg-color);
            border: 1px solid var(--border-color);
            border-radius: 4px;
            overflow: hidden;
        }

        .progress-fill {
            width: 0%;
            height: 100%;
            background-color: var(--accent-color);
            transition: width 0.15s ease-out;
        }

        .upload-btn {
            width: 100%;
            padding: 12px 16px;
            cursor: pointer;
            background-color: var(--accent-color);
            color: #ffffff;
            border-radius: 8px;
            border: none;
            transition: background-color 0.15s ease, transform 0.1s ease, opacity 0.15s ease;
        }

        .upload-btn span {
            font-size: 0.95rem;
            font-weight: 600;
            display: block;
            text-align: center;
        }

        .upload-btn:hover:not(:disabled) {
            background-color: var(--accent-hover);
        }

        .upload-btn:active:not(:disabled) {
            background-color: var(--accent-hover);
            transform: scale(0.98);
        }

        .upload-btn:disabled {
            background-color: var(--disabled-color);
            cursor: not-allowed;
            opacity: 0.6;
        }
    </style>
</head>
<body>
    <div class="main-card">
        <!-- Interactive Drag and Drop Zone -->
        <div class="drop-zone" id="dropZone" onclick="document.getElementById('fileInput').click()">
            <div class="drop-zone-icon">📁</div>
            <div class="drop-zone-text">Choose a file or drag it here</div>
            <div class="drop-zone-hint">Any file type supported</div>
        </div>

        <input type="file" id="fileInput" onchange="handleFileSelect(this.files)">

        <div class="file-info">
            <span class="file-name" id="fileName">No file selected</span>
            <span class="status-text" id="statusText">Select a file to upload</span>
        </div>

        <div class="progress-track" id="progressTrack">
            <div class="progress-fill" id="progressFill"></div>
        </div>

        <button class="upload-btn" id="uploadBtn" onclick="uploadFile()" disabled>
            <span id="btnText">Upload</span>
        </button>
    </div>

    <script>
        let selectedFile = null;

        const dropZone = document.getElementById('dropZone');
        const fileInput = document.getElementById('fileInput');
        const fileName = document.getElementById('fileName');
        const statusText = document.getElementById('statusText');
        const progressFill = document.getElementById('progressFill');
        const uploadBtn = document.getElementById('uploadBtn');
        const btnText = document.getElementById('btnText');

        // Drag & Drop Event Listeners
        ['dragenter', 'dragover'].forEach(eventName => {
            dropZone.addEventListener(eventName, (e) => {
                e.preventDefault();
                e.stopPropagation();
                dropZone.classList.add('dragover');
            }, false);
        });

        ['dragleave', 'drop'].forEach(eventName => {
            dropZone.addEventListener(eventName, (e) => {
                e.preventDefault();
                e.stopPropagation();
                dropZone.classList.remove('dragover');
            }, false);
        });

        dropZone.addEventListener('drop', (e) => {
            const dt = e.dataTransfer;
            const files = dt.files;
            if (files.length > 0) {
                handleFileSelect(files);
            }
        });

        function handleFileSelect(files) {
            if (!files || files.length === 0) return;
            selectedFile = files[0];

            fileName.innerText = selectedFile.name;
            const fileSizeMB = (selectedFile.size / (1024 * 1024)).toFixed(2);
            statusText.innerText = `Ready to upload (${fileSizeMB} MB)`;
            
            progressFill.style.width = '0%';
            btnText.innerText = 'Upload';
            uploadBtn.disabled = false;
        }

        function uploadFile() {
            if (!selectedFile) return;

            const uploadUrl = '/upload';

            uploadBtn.disabled = true;
            btnText.innerText = 'Uploading...';
            statusText.innerText = 'Starting upload...';
            progressFill.style.width = '0%';

            const formData = new FormData();
            formData.append('file', selectedFile);

            const xhr = new XMLHttpRequest();
            xhr.open('POST', uploadUrl, true);

            // Track upload progress on xhr.upload
            xhr.upload.onprogress = function (event) {
                if (event.lengthComputable) {
                    const percentComplete = Math.round((event.loaded / event.total) * 100);
                    progressFill.style.width = percentComplete + '%';

                    const loadedMB = (event.loaded / (1024 * 1024)).toFixed(1);
                    const totalMB = (event.total / (1024 * 1024)).toFixed(1);
                    statusText.innerText = `${percentComplete}% (${loadedMB} MB / ${totalMB} MB)`;
                } else {
                    statusText.innerText = 'Uploading...';
                }
            };

            xhr.onload = function () {
                if (xhr.status >= 200 && xhr.status < 300) {
                    statusText.innerText = 'Upload complete!';
                    btnText.innerText = 'Completed';
                    progressFill.style.width = '100%';

                    setTimeout(() => {
                        uploadBtn.disabled = false;
                        btnText.innerText = 'Upload Another File';
                    }, 1500);
                } else {
                    statusText.innerText = `Upload failed (Server status: ${xhr.status})`;
                    btnText.innerText = 'Retry Upload';
                    uploadBtn.disabled = false;
                }
            };

            xhr.onerror = function () {
                statusText.innerText = 'Network error occurred during upload.';
                btnText.innerText = 'Retry Upload';
                uploadBtn.disabled = false;
            };

            xhr.send(formData);
        }
    </script>
</body>
</html>"#)
}