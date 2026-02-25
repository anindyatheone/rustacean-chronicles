use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_success: bool,
    pub logs: Vec<(String, String)>
}

#[wasm_bindgen]
pub fn run_diagnostics_wasm(episode: u32, module: u32, code_inputs_js: JsValue) -> Result<JsValue, JsValue> {
    let code_inputs: Vec<String> = serde_wasm_bindgen::from_value(code_inputs_js)?;
    
    let result = match (episode, module) {
        (1, 1) => validate_ep1_mod1(code_inputs),
        (1, 2) => validate_ep1_mod2(code_inputs),
        (2, 1) => validate_ep2_mod1(code_inputs),
        (2, 2) => validate_ep2_mod2(code_inputs),
        (2, 3) => validate_ep2_mod3(code_inputs),
        (3, 1) => validate_ep3_mod1(code_inputs),
        (3, 2) => validate_ep3_mod2(code_inputs),
        (3, 3) => validate_ep3_mod3(code_inputs),
        (4, 1) => validate_ep4_mod1(code_inputs),
        (4, 2) => validate_ep4_mod2(code_inputs),
        (4, 3) => validate_ep4_mod3(code_inputs),
        (5, 1) => validate_ep5_mod1(code_inputs),
        (5, 2) => validate_ep5_mod2(code_inputs),
        (5, 3) => validate_ep5_mod3(code_inputs),
        (6, 1) => validate_ep6_mod1(code_inputs),
        (6, 2) => validate_ep6_mod2(code_inputs),
        (6, 3) => validate_ep6_mod3(code_inputs),
        (7, 1) => validate_ep7_mod1(code_inputs),
        (7, 2) => validate_ep7_mod2(code_inputs),
        (7, 3) => validate_ep7_mod3(code_inputs),
        (8, 1) => validate_ep8_mod1(code_inputs),
        (8, 2) => validate_ep8_mod2(code_inputs),
        (8, 3) => validate_ep8_mod3(code_inputs),
        (9, 1) => validate_ep9_mod1(code_inputs),
        (9, 2) => validate_ep9_mod2(code_inputs),
        (9, 3) => validate_ep9_mod3(code_inputs),
        _ => ValidationResult {
            is_success: false,
            logs: vec![("error".to_string(), format!("No validation logic found for Episode {}, Module {}", episode, module))]
        }
    };
    
    Ok(serde_wasm_bindgen::to_value(&result)?)
}

pub fn run_diagnostics(episode: u32, module: u32, code_inputs: Vec<String>) -> ValidationResult {
    match (episode, module) {
        (1, 1) => validate_ep1_mod1(code_inputs),
        (1, 2) => validate_ep1_mod2(code_inputs),
        (2, 1) => validate_ep2_mod1(code_inputs),
        (2, 2) => validate_ep2_mod2(code_inputs),
        (2, 3) => validate_ep2_mod3(code_inputs),
        (3, 1) => validate_ep3_mod1(code_inputs),
        (3, 2) => validate_ep3_mod2(code_inputs),
        (3, 3) => validate_ep3_mod3(code_inputs),
        (4, 1) => validate_ep4_mod1(code_inputs),
        (4, 2) => validate_ep4_mod2(code_inputs),
        (4, 3) => validate_ep4_mod3(code_inputs),
        (5, 1) => validate_ep5_mod1(code_inputs),
        (5, 2) => validate_ep5_mod2(code_inputs),
        (5, 3) => validate_ep5_mod3(code_inputs),
        (6, 1) => validate_ep6_mod1(code_inputs),
        (6, 2) => validate_ep6_mod2(code_inputs),
        (6, 3) => validate_ep6_mod3(code_inputs),
        (7, 1) => validate_ep7_mod1(code_inputs),
        (7, 2) => validate_ep7_mod2(code_inputs),
        (7, 3) => validate_ep7_mod3(code_inputs),
        (8, 1) => validate_ep8_mod1(code_inputs),
        (8, 2) => validate_ep8_mod2(code_inputs),
        (8, 3) => validate_ep8_mod3(code_inputs),
        (9, 1) => validate_ep9_mod1(code_inputs),
        (9, 2) => validate_ep9_mod2(code_inputs),
        (9, 3) => validate_ep9_mod3(code_inputs),
        _ => ValidationResult {
            is_success: false,
            logs: vec![("error".to_string(), format!("No validation logic found for Episode {}, Module {}", episode, module))]
        }
    }
}

