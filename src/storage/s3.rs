use crate::error::{GpError, Result};
use crate::scanner::ScannedFile;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;

const DEFAULT_BUCKET: &str = "groovepush-bucket";

pub struct S3Storage {
    client: Client,
    bucket: String,
}

impl S3Storage {
    pub async fn new(bucket: Option<String>) -> Result<Self> {
        let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        let client = Client::new(&config);
        let bucket = bucket.unwrap_or_else(|| DEFAULT_BUCKET.to_string());

        Ok(Self { client, bucket })
    }

    pub async fn upload_files(
        &self,
        project_name: &str,
        files: &[ScannedFile],
    ) -> Result<()> {
        if files.is_empty() {
            println!("アップロードするファイルはありません");
            return Ok(());
        }

        let pb = ProgressBar::new(files.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                .expect("プログレスバーのテンプレートエラー")
                .progress_chars("#>-"),
        );

        let mut handles = Vec::new();

        for file in files {
            let client = self.client.clone();
            let bucket = self.bucket.clone();
            let key = format!("{}/{}", project_name, file.relative_path.display());
            let absolute_path = file.absolute_path.clone();
            let pb = pb.clone();

            let handle = tokio::spawn(async move {
                let body = fs::read(&absolute_path).await?;
                let stream = ByteStream::from(body);

                client
                    .put_object()
                    .bucket(&bucket)
                    .key(&key)
                    .body(stream)
                    .send()
                    .await
                    .map_err(|e| std::io::Error::other(e.to_string()))?;

                pb.inc(1);
                Ok::<_, std::io::Error>(())
            });

            handles.push(handle);
        }

        for handle in handles {
            handle
                .await
                .map_err(|e| GpError::S3Error(e.to_string()))?
                .map_err(|e| GpError::S3Error(e.to_string()))?;
        }

        pb.finish_with_message("アップロード完了");
        Ok(())
    }

    pub async fn get_remote_state(
        &self,
        project_name: &str,
    ) -> Result<HashMap<String, String>> {
        let mut state = HashMap::new();
        let prefix = format!("{}/.gp/current_state.json", project_name);

        let result = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(&prefix)
            .send()
            .await;

        match result {
            Ok(output) => {
                let body = output
                    .body
                    .collect()
                    .await
                    .map_err(|e| GpError::S3Error(e.to_string()))?;
                let bytes = body.into_bytes();
                let content = String::from_utf8_lossy(&bytes);
                state = serde_json::from_str(&content)
                    .map_err(|e| GpError::S3Error(e.to_string()))?;
            }
            Err(_) => {}
        }

        Ok(state)
    }

    pub async fn save_state(
        &self,
        project_name: &str,
        files: &[ScannedFile],
    ) -> Result<()> {
        let state: HashMap<String, String> = files
            .iter()
            .map(|f| {
                (
                    f.relative_path.to_string_lossy().to_string(),
                    f.hash.clone(),
                )
            })
            .collect();

        let key = format!("{}/.gp/current_state.json", project_name);
        let body = serde_json::to_string_pretty(&state)
            .map_err(|e| GpError::S3Error(e.to_string()))?;

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&key)
            .body(ByteStream::from(body.into_bytes()))
            .send()
            .await
            .map_err(|e| GpError::S3Error(e.to_string()))?;

        Ok(())
    }

    pub fn bucket(&self) -> &str {
        &self.bucket
    }
}

pub fn extract_project_name(path: &Path) -> String {
    path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unnamed_project")
        .to_string()
}
