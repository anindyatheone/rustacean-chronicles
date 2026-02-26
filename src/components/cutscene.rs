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
        (2, false) => "Episode 2: Clear Skies".to_string(),
        (3, true) => "Episode 3: Grid Failure".to_string(),
        (3, false) => "Episode 3: Power Restored".to_string(),
        (4, true) => "Episode 4: Unidentified Cargo".to_string(),
        (4, false) => "Episode 4: Artifacts Secured".to_string(),
        (5, true) => "Episode 5: Hull Breach!".to_string(),
        (5, false) => "Episode 5: Pressure Stabilized".to_string(),
        (6, true) => "Episode 6: Data Corruption".to_string(),
        (6, false) => "Episode 6: Manifest Recovered".to_string(),
        (7, true) => "Episode 7: The Signal".to_string(),
        (7, false) => "Episode 7: First Contact".to_string(),
        (8, true) => "Episode 8: Spacial Distortion".to_string(),
        (8, false) => "Episode 8: Warp Coils Aligned".to_string(),
        (9, true) => "Episode 9: The Conformity Gate".to_string(),
        (9, false) => "Epilogue: Journey's End".to_string(),
        (ep, true) => format!("Episode {}: Prologue", ep),
        (ep, false) => format!("Episode {}: Conclusion", ep),
    };

    let text = move || match (episode.get(), is_intro.get()) {
        (1, true) => "You awaken from cryo-sleep to the blare of klaxons. The primary reactor is offline, and the AI is corrupted. It's up to you to manually reboot the core systems using Rust.".to_string(),
        (1, false) => "The power surges back online. The monitors flicker to life, but the proximity alarms are already screaming. We are not safe yet.".to_string(),
        (2, true) => "An uncharted dense asteroid field blocks the path to the nearest relay station. The navigation CPU is burned out. We have to pilot through this manually.".to_string(),
        (2, false) => "The ship clears the final asteroid, emerging into open space. Your evasive maneuvers held the hull intact, but the stress has damaged the power grid.".to_string(),
        (3, true) => "The engineering bay is in chaos. Energy reserves are critical and the battery nodes are fighting over control of the main circuit.".to_string(),
        (3, false) => "The energy flows smoothly now. Ownership of the power nodes stays strictly regulated. The ship's internal systems hum back to life.".to_string(),
        (4, true) => "Sensors detect a derelict vessel adrift nearby. It contains alien technology that must be safely categorized and secured in the cargo hold.".to_string(),
        (4, false) => "The tech is stabilized and safely bound to the ship's inventory. We've salvaged what we need to continue our journey.".to_string(),
        (5, true) => "A micro-meteorite storms tears through the starboard hull! Pressure is dropping rapidly. The ship's damage sensors are panicking in the background.".to_string(),
        (5, false) => "Bulkheads sealed and emergency forcefields established. No casualties reported. The immediate danger has passed... for now.".to_string(),
        (6, true) => "The shock from the impact corrupted the crew manifest database. Without it, life support can't allocate oxygen to the correct decks.".to_string(),
        (6, false) => "The personnel vectors and life-support hash maps are fully rebuilt. The crew breathes easy once again.".to_string(),
        (7, true) => "An anomalous, encrypted transmission penetrates the comms array. Its syntax is completely alien. We have to build a universal translator.".to_string(),
        (7, false) => "The trait bounds aligned perfectly. The transmission translates to a warning: We are rapidly approaching the Conformity Gate.".to_string(),
        (8, true) => "The gate requires a precise warp jump, but the port and starboard warp coils are heavily de-synchronized. They must fire in total unison.".to_string(),
        (8, false) => "The threads synchronized flawlessly. The warp core shines with brilliant blue energy, charging for the final push through the barrier.".to_string(),
        (9, true) => "The Conformity Gate looms ahead, a massive megastructure blocking our path home. It demands an offering of perfectly structured logic to pass.".to_string(),
        (9, false) => "The final override is accepted. The gate opens into a radiant wormhole, carrying the ship toward the safety of the Core Worlds. You survived.".to_string(),
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
