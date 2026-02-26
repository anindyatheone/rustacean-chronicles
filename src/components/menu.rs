use leptos::prelude::*;
use leptos::web_sys;

// ...

#[component]
pub fn Menu(
    #[prop(into)] current_episode: WriteSignal<u32>,
    #[prop(into)] current_module: WriteSignal<u32>,
    #[prop(into)] highest_unlocked_episode: Signal<u32>,
    #[prop(into)] set_highest_unlocked_episode: WriteSignal<u32>,
    #[prop(into)] is_cutscene: WriteSignal<bool>,
    #[prop(into)] is_intro: WriteSignal<bool>,
) -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    let reset_progress = move |_| {
        if let Some(window) = leptos::web_sys::window() {
            if let Ok(Some(local_storage)) = window.local_storage() {
                let _ = local_storage.remove_item("saved_episode");
                let _ = local_storage.remove_item("saved_module");
                let _ = local_storage.remove_item("highest_unlocked_episode");
            }
        }
        current_episode.set(1);
        current_module.set(1);
        set_highest_unlocked_episode.set(1);
        is_cutscene.set(true);
        is_intro.set(true);
        set_is_open.set(false);
    };

    let jump_to_episode = move |ep: u32| {
        current_episode.set(ep);
        current_module.set(1);
        is_cutscene.set(true);
        is_intro.set(true);
        set_is_open.set(false);
    };

    view! {
        <div class="menu-container">
            <button class="menu-toggle-btn" on:click=move |_| set_is_open.update(|open| *open = !*open)>
                "⚙️"
            </button>
            {move || if is_open.get() {
                view! {
                    <div class="menu-panel">
                        <h3>"Systems Menu"</h3>
                        
                        <div class="menu-section">
                            <h4>"Replay Logs"</h4>
                            <div class="episode-list">
                                {move || (1..=highest_unlocked_episode.get()).map(|ep| {
                                    view! {
                                        <button class="btn btn-replay" on:click=move |_| jump_to_episode(ep)>
                                            {format!("Episode {}", ep)}
                                        </button>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>

                        <div class="menu-section">
                            <h4>"Danger Zone"</h4>
                            <button class="btn btn-reset" on:click=reset_progress>
                                "Reset All Progress"
                            </button>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <span class="hidden"></span> }.into_any()
            }}
        </div>
    }
}
