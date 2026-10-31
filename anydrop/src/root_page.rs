use axum::{extract::State, response::Html};
use crate::server::AppState;

pub async fn send_page(State(state): State<AppState>) -> Html<String> {
    // 1. Pure UI HTML & CSS Template (includes node-forge CDN script in head)
    let page_template = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>anydrop ⚡</title>
    <!-- Pure JavaScript Crypto Library (Bypasses non-secure context restrictions) -->
    <script src="https://cdnjs.cloudflare.com/ajax/libs/forge/1.3.1/forge.min.js"></script>
    <style>
        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }

        :root {
            --bg-color: #0d1117;
            --card-bg: #161b22;
            --border-color: #30363d;
            --text-primary: #f0f6fc;
            --text-secondary: #8b949e;
            --accent-color: #238636;
            --accent-hover: #2ea043;
            --disabled-color: #21262d;
            --item-bg: #21262d;
        }

        @media (prefers-color-scheme: light) {
            :root {
                --bg-color: #ffffff;
                --card-bg: #f6f8fa;
                --border-color: #e1e4e8;
                --text-primary: #24292e;
                --text-secondary: #586069;
                --accent-color: #2ea44f;
                --accent-hover: #2c974b;
                --disabled-color: #d1d5da;
                --item-bg: #eaeef2;
            }
        }
        
        body {
            background-color: var(--bg-color);
            display: flex;
            align-items: center;
            justify-content: center;
            min-height: 100dvh;
            width: 100dvw;
            padding: 20px 0;
            font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        }

        .main-card {
            display: flex;
            flex-direction: column;
            width: 100%;
            max-width: 480px;
            margin: 0 16px;
            padding: 24px;
            gap: 20px;
            border-radius: 16px;
            background-color: var(--card-bg);
            border: 1px solid var(--border-color);
            box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
        }

        .header-section {
            display: flex;
            flex-direction: column;
            gap: 4px;
        }

        .title {
            color: var(--text-primary);
            font-size: 1.25rem;
            font-weight: 700;
        }

        .status-text {
            color: var(--text-secondary);
            font-size: 0.85rem;
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

        .file-list {
            display: flex;
            flex-direction: column;
            gap: 8px;
            max-height: 280px;
            overflow-y: auto;
            padding-right: 4px;
        }

        .file-item {
            display: flex;
            align-items: center;
            justify-content: space-between;
            background-color: var(--item-bg);
            padding: 10px 14px;
            border-radius: 8px;
            border: 1px solid var(--border-color);
            gap: 12px;
        }

        .file-name {
            color: var(--text-primary);
            font-size: 0.9rem;
            font-weight: 500;
            word-break: break-all;
            flex: 1;
        }

        .btn {
            padding: 10px 16px;
            cursor: pointer;
            background-color: var(--accent-color);
            color: #ffffff;
            border-radius: 8px;
            border: none;
            font-size: 0.85rem;
            font-weight: 600;
            transition: background-color 0.15s ease, transform 0.1s ease, opacity 0.15s ease;
            white-space: nowrap;
        }

        .btn:hover:not(:disabled) {
            background-color: var(--accent-hover);
        }

        .btn:active:not(:disabled) {
            transform: scale(0.98);
        }

        .btn:disabled {
            background-color: var(--disabled-color);
            cursor: not-allowed;
            opacity: 0.6;
        }

        .btn-small {
            padding: 6px 12px;
            font-size: 0.8rem;
        }

        .download-all-btn {
            width: 100%;
            padding: 12px 16px;
            font-size: 0.95rem;
        }
    </style>
</head>
<body>
    <div class="main-card">
        <div class="header-section">
            <span class="title">Shared Files ⚡</span>
            <span class="status-text" id="statusText">Ready to download</span>
        </div>

        <div class="progress-track" id="progressTrack">
            <div class="progress-fill" id="progressFill"></div>
        </div>

        <div class="file-list" id="fileList">
            {FILE_LIST_ITEMS}
        </div>

        <button class="btn download-all-btn" id="downloadAllBtn" onclick="downloadAllFiles()">
            <span id="btnText">Download All</span>
        </button>
    </div>

    <script>
        // Modular Encryption / Decryption Injection
        {DECRYPTION_MODULE}

        const filesToDownload = Array.from(document.querySelectorAll('.file-item')).map(
            el => el.getAttribute('data-filename')
        );

        function updateProgress(percent, status) {
            document.getElementById('progressFill').style.width = percent + '%';
            document.getElementById('statusText').innerText = status;
        }

        function setButtonsDisabled(disabled) {
            document.querySelectorAll('button').forEach(btn => btn.disabled = disabled);
        }

        // Single File Download Promise with Decryption Integration
        function downloadSingleFile(fileName) {
            return new Promise((resolve, reject) => {
                const encodedName = encodeURIComponent(fileName);
                const fileUrl = `/download?name=${encodedName}`;

                const xhr = new XMLHttpRequest();
                xhr.open('GET', fileUrl, true);
                xhr.responseType = 'blob';

                xhr.onprogress = function (event) {
                    if (event.lengthComputable) {
                        const percentComplete = Math.round((event.loaded / event.total) * 100);
                        const loadedMB = (event.loaded / (1024 * 1024)).toFixed(1);
                        const totalMB = (event.total / (1024 * 1024)).toFixed(1);
                        updateProgress(percentComplete, `Downloading ${fileName}: ${percentComplete}% (${loadedMB}MB / ${totalMB}MB)`);
                    } else {
                        updateProgress(0, `Downloading ${fileName}...`);
                    }
                };

                xhr.onload = async function () {
                    if (xhr.status === 200) {
                        try {
                            const rawBlob = xhr.response;
                            let finalBlob = rawBlob;

                            // If decryption module is loaded, decrypt the blob before saving
                            if (typeof processBlobData === 'function') {
                                updateProgress(100, `Decrypting ${fileName}...`);
                                finalBlob = await processBlobData(rawBlob);
                            }

                            const blobUrl = window.URL.createObjectURL(finalBlob);
                            const anchor = document.createElement('a');
                            anchor.style.display = 'none';
                            anchor.href = blobUrl;
                            anchor.download = fileName;
                            document.body.appendChild(anchor);
                            anchor.click();

                            setTimeout(() => {
                                document.body.removeChild(anchor);
                                window.URL.revokeObjectURL(blobUrl);
                                resolve();
                            }, 300);
                        } catch (decryptErr) {
                            reject(new Error("Decryption failed: " + decryptErr.message));
                        }
                    } else {
                        const reader = new FileReader();
                        reader.onload = function() {
                            const errorMsg = reader.result || `HTTP ${xhr.status}`;
                            reject(new Error(errorMsg));
                        };
                        reader.onerror = function() {
                            reject(new Error(`HTTP ${xhr.status} error`));
                        };
                        reader.readAsText(xhr.response);
                    }
                };

                xhr.onerror = function () {
                    reject(new Error('Network error'));
                };

                xhr.send();
            });
        }

        async function downloadOne(fileName) {
            setButtonsDisabled(true);
            try {
                await downloadSingleFile(fileName);
                updateProgress(100, `Downloaded ${fileName}`);
            } catch (err) {
                updateProgress(0, `Failed downloading ${fileName}: ${err.message}`);
            } finally {
                setButtonsDisabled(false);
            }
        }

        async function downloadAllFiles() {
            if (filesToDownload.length === 0) return;

            setButtonsDisabled(true);
            const btnText = document.getElementById('btnText');
            btnText.innerText = 'Downloading All...';

            for (let i = 0; i < filesToDownload.length; i++) {
                const fileName = filesToDownload[i];
                try {
                    await downloadSingleFile(fileName);
                    await new Promise(r => setTimeout(r, 400));
                } catch (err) {
                    updateProgress(0, `Failed on ${fileName}: ${err.message}`);
                    setButtonsDisabled(false);
                    btnText.innerText = 'Download All';
                    return;
                }
            }

            updateProgress(100, 'All downloads completed successfully!');
            btnText.innerText = 'Download All Again';
            setButtonsDisabled(false);
        }
    </script>
</body>
</html>
"#;

    // 2. Pure-JS Decryption Module using node-forge (AES-GCM)
    let encryption_module = r#"
        const ENCRYPTION_KEY = window.location.hash.substring(1);

        function uint8ArrayToForgeBinary(arr) {
            let binary = '';
            const len = arr.byteLength;
            const CHUNK_SIZE = 0x8000;
            for (let i = 0; i < len; i += CHUNK_SIZE) {
                const chunk = arr.subarray(i, Math.min(i + CHUNK_SIZE, len));
                binary += String.fromCharCode.apply(null, chunk);
            }
            return binary;
        }

        function forgeBinaryToUint8Array(binaryStr) {
            const len = binaryStr.length;
            const bytes = new Uint8Array(len);
            for (let i = 0; i < len; i++) {
                bytes[i] = binaryStr.charCodeAt(i);
            }
            return bytes;
        }

        async function processBlobData(blob) {
            if (!ENCRYPTION_KEY) {
                throw new Error("Missing encryption key in URL hash (#KEY)");
            }

            const arrayBuffer = await blob.arrayBuffer();
            const dataView = new DataView(arrayBuffer);
            const uint8View = new Uint8Array(arrayBuffer);

            // Pad/slice key to exact 32 bytes
            const paddedKeyStr = ENCRYPTION_KEY.padEnd(32, '0').slice(0, 32);
            const keyBytes = forge.util.createBuffer(paddedKeyStr, 'utf8').getBytes(32);

            const decryptedChunks = [];
            let offset = 0;

            while (offset < uint8View.length) {
                // 1. Read 4-byte ciphertext length header (Big-endian)
                if (offset + 4 > uint8View.length) {
                    throw new Error("Malformed stream: incomplete length header");
                }
                const ciphertextSize = dataView.getUint32(offset, false);
                offset += 4;

                // 2. Read 12-byte nonce
                if (offset + 12 > uint8View.length) {
                    throw new Error("Malformed stream: incomplete nonce");
                }
                const nonceBytes = uint8View.subarray(offset, offset + 12);
                const ivStr = uint8ArrayToForgeBinary(nonceBytes);
                offset += 12;

                // 3. Read ciphertext block (includes 16-byte authentication tag at the end)
                if (offset + ciphertextSize > uint8View.length) {
                    throw new Error("Malformed stream: incomplete ciphertext block");
                }
                const blockBytes = uint8View.subarray(offset, offset + ciphertextSize);
                offset += ciphertextSize;

                if (blockBytes.length < 16) {
                    throw new Error("Ciphertext block too small to contain auth tag");
                }
                
                // Separate ciphertext payload and the 16-byte GCM authentication tag
                const actualCiphertext = blockBytes.subarray(0, blockBytes.length - 16);
                const tagBytes = blockBytes.subarray(blockBytes.length - 16);

                const cipherStr = uint8ArrayToForgeBinary(actualCiphertext);
                const tagStr = uint8ArrayToForgeBinary(tagBytes);

                // 4. Decrypt chunk using node-forge AES-GCM
                const decipher = forge.cipher.createDecipher('AES-GCM', keyBytes);
                decipher.start({
                    iv: ivStr,
                    tag: tagStr
                });
                decipher.update(forge.util.createBuffer(cipherStr));
                const success = decipher.finish();

                if (!success) {
                    throw new Error("AES-GCM decryption failed: authentication tag verification failed.");
                }

                const decryptedBinary = decipher.output.getBytes();
                decryptedChunks.push(forgeBinaryToUint8Array(decryptedBinary));
            }

            return new Blob(decryptedChunks);
        }
    "#;

    // 3. Build file list items
    let mut file_items_html = String::new();
    for key in state.dirs_map.as_ref().unwrap() {
        let file_name = key.0;
        file_items_html.push_str(&format!(
            r#"<div class="file-item" data-filename="{0}">
                <span class="file-name">{0}</span>
                <button class="btn btn-small" onclick="downloadOne('{0}')">Download</button>
            </div>"#,
            file_name
        ));
    }

    // 4. Inject encryption module only if safe mode / encryption is active
    let injection = if state.encryption.is_some() {
        encryption_module
    } else {
        ""
    };

    let final_page = page_template
        .replace("{DECRYPTION_MODULE}", injection)
        .replace("{FILE_LIST_ITEMS}", &file_items_html);

    Html(final_page)
}

