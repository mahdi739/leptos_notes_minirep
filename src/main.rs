use chrono::{Local, NaiveDateTime};
use leptos::*;
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};
fn main() {
    console_log::init_with_level(log::Level::Debug).unwrap_or_default();
    console_error_panic_hook::set_once();

    leptos::mount_to_body(move || {
        view! {  <App/> }
    });
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Note {
    pub title: RwSignal<String>,
    pub content: RwSignal<String>,
    pub date: NaiveDateTime,
}

#[component]
fn App() -> impl IntoView {
    let (notes, set_notes, _) = use_local_storage("notes", Vec::<Note>::new());
    let (selected_note, set_selected_note) = create_signal(<Option<Note>>::None);
    if notes.with(|f| f.is_empty() == false) {
        set_selected_note.set(Some(notes.with(|f| f[0])));
    }
    view! {
      <div id="main-div">
        <div id="list">
            <button class="add-btn"
             on:click=move|_|{
              set_notes.update(|f|f.insert(
                0,
                Note {
                  date: Local::now().naive_local(),
                  title: create_rw_signal("Title".to_string()),
                  content: create_rw_signal("Content".to_string())
                }
              ));
              set_selected_note.set(Some(notes.with(|f|f[0])));
            }>ADD NEW NOTE</button>
            <ul>
            <For each=notes key=move |note|note.date let:child>
            <li class="note-item new-item"  class:selected=move||selected_note.with(|f|f.is_some()&&f.unwrap().date==child.date)
            on:click=move |_|set_selected_note.set(Some(child))>
                    <div class="items">
                        <div class="title">{child.title}</div>
                        <div class="content">{child.content}</div>
                    </div>
                    <button on:click=move|event|{
                      event.stop_propagation();
                      if notes.with(|f|f.len()==1){
                        set_selected_note.set(None);
                      }
                      else if notes.with(|f|f.last().unwrap().date!=child.date) {
                        set_selected_note.set(Some(notes.with(|f|{
                          let pos=f.iter().position(|p|p.date==child.date).unwrap();
                          f[pos+1]})));
                        }
                        else {
                        set_selected_note.set(Some(notes.with(|f|{
                          let pos=f.iter().position(|p|p.date==child.date).unwrap();
                          f[pos-1]})));
                      }
                      set_notes.update(|f|f.retain(|item|item.date!=child.date));
                } class="fa fa-trash delete-button"></button>
                    </li>
            </For>
            </ul>
        </div>
        <div id="editor">
        <Show when= move||selected_note.with(|f|f.is_some()) fallback=|| view! { <div style="margin:auto; font-size:21px;">"Pick a note"</div> }>
          <textarea id="title-editor" rows="2" prop:value= selected_note.with(|f|f.unwrap().title)
           on:input=move |event| selected_note.get().unwrap().title.set(event_target_value(&event))></textarea>
          <textarea id="content-editor" prop:value= selected_note.with(|f|f.unwrap().content)
            on:input=move |event| selected_note.get().unwrap().content.set(event_target_value(&event))></textarea>
        </Show>
        </div>
    </div>
    }
}

// trait Take {
//   fn take(&self, count: usize) -> String;
// }

// impl Take for RwSignal<String> {
//   fn take(&self, count: usize) -> String {
//     self.with(|v| (&v[..count]).to_owned())
//   }
// }

// trait Then<T> {
//     fn then(self, operation: impl Fn(&T) -> T) -> impl Fn() -> T;
// }

// impl<T> Then<T> for RwSignal<T> {
//     fn then(self, operation: impl Fn(&T) -> T) -> impl Fn() -> T {
//         move || self.with(|f| operation(f))
//     }
// }

// impl<T> Then<T> for T
// where
//     T: 'static,
// {
//     fn then(self, operation: impl Fn(&T) -> T) -> impl Fn() -> T {
//         move || operation(&self)
//     }
// }
