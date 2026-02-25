use leptos::prelude::*;
use rustacean_chronicles::components::story::StoryPanel;
use rustacean_chronicles::components::workspace::Workspace;

#[component]
fn App() -> impl IntoView {
    // Top-Level State Machine
    let (current_episode, set_current_episode) = signal(1);
    let (current_module, set_current_module) = signal(1);

    // Generic Inputs Array (supports up to 5 inputs for any given module challenge)
    let (inputs, set_inputs) = signal(vec!["".to_string(); 5]);
    
    let (terminal_output, set_terminal_output) = signal(vec![
        ("system".to_string(), "Booting execution environment...".to_string()),
        ("system".to_string(), "Ready for input.".to_string()),
    ]);
    
    let (is_completed, set_is_completed) = signal(false);

    // Derived Story State depending on Current Episode/Module
    let episode_title = move || match current_episode.get() {
        1 => "Episode 1: System Reboot".to_string(),
        2 => "Episode 2: Navigation Hazards".to_string(),
        3 => "Episode 3: Power Grid Allocations".to_string(),
        4 => "Episode 4: Cataloging Alien Artifacts".to_string(),
        5 => "Episode 5: Hull Breach!".to_string(),
        6 => "Episode 6: Manifest Recovery".to_string(),
        7 => "Episode 7: Universal Translations".to_string(),
        8 => "Episode 8: Warp Core Synchronization".to_string(),
        9 => "Episode 9: Conformity Gate (Finale)".to_string(),
        _ => "Unknown Episode".to_string(),
    };

    let module_title = move || match (current_episode.get(), current_module.get()) {
        (1, 1) => "Module 1: Boot Sequence".to_string(),
        (1, 2) => "Module 2: Mutability".to_string(),
        (2, 1) => "Module 1: Approaching Asteroid Field".to_string(),
        (2, 2) => "Module 2: Heat Dispersion".to_string(),
        (2, 3) => "Finale Challenge: Evasive Patterns".to_string(),
        (3, 1) => "Module 1: Moving Power".to_string(),
        (3, 2) => "Module 2: Borrowing Sensor Logs".to_string(),
        (3, 3) => "Finale Challenge: Data Races".to_string(),
        (4, 1) => "Module 1: Defining Structs".to_string(),
        (4, 2) => "Module 2: Struct Methods (`impl`)".to_string(),
        (4, 3) => "Finale Challenge: Enums & Match".to_string(),
        (5, 1) => "Module 1: The Null Problem".to_string(),
        (5, 2) => "Module 2: Expected Failures".to_string(),
        (5, 3) => "Finale Challenge: Error Propagation".to_string(),
        (6, 1) => "Module 1: Recovering Vector Arrays".to_string(),
        (6, 2) => "Module 2: String Manipulations".to_string(),
        (6, 3) => "Finale Challenge: Hash Maps".to_string(),
        (7, 1) => "Module 1: Defining Traits".to_string(),
        (7, 2) => "Module 2: Generics & Trait Bounds".to_string(),
        (7, 3) => "Finale Challenge: Deriving Traits".to_string(),
        (8, 1) => "Module 1: Spawning Threads".to_string(),
        (8, 2) => "Module 2: Message Passing".to_string(),
        (8, 3) => "Finale Challenge: Shared State (`Arc` & `Mutex`)".to_string(),
        (9, 1) => "Module 1: Parsing the Gate Keys".to_string(),
        (9, 2) => "Module 2: Verifying the Manifest".to_string(),
        (9, 3) => "Finale Challenge: Warp Traversal".to_string(),
        _ => "Unknown Module".to_string(),
    };

    let story_text = move || match (current_episode.get(), current_module.get()) {
        (1, 1) => vec![
            "System Log // Stardate 3491.2".to_string(),
            "The ship's main terminal is barely responsive. The primary power relay is offline.".to_string(),
        ],
        (1, 2) => vec![
            "Power relay engaged! But there's another problem.".to_string(),
            "The main terminal woke up, but the life support variables are corrupted. If we don't fix the oxygen levels soon, the crew won't survive the next cycle.".to_string(),
        ],
        (2, 1) => vec![
            "Warning! Incoming asteroid field detected.".to_string(),
            "The auto-navigation system is offline. We need to manually write the collision avoidance system.".to_string(),
        ],
        (2, 2) => vec![
            "Asteroid cleared, but the evasion maneuvers stressed the engines! The core temperature is rising critically.".to_string(),
            "We need to activate the heat sinks in a loop until the temperature drops to safe levels.".to_string(),
        ],
        (2, 3) => vec![
            "A massive asteroid is headed straight for us!".to_string(),
            "We must fire the lateral thrusters in a specific sequence: check thrusters 1 through 5, but only fire them if their ID is an even number.".to_string(),
        ],
        (3, 1) => vec![
            "Whew, that was close. But the evasive maneuvers depleted the main grid.".to_string(),
            "We need to transfer power from the auxiliary battery to the main grid. However, memory (and power) in Rust is strictly controlled!".to_string(),
        ],
        (3, 2) => vec![
            "The ship's navigation logs contain the coordinates to the nearest starbase.".to_string(),
            "We need to pass the logs to the inspection routine, but we CANNOT give up ownership, because we still need the coordinates to actually jump!".to_string(),
        ],
        (3, 3) => vec![
            "The main reactor is vibrating violently. The recalibration sequence needs to modify the reactor's state directly.".to_string(),
            "We must establish a secure, exclusive connection to the reactor to ensure no other subsystem interferes during recalibration.".to_string(),
        ],
        (4, 1) => vec![
            "The scanners picked up some loose debris around the anomaly. We've pulled it into the cargo bay for cataloging.".to_string(),
            "We need a structured data format to keep track of the artifact IDs and names without mixing up variables.".to_string(),
        ],
        (4, 2) => vec![
            "Our catalog data is structured, but we need to add diagnostic behavior to the artifacts directly out in the field.".to_string(),
        ],
        (4, 3) => vec![
            "Warning: The latest artifact emits a strange frequency. We need to classify it before storing it!".to_string(),
            "We must define strict categories (Safe, Hazardous, Edible) and process exactly how to handle each variant.".to_string(),
        ],
        (5, 1) => vec![
            "A micro-meteorite just pierced the hull! We need to locate the backup oxygen reserves in the database.".to_string(),
            "However, the database might return a coordinate, or it might return nothing. We need to handle the absence of a value safely.".to_string(),
        ],
        (5, 2) => vec![
            "We are analyzing the structural integrity of the hull where the breach occurred. The sensors might fail to read the composition entirely.".to_string(),
            "We need to extract the reading, and if it fails, throw an intentional system panic so the automated bulkheads seal immediately!".to_string(),
        ],
        (5, 3) => vec![
            "There's too much data falling through the systems right now. Sometimes, when a subsystem fails, we don't want to handle it immediately.".to_string(),
            "We just want to abort the current function and pass the failure upstream to the master control loop.".to_string()
        ],
        (6, 1) => vec![
            "The bulkheads have sealed, but the blast damaged the mainframes. The digital crew manifest has been scattered into an unordered pile of ID badges.".to_string(),
            "We need a flexible, growable array to sequentially pack the badges back together.".to_string(),
        ],
        (6, 2) => vec![
            "We've extracted the names from the registry, but the encoding is corrupted with random whitespace and trailing characters!".to_string(),
            "We need to safely format and combine strings in memory to rebuild the full names.".to_string(),
        ],
        (6, 3) => vec![
            "Now that we have names and assignments, we need to query them at light-speed during crises.".to_string(),
            "We need a collection type that immediately maps a specific 'Key' (like a zone) to a 'Value' (like a crew member) without scanning the whole list.".to_string(),
        ],
        (7, 1) => vec![
            "To override the lockdown, we need to ensure that different sensor subsystems share common behavior, like the ability to be reset.".to_string(),
            "We need to define a shared interface that all components must implement.".to_string(),
        ],
        (7, 2) => vec![
            "Now we want a generic function that can reset *any* system, regardless of its specific type.".to_string(),
            "However, we must guarantee to the rust compiler that the generic type actually implements our Reset interface!".to_string(),
        ],
        (7, 3) => vec![
            "We've intercepted a corrupted alien communication payload. We need a quick way to print out its raw struct values for debugging.".to_string(),
            "Writing a custom formatter is tedious. Let's automatically derive standard formatting traits instead.".to_string(),
        ],
        (8, 1) => vec![
            "We are preparing to jump! We need to spin up the port and starboard warp coils simultaneously.".to_string(),
            "Running them sequentially will cause a catastrophic imbalance. We must use parallel computing.".to_string(),
        ],
        (8, 2) => vec![
            "The warp coils are spinning, but the diagnostic threads need to report their status back to the main control loop safely.".to_string(),
            "We need to establish a secure communication channel between threads to prevent data races.".to_string(),
        ],
        (8, 3) => vec![
            "The AI core needs to be updated by multiple repair drones concurrently. We must share its memory state directly.".to_string(),
            "We need sophisticated smart pointers to ensure mutual exclusion across the entire repair swarm.".to_string()
        ],
        (9, 1) => vec![
            "We've arrived at the Conformity Gate. The gate requires an exact sequence of encrypted keys to open, but the incoming signals are extremely noisy!".to_string(),
            "Synthesize your knowledge of Error Handling (`Result`) and `match` to parse the pure integer keys safely.".to_string()
        ],
        (9, 2) => vec![
            "The gate is verifying our fleet manifest. It requires a specific, filtered digital signature of our crew collection.".to_string(),
            "Use Iterators and Closures to process and filter our `Vec` collection of crew boundaries in a single, memory-safe chain.".to_string()
        ],
        (9, 3) => vec![
            "The gate is opening! The gravitational sheer is enormous! We must synchronize the thrusters, shields, and life support systems simultaneously.".to_string(),
            "Synthesize your knowledge of Threads, `Arc`, and `Mutex` to safely coordinate the shared system status across multiple parallel closures!".to_string()
        ],
        _ => vec!["Load error.".to_string()],
    };

    let concept_title = move || match (current_episode.get(), current_module.get()) {
        (1, 1) => "Core Concept: Let Bindings".to_string(),
        (1, 2) => "Core Concept: Mutability".to_string(),
        (2, 1) => "Core Concept: Control Flow (if/else)".to_string(),
        (2, 2) => "Core Concept: Loops (`while`)".to_string(),
        (2, 3) => "Core Concept: Combining Loops and Branching".to_string(),
        (3, 1) => "Core Concept: Ownership & Move Semantics".to_string(),
        (3, 2) => "Core Concept: Borrowing (`&`)".to_string(),
        (3, 3) => "Core Concept: Mutable References (`&mut`)".to_string(),
        (4, 1) => "Core Concept: Structs".to_string(),
        (4, 2) => "Core Concept: The `impl` block".to_string(),
        (4, 3) => "Core Concept: Enums & Pattern Matching".to_string(),
        (5, 1) => "Core Concept: The `Option<T>` Enum".to_string(),
        (5, 2) => "Core Concept: The `Result<T, E>` Enum".to_string(),
        (5, 3) => "Core Concept: The `?` Operator".to_string(),
        (6, 1) => "Core Concept: Vectors (`Vec<T>`)".to_string(),
        (6, 2) => "Core Concept: String Modification".to_string(),
        (6, 3) => "Core Concept: HashMaps".to_string(),
        (7, 1) => "Core Concept: Traits".to_string(),
        (7, 2) => "Core Concept: Generics `<T>` and Trait Bounds".to_string(),
        (7, 3) => "Core Concept: Derivation Macros".to_string(),
        (8, 1) => "Core Concept: Multithreading".to_string(),
        (8, 2) => "Core Concept: Channels (`mpsc`)".to_string(),
        (8, 3) => "Core Concept: Atomic Reference Counting and Mutexes".to_string(),
        (9, 1) => "Synthesis: Error Handling".to_string(),
        (9, 2) => "Synthesis: Iterators & Collections".to_string(),
        (9, 3) => "Synthesis: Concurrency".to_string(),
        _ => "".to_string(),
    };

    let concept_text = move || match (current_episode.get(), current_module.get()) {
        (1, 1) => vec!["In Rust, you bind a value to a variable using the `let` keyword.".to_string()],
        (1, 2) => vec!["In Rust, variables are immutable by default. To make them changeable after they are defined, use the `let mut` syntax.".to_string()],
        (2, 1) => vec!["Rust uses `if` and `else` to branch logic based on boolean conditions. The condition must be a strictly typed boolean (no implicit truthiness).".to_string()],
        (2, 2) => vec!["Rust has three kinds of loops: `loop`, `while`, and `for`. A `while` loop evaluates a condition and runs its block as long as the condition is true.".to_string()],
        (2, 3) => vec!["We can place `if`/`else` statements inside of loops. A `for` loop is perfect for iterating over a simple range in Rust, using the `for i in 1..=5` syntax (inclusive). Remember that `%` is the modulo operator (remainder).".to_string()],
        (3, 1) => vec!["In Rust, every value has a single \"owner\". When you pass a variable (like a `String`) into a function, ownership is *moved* to that function. The original variable becomes invalid and can no longer be used. This prevents double-free bugs!".to_string()],
        (3, 2) => vec!["If you want a function to read data without taking ownership, you can *borrow* it by passing a reference. You add the `&` symbol before the variable to create an immutable reference.".to_string()],
        (3, 3) => vec!["To let a function modify a borrowed value, use a mutable reference (`&mut`). The catch? Rust's borrow checker enforces that you can only have *one* mutable reference to a piece of data at a time to guarantee thread safety and prevent data races!".to_string()],
        (4, 1) => vec!["A `struct` is a custom data type that lets you name and package together multiple related values into a single cohesive group.".to_string()],
        (4, 2) => vec!["You can attach behavior (methods) to structs using an `impl` block. The first parameter of a method is usually `&self`, which borrows the struct instance so the method can read its fields.".to_string()],
        (4, 3) => vec!["An `enum` defines a type by enumerating its possible variants. The `match` control flow operator allows you to safely compare a value against the enum variants and execute code. Rust forces you to handle *every single variant*, making unhandled cases grammatically impossible!".to_string()],
        (5, 1) => vec!["Rust doesn't have `null` or `undefined`. Instead, it uses the `Option<T>` enum, which has two variants: `Some(T)` (containing a value) or `None` (representing the absence of a value).".to_string()],
        (5, 2) => vec!["When an operation can fail, Rust uses `Result<T, E>`. It returns `Ok(T)` on success and `Err(E)` on failure. You can handle this gracefully with `match`, or force the program to completely panic and crash using `.expect(\"message\")` if the value is an `Err`.".to_string()],
        (5, 3) => vec!["The `?` operator evaluates a `Result` or `Option`. If it's `Ok`/`Some`, it unwraps the inner value. If it's `Err`/`None`, it immediately returns *early* from the function, bubbling the error up!".to_string()],
        (6, 1) => vec!["A `Vec<T>` (Vector) is a heap-allocated, growable array that stores multiple values of the *same* type sequentially in memory.".to_string()],
        (6, 2) => vec!["A `String` in Rust is actually just a wrapper around a `Vec<u8>` (bytes)! This means it's growable and mutable. Converting between `String` and string slices (`&str`) is a fundamental skill.".to_string()],
        (6, 3) => vec!["A `HashMap<K, V>` stores data in pairs (a 'Key' and a 'Value'). It's perfect for incredibly fast data lookups by name instead of by numerical index.".to_string()],
        (7, 1) => vec!["A `trait` defines shared behavior in Rust, similar to interfaces in other languages. You define the method signatures in the trait block, then use the `impl Trait for Type` syntax to attach them to specific structs.".to_string()],
        (7, 2) => vec!["Generics `<T>` allow you to write reusable code that works with any data type. Trait bounds (`T: Trait`) safely restrict those generic parameters, ensuring that the type passed in *must* implement certain traits to compile!".to_string()],
        (7, 3) => vec!["Some standard, highly-used traits (like `Debug`, `Clone`, and `PartialEq`) can be automatically implemented for your structs using the `#[derive(Trait)]` attribute macro. Once derived, `Debug` lets you print the struct nicely using `{:?}` !".to_string()],
        (8, 1) => vec!["`std::thread::spawn` creates a new thread to run code concurrently. You must call `.join()` on its returned handle to ensure the main thread waits for the parallel thread to finish execution.".to_string()],
        (8, 2) => vec!["`mpsc` (Multiple Producer, Single Consumer) channels allow threads to communicate. \"Do not communicate by sharing memory; instead, share memory by communicating.\"".to_string()],
        (8, 3) => vec!["A `Mutex` (Mutual Exclusion) enforces that only one thread accesses data at a time. An `Arc` (Atomic Reference Counted) smart pointer safely shares the ownership of that `Mutex` across multiple threads.".to_string()],
        (9, 1) => vec!["The `parse::<T>()` method parses a string into another type. It returns a `Result`, forcing you to gracefully handle cases where the string isn't actually a number!".to_string()],
        (9, 2) => vec!["Iterators (`.iter()`, `.into_iter()`) allow you to chain methods like `.filter()` and `.map()` to process collections functionally instead of using manual loops.".to_string()],
        (9, 3) => vec!["When spinning up multiple threads `thread::spawn(...)` inside a loop, an `Arc::clone(...)` must be used for each thread so they all share valid pointers to the internal mutated `Mutex`.".to_string()],
        _ => vec![],
    };

    let challenge_text = move || match (current_episode.get(), current_module.get()) {
        (1, 1) => vec![
            "Initialize the `power_level` variable to 100 to restart the system.".to_string(),
            "Type `100` into the missing slot.".to_string(),
        ],
        (1, 2) => vec![
            "The ship's automated routine sets three system variables, but later attempts to override two of them.".to_string(),
            "Review the terminal errors. Insert the `mut` keyword *only* where appropriate to allow the right variables to be updated, but leave the constant ones alone!".to_string(),
        ],
        (2, 1) => vec![
            "Read the incoming sensor data. If the `asteroid_distance` is less than 500, we must set the maneuver to \"Evade\". Otherwise, set it to \"Maintain Course\".".to_string(),
            "Fill in the missing conditions and assignments!".to_string(),
        ],
        (2, 2) => vec![
            "Write a `while` loop that continues as long as `core_temp > 50`. Inside the loop, subtract 10 from `core_temp`.".to_string(),
        ],
        (2, 3) => vec![
            "Write a `for` loop that iterates over the inclusive range `1..=5` with the variable `thruster_id`. Inside it, use an `if` statement to check if the current `thruster_id` is even (`% 2 == 0`). If it is, assign `fired_count += 1`.".to_string(),
        ],
        (3, 1) => vec![
            "The `aux_battery` was passed to `transfer_power`. The compiler will throw an error on the `println!` line because `aux_battery` no longer owns the data.".to_string(),
            "Comment out the line causing the error by typing `//` in the input box to successfully compile the power routing sequence.".to_string()
        ],
        (3, 2) => vec![
            "Add the `&` operator to pass an immutable reference of `nav_logs` to `read_logs()`, and update the function signature to accept a `&String` so it doesn't take ownership.".to_string(),
        ],
        (3, 3) => vec![
            "Pass a mutable reference (`&mut`) of `reactor_state` to the `recalibrate` function, and accept it as `&mut String` in the signature.".to_string(),
            "Inside the function, mutate the string by calling the `push_str()` method to append \" - Calibrated\".".to_string()
        ],
        (4, 1) => vec![
            "Define a `struct` named `Artifact`. Give it two fields: `id` (as a `u32` integer) and `name` (as a `String`).".to_string()
        ],
        (4, 2) => vec![
            "Declare an `impl` block for the `Artifact` struct.".to_string(),
            "Inside the block, pass `&self` to the `analyze` method so it can borrow the artifact and print its name!".to_string()
        ],
        (4, 3) => vec![
            "Use the `match` keyword to pattern match on the `status` enum.".to_string(),
            "Add the missing `Status::Edible` arm to handle the remaining variant so the code compiles safely!".to_string()
        ],
        (5, 1) => vec![
            "Use `match` to unwrap the `Option` returned by the database.".to_string(),
            "If it has a value, pattern match it to `Some` and extract the `location` variable.".to_string(),
            "If it doesn't, handle the `None` scenario.".to_string()
        ],
        (5, 2) => vec![
            "Call `.expect()` on the `Result` returned by `analyze_breach()`.".to_string(),
            "Pass it a custom error message detailing why the system must panic! (e.g. \"Sensors failed!\")".to_string()
        ],
        (5, 3) => vec![
            "Add the `?` operator to the end of the `read_sensors()` call.".to_string(),
            "This will extract the `String` data if successful, or immediately abort the `process_data` function and return the error up to `main`!".to_string()
        ],
        (6, 1) => vec![
            "Initialize `manifest` as a `Vec<u32>` generic instance using `.new()`.".to_string(),
            "Use the `.push()` method to append the second badge (`901`) to the vector.".to_string()
        ],
        (6, 2) => vec![
            "Use the `.trim()` method on `first_name` to clean off the excess whitespace.".to_string(),
            "Use the `.push_str()` method twice on `full_name` to append a space `\" \"` and the `last_name` suffix.".to_string()
        ],
        (6, 3) => vec![
            "Create a new generic `HashMap` instance using `::new()`.".to_string(),
            "Use `.insert()` to bind the Key `\"Engineering\"` to the Value `\"Liara\"`.".to_string(),
            "Use `.get()` with the `\"Engineering\"` key to query the dictionary safely.".to_string()
        ],
        (7, 1) => vec![
            "Use the `impl` keyword to declare an implementation block.".to_string(),
            "Complete the block by targeting it `for` the `Thruster` struct to attach the trait.".to_string()
        ],
        (7, 2) => vec![
            "Declare a generic parameter `<T>` for the `reboot_system` function.".to_string(),
            "Add a Trait Bound to `T` by typing out the required `Reset` trait after a colon.".to_string()
        ],
        (7, 3) => vec![
            "Add the missing `Debug` trait name inside the upper `#[derive()]` attribute.".to_string(),
            "Use the special `?` format specifier inside the curly braces to print out the struct as a debug object representation.".to_string()
        ],
        (8, 1) => vec![
            "Use `thread::spawn` to initialize the sub-routine closure.".to_string(),
            "Call `.join()` on the `handle` to block the main thread from exiting prematurely.".to_string()
        ],
        (8, 2) => vec![
            "Call `mpsc::channel()` to instantiate the transmitter and receiver tuple.".to_string(),
            "Use `tx.send()` to emit the string msg, and `rx.recv()` to capture it on the main thread.".to_string()
        ],
        (8, 3) => vec![
            "Wrap the underlying value with an `Arc::new()` wrapping a `Mutex::new()` instance.".to_string(),
            "Inside the thread closure, call `status_clone.lock()` to acquire safe mutable access to the pointer.".to_string()
        ],
        (9, 1) => vec![
            "Return an `Err` containing the string `\"Signal corrupted\"` if the guard clause trips.".to_string(),
            "Call `.parse` on the `signal` reference to evaluate the string memory.".to_string(),
            "Wrap the valid numerical evaluation in an `Ok` enum wrapper.".to_string()
        ],
        (9, 2) => vec![
            "Convert the `Vec` into an iterator using `.into_iter()`.".to_string(),
            "Use the `.filter()` method to extract elements according to the inner closure.".to_string(),
            "Combine the iterator results back into a new collection using the `.collect()` method.".to_string()
        ],
        (9, 3) => vec![
            "Initialize the `core` with an `Arc` pointer protecting a `Mutex` lock.".to_string(),
            "Pass the newly `clone`d context into each thread via the `spawn` block to synchronize the warp event!".to_string()
        ],
        _ => vec![],
    };

    let run_diagnostics = move || {
        // Run the validation engine directly with the unified inputs array
        let result = rustacean_chronicles::validation::engine::run_diagnostics(
            current_episode.get(), 
            current_module.get(), 
            inputs.get()
        );

        // Update the UI state
        set_is_completed.set(result.is_success);
        set_terminal_output.set(result.logs);
    };

    let next_module = move || {
        // Logic to move to the next module in the sequence
        match (current_episode.get(), current_module.get()) {
            (1, 1) => {
                set_current_module.set(2);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]); // Clear inputs for new module
                set_terminal_output.set(vec![
                    ("system".to_string(), "Loading next module...".to_string()),
                    ("system".to_string(), "Ready for input.".to_string()),
                ]);
            },
            (1, 2) => {
                // End of Episode 1
                set_current_episode.set(2);
                set_current_module.set(1);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]); // Clear inputs for new module
                set_terminal_output.set(vec![
                    ("system".to_string(), "Warping to next sector...".to_string()),
                    ("system".to_string(), "Episode 2: Navigation Hazards".to_string()),
                    ("system".to_string(), "Ready for input.".to_string()),
                ]);
            },
            (2, 1) => {
                set_current_module.set(2);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]);
                set_terminal_output.set(vec![
                    ("system".to_string(), "Loading next module...".to_string()),
                    ("system".to_string(), "Ready for input.".to_string()),
                ]);
            },
            (2, 2) => {
                set_current_module.set(3);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]);
                set_terminal_output.set(vec![
                    ("system".to_string(), "Loading next module...".to_string()),
                    ("system".to_string(), "Ready for input.".to_string()),
                ]);
            },
            (2, 3) => {
                // End of Episode 2
                set_current_episode.set(3);
                set_current_module.set(1);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]);
                set_terminal_output.set(vec![
                    ("system".to_string(), "Warping to next sector...".to_string()),
                    ("system".to_string(), "Episode 3: Power Grid Allocations".to_string()),
                    ("system".to_string(), "Ready for input.".to_string()),
                ]);
            },
            (3, 1) => {
                set_current_module.set(2);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]);
                set_terminal_output.set(vec![
                    ("system".to_string(), "Loading next module...".to_string()),
                    ("system".to_string(), "Ready for input.".to_string()),
                ]);
            },
            (3, 2) => {
                set_current_module.set(3);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]);
                set_terminal_output.set(vec![
                    ("system".to_string(), "Loading next module...".to_string()),
                    ("system".to_string(), "Ready for input.".to_string()),
                ]);
            },
            (3, 3) => {
                // End of Episode 3
                set_current_episode.set(4);
                set_current_module.set(1);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]);
                set_terminal_output.set(vec![
                    ("system".to_string(), "Warping to next sector...".to_string()),
                    ("system".to_string(), "Episode 4: Cataloging Alien Artifacts".to_string()),
                    ("system".to_string(), "Ready for input.".to_string()),
                ]);
            },
            (4, 1) => {
                set_current_module.set(2);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]);
                set_terminal_output.set(vec![
                    ("system".to_string(), "Loading next module...".to_string()),
                    ("system".to_string(), "Ready for input.".to_string()),
                ]);
            },
            (4, 2) => {
                set_current_module.set(3);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]);
                set_terminal_output.set(vec![
                    ("system".to_string(), "Loading next module...".to_string()),
                    ("system".to_string(), "Ready for input.".to_string()),
                ]);
            },
            (4, 3) => {
                // End of Episode 4
                set_current_episode.set(5);
                set_current_module.set(1);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]);
                set_terminal_output.set(vec![
                    ("system".to_string(), "Warping to next sector...".to_string()),
                    ("system".to_string(), "Episode 5: Hull Breach!".to_string()),
                    ("system".to_string(), "Ready for input.".to_string()),
                ]);
            },
            (5, 1) => {
                set_current_module.set(2);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]);
                set_terminal_output.set(vec![
                    ("system".to_string(), "Loading next module...".to_string()),
                    ("system".to_string(), "Ready for input.".to_string()),
                ]);
            },
            (5, 2) => {
                set_current_module.set(3);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]);
                set_terminal_output.set(vec![
                    ("system".to_string(), "Loading next module...".to_string()),
                    ("system".to_string(), "Ready for input.".to_string()),
                ]);
            },
            (5, 3) => {
                // End of Episode 5
                set_current_episode.set(6);
                set_current_module.set(1);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]);
                set_terminal_output.set(vec![
                    ("system".to_string(), "Warping to next sector...".to_string()),
                    ("system".to_string(), "Episode 6: Manifest Recovery".to_string()),
                    ("system".to_string(), "Ready for input.".to_string()),
                ]);
            },
            (6, 1) => {
                set_current_module.set(2);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]);
                set_terminal_output.set(vec![
                    ("system".to_string(), "Loading next module...".to_string()),
                    ("system".to_string(), "Ready for input.".to_string()),
                ]);
            },
            (6, 2) => {
                set_current_module.set(3);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]);
                set_terminal_output.set(vec![
                    ("system".to_string(), "Loading next module...".to_string()),
                    ("system".to_string(), "Ready for input.".to_string()),
                ]);
            },
            (6, 3) => {
                // End of Episode 6
                set_current_episode.set(7);
                set_current_module.set(1);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]);
                set_terminal_output.set(vec![
                    ("system".to_string(), "Warping to next sector...".to_string()),
                    ("system".to_string(), "Episode 7: Universal Translations".to_string()),
                    ("system".to_string(), "Ready for input.".to_string()),
                ]);
            },
            (7, 1) => {
                set_current_module.set(2);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]);
                set_terminal_output.set(vec![
                    ("system".to_string(), "Loading next module...".to_string()),
                    ("system".to_string(), "Ready for input.".to_string()),
                ]);
            },
            (7, 2) => {
                set_current_module.set(3);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]);
                set_terminal_output.set(vec![
                    ("system".to_string(), "Loading next module...".to_string()),
                    ("system".to_string(), "Ready for input.".to_string()),
                ]);
            },
            (7, 3) => {
                // End of Episode 7
                set_current_episode.set(8);
                set_current_module.set(1);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]);
                set_terminal_output.set(vec![
                    ("system".to_string(), "Warping to next sector...".to_string()),
                    ("system".to_string(), "Episode 8: Warp Core Synchronization".to_string()),
                    ("system".to_string(), "Ready for input.".to_string()),
                ]);
            },
            (8, 1) => {
                set_current_module.set(2);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]);
                set_terminal_output.set(vec![
                    ("system".to_string(), "Loading next module...".to_string()),
                    ("system".to_string(), "Ready for input.".to_string()),
                ]);
            },
            (8, 2) => {
                set_current_module.set(3);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]);
                set_terminal_output.set(vec![
                    ("system".to_string(), "Loading next module...".to_string()),
                    ("system".to_string(), "Ready for input.".to_string()),
                ]);
            },
            (8, 3) => {
                // End of Episode 8
                set_current_episode.set(9);
                set_current_module.set(1);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]);
                set_terminal_output.set(vec![
                    ("system".to_string(), "Warping to next sector...".to_string()),
                    ("system".to_string(), "Episode 9: Conformity Gate".to_string()),
                    ("system".to_string(), "Ready for input.".to_string()),
                ]);
            },
            (9, 1) => {
                set_current_module.set(2);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]);
                set_terminal_output.set(vec![
                    ("system".to_string(), "Loading next module...".to_string()),
                    ("system".to_string(), "Ready for input.".to_string()),
                ]);
            },
            (9, 2) => {
                set_current_module.set(3);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]);
                set_terminal_output.set(vec![
                    ("system".to_string(), "Loading next module...".to_string()),
                    ("system".to_string(), "Ready for input.".to_string()),
                ]);
            },
            (9, 3) => {
                // End of Episode 9
                set_current_episode.set(10);
                set_current_module.set(1);
                set_is_completed.set(false);
                set_inputs.set(vec!["".to_string(); 5]);
                set_terminal_output.set(vec![
                    ("system".to_string(), "Warping to final sector...".to_string()),
                    ("system".to_string(), "Epilogue: Journey Complete".to_string()),
                    ("system".to_string(), "Congratulations.".to_string()),
                ]);
            },
            _ => {
                let mut logs = terminal_output.get();
                logs.push(("system".to_string(), "End of Content Demo".to_string()));
                set_terminal_output.set(logs);
            }
        }
    };

    let (trigger_run, set_trigger_run) = signal(0);
    let (trigger_next, set_trigger_next) = signal(0);

    Effect::new(move |_| {
        if trigger_run.get() > 0 {
            untrack(|| run_diagnostics());
        }
    });

    Effect::new(move |_| {
        if trigger_next.get() > 0 {
            untrack(|| next_module());
        }
    });

    view! {
        <main class="app-layout">
            <StoryPanel 
                episode_title=Signal::derive(episode_title)
                module_title=Signal::derive(module_title)
                story_text=Signal::derive(story_text)
                concept_title=Signal::derive(concept_title)
                concept_text=Signal::derive(concept_text)
                challenge_text=Signal::derive(challenge_text)
                is_completed=is_completed
            />
            <Workspace 
                current_episode=current_episode
                current_module=current_module
                inputs=inputs
                set_inputs=set_inputs
                terminal_output=terminal_output
                is_completed=is_completed
                trigger_run=set_trigger_run
                trigger_next=set_trigger_next
            />
        </main>
    }
}

pub fn main() {
    // Add panic hook for better error messages in the browser console
    console_error_panic_hook::set_once();
    
    // Mount the Leptos app to the body of the index.html page
    leptos::mount::mount_to_body(App)
}
