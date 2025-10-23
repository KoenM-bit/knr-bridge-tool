use crate::config::Config;
use crate::model::{DonePayload, Job};
use anyhow::Result;
use reqwest::{multipart, Client};
use serde_json::Value;
use std::{fs, path::PathBuf, time::Duration};
use tokio::time::sleep;

pub struct Bridge {
    client: Client,
    base: String,
    robot_base: String,
    secret: String,
    poll_ms: u64,
    tmp_dir: PathBuf,
    cfg: Config,
}

impl Bridge {
    pub fn new(cfg: Config) -> Self {
        use std::path::PathBuf;

        // Try loading .env from multiple locations
        let mut env_paths = vec![
            PathBuf::from("./.env"),                    // Current directory
            PathBuf::from("./src-tauri/.env"),         // Development location
            PathBuf::from("../src-tauri/.env"),        // In case running from parent
        ];
        
        // Add path next to the executable
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                env_paths.push(exe_dir.join(".env"));
                // Also try one level up from executable (common in app bundles)
                if let Some(parent_dir) = exe_dir.parent() {
                    env_paths.push(parent_dir.join(".env"));
                }
            }
        }
        
        let mut env_loaded = false;
        for env_path in &env_paths {
            if dotenvy::from_path(env_path).is_ok() {
                println!("✅ Loaded .env from {:?}", env_path);
                env_loaded = true;
                break;
            }
        }
        
        if !env_loaded {
            println!("⚠️ Could not load .env from any of these paths: {:?}", env_paths);
        }

        let tmp_dir = std::env::temp_dir().join("opentrons-bridge");
        let _ = fs::create_dir_all(&tmp_dir);

        let base = std::env::var("BACKEND").unwrap_or_else(|_| {
            eprintln!("❌ BACKEND environment variable not set! This will cause 'relative URL without a base' errors.");
            String::new()
        });
        let secret = std::env::var("BRIDGE_SHARED_SECRET").unwrap_or_default();
        let poll_ms = std::env::var("POLL_MS")
            .ok()
            .and_then(|x| x.parse().ok())
            .unwrap_or(5000);

        println!("🔧 Bridge config loaded:");
        println!(
            "  BASE = {}\n  ROBOT_BASE = {}\n  ROBOT_ID = {}\n  POLL_MS = {}\n  TMP_DIR = {:?}",
            base, cfg.robot_base, cfg.robot_id, poll_ms, tmp_dir
        );

        Self {
            client: Client::new(),
            base,
            robot_base: cfg.robot_base.clone(),
            secret,
            poll_ms,
            tmp_dir,
            cfg,
        }
    }

    pub async fn run(&self) -> Result<()> {
        println!("🔧 Bridge started for {}", self.cfg.robot_id);
        loop {
            if let Err(e) = self.tick().await {
                eprintln!("Tick error: {:?}", e);
            }
            sleep(Duration::from_millis(self.poll_ms)).await;
        }
    }

    async fn tick(&self) -> Result<()> {
        let job = self.get_job().await?;
        if let Some(job) = job {
            println!("⚙️ Processing job {}", job.id);
            self.ack(&job.id).await?;
            if let Err(e) = self.process_job(&job).await {
                eprintln!("❌ Job {} failed: {:?}", job.id, e);
                let _ = self.mark_done(&job.id, false, Some(e.to_string())).await;
            }
        } else {
            println!("No jobs found.");
        }
        Ok(())
    }

    // ===============================================================
    // ================ BACKEND COMMUNICATION ========================
    // ===============================================================

    async fn get_job(&self) -> Result<Option<Job>> {
        if self.base.is_empty() {
            return Err(anyhow::anyhow!("BACKEND environment variable is not set. Please check your .env file and ensure BACKEND is properly configured."));
        }
        
        let url = format!("{}/jobs-get?robotId={}", self.base, self.cfg.robot_id);
        println!("📡 GET {}", url);

        let res = self
            .client
            .get(&url)
            .header("x-bridge-secret", &self.secret)
            .send()
            .await?;

        let status = res.status();
        if !status.is_success() {
            eprintln!("⚠️ jobs-get failed: {}", status);
        }

        let res: Value = res.json().await?;
        let jobs = res["jobs"].as_array().cloned().unwrap_or_default();
        Ok(jobs.first().and_then(|j| serde_json::from_value(j.clone()).ok()))
    }

    async fn ack(&self, id: &str) -> Result<()> {
        if self.base.is_empty() {
            return Err(anyhow::anyhow!("BACKEND environment variable is not set. Please check your .env file and ensure BACKEND is properly configured."));
        }
        
        let url = format!("{}/jobs-ack", self.base);
        println!("📡 POST {} (ack)", url);

        let res = self
            .client
            .post(&url)
            .header("x-bridge-secret", &self.secret)
            .json(&serde_json::json!({ "id": id }))
            .send()
            .await?;

        println!("📨 ack response: {}", res.status());
        Ok(())
    }

    async fn mark_done(&self, id: &str, success: bool, msg: Option<String>) -> Result<()> {
        if self.base.is_empty() {
            return Err(anyhow::anyhow!("BACKEND environment variable is not set. Please check your .env file and ensure BACKEND is properly configured."));
        }
        
        let url = format!("{}/jobs-done", self.base);
        println!("📡 POST {} (mark_done)", url);

        let payload = DonePayload {
            id: id.into(),
            result: if success {
                Some(serde_json::json!({"ok": true}))
            } else {
                None
            },
            error: if success { None } else { msg },
        };

        let res = self
            .client
            .post(&url)
            .header("x-bridge-secret", &self.secret)
            .json(&payload)
            .send()
            .await?;

        println!("📨 jobs-done response: {}", res.status());
        Ok(())
    }

    async fn get_bridge_token(&self) -> Result<String> {
        if self.base.is_empty() {
            return Err(anyhow::anyhow!("BACKEND environment variable is not set. Please check your .env file and ensure BACKEND is properly configured."));
        }
        
        let url = format!("{}/get-bridge-token", self.base);
        println!("📡 POST {} (get token)", url);

        let res = self
            .client
            .post(&url)
            .header("x-bridge-secret", &self.secret)
            .send()
            .await?;

        if !res.status().is_success() {
            eprintln!("⚠️ get-bridge-token failed: {}", res.status());
        }

        let json: Value = res.json().await?;
        Ok(json["token"].as_str().unwrap_or("").to_string())
    }

    // ===============================================================
    // ================ ROBOT COMMUNICATION ==========================
    // ===============================================================

    async fn process_job(&self, job: &Job) -> Result<()> {
        if self.base.is_empty() {
            return Err(anyhow::anyhow!("BACKEND environment variable is not set. Please check your .env file and ensure BACKEND is properly configured."));
        }
        
        let token = self.get_bridge_token().await?;
        let url = format!("{}/signed-download-secure?id={}", self.base, job.id);
        println!("📡 GET {} (signed download)", url);

        let data: Value = self
            .client
            .get(&url)
            .bearer_auth(&token)
            .send()
            .await?
            .json()
            .await?;

        let download_url = data["url"].as_str().unwrap();
        let tmp_path = self.tmp_dir.join(format!("{}.py", job.id));

        println!("⬇️ Downloading .py from {}", download_url);
        let bytes = self.client.get(download_url).send().await?.bytes().await?;
        fs::write(&tmp_path, &bytes)?;
        println!("💾 Saved protocol locally at {:?}", tmp_path);

        // Upload & create run
        let protocol_id = self.upload_protocol(&tmp_path).await?;
        let run_id = self.create_run(&protocol_id).await?;
        println!("✅ Uploaded protocol {}, created run {}", protocol_id, run_id);

        self.mark_done(&job.id, true, None).await?;
        println!("✅ Job {} completed successfully.", job.id);

        Ok(())
    }

    async fn upload_protocol(&self, path: &PathBuf) -> Result<String> {
        use reqwest::multipart::Part;
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        println!("📤 Uploading protocol file {:?}", path);

        let part = Part::bytes(buffer).file_name("protocol.py");
        let form = multipart::Form::new().part("files", part);

        let res = self
            .client
            .post(format!("{}/protocols", self.robot_base))
            .header("opentrons-version", "4")
            .multipart(form)
            .send()
            .await?;

        let json: Value = res.json().await?;
        Ok(json["data"]["id"].as_str().unwrap_or("").to_string())
    }

    async fn create_run(&self, protocol_id: &str) -> Result<String> {
        println!("🧭 Creating run for protocol {}", protocol_id);

        let res = self
            .client
            .post(format!("{}/runs", self.robot_base))
            .header("opentrons-version", "4")
            .json(&serde_json::json!({ "data": { "protocolId": protocol_id } }))
            .send()
            .await?;

        let json: Value = res.json().await?;
        Ok(json["data"]["id"].as_str().unwrap_or("").to_string())
    }
}