pub async fn recive_page(State(state): State<AppState>) -> Html<String> {
    // 1. Pure UI HTML & CSS Template (includes node-forge CDN script in head)
    let page_template = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>anydrop ⚡</title>
    <!-- Pure JavaScript Crypto Library (Bypasses non-secure context restrictions) -->
    <script src="https://cdnjs.cloudflare.com/ajax/libs/forge/1.3.1/forge.min.js"></script>
    <style>
        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }

        :root {
            --bg-color: #0d1117;
            --card-bg: #161b22;
            --border-color: #30363d;
            --border-dashed: #484f58;
            --text-primary: #f0f6fc;
            --text-secondary: #8b949e;
            --accent-color: #238636;
            --accent-hover: #2ea043;
            --danger-color: #da3633;
            --danger-hover: #f85149;
            --disabled-color: #21262d;
            --dropzone-hover: #1c2129;
            --item-bg: #21262d;
        }

        @media (prefers-color-scheme: light) {
            :root {
                --bg-color: #ffffff;
                --card-bg: #f6f8fa;
                --border-color: #e1e4e8;
                --border-dashed: #d1d5da;
                --text-primary: #24292e;
                --text-secondary: #586069;
                --accent-color: #2ea44f;
                --accent-hover: #2c974b;
                --danger-color: #cb2431;
                --danger-hover: #d93f4c;
                --disabled-color: #d1d5da;
                --dropzone-hover: #eff1f3;
                --item-bg: #e1e4e8;
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
            max-width: 440px;
            margin: 0 16px;
            padding: 32px 24px;
            gap: 20px;
            border-radius: 16px;
            background-color: var(--card-bg);
            border: 1px solid var(--border-color);
            box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
            max-height: 90vh;
        }

        .drop-zone {
            width: 100%;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            padding: 20px 16px;
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
            margin-bottom: 6px;
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

        #fileInput {
            display: none;
        }

        .file-list {
            width: 100%;
            max-height: 150px;
            overflow-y: auto;
            display: flex;
            flex-direction: column;
            gap: 8px;
        }

        .file-item {
            display: flex;
            align-items: center;
            justify-content: space-between;
            background-color: var(--item-bg);
            padding: 8px 12px;
            border-radius: 8px;
            border: 1px solid var(--border-color);
            font-size: 0.85rem;
        }

        .file-item-name {
            color: var(--text-primary);
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
            max-width: 260px;
        }

        .remove-file-btn {
            background: none;
            border: none;
            color: var(--text-secondary);
            cursor: pointer;
            font-size: 1rem;
            padding: 2px 6px;
            border-radius: 4px;
            transition: color 0.15s, background-color 0.15s;
        }

        .remove-file-btn:hover {
            color: var(--danger-hover);
            background-color: rgba(218, 54, 51, 0.1);
        }

        .status-text {
            color: var(--text-secondary);
            font-size: 0.85rem;
            text-align: center;
            min-height: 1.2em;
            word-break: break-word;
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
            transition: width 0.1s ease-out;
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
        <div class="drop-zone" id="dropZone" onclick="document.getElementById('fileInput').click()">
            <div class="drop-zone-icon">📁</div>
            <div class="drop-zone-text">Choose files or drag them here</div>
            <div class="drop-zone-hint">Multiple files supported {E2EE_BADGE}</div>
        </div>

        <input type="file" id="fileInput" multiple onchange="handleFileSelect(this.files)">

        <div class="file-list" id="fileList"></div>

        <div class="status-text" id="statusText">Select files to upload</div>

        <div class="progress-track" id="progressTrack">
            <div class="progress-fill" id="progressFill"></div>
        </div>

        <button class="upload-btn" id="uploadBtn" onclick="uploadFiles()" disabled>
            <span id="btnText">Upload All</span>
        </button>
    </div>

    <script>
        // Modular Encryption Injection
        {ENCRYPTION_MODULE}

        let selectedFiles = [];

        const dropZone = document.getElementById('dropZone');
        const fileInput = document.getElementById('fileInput');
        const fileListContainer = document.getElementById('fileList');
        const statusText = document.getElementById('statusText');
        const progressFill = document.getElementById('progressFill');
        const uploadBtn = document.getElementById('uploadBtn');
        const btnText = document.getElementById('btnText');

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
            for (let i = 0; i < files.length; i++) {
                selectedFiles.push(files[i]);
            }
            fileInput.value = '';
            renderFileList();
        }

        function removeFile(index) {
            selectedFiles.splice(index, 1);
            renderFileList();
        }

        function renderFileList() {
            fileListContainer.innerHTML = '';

            if (selectedFiles.length === 0) {
                statusText.innerText = 'Select files to upload';
                uploadBtn.disabled = true;
                progressFill.style.width = '0%';
                return;
            }

            let totalSize = 0;
            selectedFiles.forEach((file, index) => {
                totalSize += file.size;
                
                const item = document.createElement('div');
                item.className = 'file-item';
                
                const nameSpan = document.createElement('span');
                nameSpan.className = 'file-item-name';
                nameSpan.innerText = file.name;

                const removeBtn = document.createElement('button');
                removeBtn.className = 'remove-file-btn';
                removeBtn.innerHTML = '&times;';
                removeBtn.onclick = () => removeFile(index);

                item.appendChild(nameSpan);
                item.appendChild(removeBtn);
                fileListContainer.appendChild(item);
            });

            const totalMB = (totalSize / (1024 * 1024)).toFixed(2);
            statusText.innerText = `Ready to upload (${selectedFiles.length} files, ${totalMB} MB)`;
            uploadBtn.disabled = false;
            btnText.innerText = `Upload (${selectedFiles.length})`;
        }

        async function uploadFiles() {
            if (selectedFiles.length === 0) return;

            uploadBtn.disabled = true;
            progressFill.style.width = '0%';

            const totalFiles = selectedFiles.length;

            for (let i = 0; i < totalFiles; i++) {
                const file = selectedFiles[i];
                statusText.innerText = `Uploading [${i + 1}/${totalFiles}]: ${file.name}`;

                try {
                    await uploadSingleFile(file, (percent) => {
                        const overallPercent = Math.round(((i + (percent / 100)) / totalFiles) * 100);
                        progressFill.style.width = overallPercent + '%';
                    });
                } catch (error) {
                    statusText.innerText = error.message;
                    uploadBtn.disabled = false;
                    btnText.innerText = 'Retry Upload';
                    return;
                }
            }

            statusText.innerText = `All ${totalFiles} files uploaded successfully!`;
            btnText.innerText = 'Completed';
            progressFill.style.width = '100%';
            selectedFiles = [];

            setTimeout(() => {
                renderFileList();
                btnText.innerText = 'Upload';
            }, 2000);
        }

        function uploadSingleFile(file, onProgressCallback) {
            return new Promise(async (resolve, reject) => {
                try {
                    let payloadBlob = file;

                    // If encryption module is injected, encrypt file blob before sending
                    if (typeof encryptBuffer === 'function') {
                        statusText.innerText = `Encrypting ${file.name}...`;
                        const rawBuffer = await file.arrayBuffer();
                        const encryptedBuffer = await encryptBuffer(rawBuffer);
                        payloadBlob = new Blob([encryptedBuffer], { type: 'application/octet-stream' });
                    }

                    const formData = new FormData();
                    formData.append('file', payloadBlob, file.name);

                    const xhr = new XMLHttpRequest();
                    xhr.open('POST', '/upload', true);

                    xhr.upload.onprogress = function (event) {
                        if (event.lengthComputable) {
                            const percent = (event.loaded / event.total) * 100;
                            onProgressCallback(percent);
                        }
                    };

                    xhr.onload = function () {
                        if (xhr.status >= 200 && xhr.status < 300) {
                            resolve();
                        } else {
                            const errMsg = xhr.responseText || `Server Error (Status ${xhr.status})`;
                            reject(new Error(errMsg));
                        }
                    };

                    xhr.onerror = function () {
                        reject(new Error('Network connection error'));
                    };

                    xhr.send(formData);
                } catch (err) {
                    reject(err);
                }
            });
        }
    </script>
</body>
</html>
"#;

    // 2. Pure-JS Encryption Module using node-forge (AES-GCM packet framing)
let encryption_module = r#"
        const ENCRYPTION_KEY = window.location.hash.substring(1);

        async function encryptBuffer(rawBuffer) {
            if (!ENCRYPTION_KEY) {
                throw new Error("Missing encryption key in URL hash (#KEY)");
            }

            const rawBytes = new Uint8Array(rawBuffer);
            
            // The key in the URL hash is already valid ASCII bytes/string
            const keyBytes = forge.util.createBuffer(ENCRYPTION_KEY, 'utf8');

            // 12-byte random IV/nonce
            const iv = forge.random.getBytesSync(12);

            const cipher = forge.cipher.createCipher('AES-GCM', keyBytes);
            cipher.start({ iv: iv });
            cipher.update(forge.util.createBuffer(rawBytes));
            cipher.finish();

            const ciphertext = cipher.output.getBytes();
            const tag = cipher.mode.tag.getBytes();

            const cipherLen = ciphertext.length + tag.length;
            const packetLen = 4 + 12 + cipherLen;
            const packet = new Uint8Array(packetLen);

            // 1. Write 4-byte Big-Endian length header
            packet[0] = (cipherLen >> 24) & 0xFF;
            packet[1] = (cipherLen >> 16) & 0xFF;
            packet[2] = (cipherLen >> 8) & 0xFF;
            packet[3] = cipherLen & 0xFF;

            // 2. Copy 12-byte Nonce / IV
            for (let i = 0; i < 12; i++) {
                packet[4 + i] = iv.charCodeAt(i);
            }

            // 3. Copy Ciphertext + 16-byte Tag
            for (let i = 0; i < ciphertext.length; i++) {
                packet[16 + i] = ciphertext.charCodeAt(i);
            }
            for (let i = 0; i < tag.length; i++) {
                packet[16 + ciphertext.length + i] = tag.charCodeAt(i);
            }

            return packet.buffer;
        }
    "#;

    let injection = if state.encryption.is_some() {
        encryption_module
    } else {
        ""
    };

    let badge = if state.encryption.is_some() {
        "🔒 (E2EE)"
    } else {
        ""
    };

    let final_page = page_template
        .replace("{ENCRYPTION_MODULE}", injection)
        .replace("{E2EE_BADGE}", badge);

    Html(final_page)
}