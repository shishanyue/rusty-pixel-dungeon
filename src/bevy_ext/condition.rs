use bevy::prelude::*;
use std::hash::Hash;

pub fn pressed_button<T>(code: T) -> impl FnMut(Res<ButtonInput<T>>) -> bool + Clone
where
    T: Copy + Eq + Hash + Send + Sync + 'static,
{
    move |input: Res<ButtonInput<T>>| input.just_pressed(code)
}

pub fn in_state<S: States>(state: S) -> impl FnMut(Option<Res<State<S>>>) -> bool + Clone {
    move |current_state: Option<Res<State<S>>>| match current_state {
        Some(current_state) => *current_state == state,
        None => {
            warn_once!("No state matching the type for {} exists - did you forget to `add_state` when initializing the app?", {
                    let debug_state = format!("{state:?}");
                    let result = debug_state
                        .split("::")
                        .next()
                        .unwrap_or("Unknown State Type");
                    result.to_string()
                });

            false
        }
    }
}
pub fn run_once_in_state<S: States>(state: S) -> impl FnMut(Option<Res<State<S>>>) -> bool + Clone {
    let mut has_run = false;
    move |current_state: Option<Res<State<S>>>| match current_state {
        Some(current_state) => {
            if *current_state == state {
                if !has_run {
                    has_run = true;
                    true
                } else {
                    false
                }
            }else {
                false
            }
        }
        None => {
            warn_once!("No state matching the type for {} exists - did you forget to `add_state` when initializing the app?", {
                    let debug_state = format!("{state:?}");
                    let result = debug_state
                        .split("::")
                        .next()
                        .unwrap_or("Unknown State Type");
                    result.to_string()
                });

            false
        }
    }
}
