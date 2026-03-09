use tauri::{TitleBarStyle, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_updater::UpdaterExt;

const INIT_SCRIPT: &str = r#"
(function() {
    // Add padding for overlay title bar (macOS traffic lights)
    var style = document.createElement('style');
    style.textContent = 'body { padding-top: 28px !important; }';
    document.head.appendChild(style);

    // Intercept target="_blank" link clicks
    document.addEventListener('click', function(e) {
        var link = e.target.closest('a');
        if (link && link.href && (link.target === '_blank' || link.target === '_new')) {
            e.preventDefault();
            e.stopPropagation();
            window.location.href = 'https://anun.tech/__tauri_open_external__?url=' + encodeURIComponent(link.href);
        }
    }, true);

    // Override window.open → open in system browser
    window.open = function(url) {
        if (url) {
            window.location.href = 'https://anun.tech/__tauri_open_external__?url=' + encodeURIComponent(url);
        }
        return null;
    };
})();
"#;

fn build_window<R: tauri::Runtime>(
    manager: &impl tauri::Manager<R>,
    label: &str,
    url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    WebviewWindowBuilder::new(
        manager,
        label,
        WebviewUrl::External(url.parse()?),
    )
    .title("Anuntech")
    .hidden_title(true)
    .title_bar_style(TitleBarStyle::Overlay)
    .inner_size(1280.0, 800.0)
    .min_inner_size(800.0, 600.0)
    .center()
    .initialization_script(INIT_SCRIPT)
    .on_navigation(move |url| {
        // Intercept sentinel URL → open real URL in system browser
        if url.path() == "/__tauri_open_external__" {
            if let Some((_, external_url)) = url.query_pairs().find(|(k, _)| k == "url") {
                let _ = open::that(external_url.as_ref());
            }
            return false;
        }

        true
    })
    .build()?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            // Create main window with overlay title bar
            build_window(app, "main", "https://anun.tech")
                .expect("failed to create main window");

            // Check for updates in background
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                match handle.updater().unwrap().check().await {
                    Ok(Some(update)) => {
                        log::info!(
                            "Update available: {}",
                            update.version
                        );
                        if let Err(e) = update.download_and_install(|_, _| {}, || {}).await {
                            log::error!("Failed to install update: {}", e);
                        }
                    }
                    Ok(None) => {
                        log::info!("No update available");
                    }
                    Err(e) => {
                        log::error!("Failed to check for updates: {}", e);
                    }
                }
            });

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
