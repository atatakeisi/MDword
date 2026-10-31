//! Fonts the user added in the browser (#582), kept in IndexedDB (`localStorage` holds about
//! 5 MB, less than one CJK font) so they are there again after a reload. Every failure is logged
//! and skipped: a font that can't be stored still works for the session.

use wasm_bindgen::JsCast as _;
use wasm_bindgen::JsValue;
use wasm_bindgen::closure::Closure;
use web_sys::{IdbDatabase, IdbRequest, IdbTransactionMode};

const DB_NAME: &str = "wordcraft";
const STORE: &str = "fonts";
const EXTRA_FONTS: &[&str] = &[
    "ShipporiMincho-Regular.ttf",
    "ZenMaruGothic-Regular.ttf",
    "MPLUSRounded1c-Regular.ttf",
    "KleeOne-Regular.ttf",
    "YujiSyuku-Regular.ttf",
    "HachiMaruPop-Regular.ttf",
    "SawarabiMincho-Regular.ttf",
    "SawarabiGothic-Regular.ttf",
    "ZenOldMincho-Regular.ttf",
];

async fn fetch_font(name: &str) -> Result<Vec<u8>, String> {
    let window = web_sys::window().ok_or("no window")?;
    let response = wasm_bindgen_futures::JsFuture::from(window.fetch_with_str(&format!("./fonts/{name}"))).await.map_err(js)?;
    if js_sys::Reflect::get(&response, &"ok".into()).map_err(js)?.as_bool() != Some(true) {
        return Err(format!("fetch failed: {name}"));
    }
    let method: js_sys::Function = js_sys::Reflect::get(&response, &"arrayBuffer".into()).map_err(js)?.dyn_into().map_err(js)?;
    let promise: js_sys::Promise = method.call0(&response).map_err(js)?.dyn_into().map_err(js)?;
    let data = wasm_bindgen_futures::JsFuture::from(promise).await.map_err(js)?;
    let bytes = js_sys::Uint8Array::new(&data);
    if bytes.length() as usize > wordcraft_fonts::MAX_FONT_FILE_BYTES {
        return Err(format!("font too large: {name}"));
    }
    Ok(bytes.to_vec())
}

fn js(e: JsValue) -> String {
    format!("{e:?}")
}

/// Wait for an IndexedDB request; its result.
async fn done(req: &IdbRequest) -> Result<JsValue, String> {
    let promise = js_sys::Promise::new(&mut |resolve: js_sys::Function, reject: js_sys::Function| {
        let ok_req = req.clone();
        let ok = Closure::once_into_js(move || {
            let _ = resolve.call1(&JsValue::NULL, &ok_req.result().unwrap_or(JsValue::UNDEFINED));
        });
        let err = Closure::once_into_js(move || {
            let _ = reject.call1(&JsValue::NULL, &JsValue::from_str("IndexedDB request failed"));
        });
        req.set_onsuccess(Some(ok.unchecked_ref()));
        req.set_onerror(Some(err.unchecked_ref()));
    });
    wasm_bindgen_futures::JsFuture::from(promise).await.map_err(js)
}

/// The database, with its font store made on first use.
async fn open() -> Result<IdbDatabase, String> {
    let factory = web_sys::window().ok_or("no window")?.indexed_db().map_err(js)?.ok_or("IndexedDB is unavailable")?;
    let req = factory.open_with_u32(DB_NAME, 1).map_err(js)?;
    let upgrade_req = req.clone();
    let upgrade = Closure::<dyn FnMut()>::new(move || {
        if let Ok(db) = upgrade_req.result().and_then(|r| r.dyn_into::<IdbDatabase>())
            && !db.object_store_names().contains(STORE)
            && let Err(e) = db.create_object_store(STORE)
        {
            log::error!("couldn't make the font store: {e:?}");
        }
    });
    req.set_onupgradeneeded(Some(upgrade.as_ref().unchecked_ref()));
    let db = done(&req).await;
    req.set_onupgradeneeded(None);
    drop(upgrade);
    db?.dyn_into::<IdbDatabase>().map_err(js)
}

