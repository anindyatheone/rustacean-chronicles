use leptos::prelude::*;

#[component]
pub fn CutsceneViewer(
    #[prop(into)] episode: Signal<u32>,
    #[prop(into)] is_intro: Signal<bool>,
    #[prop(into)] on_continue: Callback<()>,
) -> impl IntoView {
    let title = move || match (episode.get(), is_intro.get()) {
        (1, true) => "Episode 1: Awakening".to_string(),
        (1, false) => "Episode 1: Diagnostics Complete".to_string(),
        (2, true) => "Episode 2: The Asteroid Field".to_string(),
        (ep, true) => format!("Episode {}: Prologue", ep),
        (ep, false) => format!("Episode {}: Conclusion", ep),
    };

    let text = move || match (episode.get(), is_intro.get()) {
        (1, true) => "You awaken from cryo-sleep to the blare of klaxons. The primary reactor is offline, and the AI is corrupted. It's up to you to manually reboot the core systems using Rust.".to_string(),
        (1, false) => "The power surges back online. The monitors flicker to life, but the proximity alarms are already screaming. We are not safe yet.".to_string(),
        (2, true) => "An uncharted dense asteroid field blocks the path to the nearest relay station. The navigation CPU is burned out. We have to pilot through this manually.".to_string(),
        (_, true) => "The journey continues deeper into the legacy sector...".to_string(),
        (_, false) => "Another challenge overcome. The ship holds together.".to_string(),
    };

    let status_text = move || match (episode.get(), is_intro.get()) {
        (1, true) => ("SYSTEM OFFLINE", "AWAITING ENGINEERING OVERRIDE..."),
        (1, false) => ("SYSTEM RESTORED", "AWAITING NAVIGATIONAL INPUT..."),
        (2, true) => ("COLLISION IMMINENT", "EVASIVE MANEUVERS REQUIRED..."),
        (2, false) => ("HAZARD CLEARED", "CORE TEMPERATURE NOMINAL..."),
        (3, true) => ("POWER GRID CRITICAL", "REROUTING ENERGY RESERVES..."),
        (3, false) => ("POWER STABILIZED", "SYSTEMS NOMINAL..."),
        (4, true) => ("UNKNOWN ARTIFACT DETECTED", "INITIATING CLASSIFICATION PROTOCOL..."),
        (4, false) => ("CATALOG UPDATED", "CARGO SECURED..."),
        (5, true) => ("HULL BREACH DETECTED", "SEALING BULKHEADS..."),
        (5, false) => ("INTEGRITY RESTORED", "DAMAGE CONTAINED..."),
        (6, true) => ("MANIFEST CORRUPTED", "INITIATING RECOVERY SEQUENCE..."),
        (6, false) => ("DATABASE REBUILT", "CREW ACCOUNTED FOR..."),
        (7, true) => ("ALIEN FREQUENCY INTERCEPTED", "ATTEMPTING TRANSLATION..."),
        (7, false) => ("TRANSLATION COMPLETE", "COMMUNICATION ESTABLISHED..."),
        (8, true) => ("WARP COILS DE-SYNCHRONIZED", "INITIATING MULTITHREADED ALIGNMENT..."),
        (8, false) => ("COILS SYNCHRONIZED", "READY FOR JUMP..."),
        (9, true) => ("CONFORMITY GATE DETECTED", "PREPARING FINAL OVERRIDE..."),
        (9, false) => ("GATE OPEN", "TRAJECTORY LOCKED..."),
        (_, true) => ("SYSTEM WARNING", "UNKNOWN ERROR..."),
        (_, false) => ("TASK COMPLETE", "AWAITING ORDERS..."),
    };

    let image_url = move || match (episode.get(), is_intro.get()) {
        (1, true) => "/assets/cutscenes/intro_1.svg".to_string(),
        (1, false) => "/assets/cutscenes/outro_1.svg".to_string(),
        (_, true) => "/assets/cutscenes/intro_1.svg".to_string(), // Fallback
        (_, false) => "/assets/cutscenes/outro_1.svg".to_string(), // Fallback
    };

    view! {
        <div class="cutscene-overlay">
            <div class="cutscene-image-container">
                <img src=image_url class="cutscene-img" alt="Cutscene illustration" />
                <div class=move || format!("cutscene-status-overlay {}", if is_intro.get() { "status-intro" } else { "status-outro" })>
                    <h1 class="status-primary">{move || status_text().0}</h1>
                    <p class="status-secondary">{move || status_text().1}</p>
                </div>
            </div>
            <div class="cutscene-dialogue-box">
                <h2 class="cutscene-title">{title}</h2>
                <p class="cutscene-text">{text}</p>
                <div class="cutscene-actions">
                    <button class="btn proceed-btn" on:click=move |_| on_continue.run(())>
                        "Continue ⏭"
                    </button>
                </div>
            </div>
        </div>
    }
}
