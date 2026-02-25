use leptos::prelude::*;

#[component]
pub fn Workspace(
    current_episode: ReadSignal<u32>,
    current_module: ReadSignal<u32>,
    inputs: ReadSignal<Vec<String>>,
    set_inputs: WriteSignal<Vec<String>>,
    terminal_output: ReadSignal<Vec<(String, String)>>,
    is_completed: ReadSignal<bool>,
    trigger_run: WriteSignal<u32>,
    trigger_next: WriteSignal<u32>,
) -> impl IntoView {
    view! {
        <section class="workspace" style="position: relative;">
            <div class="editor-pane" style="position: relative;">
                <div class="pane-header" style="justify-content: space-between; align-items: center; padding-right: 1.5rem;">
                    <span class="tab active">"src/main.rs"</span>
                </div>
                
                <div class="run-btn-container">
                    <button class="run-btn" on:click=move |_| trigger_run.update(|n| *n += 1) title="Run Diagnostics">
                        "▶"
                    </button>
                </div>
                
                <div class="editor-content">
                    {move || match (current_episode.get(), current_module.get()) {
                        (1, 1) => view! {
                            <div class="code-static pre-code">"fn main() {"</div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    let power_level = "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 50px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">";"</span>
                            </div>

                            <div class="code-static post-code">
                                "    \n    println!(\"Power level set to {}%\", power_level);\n}"
                            </div>
                        }.into_any(),
                        
                        (1, 2) => view! {
                            <div class="code-static pre-code">"fn main() {"</div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    let "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 50px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[2] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[2].clone() />
                                <span class="inline-static">" ship_name = \"USS Rustacean\";"</span>
                            </div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    let "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 50px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[1] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[1].clone() />
                                <span class="inline-static">" reactor_status = \"Offline\";"</span>
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    let "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 50px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">" life_support_level = 10;"</span>
                            </div>

                            <div class="code-static post-code">
                                "    \n    // Automated system override sequence restoring life support capacity.\n    life_support_level = 100;\n    reactor_status = \"Online\";\n    \n    println!(\"Current Oxygen Level: {}%\", life_support_level);\n    println!(\"Reactor status: {}\", reactor_status);\n    println!(\"Ship Name: {}\", ship_name);\n}"
                            </div>
                        }.into_any(),
                        
                        (2, 1) => view! {
                            <div class="code-static pre-code">"fn main() {\n    let asteroid_distance = 450;\n    let mut maneuver = \"Unknown\";\n"</div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 400px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">" {"</span>
                            </div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"        maneuver = "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 110px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[1] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[1].clone() />
                                <span class="inline-static">";"</span>
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    } "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 90px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[2] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[2].clone() />
                                <span class="inline-static">" {"</span>
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"        maneuver = "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 260px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[3] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[3].clone() />
                                <span class="inline-static">";"</span>
                            </div>

                            <div class="code-static post-code">
                                "    }\n    println!(\"Maneuver protocol: {}\", maneuver);\n}"
                            </div>
                        }.into_any(),
                        
                        (2, 2) => view! {
                            <div class="code-static pre-code">"fn main() {\n    let mut core_temp = 120;\n"</div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 300px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">" {"</span>
                            </div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"        "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 210px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[1] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[1].clone() />
                                <span class="inline-static">";"</span>
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"        println!(\"Cooling... Temp: {}\", core_temp);"</span>
                            </div>

                            <div class="code-static post-code">
                                "    }\n    println!(\"Core temperature stabilized.\");\n}"
                            </div>
                        }.into_any(),
                        
                        (2, 3) => view! {
                            <div class="code-static pre-code">"fn main() {\n    let mut fired_count = 0;\n"</div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 360px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">" {"</span>
                            </div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"        "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 350px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[1] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[1].clone() />
                                <span class="inline-static">" {"</span>
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"            "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 240px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[2] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[2].clone() />
                                <span class="inline-static">";"</span>
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"            println!(\"Thruster {} fired.\", thruster_id);"</span>
                            </div>

                            <div class="code-static post-code">
                                "        }\n    }\n    println!(\"Evasion sequence complete. Fired {} thrusters.\", fired_count);\n}"
                            </div>
                        }.into_any(),
                        
                        (3, 1) => view! {
                            <div class="code-static pre-code">"fn main() {\n    let aux_battery = String::from(\"1500 kWh\");\n    transfer_power(aux_battery);\n    \n    // The line below causes a compile error because ownership was moved!\n    // Fix the error by commenting it out (add // at the start)."</div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 50px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">" println!(\"Aux: {}\", aux_battery);"</span>
                            </div>

                            <div class="code-static post-code">
                                "}\n\nfn transfer_power(power: String) {\n    println!(\"Main grid received: {}\", power);\n}"
                            </div>
                        }.into_any(),

                        (3, 2) => view! {
                            <div class="code-static pre-code">"fn main() {\n    let nav_logs = String::from(\"Starbase X-9 is 4 parsecs away.\");\n"</div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    read_logs("</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 35px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">"nav_logs);"</span>
                            </div>

                            <div class="code-static pre-code" style="border:none; padding-bottom:0;">
                                "    \n    println!(\"Logs successfully retained: {}\", nav_logs);\n}\n"
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"fn read_logs(logs: "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 35px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[1] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[1].clone() />
                                <span class="inline-static">"String) {"</span>
                            </div>

                            <div class="code-static post-code">
                                "    println!(\"Reading: {}\", logs);\n}"
                            </div>
                        }.into_any(),

                        (3, 3) => view! {
                            <div class="code-static pre-code">"fn main() {\n    let mut reactor_state = String::from(\"Unstable\");\n"</div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    recalibrate("</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 75px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">"reactor_state);"</span>
                            </div>

                            <div class="code-static pre-code" style="border:none; padding-bottom:0;">
                                "    \n    println!(\"Reactor is now: {}\", reactor_state);\n}\n"
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"fn recalibrate(state: "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 75px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[1] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[1].clone() />
                                <span class="inline-static">"String) {"</span>
                            </div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    state."</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 120px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[2] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[2].clone() />
                                <span class="inline-static">"(\" - Calibrated\");"</span>
                            </div>

                            <div class="code-static post-code">
                                "}"
                            </div>
                        }.into_any(),
                        
                        (4, 1) => view! {
                            <div class="code-static pre-code" style="border:none; padding-bottom:0;">"struct "</div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 120px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">" {"</span>
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    id: "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 50px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[1] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[1].clone() />
                                <span class="inline-static">","</span>
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    name: "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 90px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[2] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[2].clone() />
                                <span class="inline-static">","</span>
                            </div>

                            <div class="code-static post-code">
                                "}\n\nfn main() {\n    let relic = Artifact {\n        id: 42,\n        name: String::from(\"Glowing Orb\"),\n    };\n    println!(\"Cataloged {}: {}\", relic.id, relic.name);\n}"
                            </div>
                        }.into_any(),

                        (4, 2) => view! {
                            <div class="code-static pre-code">"struct Artifact { id: u32, name: String }\n"</div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">""</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 40px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">" Artifact {"</span>
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    fn analyze("</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 75px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[1] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[1].clone() />
                                <span class="inline-static">") {"</span>
                            </div>

                            <div class="code-static post-code">
                                "        println!(\"Analyzing artifact: {}\", self.name);\n    }\n}\n\nfn main() {\n    let relic = Artifact { id: 42, name: String::from(\"Glowing Orb\") };\n    relic.analyze();\n}"
                            </div>
                        }.into_any(),

                        (4, 3) => view! {
                            <div class="code-static pre-code">"enum Status { Safe, Hazardous, Edible }\n\nfn process(status: Status) {\n"</div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 50px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">" status {"</span>
                            </div>

                            <div class="code-static pre-code" style="border:none; padding-bottom:0;">
                                "        Status::Safe => println!(\"Stowing in cargo bay.\"),\n        Status::Hazardous => println!(\"Ejecting out the airlock!\"),"
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"        "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 210px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[1] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[1].clone() />
                                <span class="inline-static">" => println!(\"Sending to the galley.\"),"</span>
                            </div>

                            <div class="code-static post-code">
                                "    }\n}\n\nfn main() {\n    process(Status::Edible);\n}"
                            </div>
                        }.into_any(),
                        
                        (5, 1) => view! {
                            <div class="code-static pre-code">"fn get_reserves() -> Option<String> {\n    Some(String::from(\"Sector 7G\"))\n}\n\nfn main() {\n    let reserves = get_reserves();\n"</div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 75px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">" reserves {"</span>
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"        "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 60px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[1] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[1].clone() />
                                <span class="inline-static">"(location) => println!(\"Found at {}\", location),"</span>
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"        "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 60px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[2] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[2].clone() />
                                <span class="inline-static">" => println!(\"No reserves found!\"),"</span>
                            </div>

                            <div class="code-static post-code">
                                "    }\n}"
                            </div>
                        }.into_any(),

                        (5, 2) => view! {
                            <div class="code-static pre-code">"fn analyze_breach() -> Result<f32, String> {\n    Err(String::from(\"Sensor interference\"))\n}\n\nfn main() {\n    // This will panic and crash the thread!\n"</div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    let data = analyze_breach()."</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 90px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">"(\""</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 225px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[1] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[1].clone() />
                                <span class="inline-static">"\");"</span>
                            </div>

                            <div class="code-static post-code">
                                "    println!(\"Breach is {} meters wide.\", data);\n}"
                            </div>
                        }.into_any(),

                        (5, 3) => view! {
                            <div class="code-static pre-code">"fn read_sensors() -> Result<String, String> {\n    Err(String::from(\"Hardware disconnected\"))\n}\n\nfn process_data() -> Result<(), String> {\n"</div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    let data = read_sensors()"</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 25px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">";"</span>
                            </div>

                            <div class="code-static post-code">
                                "    println!(\"Processing: {}\", data);\n    Ok(())\n}\n\nfn main() {\n    if let Err(e) = process_data() {\n        println!(\"Mission aborted due to error: {}\", e);\n    }\n}"
                            </div>
                        }.into_any(),

                        (6, 1) => view! {
                            <div class="code-static pre-code">"fn main() {\n"</div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    let mut manifest: "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 120px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">" = Vec::"</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 50px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[1] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[1].clone() />
                                <span class="inline-static">"();"</span>
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    manifest."</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 60px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[2] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[2].clone() />
                                <span class="inline-static">"(742);"</span>
                            </div>

                            <div class="code-static pre-code" style="border:none; padding-bottom:0;">
                                "    manifest.push(901);\n    \n    println!(\"Recovered {} badges.\", manifest.len());\n}"
                            </div>
                        }.into_any(),

                        (6, 2) => view! {
                            <div class="code-static pre-code">"fn main() {\n    let mut first_name = String::from(\"   Liara   \");\n    let last_name = \"T'Soni\";\n"</div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    let clean_first = first_name."</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 60px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">"();"</span>
                            </div>

                            <div class="code-static pre-code" style="border:none; padding-bottom:0;">
                                "    let mut full_name = String::from(clean_first);"
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    full_name."</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 120px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[1] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[1].clone() />
                                <span class="inline-static">"(\" \");"</span>
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    full_name."</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 120px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[2] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[2].clone() />
                                <span class="inline-static">"(last_name);"</span>
                            </div>

                            <div class="code-static post-code">
                                "    \n    println!(\"Restored format: {}\", full_name);\n}"
                            </div>
                        }.into_any(),

                        (6, 3) => view! {
                            <div class="code-static pre-code">"use std::collections::HashMap;\n\nfn main() {\n"</div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    let mut assignments = "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 105px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">"::new();"</span>
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    assignments."</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 90px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[1] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[1].clone() />
                                <span class="inline-static">"(String::from(\"Engineering\"), String::from(\"Liara\"));\n"</span>
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    if let Some(name) = assignments."</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 50px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[2] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[2].clone() />
                                <span class="inline-static">"(\"Engineering\") {"</span>
                            </div>

                            <div class="code-static post-code">
                                "        println!(\"Engineering Officer: {}\", name);\n    }\n}"
                            </div>
                        }.into_any(),

                        (7, 1) => view! {
                            <div class="code-static pre-code">"trait Reset {\n    fn reset(&self);\n}\n\nstruct Thruster;\n"</div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">""</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 60px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">" Reset "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 50px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[1] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[1].clone() />
                                <span class="inline-static">" Thruster {"</span>
                            </div>

                            <div class="code-static post-code">
                                "    fn reset(&self) {\n        println!(\"Thruster realigned and ready.\");\n    }\n}\n\nfn main() {\n    let t = Thruster;\n    t.reset();\n}"
                            </div>
                        }.into_any(),

                        (7, 2) => view! {
                            <div class="code-static pre-code">"trait Reset { fn reset(&self); }\n\nstruct Sensor;\nimpl Reset for Sensor {\n    fn reset(&self) { println!(\"Sensor green.\"); }\n}\n"</div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"fn reboot_system<"</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 25px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">": "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 75px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[1] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[1].clone() />
                                <span class="inline-static">">(system: &T) {"</span>
                            </div>

                            <div class="code-static post-code">
                                "    system.reset();\n}\n\nfn main() {\n    let array = Sensor;\n    reboot_system(&array);\n}"
                            </div>
                        }.into_any(),

                        (7, 3) => view! {
                            <div class="code-static pre-code" style="border:none; padding-bottom:0;">
                                "#[derive("
                            </div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"         "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 75px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">")]"</span>
                            </div>

                            <div class="code-static pre-code" style="border:none; margin-top:-5px; padding-bottom:0;">
                                "struct Payload {\n    encryption_key: u32,\n    data: String,\n}\n\nfn main() {\n    let msg = Payload { \n        encryption_key: 1024, \n        data: String::from(\"S.O.S\") \n    };\n"
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    println!(\"Intercepted: {:"</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 25px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[1] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[1].clone() />
                                <span class="inline-static">"}\", msg);"</span>
                            </div>

                            <div class="code-static post-code">
                                "}"
                            </div>
                        }.into_any(),

                        (8, 1) => view! {
                            <div class="code-static pre-code">"use std::thread;\nuse std::time::Duration;\n\nfn main() {\n"</div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    let handle = thread::"</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 75px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">"(|| {"</span>
                            </div>

                            <div class="code-static pre-code" style="border:none; padding-bottom:0;">
                                "        println!(\"Starboard warp coil spinning up...\");\n        thread::sleep(Duration::from_millis(50));\n    });\n\n    println!(\"Port warp coil spinning up...\");\n"
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    handle."</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 60px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[1] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[1].clone() />
                                <span class="inline-static">"().unwrap();"</span>
                            </div>

                            <div class="code-static post-code">
                                "    println!(\"Both coils synchronized!\");\n}"
                            </div>
                        }.into_any(),

                        (8, 2) => view! {
                            <div class="code-static pre-code">"use std::thread;\nuse std::sync::mpsc;\n\nfn main() {\n"</div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    let (tx, rx) = mpsc::"</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 105px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">"();"</span>
                            </div>

                            <div class="code-static pre-code" style="border:none; padding-bottom:0;">
                                "    thread::spawn(move || {\n        let msg = String::from(\"Diagnostic sequence nominal.\");"
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"        tx."</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 60px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[1] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[1].clone() />
                                <span class="inline-static">"(msg).unwrap();\n    });\n"</span>
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    let received = rx."</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 60px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[2] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[2].clone() />
                                <span class="inline-static">"().unwrap();"</span>
                            </div>

                            <div class="code-static post-code">
                                "    println!(\"Main Engine Room log: {}\", received);\n}"
                            </div>
                        }.into_any(),

                        (8, 3) => view! {
                            <div class="code-static pre-code">"use std::sync::{Arc, Mutex};\nuse std::thread;\n\nfn main() {\n"</div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    let ai_status = "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 50px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">"::new("</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 75px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[1] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[1].clone() />
                                <span class="inline-static">"::new(0));"</span>
                            </div>

                            <div class="code-static pre-code" style="border:none; padding-bottom:0;">
                                "    let status_clone = Arc::clone(&ai_status);\n\n    let drone = thread::spawn(move || {\n"
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"        let mut num = status_clone."</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 60px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[2] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[2].clone() />
                                <span class="inline-static">"().unwrap();"</span>
                            </div>

                            <div class="code-static post-code">
                                "        *num += 100;\n        println!(\"Drone applied {} patches.\", *num);\n    });\n\n    drone.join().unwrap();\n    println!(\"AI core at {}% restoration.\", *ai_status.lock().unwrap());\n}"
                            </div>
                        }.into_any(),

                        (9, 1) => view! {
                            <div class="code-static pre-code">"fn parse_signal(signal: &str) -> Result<u32, &'static str> {\n    if signal == \"NOISE\" {\n"</div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"        return "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 50px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">"(\"Signal corrupted\");\n    }"</span>
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    match signal."</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 75px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[1] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[1].clone() />
                                <span class="inline-static">"::<u32>() {"</span>
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"        "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 40px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[2] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[2].clone() />
                                <span class="inline-static">"(num) => Ok(num),"</span>
                            </div>

                            <div class="code-static post-code">
                                "        Err(_) => Err(\"Invalid format\"),\n    }\n}\n\nfn main() {\n    let key = parse_signal(\"1042\").unwrap();\n    println!(\"Parsed Key: {} - locked in.\", key);\n}"
                            </div>
                        }.into_any(),

                        (9, 2) => view! {
                            <div class="code-static pre-code">"fn main() {\n    let manifest = vec![\"Pilot\", \"Empty\", \"Engineer\", \"Empty\", \"Gunner\"];\n"</div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    let active_crew: Vec<&str> = manifest."</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 135px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">"()"</span>
                            </div>

                            <div class="inline-editor-row spacer-row" style="padding-left: 20px;">
                                <span class="inline-static">"."</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 90px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[1] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[1].clone() />
                                <span class="inline-static">"(|&role| role != \"Empty\")"</span>
                            </div>

                            <div class="inline-editor-row spacer-row" style="padding-left: 20px;">
                                <span class="inline-static">"."</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 105px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[2] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[2].clone() />
                                <span class="inline-static">"();"</span>
                            </div>

                            <div class="code-static post-code">
                                "    println!(\"Active crew verified: {:?}\", active_crew);\n}"
                            </div>
                        }.into_any(),

                        (9, 3) => view! {
                            <div class="code-static pre-code">"use std::sync::{Arc, Mutex};\nuse std::thread;\n\nfn main() {\n"</div>
                            
                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"    let core = "</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 50px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[0] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[0].clone() />
                                <span class="inline-static">"::new("</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 75px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[1] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[1].clone() />
                                <span class="inline-static">"::new(String::from(\"Systems: \")));"</span>
                            </div>

                            <div class="code-static pre-code" style="border:none; padding-bottom:0;">
                                "    let mut handles = vec![];\n\n    for subsystem in [\"Thrusters\", \"Shields\", \"Life Support\"] {\n        let core_clone = Arc::clone(&core);\n"
                            </div>

                            <div class="inline-editor-row spacer-row">
                                <span class="inline-static">"        handles.push(thread::"</span>
                                <input type="text" class="code-input" spellcheck="false" style="width: 75px"
                                    on:input=move |ev| {
                                        let mut curr = inputs.get();
                                        curr[2] = event_target_value(&ev);
                                        set_inputs.set(curr);
                                    }
                                    value=move || inputs.get()[2].clone() />
                                <span class="inline-static">"(move || {"</span>
                            </div>

                            <div class="code-static post-code">
                                "            let mut data = core_clone.lock().unwrap();\n            data.push_str(subsystem);\n            data.push_str(\" [OK] \");\n        }));\n    }\n\n    for h in handles { h.join().unwrap(); }\n    println!(\"Warp Traversal ready: {}\", *core.lock().unwrap());\n}"
                            </div>
                        }.into_any(),

                        _ => view! {
                            <div class="code-static pre-code">"// Loading data..."</div>
                        }.into_any()
                    }}
                </div>
            </div>
            
            <div class="terminal-pane">
                <div class="pane-header terminal-bg" style="justify-content: space-between; align-items: center; padding-right: 1.5rem;">
                    <span class="tab">"Terminal Output"</span>
                    {move || if is_completed.get() {
                        view! {
                            <button class="btn proceed-btn" style="margin-top: 0; padding: 0.35rem 0.75rem; font-size: 0.8rem;" on:click=move |_| trigger_next.update(|n| *n += 1)>
                                "Proceed ⏭"
                            </button>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </div>
                <div class="terminal-output" id="terminal-out">
                    {move || terminal_output.get().into_iter().map(|(log_type, msg)| {
                        view! {
                            <div class=format!("terminal-line {}", log_type)>{msg}</div>
                        }
                    }).collect_view()}
                    <span class="terminal-cursor"></span>
                </div>
            </div>
        </section>
    }
}