async fn store_async(name: String, bytes: Vec<u8>) -> Result<(), String> {
    let db = open().await?;
    let store = db.transaction_with_str_and_mode(STORE, IdbTransactionMode::Readwrite).map_err(js)?.object_store(STORE).map_err(js)?;
    let req = store.put_with_key(&js_sys::Uint8Array::from(bytes.as_slice()), &JsValue::from_str(&name)).map_err(js)?;
    done(&req).await.map(|_| ())
}

async fn forget_async(name: String) -> Result<(), String> {
    let db = open().await?;
    let store = db.transaction_with_str_and_mode(STORE, IdbTransactionMode::Readwrite).map_err(js)?.object_store(STORE).map_err(js)?;
    let req = store.delete(&JsValue::from_str(&name)).map_err(js)?;
    done(&req).await.map(|_| ())
}

/// Keep `bytes` (a font file as the user gave it) under `name` (its stored file name).
pub fn store(name: &str, bytes: &[u8]) {
    let (name, bytes) = (name.to_string(), bytes.to_vec());
    wasm_bindgen_futures::spawn_local(async move {
        if let Err(e) = store_async(name.clone(), bytes).await {
            log::error!("couldn't keep the font {name} for the next visit: {e}");
        }
    });
}

/// Forget the font stored under `name`.
pub fn forget(name: &str) {
    let name = name.to_string();
    wasm_bindgen_futures::spawn_local(async move {
        if let Err(e) = forget_async(name.clone()).await {
            log::error!("couldn't forget the stored font {name}: {e}");
        }
    });
}

/// Add every stored font to the font database (each checked like a new one; one that fails is
/// logged and skipped), then repaint: the app lays out again when fonts change.
pub fn load_all(ctx: egui::Context) {
    wasm_bindgen_futures::spawn_local(async move {
        let mut loaded = std::collections::HashSet::new();
        let r: Result<(), String> = async {
            let db = open().await?;
            let store = db.transaction_with_str(STORE).map_err(js)?.object_store(STORE).map_err(js)?;
            let keys = done(&store.get_all_keys().map_err(js)?).await?;
            let values = done(&store.get_all().map_err(js)?).await?;
            let (keys, values) = (js_sys::Array::from(&keys), js_sys::Array::from(&values));
            for i in 0..keys.length().min(values.length()) {
                let Some(name) = keys.get(i).as_string() else { continue };
                let Ok(bytes) = values.get(i).dyn_into::<js_sys::Uint8Array>() else { continue };
                if bytes.length() as usize > wordcraft_fonts::MAX_FONT_FILE_BYTES {
                    log::warn!("stored font {name} is too large; skipped");
                    continue;
                }
                match wordcraft_fonts::FontDb::global().install_font(&name, &bytes.to_vec()) {
                    Ok(_) | Err(wordcraft_fonts::UserFontError::AlreadyAvailable(_)) => {
                        loaded.insert(name);
                    }
                    Err(e) => log::warn!("stored font {name} skipped: {e}"),
                }
            }
            Ok(())
        }
        .await;
        if let Err(e) = r {
            log::warn!("no stored fonts: {e}");
        }
        ctx.request_repaint();
        for &name in EXTRA_FONTS {
            if loaded.contains(name) {
                continue;
            }
            match fetch_font(name).await {
                Ok(bytes) => match wordcraft_fonts::FontDb::global().install_font(name, &bytes) {
                    Ok(_) => {
                        ctx.request_repaint();
                        if let Err(e) = store_async(name.to_owned(), bytes).await {
                            log::warn!("couldn't cache {name}: {e}");
                        }
                    }
                    Err(e) => log::warn!("font {name} skipped: {e}"),
                },
                Err(e) => log::warn!("couldn't fetch {name}: {e}"),
            }
        }
    });
}
