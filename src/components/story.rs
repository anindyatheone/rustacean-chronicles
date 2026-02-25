use leptos::prelude::*;

#[component]
pub fn StoryPanel(
    episode_title: Signal<String>,
    module_title: Signal<String>,
    story_text: Signal<Vec<String>>,
    concept_title: Signal<String>,
    concept_text: Signal<Vec<String>>,
    challenge_text: Signal<Vec<String>>,
    is_completed: ReadSignal<bool>,
) -> impl IntoView {
    view! {
        <aside class="story-panel">
            <header class="story-header">
                <h2>{move || episode_title.get()}</h2>
                {move || if is_completed.get() {
                    view! { <div class="status-indicator success">"Status: OPERATIONAL"</div> }.into_any()
                } else {
                    view! { <div class="status-indicator warning">"Status: CRITICAL"</div> }.into_any()
                }}
            </header>
            
            <div class="story-content">
                <h3>{move || module_title.get()}</h3>
                {move || story_text.get().into_iter().map(|p| view! { <p>{p}</p> }).collect_view()}
                
                <div class="concept-box">
                    <h4>{move || concept_title.get()}</h4>
                    {move || concept_text.get().into_iter().map(|p| view! { <p>{p}</p> }).collect_view()}
                </div>
                
                <div class="challenge-box">
                    <h4>"Mission Objective"</h4>
                    {move || challenge_text.get().into_iter().map(|p| view! { <p>{p}</p> }).collect_view()}
                </div>
            </div>
        </aside>
    }
}
