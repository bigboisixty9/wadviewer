type FileData = (String, Vec<u8>);

// wasm

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::{window, Url, File, HtmlInputElement, FileReader};
#[cfg(target_arch = "wasm32")]
use js_sys::{Uint8Array, Array, ArrayBuffer};


#[cfg(target_arch = "wasm32")]
pub struct FileDialog {
    tx: std::sync::mpsc::Sender<FileData>,
    rx: std::sync::mpsc::Receiver<FileData>,
    input: HtmlInputElement,
    closure: Option<Closure<dyn FnMut()>>,
}

#[cfg(target_arch = "wasm32")]
impl Default for FileDialog {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        let document = window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        let input = document.create_element("input").unwrap().dyn_into::<HtmlInputElement>().unwrap();
        input.set_attribute("type", "file").unwrap();
        input.style().set_property("display", "none").unwrap();
        body.append_child(&input).unwrap();

        Self {
            rx,
            tx,
            input,
            closure: None,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl Drop for FileDialog {
    fn drop(&mut self) {
        self.input.remove();
        if self.closure.is_some() {
            std::mem::replace(&mut self.closure, None).unwrap().forget();
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl FileDialog {
    pub fn open(&mut self) {
        if let Some(closure) = &self.closure {
            self.input.remove_event_listener_with_callback("change", closure.as_ref().unchecked_ref()).unwrap();
            std::mem::replace(&mut self.closure, None).unwrap().forget();
        }

        let tx = self.tx.clone();
        let input_clone = self.input.clone();

        let closure = Closure::once(move || {
            if let Some(file) = input_clone.files().and_then(|files| files.get(0)) {
                let reader = FileReader::new().unwrap();
                let reader_clone = reader.clone();
                let name = file.name();
                let onload_closure = Closure::once(Box::new(move || {
                    let array_buffer = reader_clone.result().unwrap().dyn_into::<ArrayBuffer>().unwrap();
                    let buffer = Uint8Array::new(&array_buffer).to_vec();
                    let file_data = (name, buffer);
                    tx.send(file_data).ok();
                }));

                reader.set_onload(Some(onload_closure.as_ref().unchecked_ref()));
                reader.read_as_array_buffer(&file).unwrap();
                onload_closure.forget();
            }
        });

        self.input.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref()).unwrap();
        self.closure = Some(closure);
        self.input.click();
    }

    pub fn get(&self) -> Option<(String, Vec<u8>)> {
        if let Ok(file_data) = self.rx.try_recv() {
            Some(file_data)
        } else {
            None
        }
    }

    pub fn save(&self, filename: &str, filedata: Vec<u8>) {
        let array = Uint8Array::from(filedata.as_slice());
        let blob_parts = Array::new();
        blob_parts.push(&array.buffer());

        let file = File::new_with_blob_sequence_and_options(
            &blob_parts.into(),
            filename,
            web_sys::FilePropertyBag::new().type_("application/octet-stream")
        ).unwrap();
        let url = Url::create_object_url_with_blob(&file).unwrap();
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                let event = web_sys::Event::new("click").unwrap();
                let window: &web_sys::Window = window.dyn_ref().unwrap();
                let _result = window.open_with_url_and_target(&url, "_blank");
                Url::revoke_object_url(&url).unwrap();
            }
        }
    }
}