fn validate_ep9_mod1(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    if code_inputs.len() < 3 {
        logs.push(("error".to_string(), "Missing input".to_string()));
        return ValidationResult { is_success: false, logs };
    }
    
    let is_err = code_inputs[0].trim() == "Err";
    let is_parse = code_inputs[1].trim() == "parse";
    let is_ok = code_inputs[2].trim() == "Ok";

    if is_err && is_parse && is_ok {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("system".to_string(), "Parsed Key: 1042 - locked in.".to_string()));
         logs.push(("success".to_string(), "Noise filtered, key identified!".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed! Error state successfully mapped.".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         if !is_err {
             logs.push(("error".to_string(), "error[E0308]: mismatched types\nhelp: Return the text wrapped in an `Err()` enum since it violates the parse sequence.".to_string()));
         }
         if !is_parse {
             logs.push(("error".to_string(), "error[E0599]: no method named found for type `&str`\nhelp: Use the `.parse()` method to extract the integer from the string.".to_string()));
         }
         if !is_ok {
             logs.push(("error".to_string(), "error[E0308]: mismatched types\nhelp: Ensure the pattern match handles the `Ok(num)` outcome of the Result enum.".to_string()));
         }
         ValidationResult { is_success: false, logs }
    }
}

fn validate_ep9_mod2(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    if code_inputs.len() < 3 {
        logs.push(("error".to_string(), "Missing input".to_string()));
        return ValidationResult { is_success: false, logs };
    }
    
    let is_iter = code_inputs[0].trim() == "into_iter" || code_inputs[0].trim() == "iter";
    let is_filter = code_inputs[1].trim() == "filter";
    let is_collect = code_inputs[2].trim() == "collect";

    if is_iter && is_filter && is_collect {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("system".to_string(), "Active crew verified: [\"Pilot\", \"Engineer\", \"Gunner\"]".to_string()));
         logs.push(("success".to_string(), "Data mutation pipeline verified!".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed! Cleaned vectors aligned.".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         if !is_iter {
             logs.push(("error".to_string(), "error[E0599]: no method named `filter` found for struct `Vec`\nhelp: You must convert the vector into an iterable source by calling `.into_iter()` first.".to_string()));
         }
         if !is_filter {
             logs.push(("error".to_string(), "error[E0599]: no method named found for struct `IntoIter`\nhelp: Call the `.filter()` iterator adapter to exclude invalid strings.".to_string()));
         }
         if !is_collect {
             logs.push(("error".to_string(), "error[E0308]: mismatched types expected `Vec<&str>`, found `Filter`\nhelp: Call `.collect()` to consume the iterator and materialize it back into a new Vector.".to_string()));
         }
         ValidationResult { is_success: false, logs }
    }
}

fn validate_ep9_mod3(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    if code_inputs.len() < 3 {
        logs.push(("error".to_string(), "Missing input".to_string()));
        return ValidationResult { is_success: false, logs };
    }
    
    let is_arc = code_inputs[0].trim() == "Arc";
    let is_mutex = code_inputs[1].trim() == "Mutex";
    let is_spawn = code_inputs[2].trim() == "spawn";

    if is_arc && is_mutex && is_spawn {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("system".to_string(), "Warp Traversal ready: Systems: Thrusters [OK] Shields [OK] Life Support [OK] ".to_string()));
         logs.push(("success".to_string(), "Multithreaded Arc locking successfully achieved!".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed! Commencing warp procedure...".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         if !is_arc {
             logs.push(("error".to_string(), "error[E0433]: failed to resolve: use of undeclared type\nhelp: The base primitive requires the atomic `Arc` smart pointer to be safely copied across threads.".to_string()));
         }
         if !is_mutex {
             logs.push(("error".to_string(), "error[E0433]: failed to resolve: use of undeclared type\nhelp: The string value requires the locking `Mutex` to block simultaneous write operations.".to_string()));
         }
         if !is_spawn {
             logs.push(("error".to_string(), "error[E0425]: cannot find function in module `thread`\nhelp: Parallelize the execution with `thread::spawn(...)`.".to_string()));
         }
         ValidationResult { is_success: false, logs }
    }
}

fn validate_ep8_mod1(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    if code_inputs.len() < 2 {
        logs.push(("error".to_string(), "Missing input".to_string()));
        return ValidationResult { is_success: false, logs };
    }
    
    let is_spawn = code_inputs[0].trim() == "spawn";
    let is_join = code_inputs[1].trim() == "join";

    if is_spawn && is_join {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("system".to_string(), "Port warp coil spinning up...".to_string()));
         logs.push(("system".to_string(), "Starboard warp coil spinning up...".to_string()));
         logs.push(("system".to_string(), "Both coils synchronized!".to_string()));
         logs.push(("success".to_string(), "Multi-core sequence locked!".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed! Safe parallel processing engaged.".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         if !is_spawn {
             logs.push(("error".to_string(), "error[E0425]: cannot find function in module `thread`\nhelp: Use `thread::spawn` to spin up a new parallel thread.".to_string()));
         }
         if !is_join {
             logs.push(("error".to_string(), "error[E0599]: no method named found for struct `JoinHandle`\nhelp: Use `.join()` to wait for the spawned thread to finish before the main thread exits.".to_string()));
         }
         ValidationResult { is_success: false, logs }
    }
}

