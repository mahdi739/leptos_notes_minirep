use chrono::{DateTime, Local, NaiveDateTime, Utc};
use leptos::{leptos_dom::logging::console_log, *};
use leptos_use::{storage::use_local_storage, utils::JsonCodec};
use serde::{Deserialize, Serialize};
use web_sys::MouseEvent;
fn main() {
  console_log::init_with_level(log::Level::Debug).unwrap_or_default();
  console_error_panic_hook::set_once();

  leptos::mount_to_body(App);
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Note {
  pub title: RwSignal<String>,
  pub content: RwSignal<String>,
  pub date: DateTime<Local>,
}

pub struct AppModel {}
impl AppModel {}
#[component]
fn App() -> impl IntoView {
  // let (notes, set_notes) = create_signal(
  //   web_sys::window()
  //     .unwrap()
  //     .local_storage()
  //     .unwrap()
  //     .unwrap()
  //     .get("notes")
  //     .unwrap()
  //     .map(|res| serde_json::from_str(&res).unwrap())
  //     .unwrap_or(Vec::<Note>::new()),
  // );
  // create_effect(move |_| {
  //   web_sys::window().unwrap().local_storage().unwrap().unwrap().set("notes", serde_json::to_string(&notes.get()).unwrap().as_str()).unwrap();
  // });

  let (notes, set_notes, _) = use_local_storage::<Vec<Note>, JsonCodec>("notes");
  let (selected_note, set_selected_note) = create_signal(<Option<Note>>::None);
  if notes.with(|f| f.is_empty() == false) {
    set_selected_note.set(Some(notes.with(|f| f[0])));
  }
  let add_notes = move |_: MouseEvent| {
    set_notes.update(|f| {
      f.insert(0, Note { date: Local::now(), title: create_rw_signal("Title".to_string()), content: create_rw_signal("Content".to_string()) })
    });
    set_selected_note.set(Some(notes.with(|f| f[0])));
  };

  let delete_note = move |child: Note| {
    move |event: MouseEvent| {
      event.stop_propagation();
      if notes.with(|f| f.len() == 1) {
        set_selected_note.set(None);
      } else if notes.with(|f| f.last().unwrap().date != child.date) {
        set_selected_note.set(Some(notes.with(|f| {
          let pos = f.iter().position(|p| p.date == child.date).unwrap();
          f[pos + 1]
        })));
      } else {
        set_selected_note.set(Some(notes.with(|f| {
          let pos = f.iter().position(|p| p.date == child.date).unwrap();
          f[pos - 1]
        })));
      }
      set_notes.update(|f| f.retain(|item| item.date != child.date));
    }
  };
  let update_selected_note_title = move |event| {
    batch(|| {
      set_notes.update(|notes| {
        notes.iter().filter(|note| note.date == selected_note.get().unwrap().date).next().unwrap().title.set(event_target_value(&event));
      }); // For Updating the local storage
    });
    selected_note.get().unwrap().title.set(event_target_value(&event)) // For Updating the selected item in the list
  };
  let update_selected_note_content = move |event| {
    batch(|| {
      set_notes.update(|notes| {
        notes.iter().filter(|note| note.date == selected_note.get().unwrap().date).next().unwrap().content.set(event_target_value(&event));
      }); // For Updating the local storage
    });
    selected_note.get().unwrap().content.set(event_target_value(&event)) // For Updating the selected item in the list
  };

  view! {
    <div id="main-div">
      <div id="list">
        <button class="add-btn" on:click=add_notes>
          "ADD NEW NOTE"
        </button>
        <ul>
          <For each=move || notes.get() key=move |note| note.date let:child>
            <li
              class="note-item new-item"
              class:selected=move || { selected_note.with(|f| f.is_some() && f.unwrap().date == child.date) }
              on:click=move |_| set_selected_note.set(Some(child))
            >
              <div class="items">
                <div class="title">{child.title}</div>
                <div class="content">{child.content}</div>
              </div>
              <button on:click=delete_note(child) class="fa fa-trash delete-button"></button>
            </li>
          </For>
        </ul>
      </div>
      <div id="editor">
        <Show
          when=move || selected_note.with(|f| f.is_some())

          fallback=|| {
              view! { <div style="margin:auto; font-size:21px;">"Pick a note"</div> }
          }
        >

          <textarea id="title-editor" rows="2" prop:value=selected_note.with(|f| f.unwrap().title) on:input=update_selected_note_title></textarea>
          <textarea id="content-editor" prop:value=selected_note.with(|f| f.unwrap().content) on:input=update_selected_note_content></textarea>
        </Show>
      </div>
    </div>
  }
}
