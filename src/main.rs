use chrono::{DateTime, Local};
use leptos::prelude::*;
use reactive_stores::Store;
use reactive_stores_macro::Store;
use serde::{Deserialize, Serialize};
use web_sys::MouseEvent;

fn main() {
  console_log::init_with_level(log::Level::Debug).unwrap_or_default();
  console_error_panic_hook::set_once();

  mount_to_body(App);
}

#[derive(Store, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct State {
  #[store(key: DateTime<Local> = |note| note.date)]
  pub notes: Vec<Note>,
}

#[derive(Store, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Note {
  pub title: String,
  pub content: String,
  pub date: DateTime<Local>,
}

#[component]
fn App() -> impl IntoView {
  let state = Store::new(State::default());
  let selected_note = Store::new(<Option<Note>>::None);
  let add_notes = move |_| {
    let new_note =
      Note { date: Local::now(), title: "Title".to_string(), content: "Content".to_string() };
    state.notes().update(|it| it.insert(0, new_note.clone()));
    selected_note.set(Some(new_note));
  };
  let delete_note = move |child: Note| {
    move |event: MouseEvent| {
      event.stop_propagation();
      match state.notes().get().as_slice() {
        [_single_note] => selected_note.set(None),
        [.., before_last_note, last_note] if last_note.date == child.date => {
          selected_note.set(Some(before_last_note.to_owned()))
        }
        _ => {
          selected_note.set(
            state
              .notes()
              .get()
              .windows(2)
              .find(|window| window[0].date == child.date)
              .map(|window| window[1].to_owned()),
          );
        }
      }
      state.notes().update(|it| it.retain(|item| item.date != child.date));
    }
  };

  view! {
    <div id="main-div">
      <div id="list">
        <button class="add-btn" on:click=add_notes>
          "ADD NEW NOTE"
        </button>
        <ul>
          <For each=move || state.notes() key=move |note| note.date().get() let:child>
            <li
              class="note-item new-item"
              class:selected=move || {
                  selected_note.get().as_ref().is_some_and(|it| {
                     it.date == child.get().date // 🔴 Error happens here when getting child after removing a note
                   })
              }
              on:click=move |_| selected_note.set(Some(child.get()))
            >
              <div class="items">
                <div class="title">{move || child.title().get()}</div>
                <div class="content">{move || child.content().get()}</div>
              </div>
              <button on:click=delete_note(child.get()) class="fa fa-trash delete-button"></button>
            </li>
          </For>
        </ul>
      </div>
    </div>
  }
}