fn validate_ep8_mod2(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    if code_inputs.len() < 3 {
        logs.push(("error".to_string(), "Missing input".to_string()));
        return ValidationResult { is_success: false, logs };
    }
    
    let is_channel = code_inputs[0].trim() == "channel";
    let is_send = code_inputs[1].trim() == "send";
    let is_recv = code_inputs[2].trim() == "recv";

    if is_channel && is_send && is_recv {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("system".to_string(), "Main Engine Room log: Diagnostic sequence nominal.".to_string()));
         logs.push(("success".to_string(), "Message received across the channel!".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed! Zero data races guaranteed.".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         if !is_channel {
             logs.push(("error".to_string(), "error[E0425]: cannot find function in module `mpsc`\nhelp: Use `mpsc::channel()` to initialize the Transmitter and Receiver pair.".to_string()));
         }
         if !is_send {
             logs.push(("error".to_string(), "error[E0599]: no method named found for struct `Sender`\nhelp: Use the `.send()` method to dispatch the string across the channel.".to_string()));
         }
         if !is_recv {
             logs.push(("error".to_string(), "error[E0599]: no method named found for struct `Receiver`\nhelp: Use the `.recv()` method to block and wait for a message from the channel.".to_string()));
         }
         ValidationResult { is_success: false, logs }
    }
}

fn validate_ep8_mod3(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    if code_inputs.len() < 3 {
        logs.push(("error".to_string(), "Missing input".to_string()));
        return ValidationResult { is_success: false, logs };
    }
    
    let is_arc = code_inputs[0].trim() == "Arc";
    let is_mutex = code_inputs[1].trim() == "Mutex";
    let is_lock = code_inputs[2].trim() == "lock";

    if is_arc && is_mutex && is_lock {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("system".to_string(), "Drone applied 100 patches.".to_string()));
         logs.push(("system".to_string(), "AI core at 100% restoration.".to_string()));
         logs.push(("success".to_string(), "Thread-safe mutability achieved!".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed! Fleet systems stabilized.".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         if !is_arc {
             logs.push(("error".to_string(), "error[E0433]: failed to resolve: use of undeclared type\nhelp: Wrap the Mutex in an atomic reference counter `Arc` so its pointer can be shared across multiple threads safely without moves.".to_string()));
         }
         if !is_mutex {
             logs.push(("error".to_string(), "error[E0433]: failed to resolve: use of undeclared type\nhelp: Use a `Mutex` to tightly control locking read/write access to the inner value.".to_string()));
         }
         if !is_lock {
             logs.push(("error".to_string(), "error[E0599]: no method named found for struct `Mutex`\nhelp: You must `.lock()` the mutex to acquire safe mutable access before changing its value.".to_string()));
         }
         ValidationResult { is_success: false, logs }
    }
}

fn validate_ep7_mod1(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    if code_inputs.len() < 2 {
        logs.push(("error".to_string(), "Missing input".to_string()));
        return ValidationResult { is_success: false, logs };
    }
    
    let is_impl = code_inputs[0].trim() == "impl";
    let is_for = code_inputs[1].trim() == "for";

    if is_impl && is_for {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("system".to_string(), "Thruster realigned and ready.".to_string()));
         logs.push(("success".to_string(), "Trait successfully implemented!".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed! Systems share behavior.".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         if !is_impl {
             logs.push(("error".to_string(), "error: expected `impl` keyword\nhelp: Use the `impl` keyword to declare an implementation block.".to_string()));
         }
         if !is_for {
             logs.push(("error".to_string(), "error: expected `for` keyword\nhelp: Use the `for` keyword to associate the Trait with the specified struct.".to_string()));
         }
         ValidationResult { is_success: false, logs }
    }
}

fn validate_ep7_mod2(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    if code_inputs.len() < 2 {
        logs.push(("error".to_string(), "Missing input".to_string()));
        return ValidationResult { is_success: false, logs };
    }
    
    let generic_t = code_inputs[0].trim();
    let trait_bound = code_inputs[1].trim();

    let is_t = generic_t == "T";
    let is_reset = trait_bound == "Reset";

    if is_t && is_reset {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("system".to_string(), "Sensor green.".to_string()));
         logs.push(("success".to_string(), "Function strongly typed via boundaries!".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed! Safety guaranteed at compile time.".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         if !is_t {
             logs.push(("error".to_string(), "error[E0412]: cannot find type in this scope\nhelp: Define the generic type parameter as `T` inside the angle brackets.".to_string()));
         }
         if !is_reset {
             logs.push(("error".to_string(), "error[E0405]: cannot find trait in this scope\nhelp: Bound the generic `T` to the `Reset` trait.".to_string()));
         }
         ValidationResult { is_success: false, logs }
    }
}

