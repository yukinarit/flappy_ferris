use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use futures::Async;
use quicksilver::{graphics::Image, Error as QuicksilverError, Future, Result};

pub struct AssetLoader {
    assets: HashMap<String, RefCell<Asset<Rc<Image>>>>,
    loaded: bool,
}

impl AssetLoader {
    pub fn new() -> Self {
        AssetLoader {
            assets: HashMap::new(),
            loaded: false,
        }
    }

    pub fn load(&mut self, path: String) {
        self.assets.insert(
            path.clone(),
            RefCell::new(Asset::new(Image::load(path).map(|img| Rc::new(img)))),
        );
        self.loaded = false;
    }

    pub fn loaded(&self) -> bool {
        self.loaded
    }

    pub fn update(&mut self) -> bool {
        if self.loaded() {
            return true;
        }

        let mut loaded = true;

        for (_, v) in &self.assets {
            if let Ok(mut asset) = v.try_borrow_mut() {
                asset.execute(|_| Ok(())).unwrap();

                // Not yet ready.
                if asset.deref().get().is_none() {
                    loaded = false;
                }
            }
        }

        self.loaded = loaded;
        self.loaded()
    }

    pub fn get(&self, path: &str) -> Option<Rc<Image>> {
        self.assets.get(path).and_then(|found| {
            if let Ok(asset) = found.try_borrow() {
                asset.deref().get()
            } else {
                None
            }
        })
    }
}

/// A structure to manage the loading and use of a future
pub struct Asset<T>(AssetData<T>);

enum AssetData<T> {
    Loading(Box<dyn Future<Item = T, Error = QuicksilverError>>),
    Loaded(T),
}

impl<T> Asset<T> {
    /// Create a new asset from a future
    pub fn new(future: impl Future<Item = T, Error = QuicksilverError> + 'static) -> Asset<T> {
        Asset(AssetData::Loading(Box::new(future)))
    }

    /// Run a function if the loading is complete
    pub fn execute(&mut self, loaded: impl FnOnce(&mut T) -> Result<()>) -> Result<()> {
        self.execute_or(loaded, || Ok(()))
    }

    /// Run a function if the loading is complete, or a different function if it isn't
    pub fn execute_or(
        &mut self,
        loaded: impl FnOnce(&mut T) -> Result<()>,
        loading: impl FnOnce() -> Result<()>,
    ) -> Result<()> {
        let result = match self.0 {
            AssetData::Loading(ref mut asset) => asset.poll()?,
            _ => Async::NotReady,
        };
        if let Async::Ready(asset) = result {
            self.0 = AssetData::Loaded(asset);
        }
        match self.0 {
            AssetData::Loading(_) => loading(),
            AssetData::Loaded(ref mut data) => loaded(data),
        }
    }

    pub fn get(&self) -> Option<T>
    where
        T: Clone,
    {
        match self.0 {
            AssetData::Loading(_) => None,
            AssetData::Loaded(ref data) => Some(data.clone()),
        }
    }
}