fn validate_ep7_mod3(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    if code_inputs.len() < 2 {
        logs.push(("error".to_string(), "Missing input".to_string()));
        return ValidationResult { is_success: false, logs };
    }
    
    let derive_macro = code_inputs[0].trim();
    let debug_formatter = code_inputs[1].trim();

    let is_debug = derive_macro == "Debug";
    let is_question = debug_formatter == "?";

    if is_debug && is_question {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("system".to_string(), "Intercepted: Payload { encryption_key: 1024, data: \"S.O.S\" }".to_string()));
         logs.push(("success".to_string(), "Struct successfully debugged!".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed! Communication translated.".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         if !is_debug {
             logs.push(("error".to_string(), "error[E0433]: failed to resolve: use of undeclared type\nhelp: Pass `Debug` into the macro to automatically derive a formatter.".to_string()));
         }
         if !is_question {
             logs.push(("error".to_string(), "error: expected formatting macro specifier `?`\nhelp: The `Debug` format string uses `{ :? }` to invoke the behavior.".to_string()));
         }
         ValidationResult { is_success: false, logs }
    }
}

fn validate_ep6_mod1(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    if code_inputs.len() < 3 {
        logs.push(("error".to_string(), "Missing input".to_string()));
        return ValidationResult { is_success: false, logs };
    }
    
    let type_def = code_inputs[0].trim().replace(" ", "");
    let ctor = code_inputs[1].trim();
    let method = code_inputs[2].trim();

    let is_type = type_def == "Vec<u32>" || type_def == "Vec<i32>";
    let is_new = ctor == "new";
    let is_push = method == "push";

    if is_type && is_new && is_push {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("system".to_string(), "Recovered 2 badges.".to_string()));
         logs.push(("success".to_string(), "Vector populated correctly!".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed! Manifest partially recovered.".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         if !is_type {
             logs.push(("error".to_string(), "error[E0412]: cannot find type in this scope\nhelp: Define the generic type as `Vec<u32>`.".to_string()));
         }
         if !is_new {
             logs.push(("error".to_string(), "error[E0599]: no function or associated item named found for struct `Vec`\nhelp: Initialize a new vector using `Vec::new()`.".to_string()));
         }
         if !is_push {
             logs.push(("error".to_string(), "error[E0599]: no method named found for struct `Vec`\nhelp: Use the `.push()` method to append elements to a vector.".to_string()));
         }
         ValidationResult { is_success: false, logs }
    }
}

fn validate_ep6_mod2(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    if code_inputs.len() < 3 {
        logs.push(("error".to_string(), "Missing input".to_string()));
        return ValidationResult { is_success: false, logs };
    }
    
    let trim_method = code_inputs[0].trim();
    let push_space = code_inputs[1].trim();
    let push_last = code_inputs[2].trim();

    let is_trim = trim_method == "trim";
    let is_push_str = push_space == "push_str" && push_last == "push_str";

    if is_trim && is_push_str {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("system".to_string(), "Restored format: Liara T'Soni".to_string()));
         logs.push(("success".to_string(), "String successfully built and trimmed.".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed! Registry entries look clean.".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         if !is_trim {
             logs.push(("error".to_string(), "error[E0599]: no method found for struct `String`\nhelp: Use `.trim()` to remove leading and trailing whitespace.".to_string()));
         }
         if !is_push_str {
             logs.push(("error".to_string(), "error[E0599]: no method found for struct `String`\nhelp: Use `.push_str()` when appending string slices (`&str`).".to_string()));
         }
         ValidationResult { is_success: false, logs }
    }
}

fn validate_ep6_mod3(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    if code_inputs.len() < 3 {
        logs.push(("error".to_string(), "Missing input".to_string()));
        return ValidationResult { is_success: false, logs };
    }
    
    let is_hashmap = code_inputs[0].trim() == "HashMap";
    let is_insert = code_inputs[1].trim() == "insert";
    let is_get = code_inputs[2].trim() == "get";

    if is_hashmap && is_insert && is_get {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("system".to_string(), "Engineering Officer: Liara".to_string()));
         logs.push(("success".to_string(), "Query executed in O(1) time.".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed! Manifest search mapped.".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         if !is_hashmap {
             logs.push(("error".to_string(), "error[E0433]: failed to resolve: use of undeclared type\nhelp: Specify `HashMap`.".to_string()));
         }
         if !is_insert {
             logs.push(("error".to_string(), "error[E0599]: no method found for struct `HashMap`\nhelp: Use the `.insert()` method to add Key-Value pairs.".to_string()));
         }
         if !is_get {
             logs.push(("error".to_string(), "error[E0599]: no method found for struct `HashMap`\nhelp: Use the `.get()` method to retrieve a value by its key.".to_string()));
         }
         ValidationResult { is_success: false, logs }
    }
}

fn validate_ep5_mod1(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    if code_inputs.len() < 3 {
        logs.push(("error".to_string(), "Missing input".to_string()));
        return ValidationResult { is_success: false, logs };
    }
    
    let is_match = code_inputs[0].trim() == "match";
    let is_some = code_inputs[1].trim() == "Some";
    let is_none = code_inputs[2].trim() == "None";

    if is_match && is_some && is_none {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("system".to_string(), "Found at Sector 7G".to_string()));
         logs.push(("success".to_string(), "Oxygen reserves successfully mapped!".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed! Data parsed safely.".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         if !is_match {
             logs.push(("error".to_string(), "error: expected `match` keyword\nhelp: Use a `match` expression to safely evaluate the Option variants.".to_string()));
         }
         if !is_some {
             logs.push(("error".to_string(), "error[E0004]: non-exhaustive patterns: `Some(_)` not covered\nnote: `match` arms must be exhaustive.\nhelp: add the missing `Some` variant.".to_string()));
         }
         if !is_none {
             logs.push(("error".to_string(), "error[E0004]: non-exhaustive patterns: `None` not covered\nnote: `match` arms must be exhaustive.\nhelp: add the missing `None` variant.".to_string()));
         }
         ValidationResult { is_success: false, logs }
    }
}

fn validate_ep5_mod2(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    if code_inputs.len() < 2 {
        logs.push(("error".to_string(), "Missing input".to_string()));
        return ValidationResult { is_success: false, logs };
    }
    
    let is_expect = code_inputs[0].trim() == "expect";
    let message = code_inputs[1].trim();

    if is_expect && !message.is_empty() {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("error".to_string(), format!("thread 'main' panicked at '{}': Sensor interference", message)));
         logs.push(("success".to_string(), "System halted! Bulkheads engaging!".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed! Emergency lockdown protocol successful.".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         if !is_expect {
             logs.push(("error".to_string(), "error[E0308]: mismatched types\nexpected `f32`, found `Result<f32, String>`\nhelp: Use the `.expect()` method to extract the value and trigger a panic if it fails.".to_string()));
         }
         if message.is_empty() {
             logs.push(("error".to_string(), "warning: missing panic message\nhelp: Provide a descriptive string message to the expect method.".to_string()));
         }
         ValidationResult { is_success: false, logs }
    }
}

fn validate_ep5_mod3(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    if code_inputs.is_empty() {
        logs.push(("error".to_string(), "Missing input".to_string()));
        return ValidationResult { is_success: false, logs };
    }
    
    let is_question = code_inputs[0].trim() == "?";

    if is_question {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("system".to_string(), "Mission aborted due to error: Hardware disconnected".to_string()));
         logs.push(("success".to_string(), "Error smoothly bubbled up to the main function!".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed! Control loop intercepted the failure.".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         logs.push(("error".to_string(), "error[E0308]: mismatched types\nexpected `String`, found `Result<String, String>`\nhelp: Use the `?` operator at the end of the expression to propagate the error early.".to_string()));
         ValidationResult { is_success: false, logs }
    }
}

fn validate_ep4_mod1(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    if code_inputs.len() < 3 {
        logs.push(("error".to_string(), "Missing input".to_string()));
        return ValidationResult { is_success: false, logs };
    }
    let struct_name = code_inputs[0].trim();
    let id_type = code_inputs[1].trim();
    let name_type = code_inputs[2].trim();

    if struct_name == "Artifact" && id_type == "u32" && name_type == "String" {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("system".to_string(), "Cataloged 42: Glowing Orb".to_string()));
         logs.push(("success".to_string(), "Struct mapped successfully.".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed! Artifact instantiated.".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         if struct_name != "Artifact" {
             logs.push(("error".to_string(), "error[E0412]: cannot find type `Artifact` in this scope\nhelp: Declare the struct with the exact name `Artifact`".to_string()));
         }
         if id_type != "u32" {
             logs.push(("error".to_string(), "error[E0308]: mismatched types\nhelp: The `id` field must be an unsigned 32-bit integer (`u32`).".to_string()));
         }
         if name_type != "String" {
             logs.push(("error".to_string(), "error[E0308]: mismatched types\nhelp: The `name` field must be a `String`.".to_string()));
         }
         ValidationResult { is_success: false, logs }
    }
}

fn validate_ep4_mod2(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    if code_inputs.len() < 2 {
        logs.push(("error".to_string(), "Missing input".to_string()));
        return ValidationResult { is_success: false, logs };
    }
    let keyword = code_inputs[0].trim();
    let self_arg = code_inputs[1].trim();
    
    if keyword == "impl" && (self_arg == "&self" || self_arg == "self") {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("system".to_string(), "Analyzing artifact: Glowing Orb".to_string()));
         logs.push(("success".to_string(), "Method successfully appended to struct.".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed!".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         if keyword != "impl" {
             logs.push(("error".to_string(), "error: expected `impl` keyword\nhelp: Start the block with the `impl` keyword to attach methods to a struct.".to_string()));
         }
         if self_arg != "&self" && self_arg != "self" {
             logs.push(("error".to_string(), "error: missing self parameter\nhelp: Methods must take `&self` (or `self`/`&mut self`) as their first parameter to access the struct's data.".to_string()));
         }
         ValidationResult { is_success: false, logs }
    }
}

fn validate_ep4_mod3(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    if code_inputs.len() < 2 {
        logs.push(("error".to_string(), "Missing input".to_string()));
        return ValidationResult { is_success: false, logs };
    }
    
    let match_kwd = code_inputs[0].trim();
    let variant = code_inputs[1].trim();

    let is_match = match_kwd == "match";
    let is_variant = variant == "Status::Edible";

    if is_match && is_variant {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("system".to_string(), "Sending to the galley.".to_string()));
         logs.push(("success".to_string(), "All control flows accounted for.".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed! Safe artifact parsing engaged.".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         if !is_match {
             logs.push(("error".to_string(), "error: expected `match` keyword\nhelp: Use a `match` expression to safely evaluate the enum variants.".to_string()));
         }
         if !is_variant {
             logs.push(("error".to_string(), "error[E0004]: non-exhaustive patterns: `Status::Edible` not covered\nnote: `match` arms must be exhaustive.\nhelp: add the missing `Status::Edible` variant to the match block.".to_string()));
         }
         ValidationResult { is_success: false, logs }
    }
}

fn validate_ep3_mod1(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    if code_inputs.is_empty() {
        logs.push(("error".to_string(), "Missing code input".to_string()));
        return ValidationResult { is_success: false, logs };
    }
    let input = code_inputs[0].trim();
    if input.starts_with("//") {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("system".to_string(), "Main grid received: 1500 kWh".to_string()));
         logs.push(("success".to_string(), "Power routing successful!".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed! Auxiliary battery drained safely.".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         logs.push(("error".to_string(), "error[E0382]: borrow of moved value: `aux_battery`\n  |     transfer_power(aux_battery); // value moved here\n  |     println!(\"Aux: {}\", aux_battery); // value borrowed here after move\nhelp: Comment out the `println!` line to fix the error.".to_string()));
         ValidationResult { is_success: false, logs }
    }
}

fn validate_ep3_mod2(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    if code_inputs.len() < 2 {
        logs.push(("error".to_string(), "Missing input".to_string()));
        return ValidationResult { is_success: false, logs };
    }
    let call_arg = code_inputs[0].trim();
    let sig_arg = code_inputs[1].trim();
    
    if call_arg == "&" && sig_arg == "&" {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("system".to_string(), "Reading: Starbase X-9 is 4 parsecs away.".to_string()));
         logs.push(("success".to_string(), "Logs successfully retained: Starbase X-9 is 4 parsecs away.".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed! Ownership intact.".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         if call_arg != "&" {
             logs.push(("error".to_string(), "error[E0308]: mismatched types\nhelp: Try borrowing the argument by passing a reference: `&nav_logs`".to_string()));
         }
         if sig_arg != "&" {
             logs.push(("error".to_string(), "error[E0382]: borrow of moved value: `nav_logs`\nhelp: The function signature must accept a reference `&String` to borrow instead of moving.".to_string()));
         }
         ValidationResult { is_success: false, logs }
    }
}

fn validate_ep3_mod3(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    if code_inputs.len() < 3 {
        logs.push(("error".to_string(), "Missing input".to_string()));
        return ValidationResult { is_success: false, logs };
    }
    
    let call_arg = code_inputs[0].trim().replace(" ", "");
    let sig_arg = code_inputs[1].trim().replace(" ", "");
    let method = code_inputs[2].trim();
    
    let is_mut_call = call_arg == "&mut";
    let is_mut_sig = sig_arg == "&mut";
    let is_push = method == "push_str";

    if is_mut_call && is_mut_sig && is_push {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("success".to_string(), "Reactor is now: Unstable - Calibrated".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed! Reactor recalibrated securely.".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         if !is_mut_call {
             logs.push(("error".to_string(), "error[E0308]: mismatched types\nhelp: You must pass a mutable reference: `&mut reactor_state`".to_string()));
         }
         if !is_mut_sig {
             logs.push(("error".to_string(), "error[E0596]: cannot borrow `*state` as mutable, as it is behind a `&` reference\nhelp: The function must accept `&mut String`".to_string()));
         }
         if !is_push {
             logs.push(("error".to_string(), "error[E0599]: no method named `push_str` found for mutable reference `&mut String`\nhelp: Use the `push_str` method to append characters.".to_string()));
         }
         ValidationResult { is_success: false, logs }
    }
}

fn validate_ep2_mod3(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    
    if code_inputs.len() < 3 {
        logs.push(("error".to_string(), "Missing system variable inputs.".to_string()));
        return ValidationResult { is_success: false, logs };
    }
    
    let loop_stmt = code_inputs[0].trim();
    let if_stmt = code_inputs[1].trim();
    let body_stmt = code_inputs[2].trim();
    
    let is_for_loop = loop_stmt.starts_with("for") && loop_stmt.contains("thruster_id") && loop_stmt.contains("1..=5");
    let is_even_if = if_stmt.starts_with("if") && if_stmt.contains("thruster_id") && if_stmt.contains("%") && if_stmt.contains("2") && if_stmt.contains("==") && if_stmt.contains("0");
    let is_counted = (body_stmt.contains("fired_count") && body_stmt.contains("+=") && body_stmt.contains("1")) ||
                     (body_stmt.contains("fired_count") && body_stmt.contains("=") && body_stmt.contains("fired_count") && body_stmt.contains("+") && body_stmt.contains("1"));

    if is_for_loop && is_even_if && is_counted {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("system".to_string(), "Thruster 2 fired.\nThruster 4 fired.".to_string()));
         logs.push(("success".to_string(), "Evasion sequence complete. Fired 2 thrusters.".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed! Asteroid dodged gracefully!".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         if !is_for_loop {
             logs.push(("error".to_string(), "error: expected `for` loop\nhelp: Use `for thruster_id in 1..=5` to iterate exactly 5 times.".to_string()));
         }
         
         if !is_even_if {
             logs.push(("error".to_string(), "error: condition not met\nhelp: Check if the thruster ID is even using `if thruster_id % 2 == 0`.".to_string()));
         }
         
         if !is_counted {
             logs.push(("error".to_string(), "warning: variable not updated\nhelp: Increment the `fired_count` by 1.".to_string()));
         }
         
         ValidationResult { is_success: false, logs }
    }
}

fn validate_ep2_mod2(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    
    if code_inputs.len() < 2 {
        logs.push(("error".to_string(), "Missing system variable inputs.".to_string()));
        return ValidationResult { is_success: false, logs };
    }
    
    let loop_cond = code_inputs[0].trim();
    let body_stmt = code_inputs[1].trim();
    
    let is_while = loop_cond.starts_with("while");
    let has_temp_cond = loop_cond.contains("core_temp") && loop_cond.contains(">") && loop_cond.contains("50");
    
    let is_subtracted = (body_stmt.contains("core_temp") && body_stmt.contains("-=") && body_stmt.contains("10")) || 
                        (body_stmt.contains("core_temp") && body_stmt.contains("=") && body_stmt.contains("core_temp") && body_stmt.contains("-") && body_stmt.contains("10"));

    if is_while && has_temp_cond && is_subtracted {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("system".to_string(), "Cooling... Temp: 110\nCooling... Temp: 100\nCooling... Temp: 90\nCooling... Temp: 80\nCooling... Temp: 70\nCooling... Temp: 60\nCooling... Temp: 50".to_string()));
         logs.push(("success".to_string(), "Core temperature stabilized.".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed! Heat sinks engaged.".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         if !is_while {
             logs.push(("error".to_string(), "error: expected `while` loop\nhelp: Use a `while` loop to repeatedly execute the block.".to_string()));
         } else if !has_temp_cond {
             logs.push(("error".to_string(), "error: condition not met\nhelp: The loop should run as long as `core_temp > 50`.".to_string()));
         }
         
         if !is_subtracted {
             logs.push(("error".to_string(), "warning: infinite loop detected\nhelp: You must decrease `core_temp` by 10 inside the loop using `-=`.".to_string()));
         }
         
         ValidationResult { is_success: false, logs }
    }
}

fn validate_ep2_mod1(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    
    if code_inputs.len() < 4 {
        logs.push(("error".to_string(), "Missing system variable inputs.".to_string()));
        return ValidationResult { is_success: false, logs };
    }
    
    let cond1 = code_inputs[0].trim();
    let val1_raw = code_inputs[1].trim();
    let cond2 = code_inputs[2].trim();
    let val2_raw = code_inputs[3].trim();
    
    let val1 = val1_raw.trim_matches('"');
    let val2 = val2_raw.trim_matches('"');
    
    let is_if = cond1.starts_with("if") && cond1.contains("asteroid_distance") && cond1.contains("<") && cond1.contains("500");
    let is_else = cond2 == "else";
    let is_evade_first = val1 == "Evade";
    let is_maintain_second = val2 == "Maintain Course";

    if is_if && is_else && is_evade_first && is_maintain_second {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("success".to_string(), "Maneuver protocol: Evade".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed! Asteroid collision avoided!".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         if !is_if {
             logs.push(("error".to_string(), "error[E0425]: cannot find value `asteroid_distance` in this scope\nhelp: A valid `if` condition using `asteroid_distance < 500` is required.".to_string()));
         }
         if !is_else {
             logs.push(("error".to_string(), "error: expected `else`, found something else\nhelp: Make sure you provide a fallback route using `else`.".to_string()));
         }
         if !is_evade_first {
             logs.push(("error".to_string(), "error[E0308]: mismatched types\nhelp: The first block must assign \"Evade\" to maneuver.".to_string()));
         }
         if !is_maintain_second {
              logs.push(("error".to_string(), "error[E0308]: mismatched types\nhelp: The fallback block must assign \"Maintain Course\" to maneuver.".to_string()));
         }
         ValidationResult { is_success: false, logs }
    }
}

fn validate_ep1_mod1(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    
    if code_inputs.is_empty() {
        logs.push(("error".to_string(), "Missing code input".to_string()));
         return ValidationResult { is_success: false, logs };
    }
    
    let code = code_inputs[0].replace(" ", "");
    
    if code == "100" {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("success".to_string(), "Power level set to 100%".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed! Power relay restored.".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         logs.push(("error".to_string(), "Mission Failed: The power level must be exactly 100.".to_string()));
         ValidationResult { is_success: false, logs }
    }
}

fn validate_ep1_mod2(code_inputs: Vec<String>) -> ValidationResult {
    let mut logs = vec![("system".to_string(), "> cargo run".to_string())];
    
    if code_inputs.len() < 3 {
        logs.push(("error".to_string(), "Missing system variable inputs.".to_string()));
        return ValidationResult { is_success: false, logs };
    }
    
    let ls_code = code_inputs[0].replace(" ", "");
    let rs_code = code_inputs[1].replace(" ", "");
    let sn_code = code_inputs[2].replace(" ", "");
    
    let ls_is_mut = ls_code == "mut" || ls_code == "mut\t";
    let rs_is_mut = rs_code == "mut" || rs_code == "mut\t";
    let sn_is_mut = sn_code == "mut" || sn_code == "mut\t";

    if ls_is_mut && rs_is_mut && !sn_is_mut {
         logs.push(("system".to_string(), "Compiling rustacean-chronicles v0.1.0".to_string()));
         logs.push(("success".to_string(), "Current Oxygen Level: 100%".to_string()));
         logs.push(("success".to_string(), "Reactor status: Online".to_string()));
         logs.push(("success".to_string(), "Ship Name: USS Rustacean".to_string()));
         logs.push(("system".to_string(), "Diagnostics passed! Life support stabilized.".to_string()));
         ValidationResult { is_success: true, logs }
    } else {
         if !ls_is_mut {
             logs.push(("error".to_string(), "error[E0384]: cannot assign twice to immutable variable `life_support_level`\nhelp: consider making this binding mutable: `mut life_support_level`".to_string()));
         }
         if !rs_is_mut {
             logs.push(("error".to_string(), "error[E0384]: cannot assign twice to immutable variable `reactor_status`\nhelp: consider making this binding mutable: `mut reactor_status`".to_string()));
         }
         if sn_is_mut {
             logs.push(("warning".to_string(), "warning: variable does not need to be mutable\n  --> src/main.rs:4:9\n   |\n 4 |     let mut ship_name = \"USS Rustacean\";\n   |         ----^^^^^^^^^\n   |         |\n   |         help: remove this `mut`".to_string()));
             logs.push(("error".to_string(), "Diagnostics failed! Please resolve all warnings/errors. (Hint: The ship name shouldn't change!)".to_string()));
         }
         ValidationResult { is_success: false, logs }
    }
}
